use crate::strategies::{Slot, Strategy};

/// This strategy tries to find the most specific match of a domain respecting the domain hierarchy.
///
/// For example take the following two domains we want to match for:
/// - `www.example.com`
/// - `example.com`
///
/// Expected behavior:
/// ```not_rust
/// Input -> Match
///
/// beispiel.de -> N/A
/// example.com -> example.com
/// subdomain.example.com -> example.com
/// www.example.com -> www.example.com
/// ```
pub struct DomainLevelsStrategy {
    // aho corasick is a magical algorithm that lets is search a byte sequence (string) for
    // a bundle of keywords in a very efficient way
    matcher: aho_corasick::AhoCorasick,
    table: Vec<Slot>,
}

impl Strategy for DomainLevelsStrategy {
    fn r#match(&self, string: &str) -> Option<Slot> {
        self.matcher.find_iter(self)
            // The matches must be aligned to the end of the domain
            .filter(|m| m.end() == string.len())
            // The match may not be inside of the domain, only two cases are valid:
            // 1. The match is at the beginning of the domain, meaning that it is a exact match
            // 2. The match is aligned to a part of the domain, so a dot is on the left of the match
            .filter(|m| m.start() == 0 || string.as_bytes()[m.start() - 1] == b'.')
            // Find the longest match as it is the most specific
            .min_by(|x, y| x.start().cmp(&y.start()))
            .map(|m| self.table[m.pattern()])
    }
}