use console::style;
use std::collections::HashMap;

#[allow(non_upper_case_globals)]
mod expectations {
    // Expected broadcasts ============================
    pub const GroupAStructA: usize = 1;
    pub const GroupAStructB: usize = 1;
    pub const GroupBGroupCStructA: usize = 1;
    pub const GroupBGroupCStructB: usize = 2;
    pub const GroupBStructA: usize = 1;
    pub const StructA: usize = 1;
    pub const StructB: usize = 2;
    pub const StructC: usize = 2;
    pub const StructD: usize = 3;
    pub const StructF: usize = 2;
    pub const StructJ: usize = 2;
    // Expected broadcasts ============================
    pub const TriggerBeacons: usize = 1;
    pub const FinishConsumerTestBroadcast: usize = 1;
    pub const Connected: usize = 1;
    pub const Disconnected: usize = 1;
    pub const Shutdown: usize = 1;
    pub const TestRequestGroupAStructA: usize = 3;
    pub const TestRequestGroupAStructB: usize = 3;
    pub const TestRequestGroupBGroupCStructA: usize = 2;
    pub const TestRequestGroupBGroupCStructB: usize = 4;
    pub const TestRequestGroupBStructA: usize = 3;
    pub const TestRequestStructA: usize = 4;
    pub const TestRequestStructC: usize = 4;
    pub const TestRequestStructD: usize = 2;
    pub const TestRequestStructF: usize = 2;
    pub const TestRequestStructRmpty: usize = 2;
}

#[derive(PartialEq, Hash, PartialOrd)]
pub enum Alias {
    GroupAStructA,
    GroupAStructB,
    GroupBGroupCStructA,
    GroupBGroupCStructB,
    GroupBStructA,
    StructA,
    StructB,
    StructC,
    StructD,
    StructF,
    StructJ,
    TriggerBeacons,
    FinishConsumerTestBroadcast,
    Connected,
    Disconnected,
    Shutdown,
    TestRequestGroupAStructA,
    TestRequestGroupAStructB,
    TestRequestGroupBGroupCStructA,
    TestRequestGroupBGroupCStructB,
    TestRequestGroupBStructA,
    TestRequestStructA,
    TestRequestStructC,
    TestRequestStructD,
    TestRequestStructF,
    TestRequestStructRmpty,
}

impl Eq for Alias {}

impl std::fmt::Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GroupAStructA => write!(f, "GroupAStructA"),
            Self::GroupAStructB => write!(f, "GroupAStructB"),
            Self::GroupBGroupCStructA => write!(f, "GroupBGroupCStructA"),
            Self::GroupBGroupCStructB => write!(f, "GroupBGroupCStructB"),
            Self::GroupBStructA => write!(f, "GroupBStructA"),
            Self::StructA => write!(f, "StructA"),
            Self::StructB => write!(f, "StructB"),
            Self::StructC => write!(f, "StructC"),
            Self::StructD => write!(f, "StructD"),
            Self::StructF => write!(f, "StructF"),
            Self::StructJ => write!(f, "StructJ"),
            Self::TriggerBeacons => write!(f, "TriggerBeacons"),
            Self::FinishConsumerTestBroadcast => write!(f, "FinishConsumerTestBroadcast"),
            Self::Connected => write!(f, "Connected"),
            Self::Disconnected => write!(f, "Disconnected"),
            Self::Shutdown => write!(f, "Shutdown"),
            Self::TestRequestGroupAStructA => write!(f, "TestRequestGroupAStructA"),
            Self::TestRequestGroupAStructB => write!(f, "TestRequestGroupAStructB"),
            Self::TestRequestGroupBGroupCStructA => write!(f, "TestRequestGroupBGroupCStructA"),
            Self::TestRequestGroupBGroupCStructB => write!(f, "TestRequestGroupBGroupCStructB"),
            Self::TestRequestGroupBStructA => write!(f, "TestRequestGroupBStructA"),
            Self::TestRequestStructA => write!(f, "TestRequestStructA"),
            Self::TestRequestStructC => write!(f, "TestRequestStructC"),
            Self::TestRequestStructD => write!(f, "TestRequestStructD"),
            Self::TestRequestStructF => write!(f, "TestRequestStructF"),
            Self::TestRequestStructRmpty => write!(f, "TestRequestStructRmpty"),
        }
    }
}

pub enum StatEvent {
    Inc(Alias),
    ConsumerDone,
}

pub struct Stat {
    connections: usize,
    done: usize,
    pub tests: HashMap<Alias, (usize, usize)>,
}

impl Stat {
    pub fn new(connections: usize) -> Self {
        let mut tests = HashMap::new();
        tests.insert(
            Alias::GroupAStructA,
            (0, connections * expectations::GroupAStructA),
        );
        tests.insert(
            Alias::GroupAStructB,
            (0, connections * expectations::GroupAStructB),
        );
        tests.insert(
            Alias::GroupBGroupCStructA,
            (0, connections * expectations::GroupBGroupCStructA),
        );
        tests.insert(
            Alias::GroupBGroupCStructB,
            (0, connections * expectations::GroupBGroupCStructB),
        );
        tests.insert(
            Alias::GroupBStructA,
            (0, connections * expectations::GroupBStructA),
        );
        tests.insert(Alias::StructA, (0, connections * expectations::StructA));
        tests.insert(Alias::StructB, (0, connections * expectations::StructB));
        tests.insert(Alias::StructC, (0, connections * expectations::StructC));
        tests.insert(Alias::StructD, (0, connections * expectations::StructD));
        tests.insert(Alias::StructF, (0, connections * expectations::StructF));
        tests.insert(Alias::StructJ, (0, connections * expectations::StructJ));
        tests.insert(
            Alias::TriggerBeacons,
            (0, connections * expectations::TriggerBeacons),
        );
        tests.insert(
            Alias::FinishConsumerTestBroadcast,
            (0, connections * expectations::FinishConsumerTestBroadcast),
        );
        tests.insert(Alias::Connected, (0, connections * expectations::Connected));
        tests.insert(
            Alias::Disconnected,
            (0, connections * expectations::Disconnected),
        );
        tests.insert(Alias::Shutdown, (0, connections * expectations::Shutdown));
        tests.insert(
            Alias::TestRequestGroupAStructA,
            (0, connections * expectations::TestRequestGroupAStructA),
        );
        tests.insert(
            Alias::TestRequestGroupAStructB,
            (0, connections * expectations::TestRequestGroupAStructB),
        );
        tests.insert(
            Alias::TestRequestGroupBGroupCStructA,
            (
                0,
                connections * expectations::TestRequestGroupBGroupCStructA,
            ),
        );
        tests.insert(
            Alias::TestRequestGroupBGroupCStructB,
            (
                0,
                connections * expectations::TestRequestGroupBGroupCStructB,
            ),
        );
        tests.insert(
            Alias::TestRequestGroupBStructA,
            (0, connections * expectations::TestRequestGroupBStructA),
        );
        tests.insert(
            Alias::TestRequestStructA,
            (0, connections * expectations::TestRequestStructA),
        );
        tests.insert(
            Alias::TestRequestStructC,
            (0, connections * expectations::TestRequestStructC),
        );
        tests.insert(
            Alias::TestRequestStructD,
            (0, connections * expectations::TestRequestStructD),
        );
        tests.insert(
            Alias::TestRequestStructF,
            (0, connections * expectations::TestRequestStructF),
        );
        tests.insert(
            Alias::TestRequestStructRmpty,
            (0, connections * expectations::TestRequestStructRmpty),
        );
        Self {
            connections,
            tests,
            done: 0,
        }
    }

    pub fn apply(&mut self, event: StatEvent) {
        match event {
            StatEvent::Inc(alias) => {
                if let Some((current, _)) = self.tests.get_mut(&alias) {
                    *current += 1;
                }
            }
            StatEvent::ConsumerDone => {
                self.done += 1;
            }
        }
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
