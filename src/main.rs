use anyhow::{Ok, anyhow};
use aseprite_loader::loader::{AsepriteFile, LayerSelection};
use image::{DynamicImage, ImageBuffer, ImageFormat, imageops::FilterType};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let input_path  = args.get(1).ok_or(anyhow!( "First argument is empty"))?;
    let output_path = args.get(2).ok_or(anyhow!("Second argument is empty"))?;
    let size        = args.get(3).ok_or(anyhow!( "Third argument is empty"))?
        .parse::<u32>()?;

    generate_thumbnail(input_path, output_path, size)
}

fn generate_thumbnail(
    input_path: &String, output_path: &String, size: u32
) -> anyhow::Result<()> {
    let data = fs::read(input_path)?;
    let aseprite_file = AsepriteFile::load(&data)?;
    let (width, height) = aseprite_file.size();
    let mut buf = vec![0u8; usize::from(width) * usize::from(height) * 4];

    aseprite_file.render_frame(0, &mut buf, &LayerSelection::Visible)?;

    let (width, height) = (u32::from(width), u32::from(height));
    let image_buffer = ImageBuffer::from_raw(width, height, buf)
        .ok_or(anyhow!("Buffer is too small"))?;

    let filter = if height < size && width < size {
        FilterType::Nearest
    } else {
        FilterType::Triangle
    };

    DynamicImage::ImageRgba8(image_buffer).resize(size, size, filter)
        .save_with_format(output_path, ImageFormat::Png)?;

    Ok(())
}
