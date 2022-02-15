# age-keygen-deterministic

This is a very simple Rust CLI tool which deterministically derives a single (or multiple) [age encryption](https://github.com/FiloSottile/age) secret keys from a passphrase.
I use this because I use age for long-term backup and want to be able to restore my backup files even if my house burns down and I lost all copies of the secret key - but still remember the passphrase.


## Compile

This utility is written in Rust.
Just clone this repository and then run ```cargo build```.

## Usage

```
USAGE:
    age-keygen-deterministic [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>      optional number of secret keys which should be created [default: 1]
    -o, --offset <offset>    optional u64 offset for index of keys [default: 0]
```

If you only need a single age key, just use it without any command line arguments.
If you want to generate more than one secret key, then you can simply use the ```--count``` argument.

## Example run

If you use the passphrase ```example-passphrase-do-not-use!``` and want to generate the deterministic secret keys #2, #3 and #4, then just run the commands below:

```
$ cargo build --release
$ ./target/release/age-keygen-deterministic -c 3 -o 2
Enter passphrase: 
# secret key 2 below
AGE-SECRET-KEY-1RSWAHJR48AWPN6HHTVVGXN7X3X0YWWA7TM7H22T7TF35EZPPVHHQ7WYGRZ
# secret key 3 below
AGE-SECRET-KEY-144T9ZKX0HK6CMMGYEN6WPN82Q4K9LVR376NUJF33HKVAQ70TXMHSPV96MY
# secret key 4 below
AGE-SECRET-KEY-1FMPVFDE9WD8CSTNS4J3QRNQ5VRTFE8973FVJ2JANT56HEPZTKA4SQZZ84R
```

The public key for any of those private keys can be obtained by piping the secret key to ```age-keygen -y```.

```
$ echo AGE-SECRET-KEY-1VZ3CREDN87LLHYDVS6FK36EZEVWNZGGFFSWZDN7DL0J04WG723MQCZUS9Q | age-keygen -y
age1z568mysf7kulsml0rxt6vxp3h26hjmgcmdpz8x6dfh0zlazspquqqawzn4
``` 

## Cryptography 

Please note that you should specify at least 256 bits of entropy.
This program will accept a passphrase with only 16 characters but in general you should use longer ones.

The passphrase is stretched with Argon2id to a 64 byte master key.
Afterwards, secret keys are generated by incrementing a u64 counter ```i``` and using ```HMAC_SHA256(master_key, i)``` for the age secret keys.
Each 32 byte HMAC-SHA256 output is converted to Bech32 format and written to stdout.

Please note that there are some cryptographic relevant limitations which this simple program does not care about:

- This program does not attempt to clear any secrets from RAM before it exits. 
- This program does not make any guarantees about side-channel resistance.
- No clamping of the X25519 key is done - but ```age-keygen``` doesn't do it either (as far as I know).

I recommend to generate the keys after taking your system offline and rebooting afterwards.

### Disclaimer

I am not a professional cryptographer and have not done any review of the third-party dependencies.
Use at your own risk.
