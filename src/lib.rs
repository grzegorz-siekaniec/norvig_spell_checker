
pub mod spell_checker;

pub fn f() {
    spell_checker::f2();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
