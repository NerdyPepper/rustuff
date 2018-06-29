extern crate cursive;

use::cursive::Cursive;
use::cursive::traits::*;
use::cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};

fn main() {
    let mut mainwin = Cursive::default();

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_id("select")
        .fixed_size((10,5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    mainwin.add_layer(Dialog::around(LinearLayout::horizontal()
                                     .child(select)
                                     .child(DummyView)
                                     .child(buttons))
                      .title("Select a profile"));
    mainwin.run()
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_id("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
                               .on_submit(ok)
                               .with_id("name")
                               .fixed_width(10))
                .title("Enter a new name")
                .button("Ok", |s| {
                    let name = s.call_on_id("name", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();
                    ok(s, &name);
                })
                .button("Cancel", |s| match s.pop_layer(){
                    None => (),
                    Some(_) => ()
                }));
}

fn delete_name(s: &mut Cursive) {
    let mut selected = s.find_id::<SelectView<String>>("select").unwrap();

    match selected.selected_id() {
        None => s.add_layer(Dialog::info("Nothing to be deleted")),
        Some(x) => {
            selected.remove_item(x);
        }
    }
}

fn on_submit(s: &mut Cursive, name: &String) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\n Some info: Yes", name))
                .title(format!("{}'s Profile", name))
                .button("Quit", Cursive::quit));
}
