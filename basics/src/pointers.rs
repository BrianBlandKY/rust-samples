/*
Use this to understand Pointers, Smart Pointers, Life cycles, etc.
*/
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// must implement this for deref (*) functionality
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with data!");
    }
}

#[cfg(test)]
mod tests {
    use crate::pointers::*;
    #[test]
    fn test_custom_pointer() {
        let x = 5;
        let y = MyBox::new(x);
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
        // not required, this will be called automatically
        // once out of scope, manually destroying y as example.
        drop(y);
    }
}