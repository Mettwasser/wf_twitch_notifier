use anyhow::Error;
use warframe::worldstate::{
    CetusState,
    queryable::Cetus,
};

use crate::{
    listener::Listener,
    placeholder::apply_placeholders,
    state::State,
};

pub struct EidolonHuntListener;

impl Listener for EidolonHuntListener {
    async fn run(
        State {
            client,
            wf,
            channel_name,
            ..
        }: State,
        fmt: String,
    ) -> anyhow::Result<()> {
        wf.call_on_update::<Cetus, _>(async move |_, cetus| {
            if cetus.state == CetusState::Night {
                client
                    .say(
                        channel_name.to_string(),
                        apply_placeholders(&fmt, [&channel_name]),
                    )
                    .await
                    .unwrap();
            }
        })
        .await
        .map_err(Error::from)
    }
}
