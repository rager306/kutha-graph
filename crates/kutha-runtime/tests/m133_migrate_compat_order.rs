//! M133 S01 / Phase 207 — migrate_compat must call migrate_op_wire before
//! migrate_pins (CE residual: remapped-count + markers stay green if swapped).

fn migrate_compat_body() -> &'static str {
    let src = include_str!("../src/store.rs");
    let start = src
        .find("pub fn migrate_compat(")
        .expect("migrate_compat");
    let rest = &src[start..];
    let end = rest[1..]
        .find("\npub fn ")
        .or_else(|| rest[1..].find("\nfn "))
        .map(|i| i + 1)
        .unwrap_or(rest.len());
    &rest[..end]
}

#[test]
fn migrate_compat_calls_op_wire_before_pins() {
    let body = migrate_compat_body();
    let op = body
        .find("migrate_op_wire(")
        .expect("migrate_compat must call migrate_op_wire");
    let pins = body
        .find("migrate_pins(")
        .expect("migrate_compat must call migrate_pins");
    assert!(
        op < pins,
        "migrate_op_wire must precede migrate_pins in migrate_compat; op={op} pins={pins}"
    );
}
