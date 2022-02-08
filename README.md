# age-keygen-deterministic

This is a very simple Rust CLI tool which you can use to deterministically derive an [age encryption](https://github.com/FiloSottile/age) keypair from a passphrase.

## Compile

Clone this repository and then run ```cargo run --release```.

## Usage

This executable takes no command line arguments at all.
If you use ```example-passphrase-do-not-use!``` as an insecure passphrase, you will get the following output:

```
$ cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/age-keygen-deterministic`
Enter passphrase: 
AGE-SECRET-KEY-1LHZD5K7C0EVAA6G8CCYWRYKAMZN6FY5PR6QX8NMU958ZP3WCG3XSF2HGJM
```

You can now get the public key for this private key as you would with any other age private key, too.
You simply have to pipe the private key to ```age-keygen -y```.
If you do this for the example above, you will get the following output:

```
$ cargo run --release | age-keygen -y
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/age-keygen-deterministic`
Enter passphrase: 
age1466969q5tjnj9uapjxmqc6znul08te548ezcpwxwtj86junyrezq7mxfdm
``` 

## Disclaimer

This program does not overwrite the passphrase in memory on exit.
I am not a cryptographer and have not done a code review of the third-party dependencies.
Use at own risk.
