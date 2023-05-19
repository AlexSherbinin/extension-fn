use extension_fn::extension_fn;

#[extension_fn(<T: std::cmp::Ord> Vec<T>)]
fn sorted(mut self) -> Self {
    self.sort();
    self
}