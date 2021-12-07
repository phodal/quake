# 1. opensource technology decision

日期: 2021-11-18

## 状态

2021-11-18 提议

## 背景

Low level window Handle:

- [https://github.com/rust-windowing/winit](https://github.com/rust-windowing/winit) is a window creation and management library. It can create windows and lets you handle events (for example: the window being resized, a key being pressed, a mouse movement, etc.) produced by window.

Gui Frameworks:

- [https://github.com/emilk/egui](https://github.com/emilk/egui)  is a simple, fast, and highly portable immediate mode GUI library for Rust. egui runs on the web, natively, and in your favorite game engine (or will soon).
- [https://github.com/linebender/druid](https://github.com/linebender/druid) is a data-first Rust-native UI design toolkit.
- [https://github.com/hecrj/iced](https://github.com/hecrj/iced) is a cross-platform GUI library for Rust, inspired by Elm

Mac App Framework:

- [https://github.com/mrmekon/fruitbasket](https://github.com/mrmekon/fruitbasket) fruitbasket provides two different (but related) services for helping you run your Rust binaries as native AppKit/Cocoa applications on Mac OS X.

Tray:

- ~~[Trayicon](https://github.com/ciantic/trayicon-rs/)~~ not support macOS.
- [tray-item-rs](https://github.com/olback/tray-item-rs) is a  Multi-platform Tray Indicator.
- [ksni](https://github.com/iovxw/ksni) is A Rust implementation of the KDE/freedesktop StatusNotifierItem specification.

Tray Samples:

- [https://github.com/mrmekon/connectr](https://github.com/mrmekon/connectr) is a super lightweight Spotify controller


# Spike

- [ ] hotkey binding
    - GUI based Shortcuts
        - [ ] [livesplit-hotkey](https://github.com/LiveSplit/livesplit-core/tree/master/crates/livesplit-hotkey)
        - [ ] libinput binding [input.rs](https://github.com/Smithay/input.rs)
    - Terminal based Shortcuts
        - [ ] [Cursive](https://github.com/gyscos/Cursive) is a  A Text User Interface library for the Rust programming language.
        - [ ] [tui-rs](https://github.com/fdehau/tui-rs) is a  Build terminal user interfaces and dashboards using Rust.
        - [ ] [termion](https://crates.io/crates/termion) is a bindless library for manipulating terminals.
        - [ ] [Crossterm](https://github.com/crossterm-rs/crossterm) is a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces (see features).
        - [ ] Demo: [verco](https://github.com/vamolessa/verco) is a simple Git/Mercurial/PlasticSCM tui client based on keyboard shortcuts.
- [ ] input simulation
    - [ ] [Enigo](https://github.com/Enigo-rs/Enigo) is a  Cross platform input simulation in Rust
    - [ ] [rdev](https://github.com/Narsil/rdev) is a Simple library to listen and send events globally to keyboard and mouse on MacOS, Windows and Linux (x11).
- [ ] Gui
    - [ ] gui framework
    - [ ] webview render
- [ ] Daemon?
    - [ ] try `.plist`?
    - [ ] libs: [daemonize](https://github.com/knsd/daemonize)
    - [ ] sample: [https://github.com/Spotifyd/spotifyd](https://github.com/Spotifyd/spotifyd)
- [ ] Webview?
    - [ ] [https://github.com/Boscop/web-view](https://github.com/Boscop/web-view)
- [ ] Global auto fill
    - [ ] [espanso](https://github.com/federico-terzi/espanso) is a  Cross-platform Text Expander written in Rust.
- [ ] Script Languages. [https://github.com/alilleybrinker/langs-in-rust](https://github.com/alilleybrinker/langs-in-rust)
    - [ ] [rhai](https://github.com/rhaiscript/rhai) an embedded scripting language for Rust.
    - [ ] [Boa](https://github.com/boa-dev/boa) an experimental Javascript lexer, parser and interpreter written in Rust. Currently, it has support for some of the language.
    - [ ] [gluon](https://github.com/gluon-lang/gluon) is a small, statically-typed, functional programming language designed for application embedding.
    - [ ] [rune](https://github.com/rune-rs/rune)
    - [ ] [Goscript](https://github.com/oxfeeefeee/goscript)
- [ ] Code Highlight
    - [ ] [syntect](https://github.com/trishume/syntect) is a  Rust library for syntax highlighting using Sublime Text syntax definitions.
- [ ] Markdown
    - [ ] [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) is a pull parser for CommonMark, written in Rust.


## 决策

在这里补充上决策信息...

## 后果

在这里记录结果...
