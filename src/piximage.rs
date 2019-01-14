#[allow(unnecesary_parentheses)]

use image::{open, ImageError, Pixel, Rgb, RgbImage};

#[derive(Clone, Debug)]
pub struct PixelatedImage {
    pub image: RgbImage,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    pix_level: u8,
}

#[derive(Clone, Debug)]
struct BigPixel {
    color: Rgb<u8>,
    offset: (u32, u32),
    width: u32,
    height: u32,
}

impl PixelatedImage {
    pub fn new(filename: &str, x: u32, y: u32, w: u32, h: u32, pix_level: u8) -> Result<PixelatedImage, ImageError> {
        let image = match open(filename) {
            Ok(image) => image.to_rgb(),
            Err(e) => return Err(e),
        };

        let image = PixelatedImage {
            image: image,
            x: x,
            y: y,
            w: w,
            h: h,
            pix_level: pix_level,
        };

        Ok(image)
    }

    pub fn pixelate(&mut self) -> &mut Self {
        let mut big_pixels = Vec::new();
        let pix_width = (self.w / (self.pix_level * 10) as u32);
        let pix_height = (self.h / (self.pix_level * 10) as u32);

        let mut x = self.x;
        let mut y = self.y;
        while x < self.x + self.w {
            while y < self.y + self.h {
                let pix_ref = self.image.get_pixel_mut((x + pix_width / 2), (y + pix_height / 2));
                let big_pixel = BigPixel::new(pix_ref.to_rgb(), (x.clone(), y.clone()), pix_width.clone(), pix_height.clone());
                big_pixels.push(big_pixel);

                y += pix_height;
            }

            y = self.y;
            x += pix_width;
        }

        for pixel in big_pixels {
            let (mut x, mut y) = pixel.offset;
            let y_hold = y.clone();
            let x_end = x.clone() + pixel.width;
            let y_end = y.clone() + pixel.height;
            while x <= x_end {
                while y <= y_end {
                    self.image.put_pixel(x, y, pixel.color.clone());

                    y += 1;
                }
                
                y = y_hold;
                x += 1;
            }
        }

        self
    }
}

impl BigPixel {
    fn new(color: Rgb<u8>, offset: (u32, u32), width: u32, height: u32) -> BigPixel {
        BigPixel {
            color: color,
            offset: offset,
            width: width,
            height: height,
        }
    }
}
