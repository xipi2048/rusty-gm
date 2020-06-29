use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(Dialog::around(TextView::new("Hello...\nWould you like to play a game?"))
                    .title("Raen's Dungeon")
                    .button("Yes")
                    .button("No", |s| s.quit()));
    siv.run();
}