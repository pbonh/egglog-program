macro_rules! symbol {
    ($sym:expr) => {
        Symbol::from($sym)
    };
}

macro_rules! span {
    () => {
        DUMMY_SPAN.clone()
    };
}

macro_rules! literal {
    ($val:literal) => {
        Literal::from($val)
    };
}

macro_rules! expr {
    // For literals (integers, strings, etc.)
    ($val:literal) => {
        GenericExpr::Lit(span!(), literal!($val))
    };
    // For variables (assumed to be string literals)
    (var $var:expr) => {
        GenericExpr::Var(span!(), symbol!($var))
    };
    // For function calls with arguments
    ($func:expr, $( $args:tt ),* ) => {
        GenericExpr::Call(
            span!(),
            symbol!($func),
            vec![
                $( expr!($args) ),*
            ],
        )
    };
    // For function calls without arguments
    ($func:expr) => {
        GenericExpr::Call(
            span!(),
            symbol!($func),
            vec![],
        )
    };
}

macro_rules! variant {
    ($name:expr, [$($types:expr),*] $(, cost = $cost:expr)?) => {
        Variant {
            span: span!(),
            name: symbol!($name),
            types: vec![ $( symbol!($types) ),* ],
            cost: None $( .or(Some($cost)) )?,
        }
    };
}

macro_rules! schema {
    (inputs = [$($inputs:expr),*], output = $output:expr) => {
        Schema {
            input: vec![ $( symbol!($inputs) ),* ],
            output: symbol!($output),
        }
    };
}

macro_rules! fact {
    // Equality fact with multiple expressions
    (eq [$( $exprs:tt ),+]) => {
        GenericFact::Eq(
            span!(),
            vec![ $( expr!($exprs) ),+ ],
        )
    };
    // Single expression fact
    ($expr:tt) => {
        GenericFact::Fact(expr!($expr))
    };
}

macro_rules! schedule {
    // Saturate schedule
    (saturate $sched:tt) => {
        GenericSchedule::Saturate(
            span!(),
            Box::new(schedule!($sched)),
        )
    };
    // Repeat schedule
    (repeat $times:expr, $sched:tt) => {
        GenericSchedule::Repeat(
            span!(),
            $times,
            Box::new(schedule!($sched)),
        )
    };
    // Run schedule with ruleset and optional until conditions
    (run $ruleset:expr $(, until = [$($until:tt),*])? ) => {
        GenericSchedule::Run(
            span!(),
            GenericRunConfig {
                ruleset: symbol!($ruleset),
                until: None $( .or(Some(vec![ $( fact!($until) ),* ])) )?,
            },
        )
    };
    // Sequence of schedules
    (sequence [$( $sched:tt ),+]) => {
        GenericSchedule::Sequence(
            span!(),
            vec![ $( schedule!($sched) ),+ ],
        )
    };
}

macro_rules! sort {
    ($symbol:expr, $option:expr) => {
        Sort(DUMMY_SPAN.clone(), $symbol, Some($option))
    };
    ($symbol:expr) => {
        Sort(DUMMY_SPAN.clone(), $symbol, None)
    };
}

macro_rules! function_decl {
    ($name:expr, inputs = [$($inputs:expr),*], output = $output:expr $(, $field_name:ident = $field_value:expr )* ) => {
        GenericFunctionDecl {
            name: symbol!($name),
            schema: Schema {
                input: vec![ $( symbol!($inputs) ),* ],
                output: symbol!($output),
            },
            default: None,
            merge: None,
            merge_action: vec![],
            cost: None,
            unextractable: false,
            ignore_viz: false,
            span: span!(),
            $( $field_name: $field_value ),*
        }
    };
}

macro_rules! cmd {
    // For variants with named fields
    ($variant:ident { $($field_name:ident : $field_value:expr),* $(,)? }) => {
        GenericCommand::$variant {
            $(
                $field_name: cmd_helper!($field_name, $field_value),
            )*
        }
    };
    // For variants with unnamed fields
    ($variant:ident ( $($field_value:expr),* $(,)? )) => {
        GenericCommand::$variant(
            $(
                cmd_helper!(field, $field_value),
            )*
        )
    };
}

macro_rules! cmd_helper {
    // Fields that are Symbols
    (name, $val:expr) => {
        symbol!($val)
    };
    (ruleset, $val:expr) => {
        symbol!($val)
    };
    // Fields that are GenericExpr
    (value, $val:expr) => {
        expr!($val)
    };
    (expr, $val:expr) => {
        expr!($val)
    };
    // Fields that are Variants
    (variants, $val:expr) => {
        $val // Assuming $val is an expression like `vec![ ... ]`
    };
    // Fields that are GenericFunctionDecl
    (function_decl, $val:expr) => {
        $val
    };
    // Fields that are GenericSchedule
    (schedule, $val:expr) => {
        $val
    };
    // Fields that are GenericFact
    (facts, $val:expr) => {
        $val
    };
    // Fields that are Schema
    (schema, $val:expr) => {
        $val
    };
    // Fields that are a Sort
    (sort, $val:expr) => {
        $val
    };
    // For other fields, pass the value as is
    ($field_name:ident, $val:expr) => {
        $val
    };
}

// macro_rules! build_command {
//     (Sort $name:expr; $($constructor:expr, $($arg:expr),*);*) => {
//         GenericCommand::Sort(
//             Span,
//             Symbol::new($name),
//             Some((
//                 Symbol::new($name),
//                 vec![$(GenericExpr::Call(Span, Symbol::new($constructor), vec![$(GenericExpr::Var(Span, Symbol::new($arg))),*])),*]
//             ))
//         )
//     };
//     (Sort $name:expr) => {
//         GenericCommand::Sort(Span, Symbol::new($name), None)
//     };
//     (Datatype $name:expr; $($variant:expr $(($($type:expr),*))? $(cost $cost:expr)?),*) => {
//         GenericCommand::Datatype {
//             span: Span,
//             name: Symbol::new($name),
//             variants: vec![
//                 $(
//                     Variant {
//                         span: Span,
//                         name: Symbol::new($variant),
//                         types: vec![$( $( Symbol::new($type) ),* )?],
//                         cost: {
//                             $(
//                                 let cost_val = $cost;
//                                 Some(cost_val)
//                             )?
//                             #[allow(unreachable_code)]
//                             None
//                         }
//                     }
//                 ),*
//             ],
//         }
//     };
//     (Constructor $name:expr ($($input:expr),*) -> $output:expr; $(cost $cost:expr;)? $(unextractable)?) => {
//         GenericCommand::Constructor {
//             span: Span,
//             name: Symbol::new($name),
//             schema: Schema {
//                 input: vec![$(Symbol::new($input)),*],
//                 output: Symbol::new($output),
//             },
//             cost: {
//                 $(
//                     let cost_val = $cost;
//                     Some(cost_val)
//                 )?
//                 #[allow(unreachable_code)]
//                 None
//             },
//             unextractable: {
//                 $(
//                    true
//                 )?
//                 #[allow(unreachable_code)]
//                 false
//             }
//
//         }
//     };
//     (Relation $name:expr ($($input:expr),*)) => {
//         GenericCommand::Relation {
//             span: Span,
//             name: Symbol::new($name),
//             inputs: vec![$(Symbol::new($input)),*],
//         }
//     };
//     (Function $name:expr ($($input:expr),*) -> $output:expr; merge $merge_head:expr ($($merge_args:expr),*)) => {
//         GenericCommand::Function {
//             span: Span,
//             name: Symbol::new($name),
//             schema: Schema {
//                 input: vec![$(Symbol::new($input)),*],
//                 output: Symbol::new($output),
//             },
//             merge: Some(GenericExpr::Call(Span, Symbol::new($merge_head), vec![$(GenericExpr::Var(Span, Symbol::new($merge_args))),*])),
//         }
//     };
//     (Function $name:expr ($($input:expr),*) -> $output:expr) => {
//         GenericCommand::Function {
//             span: Span,
//             name: Symbol::new($name),
//             schema: Schema {
//                 input: vec![$(Symbol::new($input)),*],
//                 output: Symbol::new($output),
//             },
//             merge: None
//         }
//     };
// }

// macro_rules! build_command {
//     // Sort with constructor arguments
//     (Sort $name:expr; $($constructor:expr, $($arg:expr),*);*) => {
//         GenericCommand::Sort(
//             Span,
//             Symbol::new($name),
//             Some((
//                 Symbol::new($name),
//                 vec![$(GenericExpr::Call(Span, Symbol::new($constructor), vec![$(GenericExpr::Var(Span, Symbol::new($arg))),*])),*]
//             ))
//         )
//     };
//
//     // Sort without arguments
//     (Sort $name:expr) => {
//         GenericCommand::Sort(Span, Symbol::new($name), None)
//     };
//
//     // Datatype
//     (Datatype $name:expr; $($variant:expr $(($($type:expr),*))? $(cost $cost:expr)?),*) => {{
//         let variants = vec![
//             $(
//                 {
//                     let types = vec![$( $( Symbol::new($type) ),* )?];
//                     let cost = {
//                         $(
//                             Some($cost)
//                         )?
//                         #[allow(unreachable_code)]
//                         None
//                     };
//
//                     Variant {
//                         span: Span,
//                         name: Symbol::new($variant),
//                         types,
//                         cost,
//                     }
//                 }
//             ),*
//         ];
//
//         GenericCommand::Datatype {
//             span: Span,
//             name: Symbol::new($name),
//             variants,
//         }
//     }};
//
//     // Constructor
//     (Constructor $name:expr ($($input:expr),*) -> $output:expr; $(cost $cost:expr;)? $(unextractable)?) => {{
//         let cost = {
//             $(
//                 Some($cost)
//             )?
//             #[allow(unreachable_code)]
//             None
//         };
//
//         let unextractable = {
//             $(
//                true
//             )?
//             #[allow(unreachable_code)]
//             false
//         };
//
//         GenericCommand::Constructor {
//             span: Span,
//             name: Symbol::new($name),
//             schema: Schema {
//                 input: vec![$(Symbol::new($input)),*],
//                 output: Symbol::new($output),
//             },
//             cost,
//             unextractable,
//         }
//     }};
//
//     // Relation
//     (Relation $name:expr ($($input:expr),*)) => {
//         GenericCommand::Relation {
//             span: Span,
//             name: Symbol::new($name),
//             inputs: vec![$(Symbol::new($input)),*],
//         }
//     };
//
//     // Function with merge
//     (Function $name:expr ($($input:expr),*) -> $output:expr; merge $merge_head:expr ($($merge_args:expr),*)) => {
//         GenericCommand::Function {
//             span: Span,
//             name: Symbol::new($name),
//             schema: Schema {
//                 input: vec![$(Symbol::new($input)),*],
//                 output: Symbol::new($output),
//             },
//             merge: Some(GenericExpr::Call(Span, Symbol::new($merge_head), vec![$(GenericExpr::Var(Span, Symbol::new($merge_args))),*])),
//         }
//     };
//
//     // Function without merge
//     (Function $name:expr ($($input:expr),*) -> $output:expr) => {
//         GenericCommand::Function {
//             span: Span,
//             name: Symbol::new($name),
//             schema: Schema {
//                 input: vec![$(Symbol::new($input)),*],
//                 output: Symbol::new($output),
//             },
//             merge: None,
//         }
//     };
// }

macro_rules! create_command {
    // Match for `Sort` command
    (Sort, $span:expr, $name:expr, $optional:expr) => {
        GenericCommand::Sort($span, Symbol::new($name), $optional)
    };

    // Match for `Datatype` command
    (Datatype, $span:expr, $name:expr, [$( { $v_span:expr, $v_name:expr, $v_types:expr, $v_cost:expr } ),*]) => {
        GenericCommand::Datatype {
            span: $span,
            name: Symbol::new($name),
            variants: vec![
                $(
                    Variant {
                        span: $v_span,
                        name: Symbol::new($v_name),
                        types: $v_types.into_iter().map(Symbol::new).collect(),
                        cost: $v_cost,
                    }
                ),*
            ],
        }
    };

    // Match for `Constructor` command
    (Constructor, $span:expr, $name:expr, $input:expr, $output:expr, $cost:expr, $unextractable:expr) => {
        GenericCommand::Constructor {
            span: $span,
            name: Symbol::new($name),
            schema: Schema {
                input: $input.into_iter().map(Symbol::new).collect(),
                output: Symbol::new($output),
            },
            cost: $cost,
            unextractable: $unextractable,
        }
    };

    // Match for `Relation` command
    (Relation, $span:expr, $name:expr, $inputs:expr) => {
        GenericCommand::Relation {
            span: $span,
            name: Symbol::new($name),
            inputs: $inputs.into_iter().map(Symbol::new).collect(),
        }
    };

    // Match for `Function` command
    (Function, $span:expr, $name:expr, $input:expr, $output:expr, $merge:expr) => {
        GenericCommand::Function {
            span: $span,
            name: Symbol::new($name),
            schema: Schema {
                input: $input.into_iter().map(Symbol::new).collect(),
                output: Symbol::new($output),
            },
            merge: $merge,
        }
    };
}
