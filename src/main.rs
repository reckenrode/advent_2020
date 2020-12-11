use clap::Clap;

mod solutions;

#[derive(Clap)]
#[clap(author, about, version)]
struct Options {
    #[clap(subcommand)]
    solution: solutions::Solution
}

fn main() {
    let opts = Options::parse();
    opts.solution.run()
}
