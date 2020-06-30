use cursive::{
    views::{Dialog, TextView},
    Cursive,
};

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello...\nWould you like to play a game?"))
            .title("Raen's Dungeon")
            .button("No", |s| s.quit())
            .button("Yes", |s| start_game(s)),
    );

    siv.run();
    siv.clear();
}

fn start_game(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(TextView::new("Thank you for playing ^_^"));
}
