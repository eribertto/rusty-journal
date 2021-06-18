// june 18/21 start in new branch regolith
// to complete this exercise

mod cli;
use structopt::StructOpt;

//fn main() {
//    cli::CommandLineArgs::from_args();
//}
fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}