use crate::cli::GenPassOpts;
use rand::prelude::*;

const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBER: &str = "123456789";
const SYMBOL: &str = "!@#$%^&*_";
pub fn process_genpass(opts: GenPassOpts) -> anyhow::Result<String> {
    let mut password = String::new();
    let mut rng = thread_rng();
    let mut charset = String::new();
    if opts.uppercase {
        charset.push_str(UPPER);
        let x = rng.gen_range(0..UPPER.len());
        password.push(UPPER.as_bytes()[x] as char);
    }
    if opts.lowercase {
        charset.push_str(LOWER);
        let x = rng.gen_range(0..LOWER.len());
        password.push(LOWER.as_bytes()[x] as char);
    }
    if opts.number {
        charset.push_str(NUMBER);
        let x = rng.gen_range(0..NUMBER.len());
        password.push(NUMBER.as_bytes()[x] as char);
    }
    if opts.symbol {
        charset.push_str(SYMBOL);
        let x = rng.gen_range(0..SYMBOL.len());
        password.push(SYMBOL.as_bytes()[x] as char);
    }
    let charset = charset.as_bytes();
    for _ in 0..(opts.length - password.len() as u8) {
        let idx = rng.gen_range(0..charset.len());
        password.push(charset[idx] as char);
    }
    let mut password_chars = password.chars().collect::<Vec<char>>();
    password_chars.shuffle(&mut rng);

    password = password_chars.into_iter().collect::<String>();

    // print!("{}", password);

    Ok(password)
}
