use yew::prelude::*;

use crate::{blooming, Route};
use yew_router::prelude::*;

use super::letterbox::*;
use gloo_net::http::Request;
use gloo_net::Error;
use std::collections::HashMap;
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

    let nav_history = use_history().unwrap();

    let resp = history.deref();
    match resp {
        None => html! {<h1>{"Chill i dill"}</h1>},
        Some(Err(e)) => html! {<><h1>{"åh nej... :("}</h1><p>{format!("Fel: \n {}", e)}</p></>},
        Some(Ok(h)) => html! {
            <>
                <div class="container">
                {h.days.iter().map(|d| {
                                let nav_history = nav_history.clone();
                                let shuffle = d.shuffle.clone();
                                let onclick_callback = Callback::from(move |_| {
                                    let mut query_params: HashMap<String, String> = HashMap::new();
                                    query_params.insert("letters".to_owned(), shuffle.clone());

                                    nav_history
                                        .push_with_query(Route::Solution, &query_params)
                                        .unwrap();
                                });

                                html!{
                                    <div onclick={onclick_callback} class="historyday" >
                                       <h2>{format!("#{}",d.i)}</h2>
                                        <LetterBox letters={d.shuffle.clone()} />
                                    </div>
                                }
                            }).collect::<Html>()
                }
                </div>
            </>
        },
    }
}
