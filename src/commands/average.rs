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
        placeholders::{
            self,
        },
    },
    placeholder::Placeholder,
    state::State,
};

fn find_best_match(query: String, candidates: Arc<[ItemShort]>) -> Option<(String, String)> {
    candidates
        .iter()
        .max_by_key(|&candidate| {
            (jaro_winkler(&query, &candidate.i18n.get(&Language::En).unwrap().name) * 1000.0) as i32
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
        ArgumentLength::Variadic
    }

    async fn invoke(&self, state: State, author: &str, args: &[&str]) -> Result<(), CommandError> {
        let queried_item = args.join(" ");

        let items = state
            .wfm
            .items(Language::En)
            .await
            .context("Failed to fetch items")?;

        let Some((corrected_item_slug, name)) =
            tokio::task::spawn_blocking(move || find_best_match(queried_item.to_owned(), items))
                .await
                .map_err(|e| CommandError::Server(e.into()))?
        else {
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
            .send_command_response(
                &state.config.command_config.average_command.format,
                author,
                [
                    &placeholders::Average(average.to_string()) as &dyn Placeholder,
                    &placeholders::MovingAverage(moving_average.to_string()) as &dyn Placeholder,
                    &placeholders::ItemName(&name),
                ],
            )
            .await?;

        Ok(())
    }
}
