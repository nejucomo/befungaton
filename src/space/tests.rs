use indoc::indoc;
use test_case::test_case;

use crate::Space;
use crate::geometry::Position;

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
    ],
    vec![]
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
    ],
    vec![]
    ; "spin wider"
)]
#[test_case(
    indoc! { r#"
         v
        >G<
         ^
    "# },
    [
        indoc! { r#"
            ▶v
            >G<
             ^
        "# },
        indoc! { r#"
             ▼
            >G<
             ^
        "# },
        indoc! { r#"
             v
            >▶<
             ^
        "# },
        indoc! { r#"
             v
            >G◀
             ^
        "# },
        indoc! { r#"
             v
            >▼<
             ^
        "# },
        indoc! { r#"
             v
            >G<
             ▲
        "# },
        indoc! { r#"
             v
            >◀<
             ^
        "# },
        indoc! { r#"
             v
            ▶G<
             ^
        "# },
        indoc! { r#"
             v
            >▲<
             ^
        "# },
        indoc! { r#"
             ▼
            >G<
             ^
        "# }
    ],
    vec![]
    ; "ccw dance"
)]
#[test_case(
    indoc! { r#"
        =
    "# },
    [
        indoc! { r#"
            ▼
        "# },
        indoc! { r#"
            =
            ▼
        "# }
    ],
    vec![]
    ; "turn-if-zero empty stack"
)]
#[test_case(
    indoc! { r#"
        0=
    "# },
    [
        indoc! { r#"
            ▶=
        "# },
        indoc! { r#"
            0▶
        "# }
    ],
    vec![]
    ; "turn-if-zero pop 0"
)]
#[test_case(
    indoc! { r#"
        7=
    "# },
    [
        indoc! { r#"
            ▶=
        "# },
        indoc! { r#"
            7▲
        "# }
    ],
    vec![]
    ; "turn-if-zero pop 7"
)]
#[test_case(
    indoc! { r#"
        0123456789
    "# },
    [
        indoc! { r#"
            ▶123456789
        "# },
        indoc! { r#"
            0▶23456789
        "# },
        indoc! { r#"
            01▶3456789
        "# },
        indoc! { r#"
            012▶456789
        "# },
        indoc! { r#"
            0123▶56789
        "# },
        indoc! { r#"
            01234▶6789
        "# },
        indoc! { r#"
            012345▶789
        "# },
        indoc! { r#"
            0123456▶89
        "# },
        indoc! { r#"
            01234567▶9
        "# },
        indoc! { r#"
            012345678▶
        "# }
    ],
    (0..10).collect()
    ; "push all digits"
)]
fn evolve<const K: usize>(init: &str, expecteds: [&str; K], expected_stack: Vec<i32>) {
    let mut space: Space = init.parse().unwrap();
    let mut lastpos = Position::new(0, 0);

    for expected in expecteds {
        assert_eq!(&space.to_string(), expected);
        let ps = space.step_cursors();
        assert_eq!(ps.len(), 1);
        lastpos = ps[0];
    }

    let actual_stack = space.mut_cell(lastpos).unwrap().pop_cursor().unwrap().stack;
    assert_eq!(actual_stack, expected_stack);
}
