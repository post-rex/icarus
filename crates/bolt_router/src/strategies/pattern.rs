use crate::strategies::{Builder, Slot, Strategy};

pub struct RegexStrategy {
    regex_set: regex::RegexSet,
    table: Vec<Slot>,
}

pub struct RegexStrategyBuilder {
    patterns: Vec<String>,
    table: Vec<Slot>,
}

impl Strategy for RegexStrategy {
    fn r#match(&self, segment: &str) -> Option<Slot> {
        self.regex_set
            .matches(segment)
            .into_iter()
            .next()
            .map(|i| self.table[i])
    }
}

impl Builder for RegexStrategyBuilder {
    type Strategy = RegexStrategy;
    type Error = regex::Error;

    fn build(self) -> Result<Self::Strategy, Self::Error> {
        Ok(RegexStrategy {
            regex_set: self.regex_set.build()?,
            table: self.table,
        })
    }

    fn add(&mut self, pattern: &str, slot: Slot) -> &mut Self {
        self.patterns.push(pattern.to_string());
        self.table.push(slot);
        self
    }
}
