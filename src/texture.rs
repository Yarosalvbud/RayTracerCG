use std::path::Path;
use std::sync::Arc;
use egui::{Color32, ColorImage};
use image::{ImageBuffer, ImageReader, Rgba};
use nalgebra::{DMatrix, Vector3};

const KERNEL_RADIUS: i32 = 2;
const SIGMA: f32 = 0.5;

#[derive(Clone, Debug)]
pub struct Texture {
    image: Arc<ColorImage>,
    images: Arc<Vec<ColorImage>>,
}

impl Texture {
    pub fn new(path: String, is_normals: bool) -> Result<Self, String> {
        let image = Self::read_image(&path)?;
        let mut mip_levels = Vec::new();

        Self::create_mipmaps(&mut mip_levels, &image, &path, is_normals);

        Ok(Self {
            image: Arc::new(image),
            images: Arc::new(mip_levels),
        })
    }


    fn read_image(path: &str) -> Result<ColorImage, String> {
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
        Ok(ColorImage::from_rgba_unmultiplied(size, &img_rgba))
    }

    fn load_mip_maps(images: &mut Vec<ColorImage>, dest_dir: &str, file_name: &str){
        let mut mip_level = 0;
        let mut path = format!("{dest_dir}/{file_name}_{mip_level}.png");

        while Path::new(&path).exists() {
            images.push(Self::read_image(&path).unwrap());
            mip_level += 1;
            path = format!("{dest_dir}/{file_name}_{mip_level}.png");
        }
    }

    fn create_mipmaps(images: &mut Vec<ColorImage>, image: &ColorImage, path: &str, is_normals: bool) {
        let dest_dir = std::env::var("MIP_MAPS_DIR").expect("Директория для сохранения не найдена");
        let path = Path::new(path);

        let file_name = path.file_name().unwrap().to_str().unwrap().split('.').collect::<Vec<&str>>()[0];
        let mut mip_level = 0;
        if Path::new(&format!("{dest_dir}/{file_name}_{mip_level}.png")).exists() {
            Self::load_mip_maps(images, &dest_dir, &file_name);
            return;
        }

        Self::save_color_image(&image, &format!("{dest_dir}/{file_name}_{mip_level}.png")).unwrap();
        images.push(image.clone());
        mip_level += 1;
        
        let kernel = Self::kernel(KERNEL_RADIUS, SIGMA);
        let mut buff_image = image.clone();

        while buff_image.width() != 1 || buff_image.height() != 1 {
            let blur_image = Self::image_blur(&buff_image, &kernel, KERNEL_RADIUS, is_normals);
            buff_image = Self::image_downsampling(&blur_image, is_normals);

            Self::save_color_image(&buff_image, &format!("{dest_dir}/{file_name}_{mip_level}.png")).unwrap();
            images.push(buff_image.clone());
            mip_level += 1;
        }
    }

    fn save_color_image(color_image: &ColorImage, file_path: &str) -> Result<(), image::ImageError> {
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

    fn image_downsampling(image: &ColorImage, is_normals: bool) -> ColorImage {
        let new_width = (image.width() as f32 / 2.0).ceil() as usize;
        let new_height = (image.height() as f32 / 2.0).ceil() as usize;
        let mut mip_image = ColorImage::new([new_width, new_height], vec![Color32::BLACK; new_width * new_height]);

        for y in (0..image.height()).step_by(2) {
            for x in (0..image.width()).step_by(2) {
                let mut color_sum = vec![0.0, 0.0, 0.0];
                let mut count = 0;

                let mut normals_sum = Vector3::<f32>::new(0.0, 0.0, 0.0);

                for dy in 0..2 {
                    for dx in 0..2 {
                        let px = x + dx;
                        let py = y + dy;
                        if px < image.width() && py < image.height() {
                            let color_texture = image[(px, py)];
                            if is_normals {
                                let n = Vector3::new(
                                    (color_texture.r() as f32 / 255.0) * 2.0 - 1.0,
                                    (color_texture.g() as f32 / 255.0) * 2.0 - 1.0,
                                    (color_texture.b() as f32 / 255.0) * 2.0 - 1.0,
                                ).normalize();
                                normals_sum += n;
                            } else {
                                let color_linear = Self::srgb_to_linear(&vec![color_texture[0], color_texture[1], color_texture[2]]);

                                color_sum[0] += color_linear[0];
                                color_sum[1] += color_linear[1];
                                color_sum[2] += color_linear[2];
                            }

                            count += 1;
                        }
                    }
                }

                let conv_color = if is_normals {
                    if count > 0 {
                        let n_avg = (normals_sum / count as f32).normalize();
                        let r = ((n_avg.x * 0.5 + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
                        let g = ((n_avg.y * 0.5 + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
                        let b = ((n_avg.z * 0.5 + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
                        Color32::from_rgb(r, g, b)
                    } else {
                        Color32::BLACK
                    }
                } else {
                    color_sum[0] /= count as f32;
                    color_sum[1] /= count as f32;
                    color_sum[2] /= count as f32;

                    let srgb_color = Self::linear_to_srgb(&color_sum);

                    Color32::from_rgb(
                        srgb_color[0],
                        srgb_color[1],
                        srgb_color[2],
                    )
                };

                mip_image[(x / 2, y / 2)] = conv_color;
            }
        }

        mip_image
    }

    fn kernel(radius: i32, sigma: f32)->DMatrix<f32>{
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

    fn image_blur(image: &ColorImage, kernel: &DMatrix<f32>, radius: i32, is_normals: bool) -> ColorImage{
        let mut blur_image = ColorImage::new([image.width(), image.height()], vec![Color32::BLACK; image.width() * image.height()]);

        for i in 0..image.width(){
            for j in 0..image.height(){

                let mut conv_color = vec![0.0, 0.0, 0.0];

                for x in -radius..=radius{
                    for y in -radius..=radius{

                        let kernel_item = kernel[((x + radius) as usize, (y + radius) as usize)];
                        let x_image = i as i32 + x;
                        let y_image = j as i32 + y;

                        if x_image >= 0 && x_image < image.width() as i32 && y_image >= 0 && y_image < image.height() as i32 {
                            let image_color = image[(x_image as usize, y_image as usize)];
                            let mut color = Vector3::new(image_color[0] as f32, image_color[1] as f32, image_color[2] as f32);
                            if is_normals{
                                color = Vector3::new((color[0] / 255.0) * 2.0 - 1.0,
                                             (color[1] / 255.0) * 2.0 - 1.0,
                                             (color[2] / 255.0) * 2.0 - 1.0).normalize();
                            }else{
                                let linear = Self::srgb_to_linear(&vec![image_color[0], image_color[1], image_color[2]]);
                                color = Vector3::new(linear[0], linear[1], linear[2]);
                            }
                            conv_color[0] += color[0] * kernel_item;
                            conv_color[1] += color[1] * kernel_item;
                            conv_color[2] += color[2] * kernel_item;
                        }
                    }
                }
                if is_normals{
                    let mut normal = Vector3::new(conv_color[0], conv_color[1], conv_color[2]).normalize();
                    normal = (normal * 0.5 + Vector3::new(0.5, 0.5, 0.5)) * 255.0;
                    blur_image[(i, j)] = Color32::from_rgb(normal.x.clamp(0.0, 255.0) as u8,
                                                           normal.y.clamp(0.0, 255.0) as u8,
                                                           normal.z.clamp(0.0, 255.0) as u8);

                }else{
                    let color = Self::linear_to_srgb(&conv_color);
                    blur_image[(i, j)] = Color32::from_rgb(color[0], color[1], color[2]);
                }

            }
        }

        blur_image
    }

    pub fn get_color(first_color: &Color32, second_color: &Color32, rs: f32) -> Color32 {
        Color32::from_rgb(
            (first_color[0] as f32 * (1.0 - rs) + second_color[0] as f32 * rs) as u8,
            (first_color[1] as f32 * (1.0 - rs) + second_color[1] as f32 * rs) as u8,
            (first_color[2] as f32 * (1.0 - rs) + second_color[2] as f32 * rs) as u8,
        )
    }

    pub fn get_texture_color(first_color: &Color32, second_color: &Color32, rs: f32) -> Color32 {
        let first_color = Texture::srgb_to_linear(&vec![first_color[0], first_color[1], first_color[2]]);
        let second_color = Texture::srgb_to_linear(&vec![second_color[0], second_color[1], second_color[2]]);

        let interpolated_color = vec![
          first_color[0] * (1.0 - rs) + second_color[0] * rs,
          first_color[1] * (1.0 - rs) + second_color[1] * rs,
          first_color[2] * (1.0 - rs) + second_color[2] * rs,
        ];

        let srgb_color = Texture::linear_to_srgb(&interpolated_color);
        Color32::from_rgb(srgb_color[0], srgb_color[1], srgb_color[2])
    }

    pub fn resolution(&self) -> (usize, usize){
        (self.image.width(), self.image.height())
    }

    fn update_borders(&self, x: f32, y: f32, mip_level: usize) -> (usize, usize) {
        let mut x = x as usize;
        let mut y = y as usize;

        let w = self.images[mip_level].width();
        let h = self.images[mip_level].height();

        if x >= w{
            x = x % w;
        }

        if y >= h{
            y = y % h;
        }

        (x, y)
    }

    pub fn bilinear_interpolation(&self, x: f32, y: f32, mip_level: usize, interpolator: fn(&Color32, &Color32, f32) -> Color32) -> Color32{
        let mut mip_level = mip_level;

        if mip_level >= self.images.len(){
            mip_level = self.images.len() - 1;
        }

        let (x_tex, y_tex) = (x * (self.images[mip_level].width() as f32), y * (self.images[mip_level].height() as f32));
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

        let (x_top_r, y_top_r) = self.update_borders(x_top, y_top, mip_level);
        let (x_bottom_r, y_bottom_r) = self.update_borders(x_bottom, y_bottom, mip_level);
        
        let color_top = interpolator(&self.images[mip_level][(x_bottom_r, y_top_r)], &self.images[mip_level][(x_top_r, y_top_r)], rs);
        let color_bottom = interpolator(&self.images[mip_level][(x_bottom_r, y_bottom_r)], &self.images[mip_level][(x_top_r, y_bottom_r)], rs);

        interpolator(&color_bottom, &color_top, rt)
        
    }

    pub fn sample(&self, x: f32, y: f32, mip_level: usize, interpolator: fn(&Color32, &Color32, f32) -> Color32) -> Color32 {
        self.bilinear_interpolation(x.rem_euclid(1.0), y.rem_euclid(1.0), mip_level, interpolator)
    }

    pub fn trilinear_interpolation(&self, x: f32, y: f32, mip_level: f32, interpolator: fn(&Color32, &Color32, f32) -> Color32) -> Color32{
         let level_first = mip_level.floor() as usize;
         let level_second = mip_level.ceil() as usize;
         let rm = mip_level.fract();

         if level_first == level_second{
             return self.sample(x, y, level_first, interpolator);
         }

         let first_color = self.sample(x, y, level_first, interpolator);
         let second_color = self.sample(x, y, level_second, interpolator);

         interpolator(&first_color, &second_color, rm)
    }

    pub fn srgb_to_linear(color: &Vec<u8>) -> Vec<f32>{
        let mut linear_color: Vec<f32> = Vec::with_capacity(color.len());

        for item in color.iter() {
            let mut item = *item as f32 / 255.0;
            if item <= 0.04045{
                item /= 12.92;
            }else{
                item = ((item + 0.055) / 1.055).powf(2.4);
            }

            linear_color.push(item);
        }

        linear_color
    }

    pub fn linear_to_srgb(color: &Vec<f32>) -> Vec<u8>{
        let mut srgb_color: Vec<u8> = Vec::with_capacity(color.len());

        for item in color.iter() {
            let srgb = if *item <= 0.0031308{
                item * 12.92
            }else{
              1.055 * item.powf(1.0 / 2.4) - 0.055
            };

            srgb_color.push((srgb.clamp(0.0, 1.0) * 255.0).round() as u8);
        }

        srgb_color
    }
}
