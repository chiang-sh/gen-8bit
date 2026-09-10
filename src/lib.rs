//! A small tool for converting images to an 8-bit pixel art style.
use image::{ImageError, Rgba, RgbaImage};
use std::path::Path;

const PIXEL_SIZE: u32 = 8;
const PALETTE_SIZE: usize = 16;

/// Convert original image to an 8-bit pixel art style
/// 
/// The image would be downscaled, color quantized, and upscaled to its original size to achieve an 8-bit style effect.
/// 
/// # Arguments
/// 
/// * `src_path`: Path to the source image file.
/// * `dest_path`: Path to the destination image file.
/// 
/// # Errors
/// Returns `ImageError` when image processing errors occur.
/// 
/// # Example
/// ```
/// gen_8bit::gen_8bit("test_src.png", "test_dest.png")?;
/// ```
pub fn gen_8bit(src_path: &str, dest_path: &str) -> Result<(), ImageError> {
    let src_path = Path::new(src_path);
    let src_image = image::open(src_path)?.to_rgba8();
    let downscale_image = resize(
        &src_image,
        src_image.width() / PIXEL_SIZE,
        src_image.height() / PIXEL_SIZE,
    );
    let dest_image = resize(
        &color_quantization(downscale_image),
        src_image.width(),
        src_image.height(),
    );
    let dest_path = Path::new(dest_path);
    dest_image.save(dest_path)?;
    Ok(())
}

fn resize(src_image: &RgbaImage, dest_w: u32, dest_h: u32) -> RgbaImage {
    let mut dest_image = RgbaImage::new(dest_w, dest_h);
    for x in 0..dest_image.width() {
        for y in 0..dest_image.height() {
            let src_x = x * src_image.width() / dest_image.width();
            let src_y = y * src_image.height() / dest_image.height();
            let pixel = src_image.get_pixel(src_x, src_y);
            dest_image.put_pixel(x, y, *pixel);
        }
    }
    dest_image
}

fn color_quantization(src_image: RgbaImage) -> RgbaImage {
    let palette = median_cut(&src_image);
    let mut dest_image = RgbaImage::new(src_image.width(), src_image.height());
    for x in 0..dest_image.width() {
        for y in 0..dest_image.height() {
            let src_rgba = src_image.get_pixel(x, y);
            let mut nearest_rgb = palette[0];
            let mut nearest_dist = i32::MAX;
            for rgb in &palette {
                let dist_r = src_rgba.0[0] as i32 - rgb[0] as i32;
                let dist_g = src_rgba.0[1] as i32 - rgb[1] as i32;
                let dist_b = src_rgba.0[2] as i32 - rgb[2] as i32;
                let dist = dist_r * dist_r + dist_g * dist_g + dist_b * dist_b;
                if dist < nearest_dist {
                    nearest_dist = dist;
                    nearest_rgb = *rgb;
                }
            }
            dest_image.put_pixel(
                x,
                y,
                Rgba([nearest_rgb[0], nearest_rgb[1], nearest_rgb[2], src_rgba[3]]),
            );
        }
    }
    dest_image
}

fn median_cut(image: &RgbaImage) -> Vec<[u8; 3]> {
    let pixels = image.pixels();
    let mut groups: Vec<Vec<[u8; 3]>> =
        vec![pixels.map(|rgba| [rgba[0], rgba[1], rgba[2]]).collect()];

    while groups.len() < PALETTE_SIZE {
        let mut group = groups.remove(0);
        let mut min_r = 255;
        let mut max_r = 0;
        let mut min_g = 255;
        let mut max_g = 0;
        let mut min_b = 255;
        let mut max_b = 0;

        for rgb in &group {
            min_r = min_r.min(rgb[0]);
            max_r = max_r.max(rgb[0]);
            min_g = min_g.min(rgb[1]);
            max_g = max_g.max(rgb[1]);
            min_b = min_b.min(rgb[2]);
            max_b = max_b.max(rgb[2]);
        }

        let diff = [max_r - min_r, max_g - min_g, max_b - min_b];
        let max_channel = diff
            .iter()
            .enumerate()
            .max_by_key(|(_, val)| **val)
            .map(|(index, _)| index)
            .unwrap();
        group.sort_unstable_by_key(|rgb| rgb[max_channel]);
        let half = group.split_off(group.len() / 2);
        groups.push(half);
        groups.push(group);
    }

    let mut palette: Vec<[u8; 3]> = vec![];
    for group in &groups {
        let mut sum_r = 0;
        let mut sum_g = 0;
        let mut sum_b = 0;
        group.iter().for_each(|rgb| {
            sum_r += rgb[0] as u64;
            sum_g += rgb[1] as u64;
            sum_b += rgb[2] as u64;
        });
        let len = group.len() as u64;
        palette.push([
            (sum_r / len) as u8,
            (sum_g / len) as u8,
            (sum_b / len) as u8,
        ]);
    }
    palette
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_8bit() {
        gen_8bit("", "").unwrap();
    }
}
