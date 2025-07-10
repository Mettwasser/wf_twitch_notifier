use tokio::task::JoinSet;

use crate::{
    listener::{
        Listener,
        eidolon_hunts::EidolonHuntListener,
        meta_relics::MetaRelicsListener,
        s_tier_arbitrations::STierArbitrationListener,
        steel_path_disruption_fissures::SteelPathDisruptionFissuresListener,
    },
    state,
};

pub fn register_listeners(
    join_set: &mut JoinSet<Result<(), anyhow::Error>>,
    state: state::State,
) -> anyhow::Result<()> {
    let config = state.config.clone();

    if config.eidolon_hunts.enabled {
        join_set.spawn(EidolonHuntListener::run(
            state.clone(),
            config.eidolon_hunts.format.clone(),
        ));
    }

    if config.s_tier_arbitrations.enabled {
        join_set.spawn(STierArbitrationListener::run(
            state.clone(),
            config.s_tier_arbitrations.format.clone(),
        ));
    }

    if config.meta_relics.enabled {
        join_set.spawn(MetaRelicsListener::run(
            state.clone(),
            config.meta_relics.format.clone(),
        ));
    }

    if config.steel_path_disruption_fissures.enabled {
        join_set.spawn(SteelPathDisruptionFissuresListener::run(
            state.clone(),
            config.steel_path_disruption_fissures.format.clone(),
        ));
    }

    Ok(())
}
