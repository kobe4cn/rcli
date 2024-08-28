// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    Subcommand,
};

fn main() -> anyhow::Result<()> {
    // let mut op= math_op("add");
    // let (x,y)=(10,20);
    // println!("{}+{}={}",x,y,op(x,y));
    // op=math_op("sub");
    // println!("{}-{}={}",x,y,op(x,y));

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
        Subcommand::GenPass(opts) => process_genpass(opts)?,
        Subcommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
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
