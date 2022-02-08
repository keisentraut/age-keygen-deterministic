extern crate rpassword;
use argon2::{self, Config, ThreadMode, Variant, Version};
use bech32::{self, ToBase32, Variant as Bech32Variant};

fn main() {
    let passphrase = rpassword::prompt_password_stderr("Enter passphrase: ").unwrap();
    if passphrase.as_bytes().len() < 16 {
        panic!("Passphrase must be at least 16 characters.");
    }
    let salt = b"age-keygen-hardcoded-salt";
    // I explicitly hardcoded the Argon2 parameters here, because Config::default() might change in future.
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 2,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    let hash: Vec<u8> = argon2::hash_raw(passphrase.as_bytes(), salt, &config).unwrap();
    let hash_u5 = hash.to_base32();
    let b = bech32::encode("AGE-SECRET-KEY-", hash_u5, Bech32Variant::Bech32).unwrap();
    let b = b.to_uppercase();
    println!("{:}", b);
}
