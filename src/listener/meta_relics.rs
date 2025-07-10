use anyhow::Error;
use arbitration_data::model::mapped::MAP_RANKING;
use warframe::worldstate::{
    Change,
    MissionType,
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

pub struct MetaRelicsListener;

impl Listener for MetaRelicsListener {
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
            if change != Change::Added || fissure.mission_key != MissionType::Defense {
                return;
            }

            let node = fissure.node_key.split(' ').collect::<Vec<_>>()[0];

            match MAP_RANKING.get(node) {
                Some(tier)
                    if *tier == arbitration_data::Tier::S || *tier == arbitration_data::Tier::A =>
                {
                    client
                        .say(
                            channel_name.to_string(),
                            apply_placeholders(
                                &fmt,
                                [
                                    &channel_name as &dyn Placeholder,
                                    &placeholder::Node(&fissure.node),
                                    &placeholder::Difficulty(fissure.is_hard),
                                ],
                            ),
                        )
                        .await
                        .unwrap();
                }
                _ => (),
            }
        })
        .await
        .map_err(Error::from)
    }
}
