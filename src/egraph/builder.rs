use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Add;

use derive_getters::Getters;
use egglog::ast::{Command, GenericAction, Symbol};
use frunk::monoid::Monoid;
use frunk::semigroup::Semigroup;
use itertools::Itertools;

use super::facts::EgglogFacts;
use super::rules::EgglogRules;
use super::schedule::EgglogSchedules;
use super::sorts::EgglogSorts;
use super::{EgglogCommandList, EgglogSymbols};

pub type EgglogProgramSorts = (EgglogSymbols, EgglogSorts);
pub type EgglogProgramFacts = (EgglogSymbols, EgglogFacts);
pub type EgglogRuleList = Vec<EgglogRules>;
pub type EgglogScheduleList = Vec<EgglogSchedules>;

#[derive(Debug, Clone, Default, Getters)]
pub struct EgglogProgram {
    sorts: EgglogProgramSorts,
    facts: EgglogProgramFacts,
    rules: EgglogRuleList,
    schedules: EgglogScheduleList,
    bindings: EgglogSymbols,
}

pub struct EgglogProgramBuilder<State> {
    sorts: Option<EgglogProgramSorts>,
    facts: Option<EgglogProgramFacts>,
    rules: Option<EgglogRuleList>,
    schedules: Option<EgglogScheduleList>,
    bindings: Option<EgglogSymbols>,
    _state: PhantomData<State>,
}

trait InitProgram {}
trait InitSorts {}
trait InitFacts {}
trait InitRules {}
trait InitSchedules {}
trait InitBindings {}

pub struct InitState;
pub struct SortsState;
pub struct FactsState;
pub struct RulesState;
pub struct SchedulesState;
pub struct BindingsState;

impl InitProgram for InitState {}
impl InitSorts for SortsState {}
impl InitFacts for FactsState {}
impl InitRules for RulesState {}
impl InitSchedules for SchedulesState {}
impl InitBindings for BindingsState {}

fn get_sort_symbol(command: &Command) -> Vec<Symbol> {
    match command {
        Command::Sort(_span, symbol, _expr) => vec![symbol.to_owned()],
        Command::Datatype {
            span: _span,
            name: symbol,
            variants,
        } => {
            let mut variant_symbols = variants.iter().map(|variant| variant.name).collect_vec();
            variant_symbols.insert(0, symbol.to_owned());
            variant_symbols
        }
        Command::Relation {
            span: _span,
            name: symbol,
            inputs: _inputs,
        } => vec![symbol.to_owned()],
        Command::Function { name, .. } => vec![*name],
        Command::Constructor { name, .. } => vec![*name],
        _ => panic!("Egglog Command not supported in EgglogSorts {:?}.", command),
    }
}

fn get_fact_symbol(command: &Command) -> Symbol {
    match command {
        Command::Action(let_action) => {
            if let GenericAction::Let(__span, let_stmt_symbol, _let_stmt) = let_action {
                *let_stmt_symbol
            } else {
                panic!("Egglog Command not supported in EgglogFacts {:?}.", command)
            }
        }
        _ => panic!("Egglog Command not supported in EgglogFacts {:?}.", command),
    }
}

impl EgglogProgramBuilder<InitState> {
    pub const fn new() -> Self {
        Self {
            sorts: None,
            facts: None,
            rules: None,
            schedules: None,
            bindings: None,
            _state: PhantomData,
        }
    }

    pub fn sorts(self, sorts: EgglogSorts) -> EgglogProgramBuilder<SortsState> {
        let sort_symbol_lists: Vec<Vec<Symbol>> = sorts.iter().map(get_sort_symbol).collect_vec();
        let sort_symbols: EgglogSymbols = sort_symbol_lists.into_iter().flatten().collect();
        EgglogProgramBuilder {
            sorts: Some((sort_symbols, sorts)),
            facts: self.facts,
            rules: self.rules,
            schedules: None,
            bindings: None,
            _state: PhantomData,
        }
    }
}

impl EgglogProgramBuilder<SortsState> {
    pub fn facts(self, facts: EgglogFacts) -> EgglogProgramBuilder<FactsState> {
        let fact_symbol_lists: Vec<Symbol> = facts.iter().map(get_fact_symbol).collect_vec();
        let fact_symbols: EgglogSymbols = fact_symbol_lists.into_iter().collect();
        EgglogProgramBuilder {
            sorts: self.sorts,
            facts: Some((fact_symbols, facts)),
            rules: self.rules,
            schedules: None,
            bindings: None,
            _state: PhantomData,
        }
    }
}

impl EgglogProgramBuilder<FactsState> {
    pub fn rules(self, rules: EgglogRules) -> EgglogProgramBuilder<RulesState> {
        EgglogProgramBuilder {
            sorts: self.sorts,
            facts: self.facts,
            rules: Some(vec![rules]),
            schedules: None,
            bindings: None,
            _state: PhantomData,
        }
    }
}

impl EgglogProgramBuilder<RulesState> {
    pub fn schedules(self, schedules: EgglogSchedules) -> EgglogProgramBuilder<SchedulesState> {
        EgglogProgramBuilder {
            sorts: self.sorts,
            facts: self.facts,
            rules: self.rules,
            schedules: Some(vec![schedules]),
            bindings: None,
            _state: PhantomData,
        }
    }
}

impl EgglogProgramBuilder<SchedulesState> {
    pub fn bindings(self, bindings: EgglogSymbols) -> EgglogProgramBuilder<BindingsState> {
        EgglogProgramBuilder {
            sorts: self.sorts,
            facts: self.facts,
            rules: self.rules,
            schedules: self.schedules,
            bindings: Some(bindings),
            _state: PhantomData,
        }
    }
}

impl EgglogProgramBuilder<BindingsState> {
    pub fn program(self) -> EgglogProgram {
        EgglogProgram {
            sorts: self.sorts.expect("Sorts Guaranteed at compile-time."),
            facts: self.facts.expect("Facts Guaranteed at compile-time."),
            rules: self.rules.expect("Rules Guaranteed at compile-time."),
            schedules: self
                .schedules
                .expect("Schedules Guaranteed at compile-time."),
            bindings: self.bindings.expect("Bindings Guaranteed at compile-time."),
        }
    }
}

impl Semigroup for EgglogProgram {
    fn combine(&self, program_update: &Self) -> Self {
        let combined_sorts = self.sorts.1.clone().add_sorts(
            program_update
                .sorts
                .1
                .clone()
                .into_iter()
                .filter(|sort| !self.sorts.0.contains(&get_sort_symbol(sort)[0]))
                .collect_vec(),
        );
        let mut combined_sort_symbols = self.sorts.0.clone();
        combined_sort_symbols.extend(program_update.sorts.0.clone());

        let combined_facts = self.facts.1.clone().add_facts(
            program_update
                .facts
                .1
                .clone()
                .into_iter()
                .filter(|fact| !self.facts.0.contains(&get_fact_symbol(fact)))
                .collect_vec(),
        );
        let mut combined_fact_symbols = self.facts.0.clone();
        combined_fact_symbols.extend(program_update.facts.0.clone());

        let mut combined_rules = self.rules.clone();
        combined_rules.append(&mut program_update.rules.clone());
        let mut combined_schedules = self.schedules.clone();
        combined_schedules.append(&mut program_update.schedules.clone());
        let mut combined_bindings = self.bindings.clone();
        combined_bindings.extend(program_update.bindings.clone());
        Self {
            sorts: (combined_sort_symbols, combined_sorts),
            facts: (combined_fact_symbols, combined_facts),
            rules: combined_rules,
            schedules: combined_schedules,
            bindings: combined_bindings,
        }
    }
}

impl Monoid for EgglogProgram {
    fn empty() -> Self {
        Self::default()
    }
}

impl Add for EgglogProgram {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.sorts.1 = self.sorts.1.add_sorts(
            rhs.sorts
                .1
                .clone()
                .into_iter()
                .filter(|sort| !self.sorts.0.contains(&get_sort_symbol(sort)[0]))
                .collect_vec(),
        );
        self.sorts.0.extend(rhs.sorts.0);
        self.facts.1 = self.facts.1.add_facts(
            rhs.facts
                .1
                .clone()
                .into_iter()
                .filter(|fact| !self.facts.0.contains(&get_fact_symbol(fact)))
                .collect_vec(),
        );
        self.facts.0.extend(rhs.facts.0);
        self.rules.append(&mut rhs.rules);
        self.schedules.append(&mut rhs.schedules);
        self.bindings.extend(rhs.bindings);
        self
    }
}

impl From<EgglogProgram> for EgglogCommandList {
    fn from(program: EgglogProgram) -> Self {
        program
            .sorts
            .1
            .into_iter()
            .chain(
                program.facts.1.into_iter().chain(
                    program
                        .rules
                        .into_iter()
                        .flatten()
                        .chain(program.schedules.into_iter().flatten()),
                ),
            )
            .collect_vec()
    }
}

impl Display for EgglogProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command_list: EgglogCommandList = EgglogCommandList::from(self.clone());
        write!(f, "EgglogProgram Commands: {:?}", command_list)
    }
}

#[cfg(test)]
mod tests {
    use egglog::ast::{Command, Symbol};
    use egglog::EGraph;

    use super::*;

    #[test]
    fn egglog_program_default() {
        let _egglog_program = EgglogProgram::default();
    }

    #[test]
    fn egglog_program_method_order() {
        let sorts_data = EgglogSorts::default();
        let facts_data = EgglogFacts::default();
        let rules_data = EgglogRules::default();
        let schedules_data = EgglogSchedules::default();
        let symbols = EgglogSymbols::default();

        let _egglog_program = EgglogProgramBuilder::<InitState>::new()
            .sorts(sorts_data)
            .facts(facts_data)
            .rules(rules_data)
            .schedules(schedules_data)
            .bindings(symbols)
            .program();
    }

    #[test]
    fn combine_egglog_programs() {
        let sort_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts.egg");
        let input_sorts = EgglogSorts::default().add_sort_str(&sort_str);
        let facts_str = utilities::get_egglog_commands("llhd_dfg_example2_facts.egg");
        let input_facts = EgglogFacts::default().add_facts_str(&facts_str);

        let rules_str = utilities::get_egglog_commands("llhd_dfg_example2_rules.egg");
        let rules1 = EgglogRules::default().add_rule_str(&rules_str);
        let schedule1_str = utilities::get_egglog_commands("llhd_dfg_example2_schedule.egg");
        let schedule1 = EgglogSchedules::default().add_schedule_str(&schedule1_str);
        let symbols1: EgglogSymbols = [Symbol::new("foo")].into();
        let egglog_program = EgglogProgramBuilder::<InitState>::new()
            .sorts(input_sorts)
            .facts(input_facts)
            .rules(rules1)
            .schedules(schedule1)
            .bindings(symbols1)
            .program();

        let sort2_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts_updated.egg");
        let sorts2 = EgglogSorts::default().add_sort_str(&sort2_str);
        let rules2_str = utilities::get_egglog_commands("llhd_dfg_example2_rules_updated.egg");
        let rules2 = EgglogRules::default().add_rule_str(&rules2_str);
        let symbols2: EgglogSymbols = [Symbol::new("foobar"), Symbol::new("bar")].into();
        let schedule2_str =
            utilities::get_egglog_commands("llhd_dfg_example2_schedule_updated.egg");
        let schedule2 = EgglogSchedules::default().add_schedule_str(&schedule2_str);
        let egglog_program_update = EgglogProgramBuilder::<InitState>::new()
            .sorts(sorts2)
            .facts(EgglogFacts::default())
            .rules(rules2)
            .schedules(schedule2)
            .bindings(symbols2)
            .program();
        let updated_egglog_program = egglog_program.combine(&egglog_program_update);
        assert_eq!(11, updated_egglog_program.sorts.1.len());
        assert_eq!(1, updated_egglog_program.facts.1.len());
        assert_eq!(2, updated_egglog_program.rules.len());
        assert_eq!(2, updated_egglog_program.schedules.len());
        assert_eq!(3, updated_egglog_program.bindings.len());
        let updated_egglog_program_cmds: EgglogCommandList = updated_egglog_program.into();
        assert_eq!(18, updated_egglog_program_cmds.len());
        assert!(matches!(
            updated_egglog_program_cmds[0],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[1], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[2],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[3], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[4],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[5],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[6], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[7],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[8],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[9],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[10], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[11],
            Command::Action { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[12],
            Command::AddRuleset(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[13],
            Command::Rewrite(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[14],
            Command::AddRuleset(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[15],
            Command::Rule { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[16],
            Command::RunSchedule(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[17],
            Command::RunSchedule(..)
        ));
        if let Err(err_msg) = EGraph::default().run_program(updated_egglog_program_cmds) {
            panic!("Failure to run program: {:?}", err_msg);
        }
    }

    #[test]
    fn add_egglog_programs() {
        let sort_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts.egg");
        let input_sorts = EgglogSorts::default().add_sort_str(&sort_str);
        let facts_str = utilities::get_egglog_commands("llhd_dfg_example2_facts.egg");
        let input_facts = EgglogFacts::default().add_facts_str(&facts_str);

        let rules_str = utilities::get_egglog_commands("llhd_dfg_example2_rules.egg");
        let rules1 = EgglogRules::default().add_rule_str(&rules_str);
        let schedule1_str = utilities::get_egglog_commands("llhd_dfg_example2_schedule.egg");
        let schedule1 = EgglogSchedules::default().add_schedule_str(&schedule1_str);
        let symbols1: EgglogSymbols = [Symbol::new("foo")].into();
        let egglog_program = EgglogProgramBuilder::<InitState>::new()
            .sorts(input_sorts)
            .facts(input_facts)
            .rules(rules1)
            .schedules(schedule1)
            .bindings(symbols1)
            .program();

        let sort2_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts_updated.egg");
        let sorts2 = EgglogSorts::default().add_sort_str(&sort2_str);
        let rules2_str = utilities::get_egglog_commands("llhd_dfg_example2_rules_updated.egg");
        let rules2 = EgglogRules::default().add_rule_str(&rules2_str);
        let schedule2_str =
            utilities::get_egglog_commands("llhd_dfg_example2_schedule_updated.egg");
        let schedule2 = EgglogSchedules::default().add_schedule_str(&schedule2_str);
        let symbols2: EgglogSymbols = [Symbol::new("foobar"), Symbol::new("bar")].into();
        let egglog_program_update = EgglogProgramBuilder::<InitState>::new()
            .sorts(sorts2)
            .facts(EgglogFacts::default())
            .rules(rules2)
            .schedules(schedule2)
            .bindings(symbols2)
            .program();
        let updated_egglog_program = egglog_program + egglog_program_update;
        assert_eq!(81, updated_egglog_program.sorts.0.len());
        assert_eq!(11, updated_egglog_program.sorts.1.len());
        assert_eq!(1, updated_egglog_program.facts.1.len());
        assert_eq!(2, updated_egglog_program.rules.len());
        assert_eq!(2, updated_egglog_program.schedules.len());
        let updated_egglog_program_cmds: EgglogCommandList = updated_egglog_program.into();
        assert_eq!(18, updated_egglog_program_cmds.len());
        assert!(matches!(
            updated_egglog_program_cmds[0],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[1], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[2],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[3], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[4],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[5],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[6], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[7],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[8],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[9],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[10], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[11],
            Command::Action { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[12],
            Command::AddRuleset(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[13],
            Command::Rewrite(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[14],
            Command::AddRuleset(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[15],
            Command::Rule { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[16],
            Command::RunSchedule(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[17],
            Command::RunSchedule(..)
        ));
        if let Err(err_msg) = EGraph::default().run_program(updated_egglog_program_cmds) {
            panic!("Failure to run program: {:?}", err_msg);
        }
    }

    #[test]
    fn add_with_dup_egglog_programs() {
        let sort_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts.egg");
        let input_sorts = EgglogSorts::default().add_sort_str(&sort_str);
        let facts_str = utilities::get_egglog_commands("llhd_dfg_example2_facts.egg");
        let input_facts = EgglogFacts::default().add_facts_str(&facts_str);

        let rules_str = utilities::get_egglog_commands("llhd_dfg_example2_rules.egg");
        let rules1 = EgglogRules::default().add_rule_str(&rules_str);
        let schedule1_str = utilities::get_egglog_commands("llhd_dfg_example2_schedule.egg");
        let schedule1 = EgglogSchedules::default().add_schedule_str(&schedule1_str);
        let symbols1: EgglogSymbols = [Symbol::new("foo")].into();
        let egglog_program = EgglogProgramBuilder::<InitState>::new()
            .sorts(input_sorts)
            .facts(input_facts)
            .rules(rules1)
            .schedules(schedule1)
            .bindings(symbols1)
            .program();

        let sort2_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts.egg");
        let sorts2 = EgglogSorts::default().add_sort_str(&sort2_str);
        let symbols2: EgglogSymbols = [Symbol::new("foobar"), Symbol::new("bar")].into();
        let egglog_program_update = EgglogProgramBuilder::<InitState>::new()
            .sorts(sorts2)
            .facts(EgglogFacts::default())
            .rules(EgglogRules::default())
            .schedules(EgglogSchedules::default())
            .bindings(symbols2)
            .program();
        let updated_egglog_program = egglog_program + egglog_program_update;
        assert_eq!(76, updated_egglog_program.sorts.0.len());
        assert_eq!(8, updated_egglog_program.sorts.1.len());
        assert_eq!(1, updated_egglog_program.facts.1.len());
        assert_eq!(2, updated_egglog_program.rules.len());
        assert_eq!(2, updated_egglog_program.schedules.len());
        let updated_egglog_program_cmds: EgglogCommandList = updated_egglog_program.into();
        assert_eq!(12, updated_egglog_program_cmds.len());
        assert!(matches!(
            updated_egglog_program_cmds[0],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[1], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[2],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[3], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[4],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[5],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[6], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[7],
            Command::Datatype { .. }
        ));
        if let Err(err_msg) = EGraph::default().run_program(updated_egglog_program_cmds) {
            panic!("Failure to run program: {:?}", err_msg);
        }
    }

    #[test]
    fn display_egglog_program() {
        let sort_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts.egg");
        let input_sorts = EgglogSorts::default().add_sort_str(&sort_str);
        let facts_str = utilities::get_egglog_commands("llhd_dfg_example2_facts.egg");
        let input_facts = EgglogFacts::default().add_facts_str(&facts_str);

        let rules_str = utilities::get_egglog_commands("llhd_dfg_example2_rules.egg");
        let rules1 = EgglogRules::default().add_rule_str(&rules_str);
        let schedule1_str = utilities::get_egglog_commands("llhd_dfg_example2_schedule.egg");
        let schedule1 = EgglogSchedules::default().add_schedule_str(&schedule1_str);
        let symbols1: EgglogSymbols = [Symbol::new("foo")].into();
        let egglog_program = EgglogProgramBuilder::<InitState>::new()
            .sorts(input_sorts)
            .facts(input_facts)
            .rules(rules1)
            .schedules(schedule1)
            .bindings(symbols1)
            .program();
        let _egglog_program_str = egglog_program.to_string();
    }
}
