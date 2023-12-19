use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{stdin, Result as IOResult};

#[derive(Debug)]
enum Maybe<T> {
    Nil,
    Just(T),
}

impl<T> Maybe<T> {
    pub fn map<U>(&self, f: &dyn Fn(&T) -> U) -> Maybe<U> {
        match self {
            Maybe::Just(value) => Maybe::Just(f(value)),
            Maybe::Nil => Maybe::Nil,
        }
    }
}

fn might_be_there() -> Maybe<i32> {
    // Maybe::Just(5)
    Maybe::Nil
}

fn main() -> IOResult<()> {
    let bar = might_be_there();
    let f = |x: &i32| x + 5;
    let baz = bar.map(&f);

    println!("{:?}", baz);

    println!("Hello, world!");
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    println!("{}", buf);
    let mut map: HashMap<String, String> = HashMap::new();
    // map.insert(String::from("foo"), String::from(buf));
    let foo = map.get("foo");
    // match foo {
    //     Some(value) => println!("{}", value),
    //     None => println!("you suck"),
    // };
    if let Some(value) = foo.as_deref() {
        println!("{}", value);
    }
    println!("{}", foo.unwrap_or(&String::from("stuff")));
    Ok(())
    // no null -> option instead
    // no void like TS -> use unit ()
}
