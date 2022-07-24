use crate::components::atoms::title::Title;
use stylist::{style, yew::styled_component};
use yew::html;

#[styled_component(Header)]
pub fn header() -> Html {
    let stylesheet = style!(
        r#"
        height: 240px;
    "#
    )
    .unwrap();
    html! {
       <div class={stylesheet}>
            <Title text="激活码练习项目" />
       </div>
    }
}
