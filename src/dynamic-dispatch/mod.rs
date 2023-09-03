pub fn spellcheck(input: &str, spellchecker: C) -> String {
    for change in spellchecker.check(input) {
        apply_change(&mut  result, change);
    }

    result
}

trait Spellchecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

fn apply_change(string: &mut String, change: Change) {

}

enum Change {
    Delete(std::ops::Range<usize>),
    Replace(std::ops::Range<usize>, String),
}
