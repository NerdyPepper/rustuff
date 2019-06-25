use azul::prelude::*;
use azul::widgets::{label::Label, button::Button};

struct MainFrame {
    counter: usize
}

impl Layout for MainFrame {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("Count: {}", self.counter)).dom();
        let button = Button::with_label("Press Me (Upd8)").dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        Dom::new(NodeType::Div)
            .with_child(label)
            .with_child(button)
    }
}

fn update_counter(app_state: &mut AppState<MainFrame>,
                  _info: &mut CallbackInfo<MainFrame>)
    -> UpdateScreen 
{
    app_state.data.modify(|state| state.counter += 1);
    Redraw
}

fn main() {
    let init_state = MainFrame {
        counter: 0
    };
    let mut app = App::new(init_state, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();
}

