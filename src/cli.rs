use clap::{
    Parser,
    Subcommand,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {
        /// The client id
        #[arg(short, long)]
        id: String,

        /// The client secret
        #[arg(short, long)]
        secret: String,
    },
    Run {
        /// The Twitch channel to run the bot on
        channel_name: String,
    },
}
