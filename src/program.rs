use crate::facts::EgglogFacts;
use crate::rules::EgglogRules;
use crate::schedule::EgglogSchedules;
use crate::sorts::EgglogSorts;
use crate::EgglogSymbols;

pub type EgglogProgramSorts = (EgglogSymbols, EgglogSorts);
pub type EgglogProgramFacts = (EgglogSymbols, EgglogFacts);
pub type EgglogRuleList = Vec<EgglogRules>;
pub type EgglogScheduleList = Vec<EgglogSchedules>;
