use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use egui::{ColorImage, TextureHandle, TextureOptions};
use once_cell::sync::Lazy;

use crate::resource::get_resource;

pub fn get_dyn_image(
    img_path: &str,
    img_format: image::ImageFormat,
) -> Result<image::DynamicImage, String> {
    match get_resource(img_path) {
        Err(err) => Err(err.to_string()),
        Ok(res) => {
            let mut img = image::ImageReader::new(std::io::Cursor::new(&res));
            img.set_format(img_format);
            match img.decode() {
                Err(err) => Err(err.to_string()),
                Ok(img) => Ok(img),
            }
        }
    }
}

pub fn get_color_image(
    img_path: &str,
    img_format: image::ImageFormat,
) -> Result<ColorImage, String> {
    let dyn_img = get_dyn_image(img_path, img_format)?;
    let rgba = dyn_img.to_rgba8();
    let size = [rgba.width() as _, rgba.height() as _];

    Ok(ColorImage::from_rgba_unmultiplied(size, &rgba))
}

#[derive(Clone)]
pub struct ImageManager {
    images: Arc<RwLock<HashMap<String, TextureHandle>>>,
}

impl ImageManager {
    pub fn new() -> Self {
        Self {
            images: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn load_image_texture(
        &self,
        ctx: &egui::Context,
        img_path: &str,
        img_format: image::ImageFormat,
    ) -> TextureHandle {
        let read_binding = self.images.read().unwrap();
        let image = read_binding.get(img_path);
        if let Some(img) = image {
            return img.clone();
        }

        println!("Load image {img_path}");
        let _ = image;
        drop(read_binding);
        let color_img =
            get_color_image(img_path, img_format).expect("Unable to load image {img_path}");
        let texture_handle =
            ctx.load_texture("img_path_{img_path}", color_img, TextureOptions::LINEAR);
        let mut writeable_images = self.images.write().unwrap();
        writeable_images.insert(img_path.to_owned(), texture_handle.clone());
        texture_handle.clone()
    }

    pub fn widget(
        &'_ self,
        ctx: &egui::Context,
        img_path: &str,
        img_format: image::ImageFormat,
    ) -> egui::Image<'_> {
        let texture = self.load_image_texture(ctx, img_path, img_format);
        egui::Image::new(egui::ImageSource::Texture(
            egui::load::SizedTexture::from_handle(&texture),
        ))
    }
}

pub static IMAGE_MANAGER: Lazy<ImageManager> = Lazy::new(ImageManager::new);
