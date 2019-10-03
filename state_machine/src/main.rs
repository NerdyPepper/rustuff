trait SM<T> {
    fn start() -> Self;
    fn step(&mut self, input: T);
}

struct Language {
    val: u32,
    proceed: bool
}

impl Language {
    fn get_next_vals(&self, input: char) -> Self {
        let state = self.val;
        match (state, input) {
            (0, 'a') => return Language {val: 1, proceed: true},
            (1, 'b') => return Language {val: 2, proceed: true},
            (2, 'c') => return Language {val: 0, proceed: true},
            _ =>  return Language {val: 3, proceed: false}
        }
    }
}


impl SM<char> for Language {
    fn start() -> Self {
        return Language {
            val: 0,
            proceed: true
        };
    }
    fn step(&mut self, input: char) {
        let next_state = self.get_next_vals(input);
        self.val = next_state.val;
        println!("State: {}  Output: {}", next_state.val, next_state.proceed);
    }
}

fn main() {
    let mut aklang = Language::start();
    let transducers = vec!['a', 'b', 'c', 'a', 'b', 'c', 'b'];
    for &i in transducers.iter() { aklang.step(i) };
}
