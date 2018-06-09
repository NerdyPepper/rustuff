pub fn first() {
    assert_eq!(2 + 2, 4);
}

pub fn another() {
    assert!(false);
}

pub fn greetings(name: &str) -> String {
    format!("Greetings, {}", name)
}

mod tests {
    use super::*;

    #[test]
    fn lemme_see(){
    }

    #[test]
    fn greetings_contains_name() {
        let result = greetings("akshay");
        assert!(result.contains("akshay"));
    }
}
