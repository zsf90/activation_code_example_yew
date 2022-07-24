use stylist::{style, yew::styled_component};
use yew::{self, function_component, html};

mod components;

use components::organisms::head::Header;

// 定义一个函数组件
#[styled_component(App)]
fn app() -> Html {
    let stylesheet = style!(
        r#"
        
    "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <Header />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
