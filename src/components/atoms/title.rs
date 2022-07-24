// 标题组件
//
//
use stylist::{css, style, yew::styled_component};
use yew::{html, Properties};

// 定义函数组件的参数
#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
}

/// title 组件的参数有：
/// text 类型是 String
/// ## 使用方法
/// ```rust
/// <Title text = "激活码测试项目" />
/// ```
/// 使用stylist 管理样式，把 `function_component` 换成 `styled_component`。
#[styled_component(Title)]
pub fn title(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        color: green;
        font-size: 6rem;
        text-align: center;
        line-height: 240px;
        "#
    )
    .unwrap();
    html! {
        <h1 class={stylesheet}>{&props.text}</h1>
    }
}
