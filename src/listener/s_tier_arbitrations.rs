use chrono::Utc;

use crate::{
    listener::{
        Listener,
        placeholders,
    },
    placeholder::Placeholder,
    state::State,
};

pub struct STierArbitrationListener;

impl Listener for STierArbitrationListener {
    async fn run(state: State) -> anyhow::Result<()> {
        while let Ok(next_arbi) = state.arbi_data.upcoming_by_tier(arbitration_data::Tier::S) {
            if next_arbi.activation > Utc::now() {
                tracing::info!(time_to_sleep = ?(next_arbi.activation - Utc::now()).to_std()?, upcoming_arbi = ?next_arbi);
                tokio::time::sleep((next_arbi.activation - Utc::now()).to_std()?).await;
            }

            state
                .send_listener_response(
                    &state.listener_cfg().s_tier_arbitrations.format,
                    [
                        &placeholders::Node(&next_arbi.node) as &dyn Placeholder,
                        &placeholders::Planet(&next_arbi.planet),
                    ],
                )
                .await?;
        }

        Ok(())
    }
}
