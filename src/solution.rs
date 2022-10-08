use yew::prelude::*;
use yew_router::prelude::{use_location, Location};

use crate::blooming;
use crate::letterbox::LetterBox;

use gloo_net::http::Request;
use gloo_net::Error;
use std::collections::HashMap;
use std::ops::Deref;

#[function_component(Solution)]
pub fn solution() -> Html {
    let history = use_state(|| None);

    let loc = use_location().unwrap();
    let q = loc.query::<HashMap<String, String>>().unwrap();
    let letters = match q.get("letters") {
        None => "aaaaaaaaa".to_owned().clone(),
        Some(ls) => ls.clone(),
    };

    let l2 = letters.clone();

    {
        let history = history.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_history: Result<blooming::SolutionResponse, Error> =
                        Request::get(&blooming::solve_url(&l2, "s").as_str())
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
        Some(Ok(h)) => {
            let gs = g(h);
            let gg = group(&gs);

            let letter_lists: Html = gg
                .iter()
                .map(|(k, v)| {
                    html! {
                        <div class="wordlist">
                            <h3>{k}</h3>
                            <ul>
                                {v.iter().map(|v| {html!{<li>{v}</li>}}).collect::<Html>()}
                            </ul>
                        </div>

                    }
                })
                .collect();
            html! {
                <>
                    <div class="container">
                        <LetterBox letters={letters.clone()} />
                        <p class="infotext">
                            {format!("Det finns {} ord med bokstäverna  i {}", h.iter().count(), letters)}
                        </p>
                        {letter_lists}
                    </div>
                </>
            }
        }
    }
}

fn g(words: &Vec<String>) -> Vec<&str> {
    words.iter().map(|s| s.as_str()).collect()
}

fn group<'a>(strings: &Vec<&'a str>) -> HashMap<char, Vec<&'a str>> {
    let mut h = HashMap::new();

    {
        for s in strings {
            let first = s.chars().nth(0).unwrap();

            h = app(first, s, h);
        }
    }

    h
}

fn app<'a>(c: char, s: &'a str, mut m: HashMap<char, Vec<&'a str>>) -> HashMap<char, Vec<&'a str>> {
    match m.get(&c) {
        None => {
            let v = vec![s];
            m.insert(c, v);
        }
        Some(vs) => {
            let mut newvs = vs.clone();
            newvs.push(s);
            m.insert(c, newvs);
        }
    }

    m
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_group() {
        let v = vec!["apa", "bepa"];
        let groups = group(&v);

        let agroup = groups.get(&'a').unwrap();
        assert_eq!(1, agroup.iter().count());

        let bgroup = groups.get(&'b').unwrap();
        assert_eq!(1, bgroup.iter().count());
    }

    #[test]
    fn test_two_in_one() {
        let v = vec!["apa", "anka"];
        let groups = group(&v);

        let agroup = groups.get(&'a').unwrap();
        assert_eq!(2, agroup.iter().count());
    }

    #[test]
    fn test_app() {
        let mut m = HashMap::new();
        let v = vec!["apa"];
        m.insert('a', v);

        let s = "anka";
        let updated_m = app('a', s, m);

        let a_list = updated_m.get(&'a').unwrap();
        assert_eq!(2, a_list.len());

        let and = "and";
        let updated_m2 = app('a', and, updated_m);

        let a_list_2 = updated_m2.get(&'a').unwrap();
        assert_eq!(3, a_list_2.len());
    }

    #[test]
    fn test_app_loop() {
        let mut m = HashMap::new();
        let ss = vec!["apa", "bepa", "cepa"];

        for s in ss {
            m = app('a', &s, m);
        }

        let a_list = m.get(&'a').unwrap();
        assert_eq!(3, a_list.len());
    }
}
