use extension_fn::extension_fn;

#[extension_fn(trait AsRef<str>)]
pub fn count_numbers(&self) -> usize {
    self.as_ref().chars().fold(0, |count, char| {
        if char.is_numeric() {
            count + 1
        } else {
            count
        }
    } )
}

#[test]
fn asref_str_numbers_test() {
    assert_eq!(5, "23402p[pwer".to_string().count_numbers());
}