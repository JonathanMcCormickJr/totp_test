use totp_rs::{Algorithm, TOTP, Secret};

fn main() {
    let totp = TOTP::new(
        Algorithm::SHA512,
        6,
        1,
        30,
        Secret::Raw("TestSecretSuperSecret".as_bytes().to_vec()).to_bytes().unwrap(),
        Some("Github".to_string()),
        "constantoine@github.com".to_string(),
    ).unwrap();
    //let qr_code = totp.get_qr_base64().unwrap();
    //println!("{}", qr_code);   
    let token = totp.generate_current().unwrap();
    println!("{}", token);   
}