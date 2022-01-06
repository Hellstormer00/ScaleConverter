use chord_calc::argparser::Opt;
use structopt::StructOpt;

fn main() {
    let args = Opt::from_args();
    println!("{}", args.scale);
}
