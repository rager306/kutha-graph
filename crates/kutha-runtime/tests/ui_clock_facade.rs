//! M025 S02 / G-C09: public as_of / as_known_on reject wrong-role clocks and bare u64.

#[test]
fn as_of_and_as_known_on_reject_wrong_role_and_bare_u64() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/as_of_rejects_wall.rs");
    t.compile_fail("tests/ui/as_of_rejects_u64.rs");
    t.compile_fail("tests/ui/as_known_on_rejects_vt.rs");
    t.compile_fail("tests/ui/as_known_on_rejects_u64.rs");

    // Governor rust_test_asserts requires an asserting body; trybuild is the kill.
    let quantum = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/quantum.rs"));
    let fold = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/fold_query.rs"));
    assert!(
        quantum.contains("pub fn as_of(&self, vt: ValidTime)"),
        "Runtime::as_of must take ValidTime"
    );
    assert!(
        quantum.contains("pub fn as_known_on(&self, wall: WallTime)"),
        "Runtime::as_known_on must take WallTime"
    );
    assert!(
        fold.contains("pub fn as_of(&self, vt: ValidTime)"),
        "GraphFold::as_of must take ValidTime"
    );
    assert!(
        fold.contains("pub fn as_known_on(&self, wall: WallTime)"),
        "GraphFold::as_known_on must take WallTime"
    );
}
