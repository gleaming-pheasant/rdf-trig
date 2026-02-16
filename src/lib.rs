mod literal;
mod namespace;
pub mod raw;

pub use literal::Literal;
pub use namespace::{Namespace, NamespaceId};

#[cfg(test)]
mod tests {
    #[test]
    fn quick_mafs() {
        let two_plus_two = 2 + 2;
        assert_eq!(two_plus_two, 4);

        let minus_one = two_plus_two - 1;
        assert_eq!(minus_one, 3);
    }
}