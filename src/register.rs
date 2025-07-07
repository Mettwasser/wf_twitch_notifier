use std::sync::Arc;

use anyhow::Error;
use arbitration_data::{
    ArbitrationData,
    model::mapped::MAP_RANKING,
};
use chrono::Utc;
use tokio::task::JoinSet;
use twitch_irc::{
    TwitchIRCClient,
    login::RefreshingLoginCredentials,
    transport::tcp::{
        TCPTransport,
        TLS,
    },
};
use warframe::worldstate::{
    self,
    CetusState,
    Change,
    MissionType,
    queryable::{
        Cetus,
        Fissure,
    },
};

use crate::{
    config::Config,
    token_storage::SimpleTokenStorage,
};

pub fn register_listeners(
    join_set: &mut JoinSet<Result<(), anyhow::Error>>,
    config: Config,
    channel_name: String,
    client: TwitchIRCClient<TCPTransport<TLS>, RefreshingLoginCredentials<SimpleTokenStorage>>,
    wf: worldstate::Client,
    arbi_data: ArbitrationData,
) -> anyhow::Result<()> {
    let arbi_data = Arc::new(arbi_data);

    if config.eidolon_hunt_message {
        let channel_name = channel_name.clone();
        let client = client.clone();
        let wf = wf.clone();
        join_set.spawn(async move {
            wf.call_on_update::<Cetus, _>(async move |_, cetus| {
                if cetus.state == CetusState::Night {
                    client
                        .say(
                            channel_name.clone(),
                            format!(
                                "🌙🌙🌙 Hey @{}, swing yo' ass over to Cetus! It's EIDOLON TIME!",
                                channel_name
                            ),
                        )
                        .await
                        .unwrap();
                }
            })
            .await
            .map_err(Error::from)
        });
    }

    if config.arbitration_s_tier_message {
        let channel_name = channel_name.clone();
        let client = client.clone();
        let arbi_data = arbi_data.clone();

        join_set.spawn(async move {
            while let Ok(next_arbi) = arbi_data.upcoming_by_tier(arbitration_data::Tier::S) {
                if next_arbi.activation > Utc::now() {
                    tracing::info!(time_to_sleep = ?(next_arbi.activation - Utc::now()).to_std()?, upcoming_arbi = ?next_arbi);
                    tokio::time::sleep((next_arbi.activation - Utc::now()).to_std()?).await;
                }

                client
                    .say(
                        channel_name.clone(),
                        format!(
                            "💰🤑 @{}, new S-Tier Arbitration: {} on {}",
                            channel_name, next_arbi.node, next_arbi.planet
                        ),
                    )
                    .await?;
            }

            Ok(())
        });
    }

    if config.relic_meta_and_disruption_message {
        let channel_name = channel_name.clone();
        let client = client.clone();
        let wf = wf.clone();

        join_set.spawn(async move {
            Ok(wf
                .call_on_nested_update::<Fissure, _>(async move |fissure, change| {
                    if change == Change::Added {
                        if fissure.mission_key == MissionType::Defense {
                            let node = fissure.node_key.split(' ').collect::<Vec<_>>()[0];

                            match MAP_RANKING.get(node) {
                                Some(tier)
                                    if *tier == arbitration_data::Tier::S
                                        || *tier == arbitration_data::Tier::A =>
                                {
                                    client
                                        .say(
                                            channel_name.clone(),
                                            format!(
                                                "🔍 New {}-Tier Meta Fissure detected: {}{}",
                                                tier,
                                                fissure.node,
                                                if fissure.is_hard { " (STEEL PATH)" } else { "" }
                                            ),
                                        )
                                        .await
                                        .unwrap();
                                }
                                _ => (),
                            }
                        }
                    } else if fissure.mission_key == MissionType::Disruption && fissure.is_hard {
                        client
                            .say(
                                channel_name.clone(),
                                format!(
                                    "⚡ New Steel Path Disruption detected on {}",
                                    fissure.node
                                ),
                            )
                            .await
                            .unwrap();
                    }
                })
                .await?)
        });
    }

    Ok(())
}
