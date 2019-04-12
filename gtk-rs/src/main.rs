extern crate gtk;

use gtk::prelude::*;

use gtk::{ Button, Window, WindowType, Label, TextView, TextViewExt };

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let content = TextView::new();

    window.add(&content);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    //button.connect_clicked(|_| {
    //    println!("Clicked!");
    //});

    gtk::main();
}
