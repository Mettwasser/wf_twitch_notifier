pub mod eidolon_hunts;
pub mod meta_relics;
pub mod s_tier_arbitrations;
pub mod steel_path_disruption_fissures;

use crate::state::State;

pub trait Listener {
    fn run(state: State, fmt: String) -> impl Future<Output = anyhow::Result<()>> + Send;
}
