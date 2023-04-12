use extension_fn::extension_fn;

#[extension_fn(str)]
pub fn count_numbers(&self) -> usize {
    self.chars().fold(0, |count, char| {
        if char.is_numeric() {
            count + 1
        } else {
            count
        }
    } )
}

#[test]
fn str_numbers_test() {
    assert_eq!(6, "243=-234-=".count_numbers())
}