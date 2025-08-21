use std::path::Path;
use std::sync::Arc;
use egui::{Color32, ColorImage};
use image::{ImageBuffer, ImageReader, Rgba};
use nalgebra::{DMatrix};

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

    pub fn create_mipmaps(image: &ColorImage, path: &str){
        let dest_dir = std::env::var("MIP_MAPS_DIR").expect("Директория для сохранения не найдена");
        let path = Path::new(path);

        let file_name = path.file_name().unwrap().to_str().unwrap().split('.').collect::<Vec<&str>>()[0];
        let mut mip_level = 1;
        Self::save_color_image(&image, &format!("{dest_dir}/{file_name}_{mip_level}.png")).unwrap();
        mip_level += 1;

        let radius = 3;
        let kernel = Self::kernel(radius, 1.0);
        let mut buff_image = image.clone();

        while buff_image.width() != 1 || buff_image.height() != 1 {
            let blur_image = Self::image_blur(&buff_image, &kernel, radius);
            buff_image = Self::image_downsampling(&blur_image);

            Self::save_color_image(&buff_image, &format!("{dest_dir}/{file_name}_{mip_level}.png")).unwrap();
            mip_level += 1;
        }
    }

    pub fn save_color_image(color_image: &ColorImage, file_path: &str) -> Result<(), image::ImageError> {
        let width = color_image.width() as u32;
        let height = color_image.height() as u32;

        let img_buffer = ImageBuffer::from_fn(width, height, |x, y| {
            let index = (y * width + x) as usize;
            let color = color_image.pixels[index];
            Rgba([
                color.r(),
                color.g(),
                color.b(),
                color.a(),
            ])
        });

        img_buffer.save(file_path)
    }

    pub fn kernel(radius: i32, sigma: f32)->DMatrix<f32>{
        let kernel_size = (radius * 2) + 1;
        let mut kernel = DMatrix::<f32>::zeros(kernel_size as usize, kernel_size as usize);

        let mut sum = 0.0;

        let sigma_square = 2.0 * sigma * sigma;
        let gauss_const = 1.0 / (std::f32::consts::PI * sigma_square);

        for x in -radius..=radius{
            for y in -radius..=radius{
                let kernel_item = gauss_const * (-(x as f32 * x as f32 + y as f32 * y as f32) / sigma_square).exp();

                sum += kernel_item;
                kernel[((x + radius) as usize, (y + radius) as usize)] = kernel_item;
            }
        }

        kernel.iter_mut().for_each(|x| *x /= sum);

        kernel
    }

    pub fn image_blur(image: &ColorImage, kernel: &DMatrix<f32>, radius: i32) -> ColorImage{
        let mut blur_image = ColorImage::new([image.width(), image.height()], vec![Color32::BLACK; image.width() * image.height()]);

        for i in 0..image.width(){
            for j in 0..image.height(){

                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;

                for x in -radius..=radius{
                    for y in -radius..=radius{

                        let kernel_item = kernel[((x + radius) as usize, (y + radius) as usize)];
                        let x_image = i as i32 + x;
                        let y_image = j as i32 + y;

                        if x_image >= 0 && x_image < image.width() as i32 && y_image >= 0 && y_image < image.height() as i32 {
                            let image_color = image[(x_image as usize, y_image as usize)];
                            r += image_color[0] as f32 * kernel_item;
                            g += image_color[1] as f32 * kernel_item;
                            b += image_color[2] as f32 * kernel_item;
                        }
                    }
                }

                blur_image[(i, j)] = Color32::from_rgb(r.clamp(0.0, 255.0) as u8, g.clamp(0.0, 255.0) as u8, b.clamp(0.0, 255.0) as u8);
            }
        }

        blur_image
    }

    pub fn image_downsampling(image: &ColorImage) -> ColorImage{
        let (new_width, new_height) = ((image.width() as f32 / 2.0).floor() as usize, (image.height() as f32 / 2.0) as usize);
        let mut mip_image = ColorImage::new([new_width, new_height], vec![Color32::BLACK; new_width * new_height]);

        for j in (0..image.width()).step_by(2) {
            for i in (0..image.height()).step_by(2) {
                let top_left = image[(i, j)];

                let mut bottom_left = Color32::BLACK;
                if j + 1 < image.width(){
                    bottom_left = image[(i, j + 1)];
                }

                let mut top_right = Color32::BLACK;
                if i + 1 < image.height(){
                    top_right = image[(i + 1, j)];
                }

                let mut bottom_right = Color32::BLACK;
                if i + 1 < image.width() && j + 1 < image.height(){
                    bottom_right = image[(i + 1, j + 1)];
                }

                let conv_color = Color32::from_rgb(((top_left.r() as i32 + bottom_left.r() as i32 + top_right.r() as i32 + bottom_right.r() as i32) / 4) as u8,
                                                   ((top_left.g() as i32 + bottom_left.g() as i32 + top_right.g() as i32 + bottom_right.g() as i32) / 4) as u8,
                                                   ((top_left.b() as i32 + bottom_left.b() as i32 + top_right.b() as i32 + bottom_right.b() as i32) / 4) as u8);


                if i / 2 < mip_image.height() && j / 2 < mip_image.width(){
                    mip_image[(i / 2, j / 2)] = conv_color;
                }
            }
        }

        mip_image
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
