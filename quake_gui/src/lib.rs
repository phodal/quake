use dioxus::prelude::*;
use std::path::PathBuf;

pub fn launch(workspace: PathBuf) {
    dioxus::desktop::launch_with_props(app, AppProps { workspace }, |c| {
        c.with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string())
    });
}

fn app(cx: Scope<AppProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "relative",
            button {
                class: "h-60 w-60 border-2 border-dashed absolute top-1/2 left-1/2 transform -translate-x-1/2 translate-y-1/2",
                "Open workspace"
            }
        }
    ))
}

struct AppProps {
    workspace: PathBuf,
}
