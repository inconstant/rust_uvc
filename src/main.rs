extern crate rust_uvc;
extern crate libc;
extern crate rscam;

use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use rscam::{Camera, Config, ResolutionInfo};

fn main() {
    let mut camera = Camera::new("/dev/video0").unwrap();

    camera.start(&Config {
        interval: (1, 30),      // 30 fps.
        resolution: (1280, 720),
        format: b"MJPG",
        ..Default::default()
    }).unwrap();

    for i in 0..10 {
        let frame = camera.capture().unwrap();
        let mut file = File::create(&format!("frame-{}.jpg", i)).unwrap();
        file.write_all(&frame[..]).unwrap();
        thread::sleep(Duration::new(1, 0));
    }
}
/*
fn main() {
    let camera = Camera::new("/dev/video0").unwrap();

    for wformat in camera.formats() {
        let format = wformat.unwrap();
        println!("{:?}", format);

        let resolutions = camera.resolutions(&format.format).unwrap();

        if let ResolutionInfo::Discretes(d) = resolutions {
            for resol in &d {
                println!("  {}x{}  {:?}", resol.0, resol.1,
                    camera.intervals(&format.format, *resol).unwrap());
            }
        } else {
            println!("  {:?}", resolutions);
        }
    }
}
*/
