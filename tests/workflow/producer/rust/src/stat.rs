use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

mod expectations {
    pub const BeaconA: usize = 1;
    pub const BeaconsBeaconA: usize = 1;
    pub const BeaconsBeaconB: usize = 1;
    pub const BeaconsSubBeaconA: usize = 1;
    pub const GroupAStructA: usize = 1;
    pub const GroupAStructB: usize = 1;
    pub const GroupBGroupCStructA: usize = 1;
    pub const GroupBGroupCStructB: usize = 2;
    pub const GroupBStructA: usize = 1;
    pub const StructA: usize = 1;
    pub const StructB: usize = 2;
    pub const StructC: usize = 2;
    pub const StructD: usize = 3;
    pub const StructE: usize = 3;
    pub const StructF: usize = 2;
    pub const StructJ: usize = 2;
    pub const TriggerBeacons: usize = 1;
    pub const FinishConsumerTestBroadcast: usize = 1;
    pub const Connected: usize = 1;
    pub const Disconnected: usize = 1;
    pub const Shutdown: usize = 1;
    pub const StructEmpty: usize = 2;
    pub const StructEmptyA: usize = 1;
    pub const StructEmptyB: usize = 1;
}

#[derive(PartialEq, Hash, Clone)]
pub enum Alias {
    BeaconA,
    BeaconsBeaconA,
    BeaconsBeaconB,
    BeaconsSubBeaconA,
    GroupAStructA,
    GroupAStructB,
    GroupBGroupCStructA,
    GroupBGroupCStructB,
    GroupBStructA,
    StructA,
    StructB,
    StructC,
    StructD,
    StructE,
    StructF,
    StructJ,
    TriggerBeacons,
    FinishConsumerTestBroadcast,
    Connected,
    Disconnected,
    Shutdown,
    StructEmpty,
    StructEmptyA,
    StructEmptyB,
}

impl Eq for Alias {}

impl std::fmt::Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BeaconA => write!(f, "BeaconA"),
            Self::BeaconsBeaconA => write!(f, "BeaconsBeaconA"),
            Self::BeaconsBeaconB => write!(f, "BeaconsBeaconB"),
            Self::BeaconsSubBeaconA => write!(f, "BeaconsSubBeaconA"),
            Self::GroupAStructA => write!(f, "GroupAStructA"),
            Self::GroupAStructB => write!(f, "GroupAStructB"),
            Self::GroupBGroupCStructA => write!(f, "GroupBGroupCStructA"),
            Self::GroupBGroupCStructB => write!(f, "GroupBGroupCStructB"),
            Self::GroupBStructA => write!(f, "GroupBStructA"),
            Self::StructA => write!(f, "StructA"),
            Self::StructB => write!(f, "StructB"),
            Self::StructC => write!(f, "StructC"),
            Self::StructD => write!(f, "StructD"),
            Self::StructE => write!(f, "StructE"),
            Self::StructF => write!(f, "StructF"),
            Self::StructJ => write!(f, "StructJ"),
            Self::TriggerBeacons => write!(f, "TriggerBeacons"),
            Self::FinishConsumerTestBroadcast => write!(f, "FinishConsumerTestBroadcast"),
            Self::Connected => write!(f, "Connected"),
            Self::Disconnected => write!(f, "Disconnected"),
            Self::Shutdown => write!(f, "Shutdown"),
            Self::StructEmpty => write!(f, "StructEmpty"),
            Self::StructEmptyA => write!(f, "StructEmptyA"),
            Self::StructEmptyB => write!(f, "StructEmptyB"),
        }
    }
}

pub enum StatEvent {
    Inc(Alias),
}

pub struct Stat {
    done: usize,
    pub tests: HashMap<Alias, (usize, usize)>,
    pub indexes: HashMap<Alias, usize>,
}

impl Stat {
    pub fn new() -> Self {
        let mut tests = HashMap::new();
        tests.insert(Alias::BeaconA, (0, expectations::BeaconA));
        tests.insert(Alias::BeaconsBeaconA, (0, expectations::BeaconsBeaconA));
        tests.insert(Alias::BeaconsBeaconB, (0, expectations::BeaconsBeaconB));
        tests.insert(
            Alias::BeaconsSubBeaconA,
            (0, expectations::BeaconsSubBeaconA),
        );
        tests.insert(Alias::GroupAStructA, (0, expectations::GroupAStructA));
        tests.insert(Alias::GroupAStructB, (0, expectations::GroupAStructB));
        tests.insert(
            Alias::GroupBGroupCStructA,
            (0, expectations::GroupBGroupCStructA),
        );
        tests.insert(
            Alias::GroupBGroupCStructB,
            (0, expectations::GroupBGroupCStructB),
        );
        tests.insert(Alias::GroupBStructA, (0, expectations::GroupBStructA));
        tests.insert(Alias::StructA, (0, expectations::StructA));
        tests.insert(Alias::StructB, (0, expectations::StructB));
        tests.insert(Alias::StructC, (0, expectations::StructC));
        tests.insert(Alias::StructD, (0, expectations::StructD));
        tests.insert(Alias::StructE, (0, expectations::StructE));
        tests.insert(Alias::StructF, (0, expectations::StructF));
        tests.insert(Alias::StructJ, (0, expectations::StructJ));
        tests.insert(Alias::TriggerBeacons, (0, expectations::TriggerBeacons));
        tests.insert(
            Alias::FinishConsumerTestBroadcast,
            (0, expectations::FinishConsumerTestBroadcast),
        );
        tests.insert(Alias::Connected, (0, expectations::Connected));
        tests.insert(Alias::Disconnected, (0, expectations::Disconnected));
        tests.insert(Alias::Shutdown, (0, expectations::Shutdown));
        tests.insert(Alias::StructEmpty, (0, expectations::StructEmpty));
        tests.insert(Alias::StructEmptyA, (0, expectations::StructEmptyA));
        tests.insert(Alias::StructEmptyB, (0, expectations::StructEmptyB));
        Self {
            tests,
            done: 0,
            indexes: HashMap::new(),
        }
    }

    pub fn apply(&mut self, event: StatEvent) {
        match event {
            StatEvent::Inc(alias) => {
                if let Some((current, _)) = self.tests.get_mut(&alias) {
                    *current += 1;
                }
            }
        }
    }

    pub fn get_index(&mut self, alias: Alias) -> usize {
        *self.indexes.entry(alias.clone()).or_insert(0) += 1;
        self.indexes[&alias]
    }

    pub fn expectation(&self) -> usize {
        let mut all: usize = 0;
        for (_alias, (_current, expectation)) in &self.tests {
            all += expectation;
        }
        all
    }

    pub fn get_beacons_count(&self) -> usize {
        self.tests[&Alias::BeaconA].0
            + self.tests[&Alias::BeaconsBeaconA].0
            + self.tests[&Alias::BeaconsBeaconB].0
            + self.tests[&Alias::BeaconsSubBeaconA].0
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("{}\n", "=".repeat(70));
        let mut tests = vec![];
        for (alias, (current, expectation)) in &self.tests {
            tests.push((alias.to_string(), current, expectation));
        }
        tests.sort();
        for (alias, current, expectation) in &tests {
            let mut alias = alias.to_string();
            let filler = 60 - alias.len();
            alias = format!(
                "{}{}",
                alias,
                ".".repeat(if filler > 0 { filler } else { 0 })
            );
            output = format!(
                "{} {} {}: {}/{}\n",
                output,
                if current == expectation {
                    style("☑").bold().green().dim()
                } else {
                    style("☐").bold().red().dim()
                },
                style(format!("[{}]", alias)).bold().dim(),
                if current == expectation {
                    style(format!("{}", current)).dim()
                } else {
                    style(format!("{}", current)).bold().red().dim()
                },
                if current == expectation {
                    style(format!("{}", expectation)).dim()
                } else {
                    style(format!("{}", expectation)).bold().red().dim()
                }
            );
        }
        output = format!("{}{}\n", output, "=".repeat(70));
        write!(f, "{}", output)
    }
}
