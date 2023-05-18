//! No boilerplate code for extension function definitions.

#[macro_use]
extern crate public;

use args::{Args};
use builder::Builder;
use syn::{
    parse_macro_input, ItemFn,
};

mod args;
mod builder;


/// Extension function macro. Example usage:
/// ```rust
/// use extension_fn::extension_fn;
///
/// // Also you can change pub to anything e.g. pub(crate) to make this function public only for current crate.
/// #[extension_fn(str)]
/// pub fn count_numbers(&self) -> u32 {
///     self.chars().fold(0, |count, char| {
///        if char.is_numeric() {
///         count + 1
///        } else {
///           count
///        }
///    })
/// }
/// ```
/// Example with async function(you should add async-trait to your dependencies`):
/// ```rust
/// use extension_fn::extension_fn;
/// use std::net::TcpStream;
///
/// #[extension_fn(TcpStream)]
/// pub async fn do_something() {}
/// ```
/// Example for trait bound:
/// ```rust
/// use extension_fn::extension_fn;
///
/// #[extension_fn(trait AsRef<str>)]
/// fn print(&self) {
///     println!("{}", self.as_ref());
/// }
/// ```
/// Example for generics:
/// ```rust
/// use extension_fn::extension_fn;
/// use std::hash::Hash;
/// use std::collections::HashMap;
/// 
/// #[extension_fn(<K: Hash + Eq, V> HashMap<K, V>)]
/// fn insert_if_not_exists(&mut self, key: K, value: V) {
///     if self.get(&key).is_none() {
///        self.insert(key, value);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn extension_fn(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as Args);
    let fn_implementation = parse_macro_input!(input as ItemFn);
    Builder::new(&fn_implementation.sig.ident).build(fn_implementation, args).into()
}