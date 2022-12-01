use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "zanim",
    about = "This is a tool to wrap all the cli management tools"
)]
struct Opt {
    #[structopt(name = "tools", short)]
    tool: String,
    #[structopt(name = "mode", short)]
    mode: String,
}
fn main() {
    let opt = Opt::from_args();
    let mode = &opt.mode[..];
}
