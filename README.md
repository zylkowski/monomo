# monomo
Easy trait monomoprhization crate. Especially usefull with [typetag](https://crates.io/crates/typetag) crate.


## Example
Suppose we have a trait `Foo<T>` and we would like to monomorphize it in order to use it with `typetag`, 
as we cannot use generic trait.
```rust
trait Foo<T> {}

// Notice that you don't have to use typetag attribute, 
// and the trait will still be monomorphized. 
// typetag is just good example when you would like to use monomo.
monomo::rphize!(
    #[typetag::serde]
    Foo<i32>
);
```
Above macro expands to monomorphized version of `Foo<i32>`. 

In order to implement monomorphized trait for a struct we can do the following:
```rust
// Notice that you don't have to use typetag attribute, 
// and the trait will still be monomorphized. 
// typetag is just good example when you would like to use monomo.
#[monomo::rphize_impl]
#[typetage::serde]
impl Foo<i32> for Bar {
    // ...
}
```
Thanks to this you can later use `Foo<i32>` implementors as the monomorphized trait object.
```rust
struct BarContainer {
    bars: Vec<monomo::rph!(Foo<i32>)>
}
```