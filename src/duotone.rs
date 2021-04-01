use photon_rs::{effects, filters, PhotonImage};

type ColorRGB = [f32; 3];

fn heighten_contrast(img: &mut PhotonImage, contrast: f32) {
    effects::adjust_contrast(img, contrast);
}

fn rgb_to_hls(r: f32, g: f32, b: f32) -> f32 {
    let r_norm = r / 255.0;
    let g_norm = g / 255.0;
    let b_norm = b / 255.0;

    let max_val: f32;
    if r_norm >= g_norm {
        if b_norm >= r_norm {
            max_val = b_norm;
        } else {
            max_val = r_norm;
        }
    } else {
        if b_norm >= g_norm {
            max_val = b_norm;
        } else {
            max_val = g_norm;
        }
    }

    let min_val: f32;
    if r_norm <= g_norm {
        if b_norm <= r_norm {
            min_val = b_norm;
        } else {
            min_val = r_norm;
        }
    } else {
        if b_norm <= g_norm {
            min_val = b_norm;
        } else {
            min_val = g_norm;
        }
    }

    let l = min_val + (max_val - min_val) / 2.0;

    l
}

pub fn duotone(
    img: &mut PhotonImage,
    light_color: ColorRGB,
    dark_color: ColorRGB,
    contrast: f32,
) -> PhotonImage {
    filters::obsidian(img);
    heighten_contrast(img, contrast);
    let img_as_raw_pixels = img.get_raw_pixels();
    let end = img_as_raw_pixels.len();

    let mut img_duotone = Vec::<u8>::new();

    for i in (0..end).step_by(4) {
        let r = img_as_raw_pixels[i] as f32;
        let g = img_as_raw_pixels[i + 1] as f32;
        let b = img_as_raw_pixels[i + 2] as f32;

        let average = (0.299 * r + 0.587 * g + 0.114 * b) as i32;
        let average = average as f32;

        let l = (rgb_to_hls(average, average, average) * 254.0) as i32;
        let l = l as f32;

        let mut luminosity = 0_f32;
        if l > 0.0 && l < 254.0 {
            luminosity = l;
        } else if l < 0.0 {
            luminosity = 0.0;
        } else if l > 254.0 {
            luminosity = 254.0;
        }
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
