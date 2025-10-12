use indoc::indoc;
use test_case::test_case;

use crate::Space;

#[test_case(indoc! {
    "▶"
})]
fn test_display(expected: &str) {
    let space = Space::default();
    assert_eq!(&space.to_string(), expected);
}
