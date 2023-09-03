pub fn spellcheck(input: &str, spellchecker: C) -> String {
    for change in spellchecker.check(input) {
        apply_change(&mut  result, change);
    }

    result
}

trait Spellchecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

struct CapitalLetterSpellChecker;

impl Spellchecker for AntispaceChecker {
    fn check(&self, input: &str) -> Vec<Change> {
        input
            .match_indices(" ")
            .map(|(index, space)| {
                Change::Delete(index..index + space.len())
            }).collect()
    }
}

impl Spellchecker for NoopSpellchecker {
    fn check(&self, input: &str) -> Vec<Change> {
        Vec::new()
    }
}

fn apply_change(string: &mut String, change: Change) {

}

enum Change {
    Delete(std::ops::Range<usize>),
    Replace(std::ops::Range<usize>, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Hello world, I know you are looking at me.";
        let result = spellcheck(text, NoopSpellchecker);
        assert!(result == test);
        let result = spellcheck(text, AntispaceChecker);
    }
}
