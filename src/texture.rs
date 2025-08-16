use std::sync::Arc;
use egui::{Color32, ColorImage};
use image::ImageReader;

#[derive(Clone, Debug)]
pub struct Texture {
    image: Arc<ColorImage>,
}
impl Texture {
    pub fn new(path: String) -> Result<Self, String> {
        let img = match ImageReader::open(&path) {
            Ok(reader) => reader.decode(),
            Err(e) => return Err(format!("Невозможно прочитать изображение: {}", e)),
        };

        let img = match img {
            Ok(img) => img,
            Err(e) => return Err(format!("Невозможно открыть изображение: {}", e)),
        };

        let img_rgba = img.to_rgba8();

        let size = [img_rgba.width() as usize, img_rgba.height() as usize];
        let color_image = ColorImage::from_rgba_unmultiplied(size, &img_rgba);

        Ok(Self {
            image: Arc::new(color_image),
        })
    }
    
    fn get_color(first_color: &Color32, second_color: &Color32, rs: f32) -> Color32 {
        Color32::from_rgb(
            (first_color[0] as f32 * (1.0 - rs) + second_color[0] as f32 * rs) as u8,
            (first_color[1] as f32 * (1.0 - rs) + second_color[1] as f32 * rs) as u8,
            (first_color[2] as f32 * (1.0 - rs) + second_color[2] as f32 * rs) as u8,
        )
    }
    
    fn update_borders(&self, x: f32, y: f32) -> (usize, usize) {
        let mut x = x as usize;
        let mut y = y as usize;
        
        if x >= self.image.width(){
            x = 0;
        }
        
        if y >= self.image.height(){
            y = 0;
        }

        (x, y)
    }
    
    pub fn bilinear_interpolation(&self, x: f32, y: f32) -> Color32{
        let (x_tex, y_tex) = (x * (self.image.width() as f32), y * (self.image.height() as f32));
        let (x_top, y_top) = (x_tex.ceil(), y_tex.ceil());
        let (x_bottom, y_bottom) = (x_tex.floor(), y_tex.floor());
        
        let mut rs = 0.0;
        if (x_top - x_bottom).abs() > f32::EPSILON {
            rs = (x_tex - x_bottom) / (x_top - x_bottom);
        }

        let mut rt = 0.0;
        if (y_top - y_bottom).abs() > f32::EPSILON {
            rt = (y_tex - y_bottom) / (y_top - y_bottom);
        }

        let (x_top_r, y_top_r) = self.update_borders(x_top, y_top);
        let (x_bottom_r, y_bottom_r) = self.update_borders(x_bottom, y_bottom);
        
        let color_top = Self::get_color(&self.image[(x_bottom_r, y_top_r)], &self.image[(x_top_r, y_top_r)], rs);
        let color_bottom = Self::get_color(&self.image[(x_bottom_r, y_bottom_r)], &self.image[(x_top_r, y_bottom_r)], rs);
        
        Self::get_color(&color_bottom, &color_top, rt)
    }

    pub fn sample(&self, x: f32, y: f32) -> Color32 {
        let mut x = x;
        let mut y = y;
        
        if x > 1.0 {
            x = x.fract();
        }
       
        if  y > 1.0 {
            y = y.fract();
        }
        
        if x < 0.0{
            x = x.fract() + 1.0;
        }
        
        if y < 0.0{
            y = y.fract() + 1.0;
        }
        
        self.bilinear_interpolation(x, y)
    }
}
