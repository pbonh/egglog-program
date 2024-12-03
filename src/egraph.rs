#[allow(dead_code)]
pub mod egglog_names;
pub mod facts;
#[macro_use]
#[allow(unused_macros)]
pub mod macros;
pub mod builder;
pub mod rules;
pub mod schedule;
pub mod sorts;
use std::collections::HashSet;

pub use builder::*;
use egglog::ast::{Command, Symbol};

pub type EgglogCommandList = Vec<Command>;
pub type EgglogSymbols = HashSet<Symbol>;

#[cfg(test)]
mod tests {

    #[test]
    fn egglog_type_macros() {
        use egglog::ast::*;

        // let set_option_cmd = cmd!(SetOption {
        //     name: "node_limit",
        //     value: 1000,
        // });

        // let vec_symbol = Symbol::new("Vec");
        // let int_vec_symbol = Symbol::new("IntVec");
        // let i64_symbol = Symbol::new("i64");
        // let sort_cmd: Command = cmd!(Sort(
        //     span!(),
        //     int_vec_symbol,
        //     Some((vec_symbol, vec![expr!(i64_symbol)])),
        // ));
        // utilities::check_egglog_program(vec![sort_cmd.clone()]);
        // assert_eq!("(sort IntVec (Vec i64))", sort_cmd.to_string());

        let datatype_cmd: Command = cmd!(Datatype {
            span: span!(),
            name: "Math",
            variants: vec![
                variant!("Num", ["i64"]),
                variant!("Var", ["String"]),
                variant!("Add", ["Math", "Math"]),
                variant!("Mul", ["Math", "Math"]),
            ],
        });
        utilities::check_egglog_program(vec![datatype_cmd.clone()]);
        assert_eq!(
            "(datatype Math (Num i64) (Var String) (Add Math Math) (Mul Math Math))",
            datatype_cmd.to_string()
        );

        // let function_cmd = cmd!(
        //     Function(function_decl!(
        //         "Add",
        //         inputs = ["Math", "Math"],
        //         output = "Math"
        //     ))
        // );
        // Optional fields can be added here
        // default = expr!(0),
        // cost = Some(1),

        // let print_function_cmd = cmd!(PrintFunction(span!(), "Add", 20));

        // let rewrite_cmd = cmd!(Rewrite(
        //     symbol!("commute_add"),
        //     GenericRewrite {
        //         span: span!(),
        //         lhs: expr!("Add", var "a", var "b"),
        //         rhs: expr!("Add", var "b", var "a"),
        //         conditions: vec![],
        //     },
        //     false,
        // ));
        //
        // let run_schedule_cmd = cmd!(
        //     RunSchedule(
        //         schedule!(sequence [
        //             saturate run "my_ruleset_1",
        //             run "my_ruleset_2", until = [("eq", [expr!(var "x"), 0])]
        // ])));
        //
        // let check_cmd = cmd!(
        //     Check(
        //         span!(),
        //         facts = [
        //             eq [expr!(var "x"), 0],
        //             expr!("greater_than", var "y", 10)
        //         ]
        //     )
        // );
    }
}
