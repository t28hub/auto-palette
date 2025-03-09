use anyhow::Context;
use arboard::Clipboard;
use image::{DynamicImage, RgbaImage};

pub fn get_image_from_clipboard() -> anyhow::Result<DynamicImage> {
    let mut clipboard = Clipboard::new().context("couldn't access system clipboard")?;
    let image_data = clipboard
        .get_image()
        .context("couldn't get image from clipboard")?;

    let maybe_image_buffer = RgbaImage::from_raw(
        image_data.width as u32,
        image_data.height as u32,
        image_data.bytes.to_vec(),
    );

    let image = match maybe_image_buffer {
        Some(image_buffer) => DynamicImage::ImageRgba8(image_buffer),
        None => {
            return Err(anyhow::anyhow!(
                "couldn't construct RGBA image from clipboard bytes"
            ));
        }
    };

    Ok(image)
}
