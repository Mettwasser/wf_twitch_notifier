use chrono::{
    DateTime,
    Utc,
};
use clap::{
    Parser,
    Subcommand,
};

fn parse_datetime_utc(s: &str) -> Result<DateTime<Utc>, String> {
    let format = "%Y-%m-%d %H:%M:%S%.f %z %Z";

    // 1. Parse the string with its timezone info
    // 2. On success, convert it to UTC and return
    DateTime::parse_from_str(s, format)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| e.to_string())
}

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

        /// The client access token
        #[arg(short, long)]
        access_token: String,

        /// The client refresh token
        #[arg(short, long)]
        refresh_token: String,

        /// The client expires at
        #[arg(short, long, value_parser = parse_datetime_utc)]
        expires_at: DateTime<Utc>,
    },
    Run {
        /// The Twitch channel to run the bot on
        channel_name: String,
    },
}
