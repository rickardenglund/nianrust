use yew::prelude::*;

use crate::blooming;

use super::letterbox::*;
use gloo_net::http::Request;
use gloo_net::Error;
use std::ops::Deref;

#[function_component(Old)]
pub fn old() -> Html {
    let history = use_state(|| None);
    {
        let history = history.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_history: Result<blooming::HistoryResponse, Error> =
                        Request::get(blooming::history_url().as_str())
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await;

                    history.set(Some(fetched_history));
                });
                || ()
            },
            (),
        );
    }

    let resp = history.deref();
    match resp {
        None => html! {<h1>{"Chill i dill"}</h1>},
        Some(Err(e)) => html! {<><h1>{"åh nej... :("}</h1><p>{format!("Fel: \n {}", e)}</p></>},
        Some(Ok(h)) => html! {
            <>
                <div class="container">
                {h.days.iter().map(|d| html!{
                                <div class="historyday" >
                                   <h2>{format!("#{}",d.i)}</h2>
                                    <LetterBox letters={d.shuffle.clone()} />
                                </div>
                                            }
                            ).collect::<Html>()
                }
                </div>
            </>
        },
    }
}

