fn main() {

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)]
    enum UsState {
        CA,
        PA,
        MA,
        NY,
        WA,
    }

    fn value_in_cents(coin: Coin) -> i8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                        println!("State quarter from {:?}!", state);
                        25
                    }
                }
    }
    //let penny = value_in_cents(Coin::Penny);
    //println!("{}", penny);

    //let cali_quarter = value_in_cents(Coin::Quarter(UsState::CA));
    //println!("{}",cali_quarter);

    /*------------------------------------------*/

    let coin = Coin::Penny;
    //let coin2 = Coin::Dime;
    let mut count = 0;

    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }

    println!("Count: {}", count);
    /*------------------------------------------*/
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}", state);
    }
    else {
        count += 1;
    }
    println!("Count: {}", count);
    /*------------------------------------------*/

    fn plus_one(x: Option<i32>) -> Option<i32> {

        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //println!("{:?}+Some(1)={:?}", five, six);

    let some_u8_val = 0u8;
    match some_u8_val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    //println!("{}",some_u8_val);
    //let some_value = Some(1);
    // Using an 'if let' to handle values that match one pattern.
    // Will only execute if some_value = Some(1)
    /*if let Some(1) = some_value {
        println!("one");
    }*/

}
