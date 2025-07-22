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

#[derive(Clone, Debug, Default)]
struct Filters {
    pub mod_rank: Option<u8>,
}

fn extract_filters(input: String) -> Result<Filters, CommandError> {
    if input.is_empty() {
        return Err(CommandError::Client(
            "Signalled filter, but filters are empty".to_owned(),
        ));
    }

    let mut filters = Filters::default();

    for filter in input.split_whitespace() {
        let Some(prefix) = filter.chars().next() else {
            unreachable!("Length checked before");
        };

        match prefix {
            'r' => {
                let rank_str = filter.get(1..).unwrap();

                if rank_str.is_empty() {
                    return Err(CommandError::Client(
                        "Expected numbers after the `r`".to_owned(),
                    ));
                }

                let Ok(rank) = rank_str.parse::<u8>() else {
                    return Err(CommandError::Client(format!("{rank_str} is not a number!")));
                };

                filters.mod_rank = Some(rank)
            }
            _ => return Err(CommandError::Client(format!("Invalid filter: {filter}"))),
        }
    }

    Ok(filters)
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
        let input = args.join(" ");
        let mut parts = input.splitn(2, "||");
        let queried_item = parts
            .next()
            .expect("Queried item can't be empty")
            .trim()
            .to_owned();

        let maybe_filters = parts
            .next()
            .map(|s| s.trim().to_owned())
            .map(extract_filters)
            .transpose()?;

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

        let statistics = reqwest::get(format!(
            "https://api.warframe.market/v1/items/{corrected_item_slug}/statistics"
        ))
        .await
        .context("Request to WFM statistics failed")?
        .json::<Statistics>()
        .await
        .context("Deserializing the json failed")?
        .payload
        .statistics_closed
        .the_48_hours;

        if statistics.is_empty() {
            return Err(CommandError::Client(format!(
                "{name} hasn't had any sales in the last 48 hours!"
            )));
        }

        let has_mod_rank = statistics.first().unwrap().mod_rank.is_some();

        let statistics = match maybe_filters {
            Some(filters) => {
                // this is reversed to easily get the last, entry, as this is the most recent one
                let mut iter: Box<dyn Iterator<Item = Statistic>> =
                    Box::new(statistics.into_iter().rev());

                if has_mod_rank {
                    let rank = filters.mod_rank.unwrap_or(0);
                    iter =
                        Box::new(iter.filter(move |statistic| statistic.mod_rank.unwrap() == rank));
                }

                let statistics = iter.collect::<Vec<_>>();

                if statistics.is_empty() {
                    return Err(CommandError::Client(format!(
                        "{name} hasn't had any sales in the last 48 hours! (filters applied)"
                    )));
                }

                statistics
            }
            None => {
                if has_mod_rank {
                    statistics
                        .into_iter()
                        .rev()
                        .filter(|stat| stat.mod_rank.unwrap() == 0)
                        .collect::<Vec<_>>()
                } else {
                    statistics
                }
            }
        };

        let average = statistics.first().unwrap().avg_price;
        let moving_average = statistics.first().unwrap().moving_avg;
        let amount_sold: u32 = statistics.iter().map(|stat| stat.volume).sum();

        state
            .send_command_response(
                &state.config.command_config.average_command.format,
                author,
                [
                    &placeholders::Average(average.to_string()) as &dyn Placeholder,
                    &placeholders::MovingAverage(
                        moving_average
                            .map(|avg| avg.to_string())
                            .unwrap_or_else(|| "unknown".to_owned()),
                    ) as &dyn Placeholder,
                    &placeholders::ItemName(&name),
                    &placeholders::AmountSold(amount_sold.to_string()),
                ],
            )
            .await?;

        Ok(())
    }
}

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub payload: Payload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    pub statistics_closed: StatisticsClosed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatisticsClosed {
    #[serde(rename = "48hours")]
    pub the_48_hours: Vec<Statistic>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistic {
    pub datetime: String,

    pub volume: u32,

    pub closed_price: i64,

    pub avg_price: f64,

    pub moving_avg: Option<f64>,

    pub mod_rank: Option<u8>,
}
