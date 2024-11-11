#![allow(non_snake_case)]

use itertools::Itertools;

use serde::{Deserialize, Serialize};

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

use reqwest;

use snowy_model::TeamView;

use crate::error::Error;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct FetchTeamStateArgs<'a> {
    api_base_url: &'a str,
    team_id: &'a str,
}

async fn fetch_team_state(api_base_url: &str, team_id: &str) -> Result<TeamView, Error> {
    Ok(reqwest::get(&format!("{}/team/{}", api_base_url, team_id))
        .await?
        .json::<TeamView>()
        .await?)
}

pub fn App() -> Element {
    let api_base_url = "http://localhost:8000/api";
    let team_id = "team-1";

    let mut team_state =
        use_resource(move || async move { fetch_team_state(api_base_url, team_id).await });

    rsx! {
        link { rel: "stylesheet", href: "styles.css" }
        main { class: "container",

            div { class: "row",
                div { class: "col",
                    h2 { "Team Weather" }
                    h4 { "Lets see how your team is feeling... the weather." }
                }
            }

            div { class: "row",
                div { class: "col",
                    button { class: "btn", onclick: move |_| team_state.restart(), "Refresh Team State" }
                }
            }

            div { class: "row",
                div { class: "card",
                    div { class: "row",
                        div { class: "col",
                            h3 { "Average Temperatures" }
                            match &*team_state.read() {
                                Some(Ok(state)) => rsx! {
                                    match &state.avg_minimum_temperature {
                                        Some(temp) => rsx! { p { {format!("Min: {}°C", temp.0)} } },
                                        None => rsx! { p { "Min: ---" } },
                                    }
                                    match &state.avg_maximum_temperature {
                                        Some(temp) => rsx! { p { {format!("Max: {}°C", temp.0)} } },
                                        None => rsx! { p { "Max: ---" } },
                                    }
                                },
                                Some(Err(_)) => rsx! { p { {format!("Error getting temperatures")} } },
                                None => rsx! { p { "Loading..." } },
                            }
                        }
                        div { class: "col",
                            h3 { "Top 3 Weather Conditions" }
                            match &*team_state.read() {
                                Some(Ok(state)) => rsx! {
                                    for (weather_code , count) in state.weather_condition_distribution
                                        .iter()
                                        .sorted_by_key(|(_, &count)| count)
                                        .take(3)
                                    {
                                        p { {format!("{:?}: {}", weather_code, count)} }
                                    }
                                },
                                Some(Err(_)) => rsx! { p { {format!("Error getting weather conditions")} } },
                                None => rsx! { p { "Loading..." } },
                            }
                        }
                    }
                }
            }

            div { class: "row",
                div { class: "card-grid",
                    div { class: "card",
                        h3 { "Sunny Dev" }
                        h4 { "28°C" }
                        p { "Tropical Paradise" }
                    }
                    div { class: "card",
                        h3 { "Rainy Coder" }
                        h4 { "15°C" }
                        p { "Misty Mountains" }
                    }
                    div { class: "card",
                        h3 { "Snowy Tester" }
                        h4 { "-2°C" }
                        p { "Arctic Circle" }
                    }
                }
            }

            div { class: "row",
                div { class: "col",
                    h3 { "Team State Debugging" }
                    pre { {serde_json::to_string_pretty(&*team_state.read()).unwrap()} }
                }
            }
        }
    }
}
