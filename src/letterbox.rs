use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LetterBoxProps {
    pub letters: String,
}


#[function_component(LetterBox)]
pub fn letter_box(LetterBoxProps {letters}:&LetterBoxProps) -> Html {
    let letters = letters.chars().map(|l| html!{
        <div class="letter">{l}</div>
    }).collect::<Html>();

    html!{
        <>
            <div class="letterbox">
                {letters}
            </div>
        </>
    }
}