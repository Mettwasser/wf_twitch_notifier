use anyhow::Error;
use warframe::worldstate::{
    CetusState,
    queryable::Cetus,
};

use crate::{
    listener::Listener,
    placeholder::Placeholder,
    state::State,
};

pub struct EidolonHuntListener;

impl Listener for EidolonHuntListener {
    async fn run(state: State) -> anyhow::Result<()> {
        state
            .wf
            .call_on_update_with_state::<_, Cetus, _>(callback, state.clone())
            .await
            .map_err(Error::from)
    }
}

async fn callback(state: State, _before: &Cetus, cetus: &Cetus) {
    if cetus.state == CetusState::Night {
        state
            .send_listener_response::<&dyn Placeholder>(
                &state.listener_cfg().eidolon_hunts.format,
                [],
            )
            .await
            .unwrap();
    }
}
