use dioxus::prelude::*;

// ANCHOR: App
#[component]
pub fn App(cx: Scope) -> Element {
    let hello = "Hello Dioxus!";

    cx.render(rsx!(TitleCard { title: hello }))
}
// ANCHOR_END: App

// ANCHOR: TitleCard
#[derive(Props)]
struct TitleCardProps<'a> {
    title: &'a str,
}

#[component]
fn TitleCard<'a>(cx: Scope<'a, TitleCardProps<'a>>) -> Element {
    cx.render(rsx! {
        h1 { "{cx.props.title}" }
    })
}
// ANCHOR_END: TitleCard
