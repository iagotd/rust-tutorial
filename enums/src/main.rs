enum Fruit {
    Apple,
    Orange,
    Lemon,
}

fn main() {
    let my_fruit: Fruit = Fruit::Lemon;
    
    match my_fruit {
        Fruit::Apple => println!("I have a, apple!"),
        Fruit::Orange => println!("I have a, orange!"),
        Fruit::Lemon => println!("I have a lemon!"),
    }
}
