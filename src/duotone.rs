use photon_rs::{effects, filters, PhotonImage};

type ColorRGB = [f32; 3];

fn min(num1: f32, num2: f32) -> f32 {
    if num1 <= num2 {
        num1
    } else {
        num2
    }
}
fn max(num1: f32, num2: f32) -> f32 {
    if num1 >= num2 {
        num1
    } else {
        num2
    }
}

fn lightness_from_rgb(r: f32, g: f32, b: f32) -> f32 {
    let r_norm = r / 255.0;
    let g_norm = g / 255.0;
    let b_norm = b / 255.0;

    let max_val: f32 = max(b_norm, max(r_norm, g_norm));
    let min_val: f32 = min(b_norm, min(r_norm, g_norm));

    let lightness = min_val + (max_val - min_val) / 2.0;

    lightness
}

pub fn duotone(
    img: &mut PhotonImage,
    light_color: ColorRGB,
    dark_color: ColorRGB,
    contrast: f32,
) -> PhotonImage {
    filters::obsidian(img);
    effects::adjust_contrast(img, contrast);
    let img_as_raw_pixels = img.get_raw_pixels();
    let end = img_as_raw_pixels.len();

    let mut img_duotone = Vec::<u8>::new();

    for i in (0..end).step_by(4) {
        let r = img_as_raw_pixels[i] as f32;
        let g = img_as_raw_pixels[i + 1] as f32;
        let b = img_as_raw_pixels[i + 2] as f32;

        let average = (0.299 * r + 0.587 * g + 0.114 * b) as i32;
        let average = average as f32;

        let lightness = (lightness_from_rgb(average, average, average) * 254.0) as i32;
        let lightness = lightness as f32;

        let luminosity = min(254.0, max(0.0, lightness));
        let ratio = luminosity / 255.0;

        let new_r = (light_color[0] * ratio + dark_color[0] * (1.0 - ratio)) as u8;
        let new_g = (light_color[1] * ratio + dark_color[1] * (1.0 - ratio)) as u8;
        let new_b = (light_color[2] * ratio + dark_color[2] * (1.0 - ratio)) as u8;

        img_duotone.push(new_r);
        img_duotone.push(new_g);
        img_duotone.push(new_b);
        img_duotone.push(255);
    }
    PhotonImage::new(img_duotone, img.get_width(), img.get_height())
}
