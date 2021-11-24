// use tray_item::TrayItem;
//
// pub fn tray(toggle: fn()) {
//     let mut tray = TrayItem::new("Quake", "").unwrap();
//
//     tray.add_menu_item("Toggle", toggle).unwrap();
//
//     let inner = tray.inner_mut();
//     inner.add_quit_item("Quit");
//     inner.display();
// }