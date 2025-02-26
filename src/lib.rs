pub mod egraph;
#[allow(dead_code)]
pub mod program;
pub use egraph::*;
pub use facts::EgglogFacts;
pub use rules::EgglogRules;
pub use schedule::EgglogSchedules;
pub use sorts::EgglogSorts;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
