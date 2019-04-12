extern crate sciter;

use sciter::dom::Element;

fn main() {
    let mut frame = sciter::Window::new();
    frame.load_file("hello.html");

    let root = Element::from_window(frame.get_hwnd()).unwrap();

    if let Some(mut ele) = root.find_first("#title").unwrap() {
        ele.set_text("Coco");
    }

    if let Some(mut ele) = root.find_first("#content").unwrap() {
        ele.set_text(r#"
                     Throughout the whole countryside the Lucas farm, was known as "the Manor." No one knew why. The peasants doubtless attached to this word, "Manor," a meaning of wealth and of splendor, for this farm was undoubtedly the largest, richest and the best managed in the whole neighborhood.
The immense court, surrounded by five rows of magnificent trees, which sheltered the delicate apple trees from the harsh wind of the plain, inclosed in its confines long brick buildings used for storing fodder and grain, beautiful stables built of hard stone and made to accommodate thirty horses, and a red brick residence which looked like a little chateau.
Thanks for the good care taken, the manure heaps were as little offensive as such things can be; the watch-dogs lived in kennels, and countless poultry paraded through the tall grass.
Every day, at noon, fifteen persons, masters, farmhands and the women folks, seated themselves around the long kitchen table where the soup was brought in steaming in a large, blue-flowered bowl.
The beasts-horses, cows, pigs and sheep-were fat, well fed and clean. Maitre Lucas, a tall man who was getting stout, would go round three times a day, overseeing everything and thinking of everything.
A very old white horse, which the mistress wished to keep until its natural death, because she had brought it up and had always used it, and also because it recalled many happy memories, was housed, through sheer kindness of heart, at the end of the stable.
                     "#);
    }

    frame.run_app();
}
