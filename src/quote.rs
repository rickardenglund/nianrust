use yew::prelude::*;
use gloo_net::http::Request;

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct QuoteResponse {
    pub text:String,
    pub author:String,
}

#[function_component(Quote)]
pub fn daily_quote() -> html {
    let q = use_state(|| QuoteResponse { text: "-".to_owned(), author: "?".to_owned() });
    {
        let q = q.clone();
        use_effect_with_deps(move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_quote: QuoteResponse = Request::get("https://citat.info/api/json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    q.set(fetched_quote);
                });

            ||()
        }, ());
    }

    html!{
        <div class="quote">
            <p>{format!("\"{}\"",q.text)}</p>
            <p>{format!("- {}", q.author)}</p>
            <a href="citat.info">{"citat.info"}</a>
        </div>
    }
}
