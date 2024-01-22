use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use ratatui::buffer::Buffer;
use ratatui::prelude::{Color, Rect};
use ratatui::widgets::Widget;
use termimage::{AnsiOutputFormat, ops, Options};
use image::{DynamicImage, GenericImageView, Pixel, Rgb};


#[derive(Clone)]
pub struct ThumbnailWidget {
    pub last_size: Rect,
    pub img: Options,
    pub image: DynamicImage,
}




impl ThumbnailWidget {
    pub fn new(path_to_thumbnail: PathBuf) -> Self {
        let default = "C:\\Users\\nkh15448\\ratatuni\\thumbnails\\DEFAULT.jpg";
        Self {
            last_size: Rect::default(),
            img: Options{
                image: (default.to_string(), PathBuf::from(default)),
                size: (50, 30),
                preserve_aspect: true,
                ansi_out: Some(AnsiOutputFormat::Truecolor),
            },
            image: DynamicImage::default(),
        }
    }


    pub fn setup(&mut self, size:Rect) {
        if self.last_size.width == size.width && self.last_size.height == size.height {
            return;
        }
        let Rect { width, height, .. } = size.clone();
        self.last_size = size;
        let format = ops::guess_format(&self.img.image).unwrap();
        let imag = ops::load_image(&self.img.image, format).unwrap();
        let img_s = ops::image_resized_size(imag.dimensions(), (width as u32, ((height * 2) as u32)), self.img.preserve_aspect.clone());
        let resized = ops::resize_image(&imag, img_s);
        self.image = resized;
        // println!("finished setting up the image");
    }




}

impl Widget for ThumbnailWidget {
    fn render(self, area: Rect, buf: &mut Buffer){

        for (xi, x) in (area.left() .. area.right()).enumerate() {
            for (yi, y ) in (area.top().. (area.bottom())).enumerate(){
                if xi >= self.image.width() as usize || (yi*2 + 1) >= self.image.height() as usize {
                    continue;
                }
                // println!("x: {}, y: {}", xi, yi);

                // thread::sleep(Duration::from_millis(100));

                let color = self.image.get_pixel(xi as u32, (yi *2) as u32).to_rgb();
                let (r,g,b) = (*color.0.get(0).unwrap(), *color.0.get(1).unwrap(),   *color.0.get(2).unwrap());
                let c = Color::Rgb(r, g, b);
                let color = self.image.get_pixel(xi as u32, (yi * 2 + 1 ) as u32).to_rgb();
                let (r,g,b) = (*color.0.get(0).unwrap(), *color.0.get(1).unwrap(), *color.0.get(2).unwrap());
                let c2 = Color::Rgb(r,g,b);
                buf.get_mut(x as u16,y as u16).set_char('▀').set_fg(c).set_bg(c2);
            }
        }

    }
}