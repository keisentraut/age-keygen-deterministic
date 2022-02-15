extern crate rpassword;
use argon2::{self, Config, ThreadMode, Variant, Version};
use bech32::{self, ToBase32, Variant as Bech32Variant};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "age-keygen-deterministic",
    about = "Tool for deterministic age key generation from passphrase.",
    author = "Klaus Eisentraut",
    version = "0.2"
)]
struct Opt {
    #[structopt(default_value = "0", short, long)]
    /// optional u64 offset for index of keys
    offset: u64,
    #[structopt(default_value = "1", short, long)]
    /// optional number of secret keys which should be created
    count: u64,
}

fn main() {
    let opt = Opt::from_args();
    let (offset, count) = (opt.offset, opt.count);
    let offset_end = offset.checked_add(count).unwrap();

    let passphrase = rpassword::prompt_password_stderr("Enter passphrase: ").unwrap();
    if passphrase.as_bytes().len() < 16 {
        panic!("Passphrase must be at least 16 characters.");
    }

    // I explicitly hardcoded the Argon2 parameters here, because Config::default() might change in future.
    let salt = b"age-keygen-deterministic-hardcoded-salt";
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 2,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 64,
    };
    let master_key: Vec<u8> = argon2::hash_raw(passphrase.as_bytes(), salt, &config).unwrap();

    // now derive keys by calculating HMAC_SHA256(master, i) with varying values of i
    for index in offset..offset_end {
        let mut hmac = Hmac::<Sha256>::new_from_slice(&master_key).unwrap();
        hmac.update(&index.to_be_bytes());
        let key = hmac.finalize().into_bytes();
        let key_u5 = key.to_base32();
        let key_b = bech32::encode("AGE-SECRET-KEY-", key_u5, Bech32Variant::Bech32).unwrap();
        let key_b = key_b.to_uppercase();
        println!("# secret key {:} below", index);
        println!("{:}", key_b);
    }
}
