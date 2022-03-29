use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "0.0.0.0:8080", env = "VOLUME_ADDR")]
    pub addr: String,
}

pub fn parse() -> Args {
    let args = Args::parse();
    // initialize tracing
    tracing_subscriber::fmt::init();
    args
}
