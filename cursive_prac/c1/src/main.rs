extern crate cursive;

use cursive::Cursive;
use cursive::views::{ Dialog, SelectView };

fn main() {
    let mut tw = Cursive::default();
    let mut select = SelectView::<String>::new();
    select.add_all_str(vec!["a", "b", "c"]);
    tw.add_layer(Dialog::around(select).title("This is a dialog"));

    tw.run();
}

fn show_next(s: &mut Cursive){
    s.pop_layer();
    s.add_layer(Dialog::text("This is the question?")
                .title("Question 1")
                .button("Yeah", |s| show_answer(s, "You selected yes"))
                .button("nah", |s| show_answer(s, "You selected nah")));
}

fn show_answer(s: &mut Cursive, ans: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(ans)
                .title("Results")
                .button("Quit", |s| s.quit()));
}
