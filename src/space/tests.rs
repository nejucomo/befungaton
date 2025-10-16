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
    ]
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
    ]
    ; "turn-if-zero empty stack"
)]
#[test_case(
    indoc! { r#"
        v
        >0==
    "# },
    [
        indoc! { r#"
            ▼
            >0==
        "# },
        indoc! { r#"
            v
            ▶0==
        "# },
        indoc! { r#"
            v
            >▶==
        "# },
        indoc! { r#"
            v
            >0▶=
        "# },
        indoc! { r#"
            v
            >0=▼
        "# }
    ]
    ; "turn-if-zero pop 0 to empty"
)]
#[test_case(
    indoc! { r#"
        v
        >70==
    "# },
    [
        indoc! { r#"
            ▼
            >70==
        "# },
        indoc! { r#"
            v
            ▶70==
        "# },
        indoc! { r#"
            v
            >▶0==
        "# },
        indoc! { r#"
            v
            >7▶==
        "# },
        indoc! { r#"
            v
            >70▶=
        "# },
        indoc! { r#"
            v
            >70=▲
        "# }
    ]
    ; "turn-if-zero pop 0 non empty"
)]
#[test_case(
    indoc! { r#"
        v >=
        >3=
    "# },
    [
        indoc! { r#"
            ▼ >=
            >3=
        "# },
        indoc! { r#"
            v >=
            ▶3=
        "# },
        indoc! { r#"
            v >=
            >▶=
        "# },
        indoc! { r#"
            v >=
            >3▲
        "# },
        indoc! { r#"
            v ▶=
            >3=
        "# },
        indoc! { r#"
            v >▼
            >3=
        "# }
    ]
    ; "turn-if-zero pop 3 to empty"
)]
#[test_case(
    indoc! { r#"
        v  >=
        >53=
    "# },
    [
        indoc! { r#"
            ▼  >=
            >53=
        "# },
        indoc! { r#"
            v  >=
            ▶53=
        "# },
        indoc! { r#"
            v  >=
            >▶3=
        "# },
        indoc! { r#"
            v  >=
            >5▶=
        "# },
        indoc! { r#"
            v  >=
            >53▲
        "# },
        indoc! { r#"
            v  ▶=
            >53=
        "# },
        indoc! { r#"
            v  >▲
            >53=
        "# }
    ]
    ; "turn-if-zero pop 3 non empty"
)]
fn evolve<const K: usize>(init: &str, expecteds: [&str; K]) {
    let mut space: Space = init.parse().unwrap();

    for expected in expecteds {
        assert_eq!(&space.to_string(), expected);
        space.step_cursors();
    }
}
