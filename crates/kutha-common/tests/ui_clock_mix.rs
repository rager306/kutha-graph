//! TH-01: type error (E0308), not a syntax-error rustdoc compile_fail.

#[test]
fn clock_mix_is_e0308() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/clock_mix.rs");
}
