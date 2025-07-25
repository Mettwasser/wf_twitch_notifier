pub mod config;
pub mod eidolon_hunts;
pub mod meta_relics;
pub mod placeholders;
mod register;
pub mod s_tier_arbitrations;
pub mod steel_path_disruption_fissures;

use crate::state::State;

pub trait Listener {
    fn run(state: State) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub use register::register_listeners;
