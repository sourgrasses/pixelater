#[macro_use]
extern crate helix;
extern crate image;
extern crate rayon;

mod piximage;

use crate::piximage::PixelatedImage;

use helix::ruby;

use std::fs::File;

ruby! {
    class PixelSelection {
        struct {
            x: u32,
            y: u32,
            w: u32,
            h: u32,
            image: Option<PixelatedImage>,
        }

        def initialize(helix) {
            PixelSelection {
                helix,
                x: 0,
                y: 0,
                w: 0,
                h: 0,
                image: None,
            }
        }

        def set_start(&mut self, x: u32, y: u32) {
            self.x = x;
            self.y = y;
        }

        def set_size(&mut self, x: u32, y: u32) {
            self.w = x - self.x;
            self.h = y - self.y;
        }

        def pixelate(&mut self, filename: String, pix_level: u32) {
            let mut image = PixelatedImage::new(&filename, self.x, self.y, self.w, self.h, pix_level as u8).unwrap();
            image.pixelate();
            self.image = Some(image);

            if let Some(ref image) = self.image.clone() {
                let _ = File::create("tmp.jpg").unwrap();
                image.image.save("tmp.jpg").unwrap();
            } else {
                eprintln!("Error writing tmp.jpg");
            };
        }

        def save(&self, filename: String) {
            if let Some(ref image) = self.image {
                let _ = File::create(&filename).unwrap();
                image.image.save(&filename).unwrap();
            } else {
                eprintln!("Error writing {}", filename);
            };
        }
    }
}
