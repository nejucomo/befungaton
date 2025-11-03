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
#[test_case(
    indoc! { r#"
        :
    "# },
    [
        indoc! { r#"
            ▶
        "# }
    ],
    vec![]
    ; "dup empty stack"
)]
#[test_case(
    indoc! { r#"
        7:
    "# },
    [
        indoc! { r#"
            ▶:
        "# },
        indoc! { r#"
            7▶
        "# },
    ],
    vec![7, 7]
    ; "dup 7"
)]
#[test_case(
    indoc! { r#"
        23+
    "# },
    [
        indoc! { r#"
            ▶3+
        "# },
        indoc! { r#"
            2▶+
        "# },
        indoc! { r#"
            23▶
        "# }
    ],
    vec![5]
    ; "add 2 3"
)]
#[test_case(
    indoc! { r#"
        23-
    "# },
    [
        indoc! { r#"
            ▶3-
        "# },
        indoc! { r#"
            2▶-
        "# },
        indoc! { r#"
            23▶
        "# }
    ],
    vec![-1]
    ; "sub 2 3"
)]
#[test_case(
    indoc! { r#"
        23*
    "# },
    [
        indoc! { r#"
            ▶3*
        "# },
        indoc! { r#"
            2▶*
        "# },
        indoc! { r#"
            23▶
        "# }
    ],
    vec![6]
    ; "mul 2 3"
)]
#[test_case(
    indoc! { r#"
        52/
    "# },
    [
        indoc! { r#"
            ▶2/
        "# },
        indoc! { r#"
            5▶/
        "# },
        indoc! { r#"
            52▶
        "# }
    ],
    vec![2]
    ; "div 5 2"
)]
#[test_case(
    indoc! { r#"
        52%
    "# },
    [
        indoc! { r#"
            ▶2%
        "# },
        indoc! { r#"
            5▶%
        "# },
        indoc! { r#"
            52▶
        "# }
    ],
    vec![1]
    ; "rem 5 2"
)]
#[test_case(
    indoc! { r#"
        23570;
    "# },
    [
        indoc! { r#"
            ▶3570;
        "# },
        indoc! { r#"
            2▶570;
        "# },
        indoc! { r#"
            23▶70;
        "# },
        indoc! { r#"
            235▶0;
        "# },
        indoc! { r#"
            2357▶;
        "# },
        indoc! { r#"
            23570▶
        "# },
    ],
    vec![7, 3, 5, 2]
    ; "swapn"
)]
fn evolve<const K: usize>(init: &str, expecteds: [&str; K], expected_stack: Vec<i32>) {
    let mut space: Space = init.parse().unwrap();
    let mut lastpos = Position::new(0, 0);

    /// Is there no simpler way?
    fn after_first_char(s: &str) -> &str {
        let mut it = s.chars();
        it.next();
        it.as_str()
    }

    assert_eq!(
        after_first_char(&space.to_string()),
        after_first_char(init),
        "Space::from_str(s).to_string() != s ; neglecting first char"
    );

    for expected in expecteds {
        assert_eq!(&space.to_string(), expected);
        let ps = space.step_cursors();
        assert_eq!(ps.len(), 1);
        lastpos = ps[0];
    }

    let actual_stack = space.mut_cell(lastpos).unwrap().pop_cursor().unwrap().stack;
    assert_eq!(actual_stack, expected_stack);
}

#[test_case(
    indoc! { r#"
        19+:::**0;:3*:1;*7+++
    "# },
    20,
    vec![1337]
    ; "elite"
)]
fn check_final_stack(init: &str, steps: usize, expected: Vec<i32>) {
    let mut space: Space = init.parse().unwrap();
    let mut lastpos = Position::new(0, 0);

    for _ in 0..steps {
        let ps = space.step_cursors();
        assert_eq!(ps.len(), 1);
        lastpos = ps[0];
    }

    let actual = space.mut_cell(lastpos).unwrap().pop_cursor().unwrap().stack;
    assert_eq!(actual, expected);
}
