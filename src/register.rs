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
    let message_config = &state.config.clone().message_config;

    if message_config.eidolon_hunts.enabled {
        join_set.spawn(EidolonHuntListener::run(
            state.clone(),
            message_config.eidolon_hunts.format.clone(),
        ));
    }

    if message_config.s_tier_arbitrations.enabled {
        join_set.spawn(STierArbitrationListener::run(
            state.clone(),
            message_config.s_tier_arbitrations.format.clone(),
        ));
    }

    if message_config.meta_relics.enabled {
        join_set.spawn(MetaRelicsListener::run(
            state.clone(),
            message_config.meta_relics.format.clone(),
        ));
    }

    if message_config.steel_path_disruption_fissures.enabled {
        join_set.spawn(SteelPathDisruptionFissuresListener::run(
            state.clone(),
            message_config.steel_path_disruption_fissures.format.clone(),
        ));
    }

    Ok(())
}
