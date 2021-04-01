mod duotone;
mod image_io;

fn main() {
    let read_path = "/home/gp/Documents/duotone/assets/t5.png";
    let save_path = "/home/gp/Documents/duotone/assets/t5_duotone.png";
    let mut img = image_io::read(read_path);
    let img_duotoned = duotone::duotone(&mut img, [0.0, 0.0, 0.0], [0.0, 255.0, 255.0], -10.0);
    image_io::write(img_duotoned, save_path);
}
