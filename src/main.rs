mod duotone;
mod image_io;

fn main() {
    let read_path = "/home/gp/Documents/duotone/assets/t3.jpg";
    let save_path = "/home/gp/Documents/duotone/assets/t3_duotone.jpg";

    let mut img = image_io::read(read_path);

    let light_color: [f32; 3] = [0.0, 255.0, 255.0];
    let dark_color: [f32; 3] = [0.0, 0.0, 0.0];
    let contrast: f32 = -10.0;

    let img_duotoned = duotone::duotone(&mut img, light_color, dark_color, contrast);

    image_io::write(img_duotoned, save_path);
}
