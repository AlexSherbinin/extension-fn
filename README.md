This crate provides the extension_fn macro for extending types with extension functions.
# Example
For example there is a count_numbers extension function for str:
```rust
/// Replacement for:
/// Sealed is internal trait that used to provide only one implementation of trait and nobody outside module can implement this
/// pub trait CountNumbers: Sealed {
///     fn count_numbers(&self) -> u32;
/// }
/// impl CountNumbers for str {
///     fn count_numbers(&self) -> u32 { ... }
/// }
#[extension_fn(str)]
pub fn count_numbers(&self) -> u32 {
     self.chars().fold(0, |count, char| {
        if char.is_numeric() {
         count + 1
        } else {
           count
        }
    })
}
```
You can extend using async functions by adding async-trait to your dependencies:
```toml
[dependencies]
async-trait = "*"
```
Also you can extend types that matching trait bound:
```rust
#[extension_fn(trait AsRef<str>)]
pub fn count_numbers(&self) { ... }
```