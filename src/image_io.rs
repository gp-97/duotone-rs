use photon_rs::{
    native::{open_image, save_image},
    PhotonImage,
};

pub fn read(read_path: &str) -> PhotonImage {
    match open_image(read_path) {
        Ok(img) => img,
        Err(_err) => {
            eprintln!("[ERROR]: Couldn't open the file. Check filepath and/or filename");
            std::process::exit(1);
        }
    }
}

pub fn write(img: PhotonImage, save_path: &str) {
    save_image(img, save_path);
}
