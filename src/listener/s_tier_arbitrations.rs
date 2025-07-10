use chrono::Utc;

use crate::{
    listener::Listener,
    placeholder,
    placeholder::{
        Placeholder,
        apply_placeholders,
    },
    state::State,
};

pub struct STierArbitrationListener;

impl Listener for STierArbitrationListener {
    async fn run(
        State {
            client,
            arbi_data,
            channel_name,
            ..
        }: State,
        fmt: String,
    ) -> anyhow::Result<()> {
        while let Ok(next_arbi) = arbi_data.upcoming_by_tier(arbitration_data::Tier::S) {
            if next_arbi.activation > Utc::now() {
                tracing::info!(time_to_sleep = ?(next_arbi.activation - Utc::now()).to_std()?, upcoming_arbi = ?next_arbi);
                tokio::time::sleep((next_arbi.activation - Utc::now()).to_std()?).await;
            }

            client
                .say(
                    channel_name.to_string(),
                    apply_placeholders(
                        &fmt,
                        [
                            &channel_name as &dyn Placeholder,
                            &placeholder::Node(&next_arbi.node),
                            &placeholder::Planet(&next_arbi.planet),
                        ],
                    ),
                )
                .await?;
        }

        Ok(())
    }
}
