use totp_rs::{Algorithm, TOTP, Secret};

use base64::decode;
use image::DynamicImage;
use std::fs::File;
use std::io::Write;

fn main() {
    let totp = TOTP::new(
        Algorithm::SHA512,
        6,
        1,
        30,
        Secret::Raw("TestSecretSuperSecre24t42t2t".as_bytes().to_vec()).to_bytes().unwrap(),
        Some("Jonathan".to_string()),
        "user-email@domain.com".to_string(),
    ).unwrap();
    let qr_code = totp.get_qr_base64().unwrap();
    println!("{}", qr_code);   
    let token = totp.generate_current().unwrap();
    println!("{}", token);   



    let base64_qr = qr_code;

    // Decode the base64 string
    let decoded_data = decode(base64_qr).unwrap();

    // Create a dynamic image from the decoded data
    let image = image::load_from_memory(&decoded_data).unwrap();

    // Save the image as a PNG file
    let mut file = File::create("qr_code.png").unwrap();
    image.write_to(&mut file, image::ImageFormat::Png).unwrap();

    println!("QR code saved as 'qr_code.png'");
}