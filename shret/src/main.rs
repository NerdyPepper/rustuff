extern crate rand;
use rand::prelude::*;

fn main() {

    let mut rng = rand::thread_rng();

    let adjs_shret = vec!["lazy", "oof", "tired", "dead",
    "disinterested", "happy", "owo", "gay", "cute", "creepy", "floofy", "big",
    "dreamy", "crazy", "funny", "kEwL", "chill", "tall", "idle", "inert", "dull",
    "slothful", "roman"
    ];

    let veggies_shret = vec!["bean", "pepper", "tomato", "cabbage", "potato", 
    "radish", "corn", "onion", "carrot", "brinjal", "lemon", "brocolli" ];

    let rand_adj = rng.gen_range(0, adjs_shret.len());
    let rand_veggies = rng.gen_range(0, veggies_shret.len());
    println!("{}{}", adjs_shret[rand_adj], veggies_shret[rand_veggies]);

}
