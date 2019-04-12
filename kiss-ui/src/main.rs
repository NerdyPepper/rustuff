#[macro_use]
extern crate kiss_ui;

use kiss_ui::container::Vertical;
use kiss_ui::dialog::Dialog;
use kiss_ui::text::{ Label, TextBox };
use kiss_ui::button::Button;

fn main() {
    kiss_ui::show_gui(|| {
        Dialog::new(
            Vertical::new(
                children![
                Label::new("Title text"),
                Label::new("Content textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent textContent text"),
                Button::new(),
                Button::new()
                ]
                )
            )
            .set_title("First")
            .set_size_pixels(700, 700)
    });
}

