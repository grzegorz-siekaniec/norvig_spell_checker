#[macro_use]
extern crate log;

pub mod spell_checker;

#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 5);
        // assert_eq!(2 + 2, 4);
        // assert_eq!(2 + 2, 6);
    }

    #[test]
    fn it_works2() {
        // panic::catch_unwind(|| {
        //     assert_eq!(2 + 2, 5);
        // });
        // assert_eq!(2 + 2, 4);
        // assert_eq!(2 + 2, 6);
    }
}
