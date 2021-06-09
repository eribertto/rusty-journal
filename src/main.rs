mod cli;
use structopt::StructOpt;

//fn main() {
//    cli::CommandLineArgs::from_args();
//}
fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}