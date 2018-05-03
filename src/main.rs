extern crate gpio;

use gpio::GpioOut;
use std::net::UdpSocket;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (sender, receiver) = channel();

    let artnet_thread = thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:6454").unwrap();

        loop {
            let mut buf = [0; 530];
            let _ = socket.recv_from(&mut buf).unwrap();

            // let universe: u16 = (buf[14] as u16 * 256) + buf[15] as u16;
            let length = (buf[16] as u16 * 256) + buf[17] as u16;

            let slice_end = 18 + length as usize;
            let data = buf[18..slice_end].to_vec();

            sender.send(data).unwrap();
        }
    });

    let gpio_thread = thread::spawn(move || {
        let mut led = gpio::sysfs::SysFsGpioOutput::open(7).unwrap();
        let mut relais_1 = gpio::sysfs::SysFsGpioOutput::open(2).unwrap();
        let mut relais_2 = gpio::sysfs::SysFsGpioOutput::open(3).unwrap();*/
        loop {
            let data = receiver.recv().unwrap();

            if data.len() >= 2 {
                led.set_value(data[0] > 0).unwrap();
                relais_1.set_value(data[0] > 0).unwrap();
                relais_2.set_value(data[0] > 0).unwrap();
            }
        }
    });

    artnet_thread.join().unwrap();
    gpio_thread.join().unwrap();
}
