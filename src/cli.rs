use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "Weather CLI")]
#[clap(about = "A CLI tool to fetch current weather information", long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub city: String,
}