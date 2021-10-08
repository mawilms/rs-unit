#![warn(clippy::all, clippy::pedantic)]

pub fn hello_world() {
    println!("Hello World");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
