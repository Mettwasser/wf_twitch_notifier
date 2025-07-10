use anyhow::Error;
use warframe::worldstate::{
    Change,
    MissionType,
    Tier,
    queryable::Fissure,
};

use crate::{
    listener::Listener,
    placeholder::{
        self,
        Placeholder,
        apply_placeholders,
    },
    state::State,
};

pub struct SteelPathDisruptionFissuresListener;

impl Listener for SteelPathDisruptionFissuresListener {
    async fn run(
        State {
            client,
            wf,
            channel_name,
            ..
        }: State,
        fmt: String,
    ) -> anyhow::Result<()> {
        wf.call_on_nested_update::<Fissure, _>(async move |fissure, change| {
            if change == Change::Added
                && fissure.tier != Tier::Requiem
                && fissure.mission_key == MissionType::Disruption
                && fissure.is_hard
            {
                client
                    .say(
                        channel_name.to_string(),
                        apply_placeholders(
                            &fmt,
                            [
                                &channel_name as &dyn Placeholder,
                                &placeholder::Node(&fissure.node),
                            ],
                        ),
                    )
                    .await
                    .unwrap();
            }
        })
        .await
        .map_err(Error::from)
    }
}
