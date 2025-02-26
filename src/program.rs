use crate::facts::EgglogFacts;
use crate::rules::EgglogRules;
use crate::schedule::EgglogSchedules;
use crate::sorts::EgglogSorts;
use egglog::ast::{Command, Symbol};
use indexmap::IndexSet;

pub type EgglogProgramSorts = (EgglogSymbols, EgglogSorts);
pub type EgglogProgramFacts = (EgglogSymbols, EgglogFacts);
pub type EgglogRuleList = Vec<EgglogRules>;
pub type EgglogScheduleList = Vec<EgglogSchedules>;
pub type EgglogCommandList = Vec<Command>;
pub type EgglogSymbols = IndexSet<Symbol>;
