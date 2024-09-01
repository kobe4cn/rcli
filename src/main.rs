use std::fs;

// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_decrypt, process_encode, process_encrypt, process_genpass,
    process_jwt_sign, process_jwt_verify, process_key_generate, process_sign, process_verify,
    Base64SubCommand, JwtSubCommand, Opts, Subcommand, TextSubCommand,
};

use zxcvbn::zxcvbn;
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, &opts.format)?
        }
        Subcommand::GenPass(opts) => {
            let password = process_genpass(opts)?;
            println!("{}", password);
            let estimate = zxcvbn(&password, &[]);
            eprintln!("{:?}", estimate.score());
        }
        Subcommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64SubCommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                println!("{}", decode);
            }
        },
        Subcommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(opts) => {
                let ret = process_verify(&opts.input, &opts.key, &opts.signature, opts.format)?;
                println!("{}", ret);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_key_generate(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = &opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    rcli::TextSignFormat::ED25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
                println!("{:?}", key);
            }
            TextSubCommand::Encrypt(opts) => {
                let ret = process_encrypt(&opts.input, &opts.key, &opts.nonce)?;
                println!("{}", ret);
            }
            TextSubCommand::Decrypt(opts) => {
                let ret = process_decrypt(&opts.input, &opts.key, &opts.nonce)?;
                println!("{}", ret);
            }
            TextSubCommand::Chakey(opts) => {
                let key = rcli::process_chacha_key_generate()?;
                println!("{:?}", key);
                let name = &opts.output.join("chachakey.txt");
                let nonce = &opts.output.join("nonce.txt");
                fs::write(name, &key[0])?;
                fs::write(nonce, &key[1])?;
                println!("key: {:?}", key[0]);
                println!("nonce: {:?}", key[1]);
            }
        },
        Subcommand::Jwt(subcmd) => match subcmd {
            JwtSubCommand::Sign(opts) => {
                // println!("{:?}", opts);
                let ret = process_jwt_sign(&opts.sub, &opts.aud, &opts.key, &opts.exp)?;
                println!("{}", ret);
            }
            JwtSubCommand::Verify(opts) => {
                let ret = process_jwt_verify(&opts.token, &opts.key)?;
                println!("{:?}", ret);
                // let ret = process_jwt_verify(&opts)?;
                // println!("{}", ret);
            }
        },
    }

    // print!("{:?}", opts);
    Ok(())
}

// type MathOp=fn(i32,i32)->i32;
// fn math_op(op:&str)->MathOp{
//     match op{
//         "add"=>add,

//         _=>subtract,
//     }
// }
// fn add(a:i32,b:i32)->i32{
//     a+b
// }
// fn subtract(a:i32,b:i32)->i32{
//     a-b
// }
