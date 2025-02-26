use super::facts::EgglogFacts;
use super::rules::EgglogRules;
use super::schedule::EgglogSchedules;
use super::sorts::EgglogSorts;
use super::EgglogSymbols;

pub type EgglogProgramSorts = (EgglogSymbols, EgglogSorts);
pub type EgglogProgramFacts = (EgglogSymbols, EgglogFacts);
pub type EgglogRuleList = Vec<EgglogRules>;
pub type EgglogScheduleList = Vec<EgglogSchedules>;
