// mod main;
//
// use rdev::{simulate, Button, EventType, Key, SimulateError};
// use std::{thread, time};
// use std::time::Instant;
// use xcap::Monitor;
//
// fn main() {
//     println!("Hello, world!");
//
//     let monitors = Monitor::all().unwrap();
//     let start = Instant::now();
//     for monitor in monitors {
//         let image = monitor.capture_image().unwrap();
//
//         image
//             .save(format!("target/monitor-{}.png", normalized(monitor.name())))
//             .unwrap();
//     }
//
//     println!("运行耗时: {:?}", start.elapsed());
//
// }
//
//
//
// fn send(event_type: &EventType) {
//     let delay = time::Duration::from_millis(20);
//     match simulate(event_type) {
//         Ok(()) => (),
//         Err(SimulateError) => {
//             println!("We could not send {:?}", event_type);
//         }
//     }
//     // Let ths OS catchup (at least MacOS)
//     thread::sleep(delay);
// }
//
// fn normalized(filename: &str) -> String {
//     filename
//         .replace("|", "")
//         .replace("\\", "")
//         .replace(":", "")
//         .replace("/", "")
// }