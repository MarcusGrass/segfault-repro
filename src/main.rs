#![no_std]
#![no_main]
use argon2::{Algorithm, Params, Version};

#[no_mangle]
extern "C" fn main() -> i32 {
    tiny_std::unix::host_name::host_name().unwrap();
    unix_print::unix_println!("Start");
    let mut key = [0u8; 32];
    let pass_bytes = [2u8; 128];
    let salt = [1u8; 32];
    argon2::Argon2::new(
        Algorithm::Argon2i,
        Version::V0x13,
        Params::new(
            65536,
            10,
            4,
            Some(32),
        )
            .unwrap()
    )
        .hash_password_into(&pass_bytes, &salt, &mut key).unwrap();
    unix_print::unix_println!("Done");
    0
}
