extern crate scrap;
extern crate png;
extern crate base64;

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::{io, thread};
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use png::{ColorType, Encoder};

fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());
    let mut s = 0;

    loop {
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        println!("Captured! Saving...");

        // Flip the ARGB image into a BGRA image.
        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }

        // let mut buffers = Vec::new();
        // 调用 encode_png 函数，将数据写入缓冲区
        if let Err(e) = encode_png(File::create(format!("example_{}.png",s)).expect("file chant"), w as u32, h as u32, &bitflipped) {
            eprintln!("Failed to encode PNG: {}", e);
            continue;
        }

      //   // 将缓冲区转换为 Base64 字符串
      //   let encoded = base64::encode(&buffers);
      //
      //   // 打印结果
      // //  println!("Base64 Encoded: {}", encoded);
      //
      //   let mut file = File::create("example.txt").unwrap();
      //
      //   // 将字符串写入文件
      //   file.write_all(encoded.as_bytes()).expect("...");
      //
      //   // 写入完成后关闭文件
      //   file.flush();

        println!("{:?}", stride);
        s=s+1;
        if s >= 10 {
            break;
        }


    }
}

pub fn encode_png<W: Write>(mut sink: W, width: u32, height: u32, image: &[u8]) -> io::Result<()> {
    let mut encoder = Encoder::new(&mut sink, width, height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(image)?;
    writer.finish()?;
    Ok(())
}