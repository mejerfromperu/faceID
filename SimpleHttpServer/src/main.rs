use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use serial::SerialPort;
use std::io::Read;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::thread::sleep;
use std::time::Duration;
use std::{io, thread};
use reqwest::{Error, blocking::Response};
use serial_windows::COMPort;
use uuid::{Uuid};


struct AppState {
    num: AtomicI32,
    may_access: AtomicBool,
    measurements: Vec<WaterReading>,
    guid: String,
}
#[allow(dead_code)]
#[derive(Debug)]
struct WaterReading {
    height: u16,
    temperature: i8,
}


#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    match data.may_access.load(SeqCst) {
        true => { // Hvis der er blevet sendt en valid POST-request på /echo vil denne være valid for første bruger.
            let _ = data.may_access.store(false, SeqCst);

            HttpResponse::Ok().body(format!("{:?}", &data.measurements)) // Using debug-mode here since vec! does not work with default Display

        }
        false => { // Der vil være en counter, som tæller op hver gang en klient forsøger at trænge ind på siden, selvom de ikke har fået ansigtsgodkendelse.
            data.num.fetch_add(1, SeqCst);
            println!("{}", &data.num.load(SeqCst));

            HttpResponse::Ok().body(format!("Du er nu blevet afvist {} gange", &data.num.load(SeqCst)))
        }
    }
}

#[post("/echo")]
async fn echo(req_body: String, data: web::Data<AppState>) -> impl Responder {

    if req_body.eq(&data.guid) { // Eget genererede GUID - bør måske genereres dynamisk for faktisk at være safe.
        data.may_access.store(true, SeqCst);
        return HttpResponse::Ok().body("You may now enter");
    }
    HttpResponse::NotAcceptable()
        .body("You may now leave, you know not the proper GUID, you insecure fraudster")
}



#[actix_web::main]
async fn main() -> io::Result<()> {

    let guid = Uuid::new_v4();
    let guid_clone = guid.clone();

    thread::spawn(move || {
        sleep(Duration::from_secs(2));
        let mut port :COMPort = match serial_windows::COMPort::open("COM4") {
            Ok(c) => c,
            Err(e) => {
                panic!("couldn't open COM4: {:?}", e);
            }
        };

        setup(&mut port);


        let mut buf;

        let mut i;

        loop {
            i = 0_usize;
            buf = [0_u8; 32];
            let _ = &port.read(&mut buf);
            let client = reqwest::blocking::Client::new();
            loop {
                println!("{}", buf[i] as char);
                println!("{}", buf[i]);
                if i < 31 {
                    i += 1;
                } else {
                    if buf.contains(&98) { // This is supposed to represent a valid character, in this case 'b'
                        let req :Result<Response, Error> = client
                            .post("http://127.0.0.1:8080/echo")
                            .body(guid_clone.to_string())
                            .send();
                        match req {
                            Ok(_) => {},
                            Err(_) => {println!("Something went wrong while trying to send request, perhaps server is not yet open.")},
                        }
                    }

                    break;
                }
            }

            sleep(Duration::from_millis(1000));
        }
    });

    let mut rng = rand::rng();
    let mut readings = Vec::with_capacity(1000);

    for _ in 0..1000 {
        let height = rng.random_range(0..=250);
        let temperature = rng.random_range(0..=22); // Random i8 value
        readings.push(WaterReading {
            height,
            temperature,
        });
    }
    let appdata = web::Data::new(AppState {
        num: AtomicI32::new(4),
        may_access: AtomicBool::new(false),
        measurements: readings,
        guid: guid.to_string(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(appdata.clone())
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn setup<T: SerialPort>(port: &mut T) {
    let config_attempt = port.reconfigure(&|settings| {
        match settings.set_baud_rate(serial::Baud38400) {
            Err(_) => {println!("Failed to set baudrate, application exiting");},
            _ => {}
        }
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop2);
        settings.set_flow_control(serial::FlowNone);
        Ok(())

    });
    match config_attempt {
        Err(_) => {panic!("Something went wrong while trying to setup serial port, application exiting");},
        _ => {}
    }


    match port.set_timeout(Duration::from_millis(3000)) {
        Err(_) => {panic!("Something went wrong while trying to interact with serial port, application exiting");}
        _ => {}
    }
}
