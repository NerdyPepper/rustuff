use azul::prelude::*;
use azul::widgets::label::Label;

struct DataModel {
    text: String,
}

impl Layout for DataModel {
    fn layout(&self, _ : LayoutInfo<Self>) -> Dom<Self> {
        let text_area_text = Label::new(self.text.clone()).dom();
        Dom::new(NodeType::Div)
            .with_child(text_area_text)
            .with_callback(On::TextInput, Callback(update_text_area))
    }
}

fn update_text_area(state: &mut AppState<DataModel>, 
                    event : &mut CallbackInfo<DataModel>) -> UpdateScreen {
    let ch = state.windows[event.window].get_keyboard_state().current_char?;
    state.data.modify(|state| state.text.push(ch));
    Redraw
}

fn main() {
    println!("Hello, world!");
}
