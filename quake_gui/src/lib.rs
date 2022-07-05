#![allow(non_snake_case)]

use dioxus::prelude::*;
use quake_core::entry::{entry_paths::EntryPaths, EntryDefine, EntryDefines};
use std::path::PathBuf;

pub fn launch(workspace: PathBuf) {
    dioxus::desktop::launch_with_props(app, AppProps { workspace }, |c| {
        c.with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string())
    });
}

fn app(cx: Scope<AppProps>) -> Element {
    let defines = EntryDefines::from_path(&cx.props.workspace.join(EntryPaths::entries_define()));

    rsx!(cx, div {
        main {
            class: "grid grid-cols-3",
            EntryList {
                defines: defines,
            }
        }
    })
}

struct AppProps {
    workspace: PathBuf,
}

#[derive(Props, PartialEq)]
struct EntryListProps {
    defines: EntryDefines,
}

fn EntryList(cx: Scope<EntryListProps>) -> Element {
    rsx!(cx, ul {
        class: "flex flex-col gap-y-1 col-span-1",
        cx.props.defines.entries.iter().map(|define| {
            rsx!(
                EntryListItem {
                    key: "{define.entry_type}",
                    define: define,
                }
            )
        })
    })
}

#[derive(Props, PartialEq)]
struct EntryListItemProps<'a> {
    define: &'a EntryDefine,
}

fn EntryListItem<'a>(cx: Scope<'a, EntryListItemProps<'a>>) -> Element {
    rsx!(cx, li {
        class: "hover:bg-blue-100 rounded-lg px-2",
        "{cx.props.define.entry_type}"
    })
}
