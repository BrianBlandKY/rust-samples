
pub struct SomeData {
    data: String,
}

pub trait AbstractStuff {
    fn do_nothing(&self) -> Result<Box<SomeData>, String>;
    fn do_something(&self) -> Result<Box<SomeData>, String>;
}

pub struct SomeWorker {}

impl AbstractStuff for SomeWorker {
    fn do_nothing(&self) -> Result<Box<SomeData>, String> {
        Err(String::from("did nothing"))
    }

    fn do_something(&self) -> Result<Box<SomeData>, String> {
        Ok(Box::new(SomeData{data: String::from("TEST")}))
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::*;
    #[test]
    fn test_trait_implementation() {
        let worker = SomeWorker{};
        let mut d : Box<SomeData>;
        match worker.do_something() {
            Ok(x) => d = x,
            Err(e) => panic!("error {}", e),
        };

        println!("we got a value! {}", d.data);

        match worker.do_nothing() {
            Ok(x) => panic!("should not get here"),
            Err(e) => println!("received expected error {}", e),
        };
    }
}
