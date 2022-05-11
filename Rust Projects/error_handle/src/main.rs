use std::fs::File;
//use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;
fn main() -> Result<(),Box<dyn Error>> // Another valid return type for main is Result<T,E> {
// Box<dyn Error> is a trait object, what it means "is any kind of error"
    /*let v = vec![1,2,3,4];

    v[99];

    enum Result<T,E> {
        Ok(T),
        Err(E),
    }
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        }
    };
    // Unwrap function returns file inside the Ok, if Err, unwrap will call panic!
    //let f = File::open("hello.txt").unwrap();
    // Using .expect enables us to choose the panic! error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    fn read_username_from_file() -> Result<String, io::Error> {

        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }

    }*/

    // Shortcut for propagating errors: ? operator
    fn read_username_from_file() -> Result<String, io::Error> {

        // #: If the value of the Result is an Ok, the value inside the Ok
        // will be returned and program will continue, if the value is
        // an Err, the Err will be returned from the whole function and
        // propagated to the calling code
        // let mut f = File::open("hello.txt")?;
        // let mut s = String::new();
        // f.read_to_string(&mut s)? //#;
        // Ok(s)

        // Shortened version:
        // let mut s = String::new();
        // File::open("hello.txt")?.read_to_string(&mut s)?;
        // Ok(s)

        //EVEN shorter version;
        fs::read_to_string("hello.txt")


    }
    let f = File::open("hello.txt")?;
    Ok(())


}
