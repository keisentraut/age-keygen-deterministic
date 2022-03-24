use argon2::{self, Config, ThreadMode, Variant, Version};
use bech32::{self, ToBase32, Variant as Bech32Variant};
use hmac::{Hmac, Mac};
use rpassword::prompt_password;
use sha2::Sha256;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "age-keygen-deterministic",
    about = "Tool for deterministic age key generation from passphrase.",
    author = "Klaus Eisentraut",
    version = "0.3"
)]
struct Opt {
    #[structopt(default_value = "0", short, long)]
    /// optional u64 offset for index of keys
    offset: u64,
    #[structopt(default_value = "1", short, long)]
    /// optional number of secret keys which should be created
    count: u64,
}

struct AgeKeyGenerator {
    master_key: Vec<u8>,
}

impl AgeKeyGenerator {
    fn new(passphrase: String) -> Self {
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
        AgeKeyGenerator {
            master_key: argon2::hash_raw(passphrase.as_bytes(), salt, &config).unwrap(),
        }
    }

    fn get_key(self: &Self, index: u64) -> String {
        // now derive keys by calculating HMAC_SHA256(master, i) with varying values of i
        let mut hmac = Hmac::<Sha256>::new_from_slice(&self.master_key).unwrap();
        hmac.update(&index.to_be_bytes());
        let key = hmac.finalize().into_bytes();
        let key_u5 = key.to_base32();
        let key_b = bech32::encode("AGE-SECRET-KEY-", key_u5, Bech32Variant::Bech32).unwrap();
        key_b.to_uppercase()
    }
}

fn main() {
    let opt = Opt::from_args();
    let (offset, count) = (opt.offset, opt.count);
    let offset_end = offset
        .checked_add(count)
        .expect("Integer overflow during offset calculation.");

    let passphrase = rpassword::prompt_password("Enter passphrase: ").unwrap();
    if passphrase.as_bytes().len() < 16 {
        panic!("Passphrase must be at least 16 characters.");
    }

    let agk = AgeKeyGenerator::new(passphrase);
    for i in offset..offset_end {
        println!("# secret key {:} below", i);
        println!("{:}", agk.get_key(i));
    }
}

#[cfg(test)]
mod tests {
    use crate::AgeKeyGenerator;
    #[test]
    fn test_key_generation() {
        let agk = AgeKeyGenerator::new("example-passphrase-do-not-use!".to_string());
        assert_eq!(
            agk.get_key(0),
            "AGE-SECRET-KEY-1VZ3CREDN87LLHYDVS6FK36EZEVWNZGGFFSWZDN7DL0J04WG723MQCZUS9Q"
                .to_string()
        );
        // test some more, out-of-order
        assert_eq!(
            agk.get_key(4),
            "AGE-SECRET-KEY-1FMPVFDE9WD8CSTNS4J3QRNQ5VRTFE8973FVJ2JANT56HEPZTKA4SQZZ84R"
                .to_string()
        );
        assert_eq!(
            agk.get_key(2),
            "AGE-SECRET-KEY-1RSWAHJR48AWPN6HHTVVGXN7X3X0YWWA7TM7H22T7TF35EZPPVHHQ7WYGRZ"
                .to_string()
        );
        assert_eq!(
            agk.get_key(3),
            "AGE-SECRET-KEY-144T9ZKX0HK6CMMGYEN6WPN82Q4K9LVR376NUJF33HKVAQ70TXMHSPV96MY"
                .to_string()
        );
    }
}
