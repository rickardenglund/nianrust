use yew::prelude::*;

use crate::blooming;
use gloo_net::http::Request;
use gloo_net::Error;
use std::ops::Deref;

use super::blooming::Day;
use super::grades::*;
use super::letterbox::*;

#[function_component(Today)]
pub fn today() -> Html {
    let day = use_state(|| None);
    {
        let day = day.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_day: Result<Day, Error> =
                        Request::get(blooming::today_url().as_str())
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await;

                    day.set(Some(fetched_day));
                });
                || ()
            },
            (),
        );
    }

    let resp = day.deref();
    match resp {
        None => html! {<h1>{"Chill i dill"}</h1>},
        Some(Err(e)) => html! {<><h1>{"åh nej... :("}</h1><p>{format!("Fel: \n {}", e)}</p></>},
        Some(Ok(d)) => html! {
            <>
            <h1>{format!("Dagens Bokstäver #{}", d.puzzle_number)}</h1>
                <div class="container">
                    <LetterBox letters={d.letters.clone()} />
                    <Grades total_words={d.n_words.clone()} />
                    // <Quote />
                </div>
            </>
        },
    }
}

