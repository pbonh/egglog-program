#[allow(dead_code)]
pub mod egglog_names;
pub mod facts;
#[macro_use]
#[allow(unused_macros)]
pub mod macros;
#[allow(dead_code)]
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
    use egglog::ast::*;

    #[test]
    fn egglog_type_macros() {
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

        let _sort_command: Command = create_command!(Sort, span!(), "42", None);

        let _datatype_command: Command = create_command!(
            Datatype,
            span!(),
            "43",
            [
                { span!(), "44", vec!["45", "46"], Some(1) },
                { span!(), "47", vec!["48", "49"], None }
            ]
        );

        let _constructor_command: Command = create_command!(
            Constructor,
            span!(),
            "50",
            vec!["51", "52"],
            "53",
            Some(2),
            false
        );

        let _relation_command: Command = create_command!(Relation, span!(), "54", vec!["55", "56"]);

        let _function_command: Command = create_command!(
            Function,
            span!(),
            "57",
            vec!["58", "59"],
            "60",
            Some(GenericExpr::Call(span!(), Symbol::new("61"), vec![]))
        );
    }

    // use super::*;
    //
    // pub fn generate_llhd_dfg_commands() -> EgglogCommandList {
    //     let mut commands = Vec::new();
    //
    //     // Basic sorts
    //     commands.push(create_command!(Sort, span!(), "LLHDTy", None));
    //     commands.push(create_command!(
    //         Sort,
    //         span!(),
    //         "LLHDVecTy",
    //         Some(("Vec", vec!["LLHDTy"]))
    //     ));
    //
    //     // LLHDTy constructors
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "Void",
    //         vec![],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "Time",
    //         vec![],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "IntTy",
    //         vec!["i64"],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "Enum",
    //         vec!["i64"],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "Pointer",
    //         vec!["LLHDTy"],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "Signal",
    //         vec!["LLHDTy"],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "ArrayTy",
    //         vec!["i64", "LLHDTy"],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "StructTy",
    //         vec!["LLHDVecTy"],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "FuncTy",
    //         vec!["LLHDVecTy", "LLHDTy"],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //     commands.push(create_command!(
    //         Constructor,
    //         span!(),
    //         "EntityTy",
    //         vec!["LLHDVecTy", "LLHDVecTy"],
    //         "LLHDTy",
    //         None,
    //         false
    //     ));
    //
    //     // LLHDUnitKind datatype
    //     commands.push(create_command!(
    //         Datatype,
    //         span!(),
    //         "LLHDUnitKind",
    //         [
    //             { span!(), "Entity", vec![], None },
    //             { span!(), "Function", vec![], None },
    //             { span!(), "Process", vec![], None }
    //         ]
    //     ));
    //
    //     // LLHDValue datatype
    //     commands.push(create_command!(
    //         Datatype,
    //         span!(),
    //         "LLHDValue",
    //         [{ span!(), "Value", vec!["LLHDTy", "i64"], None }]
    //     ));
    //
    //     // LLHDVecValue sort
    //     commands.push(create_command!(
    //         Sort,
    //         span!(),
    //         "LLHDVecValue",
    //         Some(("Vec", vec!["LLHDValue"]))
    //     ));
    //
    //     // LLHDBlock datatype
    //     commands.push(create_command!(
    //         Datatype,
    //         span!(),
    //         "LLHDBlock",
    //         [{ span!(), "Block", vec!["i64"], None }]
    //     ));
    //
    //     // LLHDVecBlock sort
    //     commands.push(create_command!(
    //         Sort,
    //         span!(),
    //         "LLHDVecBlock",
    //         Some(("Vec", vec!["LLHDBlock"]))
    //     ));
    //
    //     // LLHDExtUnit datatype
    //     commands.push(create_command!(
    //         Datatype,
    //         span!(),
    //         "LLHDExtUnit",
    //         [{ span!(), "ExtUnit", vec!["i64"], None }]
    //     ));
    //
    //     // LLHDTimeValue datatype
    //     commands.push(create_command!(
    //         Datatype,
    //         span!(),
    //         "LLHDTimeValue",
    //         [{ span!(), "TimeValue", vec!["i64"], None }]
    //     ));
    //
    //     // LLHDRegMode datatype
    //     commands.push(create_command!(
    //         Datatype,
    //         span!(),
    //         "LLHDRegMode",
    //         [
    //             { span!(), "Low", vec![], None },
    //             { span!(), "High", vec![], None },
    //             { span!(), "Rise", vec![], None },
    //             { span!(), "Fall", vec![], None },
    //             { span!(), "Both", vec![], None }
    //         ]
    //     ));
    //
    //     // LLHDVecRegMode sort
    //     commands.push(create_command!(
    //         Sort,
    //         span!(),
    //         "LLHDVecRegMode",
    //         Some(("Vec", vec!["LLHDRegMode"]))
    //     ));
    //
    //     // LLHDDFG datatype (large datatype with many variants)
    //     commands.push(create_command!(
    //         Datatype,
    //         span!(),
    //         "LLHDDFG",
    //         [
    //             { span!(), "ValueRef", vec!["LLHDValue"], None },
    //             { span!(), "ConstInt", vec!["i64", "LLHDTy", "String"], None },
    //             { span!(), "ConstTime", vec!["i64", "LLHDTy", "String"], None },
    //             { span!(), "Alias", vec!["i64", "LLHDTy", "LLHDDFG"], None },
    //             { span!(), "ArrayUniform", vec!["i64", "LLHDTy", "i64", "LLHDDFG"], None },
    //             { span!(), "Array", vec!["i64", "LLHDVecValue"], None },
    //             { span!(), "Struct", vec!["i64", "LLHDVecValue"], None },
    //             { span!(), "Not", vec!["i64", "LLHDTy", "LLHDDFG"], None },
    //             { span!(), "Neg", vec!["i64", "LLHDTy", "LLHDDFG"], None },
    //             { span!(), "Add", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Sub", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "And", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Or", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Xor", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Smul", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Sdiv", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Smod", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Srem", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Umul", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Udiv", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Umod", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Urem", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Eq", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Neq", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Slt", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Sgt", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Sle", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Sge", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Ult", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Ugt", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Ule", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Uge", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Shl", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Shr", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Mux", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Reg", vec!["i64", "LLHDTy", "LLHDVecValue", "LLHDVecRegMode"], None },
    //             { span!(), "InsField", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "i64", "i64"], None },
    //             { span!(), "InsSlice", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "i64", "i64"], None },
    //             { span!(), "ExtField", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "i64", "i64"], None },
    //             { span!(), "ExtSlice", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "i64", "i64"], None },
    //             { span!(), "Con", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Del", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Call", vec!["i64", "LLHDTy", "LLHDExtUnit", "i64", "LLHDVecValue"], None },
    //             { span!(), "Inst", vec!["i64", "LLHDTy", "LLHDExtUnit", "i64", "LLHDVecValue"], None },
    //             { span!(), "Sig", vec!["i64", "LLHDTy", "LLHDDFG"], None },
    //             { span!(), "Prb", vec!["i64", "LLHDTy", "LLHDDFG"], None },
    //             { span!(), "Drv", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "DrvCond", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Var", vec!["i64", "LLHDTy", "LLHDDFG"], None },
    //             { span!(), "Ld", vec!["i64", "LLHDTy", "LLHDDFG"], None },
    //             { span!(), "St", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDDFG"], None },
    //             { span!(), "Halt", vec!["i64"], None },
    //             { span!(), "Ret", vec!["i64"], None },
    //             { span!(), "RetValue", vec!["i64", "LLHDTy", "LLHDDFG"], None },
    //             { span!(), "Phi", vec!["i64", "LLHDVecValue", "LLHDVecBlock"], None },
    //             { span!(), "Br", vec!["i64", "LLHDBlock"], None },
    //             { span!(), "BrCond", vec!["i64", "LLHDTy", "LLHDDFG", "LLHDBlock", "LLHDBlock"], None },
    //             { span!(), "Wait", vec!["i64", "LLHDBlock", "LLHDVecValue"], None },
    //             { span!(), "WaitTime", vec!["i64", "LLHDBlock", "LLHDVecValue"], None }
    //         ]
    //     ));
    //
    //     // LLHDUnitDFG datatype
    //     commands.push(create_command!(
    //         Datatype,
    //         span!(),
    //         "LLHDUnitDFG",
    //         [
    //             { span!(), "LLHDUnit", vec!["i64", "LLHDUnitKind", "String", "LLHDVecValue", "LLHDVecValue", "LLHDDFG"], None },
    //             { span!(), "LLHDUnitDecl", vec!["i64", "LLHDUnitKind", "String", "LLHDVecValue", "LLHDVecValue"], None }
    //         ]
    //     ));
    //
    //     commands
    // }
}
