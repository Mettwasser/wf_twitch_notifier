use std::sync::LazyLock;

use anyhow::Error;
use arbitration_data::model::mapped::MAP_RANKING;
use regex::Regex;
use warframe::worldstate::{
    Change,
    MissionType,
    queryable::Fissure,
};

use crate::{
    listener::{
        Listener,
        placeholders,
    },
    placeholder::Placeholder,
    state::State,
};

static NODE_MATCHER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*) \(").unwrap());

pub struct MetaRelicsListener;

impl Listener for MetaRelicsListener {
    async fn run(state: State) -> anyhow::Result<()> {
        state
            .wf
            .call_on_nested_update_with_state::<_, Fissure, _>(callback, state.clone())
            .await
            .map_err(Error::from)
    }
}

async fn callback(state: State, fissure: &Fissure, change: Change) {
    if change != Change::Added || fissure.mission_key != MissionType::Defense {
        return;
    }

    // fissure.node_key contains a formatted node, including the planet.
    // MAP_RANKING expects just the node name
    let Some(node) = extract_node(&fissure.node_key) else {
        return;
    };

    match MAP_RANKING.get(node) {
        Some(tier) if *tier == arbitration_data::Tier::S || *tier == arbitration_data::Tier::A => {
            state
                .send_listener_response(
                    &state.listener_cfg().meta_relics.format,
                    [
                        &placeholders::Node(&fissure.node) as &dyn Placeholder,
                        &placeholders::Difficulty {
                            is_hard: fissure.is_hard,
                        },
                    ],
                )
                .await
                .unwrap();
        }
        _ => (),
    }
}

fn extract_node(node: &str) -> Option<&str> {
    NODE_MATCHER
        .captures(node)
        .and_then(|captures| captures.get(1).map(|m| m.as_str()))
}

#[cfg(test)]
mod tests {
    use crate::listener::meta_relics::extract_node;

    #[test]
    fn test_node_matcher() {
        assert_eq!(extract_node("Yuvarium (Lua)"), Some("Yuvarium"));
        assert_eq!(extract_node("Nu-gua Mines (Neptune)"), Some("Nu-gua Mines"));
        assert_eq!(extract_node("R-9 Cloud (Veil)"), Some("R-9 Cloud"));
    }
}
