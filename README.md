# age-keygen-deterministic

This is a very simple Rust CLI tool which deterministically derives a single [age encryption](https://github.com/FiloSottile/age) secret key from a passphrase.
I use this because I use age for long-term backup and want to be able to restore my backup even if my house burns down and I lost all copies of the secret key (but still remember the passphrase).

This program will only accept a passphrase with at least 16 characters.
The passphrase is stretched with Argon2id to the 32 byte age secret key and then converted to Bech32 format.

## Compile

Clone this repository and then run ```cargo run --release```.

## Usage

This executable takes no command line arguments at all.
If you would use the passphrase ```example-passphrase-do-not-use!``` then you will get the following secret key:

```
$ cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/age-keygen-deterministic`
Enter passphrase: 
AGE-SECRET-KEY-1LHZD5K7C0EVAA6G8CCYWRYKAMZN6FY5PR6QX8NMU958ZP3WCG3XSF2HGJM
```

The public key for this private key can be calculated by piping the secret key to ```age-keygen -y```.

```
$ echo AGE-SECRET-KEY-1LHZD5K7C0EVAA6G8CCYWRYKAMZN6FY5PR6QX8NMU958ZP3WCG3XSF2HGJM | age-keygen -y
age1466969q5tjnj9uapjxmqc6znul08te548ezcpwxwtj86junyrezq7mxfdm
``` 

## Disclaimer

This program does not overwrite the passphrase and/or other secrets in its memory.
I am not a cryptographer and have not done a code review of the third-party dependencies.
Use at own risk.
