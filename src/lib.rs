extern crate monomo_macros;
pub use monomo_macros::*;

#[cfg(test)]
mod tests {
    use crate::*;

    trait Foo<T> {}
    struct B;

    rphize!(Foo<i32>);

    #[rphize_impl]
    impl Foo<i32> for B {}
}
