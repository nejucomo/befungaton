use indoc::indoc;
use test_case::test_case;

use crate::Space;

#[test_case(
    indoc! { r#"
        >v
        ^<
    "# },
    [
        indoc! { r#"
            ▶v
            ^<
        "# },
        indoc! { r#"
            >▼
            ^<
        "# },
        indoc! { r#"
            >v
            ^◀
        "# },
        indoc! { r#"
            >v
            ▲<
        "# }
    ]
    ; "spin"
)]
#[test_case(
    indoc! { r#"
        > v
        ^ <
    "# },
    [
        indoc! { r#"
            ▶ v
            ^ <
        "# },
        indoc! { r#"
            >▶v
            ^ <
        "# },
        indoc! { r#"
            > ▼
            ^ <
        "# },
        indoc! { r#"
            > v
            ^ ◀
        "# },
        indoc! { r#"
            > v
            ^◀<
        "# },
        indoc! { r#"
            > v
            ▲ <
        "# }
    ]
    ; "spin wider"
)]
fn evolve<const K: usize>(init: &str, expecteds: [&str; K]) {
    let mut space: Space = init.parse().unwrap();

    for expected in expecteds {
        assert_eq!(&space.to_string(), expected);
        space.step_cursors();
    }
}
