mod blooming;
mod grades;
mod letterbox;
mod old;
mod solution;
mod today;

use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Today,
    #[at("/history")]
    History,
    #[at("/solution")]
    Solution,
}

#[function_component(MyNav)]
fn mynav() -> Html {
    let maybe_history = use_history();
    if let None = maybe_history {
        return html! {<h1>{"NoHistory"}</h1>};
    }
    let history = maybe_history.unwrap();

    let history_btn = {
        let history = history.clone();
        let onclick_callback = Callback::from(move |_| history.push(Route::History));
        html! {
            <button onclick={onclick_callback.clone()}>{"Senaste veckan"}</button>
        }
    };

    let today_btn = {
        let history = history.clone();
        let onclick_callback = Callback::from(move |_| history.push(Route::Today));
        html! {
            <button onclick={onclick_callback.clone()}>{"Dagens"}</button>
        }
    };

    html! {
        <nav>
            {today_btn}
            {history_btn}
         </nav>
    }
}
#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
            <MyNav />
        </BrowserRouter>
        </>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Today => html! {<today::Today />},
        Route::History => html! {
            <old::Old />
        },
        Route::Solution => html! {<solution::Solution />},
    }
}

fn main() {
    yew::start_app::<App>();
}
