fn main() {
    println!("Hello, flow control: IF!");
    let x: bool = true;

    if x {
        println!("as expected, x is true!");
    } else {
        println!("x is {}!", x);
    }

    println!("Hello, flow control: While!");
    /* loop {
        //This is an infinite loop
    }  */
    
    let mut n = 0;
    loop{
        n += 1;
        if n > 10 {
            break;
        }

        if n%2 == 0 {
            continue;
        }

        println!("Hi: {}", n);
    } 

    let mut n = 1;

    while n <= 3 {
        println!("This is a while loop: {}", n);
        n += 1;
    }

    //The expression 1..3 is a range which is an iterator
    //For loops need iterators
    for i in 1..3 {
        println!("For with an expression that is an iterator: {}", i);
    }

    //Vectors also have an iter() method
    //We can include .enumerate() to include the index
    let cities = vec!["Paris", "London", "Madrid"];
    for (index, city) in cities.iter().enumerate() {
        println!("City-{}: {}", index, city);
    }
}
