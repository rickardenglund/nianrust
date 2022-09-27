use yew::prelude::*;
use gloo_net::http::Request;

mod letterbox;
use letterbox::*;

mod grades;
use grades::*;

mod day;
use crate::day::Day;

#[function_component(App)]
fn app() -> Html {
    let day = use_state(|| day::new());
    {
        let day = day.clone();
        use_effect_with_deps(move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_day: Day = Request::get("https://blooming-hollows-80357.herokuapp.com/today")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    day.set(fetched_day);
                });

            ||()
        }, ());
    }

    html! {
        <>
        <h1>{format!("Dagens Bokstäver #{}", day.puzzle_number)}</h1>
            <div class="container">
                <LetterBox letters={day.letters.clone()} />
                <Grades total_words={day.n_words.clone()} />
                // <Quote />
            </div>
        </>
    }
}



fn main() {
    yew::start_app::<App>();
}
