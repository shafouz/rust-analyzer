use ide_db::RootDatabase;

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
    let t = TestStruct { a: 1, b: 2, c: "test", d: 1 };
    let r = RootDatabase::default();
}

enum TestEnum {
    A,
    B,
}
fn test1() {
    let t = TestEnum::A;
}

struct TestTuple(i32, i32);
fn test2() {
    let t = TestTuple(1, 2);
}

union TestUnion {
    a: i32,
    b: i32,
}
fn test3() {
    let t = TestUnion { a: 1 };
}

type Alias<'a> = TestStruct<'a, i32>;
fn test4() {
    let t = Alias { a: 1, b: 2, c: "test", d: 1 };
}

fn test5() {
    let t = TestStruct { a: 1, b: 2, c: "test", d: 1 };
}
