use image::RgbImage;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/dist/"]
#[include = "*.*"]
pub struct Asset;

const ICON_SIZE: u32 = 60;

impl Asset {
    fn get_icon(file_path: &str) -> RgbImage {
        let buf = Asset::get(file_path).unwrap().data.to_vec();
        let image = image::load_from_memory_with_format(&buf, image::ImageFormat::Png).unwrap();

        image.to_rgb8()
    }

    pub fn get_icon_save() -> RgbImage {
        Asset::get_icon("save.png")
    }

    pub fn get_icon_exit() -> RgbImage {
        Asset::get_icon("exit.png")
    }

    pub fn get_icon_clear() -> RgbImage {
        Asset::get_icon("clear.png")
    }

    pub fn get_icon_down() -> RgbImage {
        Asset::get_icon("down.png")
    }
}
