#[derive(clap::Parser)]
pub struct Config {
    #[clap(short, env)]
    pub env: String,

    #[clap(short, env)]
    pub database_url: String,
}
