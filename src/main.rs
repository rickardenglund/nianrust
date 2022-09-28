use std::ops::Deref;

use yew::{prelude::*};
use gloo_net::http::Request;

use gloo_net::Error;

mod letterbox;
use letterbox::*;

mod grades;
use grades::*;

mod day;
use crate::day::Day;

#[function_component(App)]
fn app() -> Html {
    let day = use_state(|| None);
    {
        let day = day.clone();
        use_effect_with_deps(move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_day:Result<Day, Error> = Request::get("https://blooming-hollows-80357.herokuapp.com/today")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await;
                        
                    day.set(Some(fetched_day));
                });
            ||()
        }, ());
    }

        let resp = day.deref();
        match resp {
            None => html!{<h1>{"Chill i dill"}</h1>},
            Some(Err(e)) => html!{<><h1>{"åh nej... :("}</h1><p>{format!("Fel: \n {}", e)}</p></>},
            Some(Ok(d))=> html! {
                <>
                <h1>{format!("Dagens Bokstäver #{}", d.puzzle_number)}</h1>
                    <div class="container">
                        <LetterBox letters={d.letters.clone()} />
                        <Grades total_words={d.n_words.clone()} />
                        // <Quote />
                    </div>
                </>
            }
        }
}



fn main() {
    yew::start_app::<App>();
}
