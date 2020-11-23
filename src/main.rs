// use std::env;
use structopt::StructOpt;

mod calculation;

#[derive(Debug, StructOpt)]
#[structopt(name = "numerika", about = "Execute a calculation.", long_about = r#"
♪♫ We're all living in Numerika
Numerika ist wunderbar
We're all living in Numerika
Numerika, Numerika ♬
"#)]
struct NumerikaOpt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,

    /// Calculation: the calculation that should be executed.
    #[structopt(name = "CALCULATION")]
    calculation: String,
}

fn main() {
    let opt = NumerikaOpt::from_args();
    let result = calculation::parse_and_compute(opt.calculation, opt.verbosity);
    match result {
        Ok(r) => println!("{:?}", r),
        Err(err) => println!("{:?}", err),
    }
}
