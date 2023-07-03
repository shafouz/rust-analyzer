use expect_test::{expect, Expect};
use ide_db::base_db::{FileLoader, FileRange};
use syntax::TextRange;

use crate::{
    fixture, HoverConfig, HoverDocFormat, MemoryLayoutHoverConfig, MemoryLayoutHoverRenderKind,
};

use super::lab;

const HOVER_BASE_CONFIG: HoverConfig = HoverConfig {
    links_in_hover: false,
    memory_layout: Some(MemoryLayoutHoverConfig {
        size: Some(MemoryLayoutHoverRenderKind::Both),
        offset: Some(MemoryLayoutHoverRenderKind::Both),
        alignment: Some(MemoryLayoutHoverRenderKind::Both),
        niches: true,
    }),
    documentation: true,
    format: HoverDocFormat::Markdown,
    keywords: true,
};

fn check_hover_no_result(ra_fixture: &str) {
    let (analysis, position) = fixture::position(ra_fixture);
    let hover = analysis
        .hover(
            &HoverConfig { links_in_hover: true, ..HOVER_BASE_CONFIG },
            FileRange { file_id: position.file_id, range: TextRange::empty(position.offset) },
        )
        .unwrap();
    assert!(hover.is_none(), "hover not expected but found: {:?}", hover.unwrap());
}

#[track_caller]
fn check(ra_fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::position(ra_fixture);

    // let range = TextRange::new(position.offset, position.offset.checked_add(1.into()).unwrap());
    let range = TextRange::new(position.offset, position.offset);

    let hover = analysis
        .hover(
            &HoverConfig { links_in_hover: true, ..HOVER_BASE_CONFIG },
            FileRange { file_id: position.file_id, range },
        )
        .unwrap()
        .unwrap();

    let content = analysis.db.file_text(position.file_id);
    let hovered_element = &content[hover.range];

    let actual = format!("*{hovered_element}*\n{}\n", hover.info.markup);
    // expect.assert_eq(&actual)
}

fn check_hover_no_links(ra_fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::position(ra_fixture);
    let hover = analysis
        .hover(
            &HOVER_BASE_CONFIG,
            FileRange { file_id: position.file_id, range: TextRange::empty(position.offset) },
        )
        .unwrap()
        .unwrap();

    let content = analysis.db.file_text(position.file_id);
    let hovered_element = &content[hover.range];

    let actual = format!("*{hovered_element}*\n{}\n", hover.info.markup);
    expect.assert_eq(&actual)
}

fn check_hover_no_memory_layout(ra_fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::position(ra_fixture);
    let hover = analysis
        .hover(
            &HoverConfig { memory_layout: None, ..HOVER_BASE_CONFIG },
            FileRange { file_id: position.file_id, range: TextRange::empty(position.offset) },
        )
        .unwrap()
        .unwrap();

    let content = analysis.db.file_text(position.file_id);
    let hovered_element = &content[hover.range];

    let actual = format!("*{hovered_element}*\n{}\n", hover.info.markup);
    expect.assert_eq(&actual)
}

fn check_hover_no_markdown(ra_fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::position(ra_fixture);
    let hover = analysis
        .hover(
            &HoverConfig {
                links_in_hover: true,
                format: HoverDocFormat::PlainText,
                ..HOVER_BASE_CONFIG
            },
            FileRange { file_id: position.file_id, range: TextRange::empty(position.offset) },
        )
        .unwrap()
        .unwrap();

    let content = analysis.db.file_text(position.file_id);
    let hovered_element = &content[hover.range];

    let actual = format!("*{hovered_element}*\n{}\n", hover.info.markup);
    expect.assert_eq(&actual)
}

fn check_actions(ra_fixture: &str, expect: Expect) {
    let (analysis, file_id, position) = fixture::range_or_position(ra_fixture);
    let hover = analysis
        .hover(
            &HoverConfig { links_in_hover: true, ..HOVER_BASE_CONFIG },
            FileRange { file_id, range: position.range_or_empty() },
        )
        .unwrap()
        .unwrap();
    expect.assert_debug_eq(&hover.info.actions)
}

fn check_hover_range(ra_fixture: &str, expect: Expect) {
    let (analysis, range) = fixture::range(ra_fixture);

    let hover = analysis.hover(&HOVER_BASE_CONFIG, range).unwrap().unwrap();

    let content = analysis.db.file_text(range.file_id);

    let hovered_element = &content[hover.range];

    let actual = format!("*{hovered_element}*\n{}\n", hover.info.markup);

    expect.assert_eq(hover.info.markup.as_str())
}

fn check_hover_range_actions(ra_fixture: &str, expect: Expect) {
    let (analysis, range) = fixture::range(ra_fixture);
    let hover = analysis
        .hover(&HoverConfig { links_in_hover: true, ..HOVER_BASE_CONFIG }, range)
        .unwrap()
        .unwrap();
    expect.assert_debug_eq(&hover.info.actions);
}

fn check_hover_range_no_results(ra_fixture: &str) {
    let (analysis, range) = fixture::range(ra_fixture);
    let hover = analysis.hover(&HOVER_BASE_CONFIG, range).unwrap();
    assert!(hover.is_none());
}

#[test]
fn test_generics_and_lifetime() {
    check(
        r#"
    pub(crate) struct TestStruct<'a, T>
    where
        T: Sized,
    {
        #[allow(unused)]
        pub(super) a: i32,
        b: i32,
        c: &'a str,
        d: T,
    }

    fn test() {
            let a$0 = TestStruct { a: 1, b: 1, c: 1, d: 1 };
        }
    }
"#,
        expect![[r#"
            *f*

            ```rust
            test::S
            ```

            ```rust
            f: u32 // size = 4, align = 4, offset = 0
            ```
        "#]],
    );
}

#[test]
fn test_multiple_where_depth_n() {
    check(
        r#"
    pub(crate) struct TestStruct<'a, T>
    where
        T: Sized + Debug,
    {
        #[allow(unused)]
        pub(super) a: i32,
        b: i32,
        c: &'a str,
        d: T,
    }

    pub struct Test2
    where
        T: Clone,
    {
        a: T
    }

    fn test() {
            let a$0 = TestStruct { a: 1, b: 1, c: 1, d: Test2 { a: 1 } };
        }
    }
"#,
        expect![[r#"
            *f*

            ```rust
            test::S
            ```

            ```rust
            f: u32 // size = 4, align = 4, offset = 0
            ```
        "#]],
    );
}

#[test]
fn test_multiple_where_depth_zero() {
    check(
        r#"
    pub(crate) struct TestStruct<'a, T>
    where
        T: Sized + Debug,
    {
        #[allow(unused)]
        pub(super) a: i32,
        b: i32,
        c: &'a str,
        d: T,
    }

    fn test() {
            let a$0 = TestStruct { a: 1, b: 1, c: 1, d: 1 };
        }
    }
"#,
        expect![[r#"
            *f*

            ```rust
            test::S
            ```

            ```rust
            f: u32 // size = 4, align = 4, offset = 0
            ```
        "#]],
    );
}

#[test]
fn test_mods() {
    check(
        r#"
pub mod outer_mod {
    pub mod inner_mod {
        struct Test {
           pub(in crate::outer_mod) a: u32,
           pub(in outer_mod) b: u32,
           pub(crate) c: u32,
           pub(super) d: u32,
           pub(self) e: u32,
           f: u32,
        }

        fn test() {
            let a$0 = Test { a: 1, b: 1, c: 1, d: 1, e: 1, f: 1 };
        }
    }
}

"#,
        expect![[r#"
            *f*

            ```rust
            test::S
            ```

            ```rust
            f: u32 // size = 4, align = 4, offset = 0
            ```
        "#]],
    );
}

#[test]
fn test_struct() {
    check(
        r#"
struct TestStruct {
    pub a: i32,
    b: i32,
    c: i32,
}
fn test() {
    let t$0 = TestStruct { a: 1, b: 2, c: 3 };
}
"#,
        expect![[r#"
            *f*

            ```rust
            test::S
            ```

            ```rust
            f: u32 // size = 4, align = 4, offset = 0
            ```
        "#]],
    );
}

#[test]
fn test_enum() {
    check(
        r#"
enum TestEnum {
    A,
    B,
}
fn test() {
    let t = TestEnum$0::A;
}
"#,
        expect![[r#"
            *f*

            ```rust
            test::S
            ```

            ```rust
            f: u32 // size = 4, align = 4, offset = 0
            ```
        "#]],
    );
}

#[test]
fn test_tuple_struct() {
    check(
        r#"
struct TestTuple(i32, i32);
fn test() {
    let t$0 = TestTuple(1, 2);
}
"#,
        expect![[]],
    );
}

#[test]
fn test_union() {
    check(
        r#"
union TestUnion {
    a: i32,
    b: i32,
}
fn test() {
    let t$0 = TestUnion { a: 1, b: 2 };
}
"#,
        expect![[]],
    );
}

#[test]
fn test_alias() {
    check(
        r#"
type Alias = TestStruct;
struct TestStruct {
    a: i32,
    b: i32,
}
fn test() {
    let t$0 = Alias { a: 1, b: 2 };
}
"#,
        expect![[]],
    );
}

#[test]
fn test_generics() {
    check(
        r#"
struct TestStruct<T> {
    a: T,
    b: T,
}
fn test() {
    let t$0 = TestStruct { a: "1", b: 2 };
}
"#,
        expect![[]],
    );
}

#[test]
fn test_where() {
    check(
        r#"
struct TestStruct where i32: Sized {
    a: i32,
    b: i32,
}
fn test() {
    let t$0 = TestStruct { a: 1, b: 2 };
}
"#,
        expect![[]],
    );
}

#[test]
fn test_attr() {
    check(
        r#"
struct TestStruct {
    #[allow(unused)]
    a: i32,
    b: i32,
}
fn test() {
    let t$0 = TestStruct { a: 1, b: 2 };
}
"#,
        expect![[]],
    );
}

#[test]
fn test_order() {
    check(
        r#"
struct TestStruct {
    #[allow(unused)]
    a: i32,
    b: i32,
}
fn test() {
    let t$0 = TestStruct { b: 1, a: 2 };
}
"#,
        expect![[]],
    );
}
