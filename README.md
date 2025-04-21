# FaceID
## _Facial recognition | HTTP-server | Rust => Blazingly fast and secure data-storage🦀⚡_
Download most recent version: 

[![some]][link]

[some]: https://img.shields.io/badge/current_stable_release-1.0-blue
[link]: https://github.com/mejerfromperu/faceID/releases/download/ver1.0/SimpleHttpServer.exe

---
## Pre-requisites
To run project, first run on an x86-based Windows PC, as Rust-code expects the platform to be this platform. 

Next - connect the HUSKYLENS to an Arudino Uno (this project uses an R3-unit) using I2C-protocol. Connection-schema and settings to use on the HUSKYLENS are found [here](https://wiki.dfrobot.com/HUSKYLENS_V1.0_SKU_SEN0305_SEN0336#target_30)


![plot](schema.png)

Make sure to note which COM-port the Arduino is connected within the Arduino IDE, as this is important for which port the Rust-server attempts to read from. 

When the Arduino is connected to the HUSKYLENS, attempt to flash the Arduino with the code within `SimpleHttpServer/src/arduino_code/arduino_code.ino`
If it goes well, close the IDE as the port will be blocked and cannot be accessed from Rust. If you note a different COM-port than 4, edit this within the main-function in `SimpleHttpServer/src/main.rs` 

---
If you are totally sure your Arduino is connected to COM4, you can just run SimpleHttpServer.exe from releases or from the button topside. 
If you'd rather run the server yourself, either open SimpleHttpServer in your Rust-IDE of choice and build+run or open a new terminal and type `cargo run` to run in debug-mode or `cargo run -r` to run in release-mode

Regardless of the way you choose to run the project, it panics and exits if no connection is made to the right port. 

If things go smooth, however, HTTP-server is run on 127.0.0.1:8080 and you can go to root to either access data if you get permission, or get blocked, if you do not have permission




