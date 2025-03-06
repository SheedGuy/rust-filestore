

#[derive(clap::Parser)]
pub struct Config {
    #[clap(env)]
    pub database_url: String
}