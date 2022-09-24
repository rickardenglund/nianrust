use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;


#[derive(Clone, PartialEq, Deserialize)]
struct Day {
    #[serde(rename = "Letters")]
    letters: String,
    #[serde(rename = "PuzzleNumber")]
    puzzle_number:u32,
    #[serde(rename = "NWords")]
    n_words:u16,
}


#[function_component(App)]
fn app() -> Html {
    let day = use_state(|| Day{letters:"Laddar".to_owned(), puzzle_number: 0, n_words:0});
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
            <h1>{format!("#{}", day.puzzle_number)}</h1>
            <LetterBox letters={day.letters.clone()}/>
            <p>{format!("target: {}", day.n_words)}</p>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct LetterBoxProps {
    letters: String,
}


#[function_component(LetterBox)]
fn letter_box(LetterBoxProps {letters}:&LetterBoxProps) -> Html {
    let letters = letters.chars().map(|l| html!{
        <div>{l}</div>
    }).collect::<Html>();

    html!{
        <div class="letterbox">
            {letters}
        </div>
    }
}


fn main() {
    yew::start_app::<App>();
}
