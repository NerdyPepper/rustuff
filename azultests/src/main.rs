extern crate azul;

use azul::prelude::*;
use azul::widgets::{label::Label, button::Button};

struct CounterApp {
    counter : usize,
}

impl Layout for CounterApp {
    fn layout( &self, info: WindowInfo<Self> ) -> Dom<Self> {
        let label = Label::new(format!("{}", self.counter)).dom();
        let button = Button::with_label("Update counter").dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        Dom::new(NodeType::Div)
            .with_id("mydiv")
            .with_child(label)
            .with_child(button)
    }
}

fn main() {
    let app = App::new(MyDataModel { }, AppConfig::default());
    let window = Window::new(WindowCreateOptions::default(), Css::native()).unwrap();
    app.run(window).unwrap();
}
