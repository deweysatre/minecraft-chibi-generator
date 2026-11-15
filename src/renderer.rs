use crate::cli::View;
use crate::positions::{Rect, mapping};
use anyhow::Result;
use image::{ImageBuffer, Rgba, RgbaImage};

pub fn compose_view(skin: &RgbaImage, slim: bool, view: View) -> Result<RgbaImage> {
    let mut canvas = ImageBuffer::from_pixel(24, 33, Rgba([0, 0, 0, 0]));
    for specification in mapping(view) {
        let destination = if slim {
            specification.destination
        } else {
            specification
                .wide_destination
                .unwrap_or(specification.destination)
        };
        blit_scaled(
            skin,
            &mut canvas,
            specification.base,
            destination,
            specification.mirror_x,
        );
        if let Some(overlay) = specification.overlay {
            blit_scaled(
                skin,
                &mut canvas,
                overlay,
                destination,
                specification.mirror_x,
            );
        }
        if specification.side {
            darken_rect(&mut canvas, destination, 0.86);
        }
    }
    Ok(canvas)
}

fn darken_rect(image: &mut RgbaImage, rect: Rect, factor: f32) {
    for y in rect.1..(rect.1 + rect.3).min(image.height()) {
        for x in rect.0..(rect.0 + rect.2).min(image.width()) {
            let pixel = image.get_pixel_mut(x, y);
            pixel.0[0] = (f32::from(pixel.0[0]) * factor).round() as u8;
            pixel.0[1] = (f32::from(pixel.0[1]) * factor).round() as u8;
            pixel.0[2] = (f32::from(pixel.0[2]) * factor).round() as u8;
        }
    }
}

fn blit_scaled(
    source: &RgbaImage,
    destination: &mut RgbaImage,
    source_rect: Rect,
    destination_rect: Rect,
    mirror_x: bool,
) {
    for dy in 0..destination_rect.3 {
        for dx in 0..destination_rect.2 {
            let sample_x =
                (((dx * 2 + 1) * source_rect.2) / (destination_rect.2 * 2)).min(source_rect.2 - 1);
            let sample_x = if mirror_x {
                source_rect.2 - 1 - sample_x
            } else {
                sample_x
            };
            let sample_y =
                (((dy * 2 + 1) * source_rect.3) / (destination_rect.3 * 2)).min(source_rect.3 - 1);
            let x = destination_rect.0 + dx;
            let y = destination_rect.1 + dy;
            if x >= destination.width() || y >= destination.height() {
                continue;
            }
            let pixel = *source.get_pixel(
                (source_rect.0 + sample_x).min(source.width() - 1),
                (source_rect.1 + sample_y).min(source.height() - 1),
            );
            if pixel.0[3] > 0 {
                destination.put_pixel(x, y, pixel);
            }
        }
    }
}

pub fn trim_transparent_border(image: &RgbaImage) -> RgbaImage {
    let mut bounds: Option<(u32, u32, u32, u32)> = None;
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel.0[3] == 0 {
            continue;
        }
        bounds = Some(match bounds {
            Some((min_x, min_y, max_x, max_y)) => {
                (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
            }
            None => (x, y, x, y),
        });
    }

    match bounds {
        Some((min_x, min_y, max_x, max_y)) => {
            image::imageops::crop_imm(image, min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
                .to_image()
        }
        None => RgbaImage::from_pixel(1, 1, Rgba([0, 0, 0, 0])),
    }
}
