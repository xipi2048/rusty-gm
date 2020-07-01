use cursive::{
    event::Key,
    menu::MenuTree,
    traits::*,
    views::{Dialog, ResizedView, StackView, TextView},
    Cursive, Vec2,
};

fn main() {
    let mut siv = cursive::default();

    siv.menubar()
        .add_subtree(
            "File",
            MenuTree::new()
                .leaf("New Game", start_game)
                .leaf("Load Game", |s| {})
                .leaf("Manage Saves", |s| {})
                .leaf("Settings/Options", |s| {})
                .delimiter()
                .leaf("Exit", |s| s.quit()),
        )
        .add_subtree(
            "Network",
            MenuTree::new().leaf("Join", |s| {}).leaf("Host", |s| {}),
        )
        .add_delimiter()
        .add_subtree(
            "Help",
            MenuTree::new().leaf("About", |s| {}).leaf("Manual", |s| {}),
        );

    siv.select_menubar();
    siv.set_autohide_menu(false);

    siv.add_global_callback(Key::F7, |s| s.select_menubar());

    siv.run();
    siv.clear();
}

fn start_game(siv: &mut Cursive) {
    siv.set_autohide_menu(true);
    siv.pop_layer();
    siv.add_layer(TextView::new("Thank you for playing ^_^"));
}
