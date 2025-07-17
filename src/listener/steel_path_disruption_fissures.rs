use anyhow::Error;
use warframe::worldstate::{
    Change,
    MissionType,
    Tier,
    queryable::Fissure,
};

use crate::{
    listener::{
        Listener,
        placeholders,
    },
    placeholder::Placeholder,
    state::State,
};

pub struct SteelPathDisruptionFissuresListener;

impl Listener for SteelPathDisruptionFissuresListener {
    async fn run(state: State) -> anyhow::Result<()> {
        state
            .wf
            .call_on_nested_update_with_state::<_, Fissure, _>(callback, state.clone())
            .await
            .map_err(Error::from)
    }
}

async fn callback(state: State, fissure: &Fissure, change: Change) {
    if change == Change::Added
        && fissure.tier != Tier::Requiem
        && fissure.mission_key == MissionType::Disruption
        && fissure.is_hard
    {
        state
            .send_listener_response(
                &state.listener_cfg().steel_path_disruption_fissures.format,
                [&placeholders::Node(&fissure.node) as &dyn Placeholder],
            )
            .await
            .unwrap();
    }
}
