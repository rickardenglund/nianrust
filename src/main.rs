mod letterbox;
mod grades;
mod day;
mod today;


use yew::{prelude::*};


#[derive(Debug)]
pub enum Msg {
    Today,
    Old,
}

pub struct App {
    s: Msg,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App{s: Msg::Today}
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.s = msg;

        true
    }

    fn view(&self, ctx: &Context<Self>) ->Html {
        let onclick_today = ctx.link().callback(|_| Msg::Today);
        let onclick_old = ctx.link().callback(|_| Msg::Old);



        let body = match self.s {
            Msg::Today => html!{<today::Today />},
            Msg::Old => html! {<h1>{"Old"}</h1>},
        };

        html!{
            <>
                {body}
                <nav>
                    <button onclick={onclick_today}>{"Today"}</button>
                    <button onclick={onclick_old}>{"Tidigare"}</button>
                </nav>
            </>
        }
    }
}



fn main() {
    yew::start_app::<App>();
}
