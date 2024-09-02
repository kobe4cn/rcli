// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{CmdExcetor, Opts};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await?;

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
