use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GradeProps {
    pub total_words: u32,
}


#[function_component(Grades)]
pub fn grades_box(GradeProps {total_words}:&GradeProps) -> html {
    html!{
        <div class="grades">
            <div>
                <p>{format!("G: {}", n(total_words, 0.5))}</p>
                <p>{format!("VG: {}", n(total_words, 0.7))}</p>
                <p>{format!("MVG: {}", n(total_words, 0.9))}</p>
            </div>
        </div>
    }
}

fn n(n: &u32, part: f32) ->u32 {
    let a = n.clone() as f32 * part;
    a as u32
}