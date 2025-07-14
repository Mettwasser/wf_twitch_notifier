use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use textdistance::nstr::jaro_winkler;
use warframe::market::{
    ItemShort,
    Language,
};

use crate::{
    commands::{
        ArgumentLength,
        Command,
        CommandError,
    },
    state::State,
};

fn find_best_match(query: &str, candidates: Arc<[ItemShort]>) -> Option<(String, String)> {
    candidates
        .iter()
        .max_by_key(|&candidate| {
            (jaro_winkler(query, &candidate.i18n.get(&Language::En).unwrap().name) * 1000.0) as i32
        })
        .map(|s| {
            (
                s.slug.clone(),
                s.i18n.get(&Language::En).unwrap().name.clone(),
            )
        })
}

pub struct Average;

#[async_trait]
impl Command for Average {
    fn command_prefix(&self) -> &'static str {
        "!avg"
    }

    fn length(&self) -> ArgumentLength {
        ArgumentLength::Flexible
    }

    async fn invoke(&self, state: State, author: &str, args: &[&str]) -> Result<(), CommandError> {
        let queried_item = args.join(" ");

        let items = state
            .wfm
            .items(Language::En)
            .await
            .context("Failed to fetch items")?;

        let Some((corrected_item_slug, name)) = find_best_match(&queried_item, items) else {
            return Err(CommandError::Client(
                "Couldn't find the item you're looking for!".to_owned(),
            ));
        };

        let item = reqwest::get(format!(
            "https://api.warframe.market/v1/items/{corrected_item_slug}/statistics"
        ))
        .await
        .context("Request to WFM statistics failed")?
        .json::<serde_json::Value>()
        .await
        .context("Deserializing the json failed")?;

        let average = &item["payload"]["statistics_closed"]["48hours"][0]["avg_price"];
        let moving_average = &item["payload"]["statistics_closed"]["48hours"][0]["moving_avg"];

        state
            .client
            .say(
                state.channel_name.to_string(),
                format!(
                    r#"@{author} "{name}" average: {average} || moving average: {moving_average}"#
                ),
            )
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }
}
