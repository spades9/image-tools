use std::{fs::File, io::BufWriter};

use image::{io::Reader as ImageReader, ImageFormat, ImageEncoder, ImageBuffer, RgbaImage, EncodableLayout, ColorType, GenericImageView, DynamicImage};
use log::info;
use image::codecs::ico::{IcoEncoder,IcoFrame};

pub fn get_image_format(path: &str) -> Option<&str>{
    if let Ok(img) = ImageReader::open(path){
        if let Some(format) = img.format(){
            match format {
                ImageFormat::Png => {
                    return Some("png");
                }
                ImageFormat::Jpeg => {
                    return Some("jpeg");
                }
                ImageFormat::Ico => {
                    return Some("ico");
                }
                _ => {
                    return None;
                }
            }
        }
    }
    None
}


pub fn convert_image_format(path: &str,index: i32){
    if let Ok(img) = image::open(path){
        let save_path = format!("E:/data/test/{}.ico",index);
        info!("开始保存:{}",save_path);
        //let ico = IcoEncoder::new(data);
        let width = img.width();
        let height = img.height();

        let mut ico_image = RgbaImage::new(width, height);
        info!("width:{},height:{}",width,height);
        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x, y);
                ico_image.put_pixel(x, y, pixel);
            }
        }

        let file = File::create(save_path).expect("create file failed!!");
        let mut output_writer = BufWriter::new(file);
        //let _ = ico_image.save_with_format(save_path, ImageFormat::Ico);
        let b = ico_image.as_bytes();
        //let _ = ico_image.write_to(&mut buf, ImageFormat::Ico);
        IcoEncoder::new(&mut output_writer).encode_images(ico_image);
        
        //IcoEncoder::new(&mut buf).encode_images(&image_buffer);
        //IcoFrame::as_png(data, img., height, color_type)
        //let _ = img.decode().unwrap().save_with_format(save_path, ImageFormat::Ico);
    }
}