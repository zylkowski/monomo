extern crate monomo_macros;
pub use monomo_macros::*;

// #[cfg(test)]
mod tests {
    use crate::rphize;

    trait Foo<T> {}
    struct B<T> {
        a: T,
    }
    rphize!(Foo<B<i32>>);

    // struct _Bar {
    //     baz: Box<crate::rph!(Foo<i32>)>,
    // }
}
