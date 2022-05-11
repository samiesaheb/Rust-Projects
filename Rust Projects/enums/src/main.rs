fn main() {
	/*
    enum IpAddrKind {

    	V4,
    	V6,

    }
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // Defining a struct that uses enum as one of the types for its fields
    struct IpAddr {

    	kind: IpAddrKind,
    	address: String,

    }

    let home = IpAddr {

    	kind: IpAddrKind::V4,
    	address: String::from("192.168.1.44"),
    }*/
    #[derive(Debug)]
    enum IpAddr {

    	//V4(String),
    	V4(u8,u8,u8,u8),
    	V6(String),

    }
    #[derive(Debug)]
    enum Message {

        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),

    }
    /*
    impl Message {
        fn call(&self) {

        }
    }*/


    //let home = IpAddr::V4(String::from("192.168.1.44"));
    let home = IpAddr::V4(192,168,1,44);

    /*let quit = Message::Quit;
    let movexy = Message::Move{x: 40, y: 60};
    let write = Message::Write(String::from("Not quitting"));
    let changecolor = Message::ChangeColor(255,0,0);*/

    println!("{:?}", home);


    /*println!("{:?}", quit);
    println!("{:?}", movexy);
    println!("{:?}", write);
    println!("{:?}", changecolor);*/

    let some_number = Some(5);
    let absent_number: Option<i8> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(10);
    //let sum = x+y;

    println!("{:?}", absent_number);


}
