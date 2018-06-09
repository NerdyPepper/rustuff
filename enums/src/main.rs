use Coin::Rupee;

fn main() {

    println!("{}", value_in_paisa(Coin::Paisa));
    println!("{}", value_in_paisa(Rupee(IndState::Tamil)));

    let m: u8 = 2;

    match m {
        1 => println!("One"),
        2 => println!("Two"),
        _ => (),
    }

}

#[derive(Debug)]
enum IndState{
    Karna,
    Tamil,
}

enum Coin {
    Paisa,
    Rupee(IndState),
}

fn value_in_paisa(coin: Coin) -> u32 {
    match coin {
        Coin::Paisa => 1,
        Rupee(state) => {
            println!("This rupee is from {:?}", state );
            100
        }
    }
}
