use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::executor::block_on;
use rand::Rng;
use serial::SerialPort;
use std::future::Future;
use std::io::Read;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::thread::sleep;
use std::time::Duration;
use std::{io, thread};

struct AppState {
    num: AtomicI32,
    may_access: AtomicBool,
    measurements: Vec<WaterReading>,
}

#[derive(Debug)]
struct WaterReading {
    height: u16,
    temperature: i8,
}

#[get("/")]
async fn hello(mut data: web::Data<AppState>) -> impl Responder {
    match &data.may_access.load(SeqCst) {
        true => {
            &data.may_access.store(false, SeqCst);

            HttpResponse::Ok().body(format!("{:?}", &data.measurements))

        }
        false => {
            &data.num.fetch_add(1, SeqCst);
            println!("{}", &data.num.load(SeqCst));

            HttpResponse::Ok().body(format!("Du er nu blevet afvist {} gange", &data.num.load(SeqCst)))
        }
    }
}

#[post("/echo")]
async fn echo(req_body: String, mut data: web::Data<AppState>) -> impl Responder {

    if req_body == "7fb0d68c-52af-4972-914a-fdeaebe1dcba" {
        &data.may_access.store(true, SeqCst);
        return HttpResponse::Ok().body("You may now enter");
    }
    HttpResponse::NotAcceptable()
        .body("You may now leave, you know not the proper GUID, you insecure fraudster")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    thread::spawn(|| {
        sleep(Duration::from_secs(2));
        let mut port = serial_windows::COMPort::open("COM4").unwrap();
        interact(&mut port).unwrap();

        let mut buf;

        let mut i;

        loop {
            i = 0_usize;
            buf = [0_u8; 32];
            port.read(&mut buf).unwrap();
            let client = reqwest::blocking::Client::new();
            let mut req;
            loop {
                println!("{}", buf[i] as char);
                println!("{}", buf[i]);
                if i < 31 {
                    i += 1;
                } else {
                    if buf.contains(&98) {
                        req = client
                            .post("http://127.0.0.1:8080/echo")
                            .body("7fb0d68c-52af-4972-914a-fdeaebe1dcba")
                            .send()
                            .unwrap();
                        println!("{:?}", req.text().unwrap());
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
    });

    HttpServer::new(move || {
        App::new()
            .app_data(appdata.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud38400)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop2);
        settings.set_flow_control(serial::FlowNone);

        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(3000))?;

    Ok(())
}
