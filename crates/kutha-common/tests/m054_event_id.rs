//! M054 S05 / Phase 121 — EventId UUID contract (CE whole-repo #24).
//! User ingest stays UUID v7; content-derived ids are RFC 9562 UUID v8.

use kutha_common::{Event, EventId, Op};

fn behavior(ingested: u64, subject: u32, pin: &str) -> Event {
    Event::stable_derived(
        Op::Behavior {
            name: "alsoKnows".into(),
            caused_by: EventId::nil(),
            rule_version: pin.into(),
            subject,
            relation: 1,
            object: 2,
            valid_from: 2010,
            valid_to: None,
        },
        ingested,
    )
}

fn is_rfc4122_v8(id: EventId) -> bool {
    id.get_version_num() == 8 && id.as_bytes()[8] & 0b1100_0000 == 0b1000_0000
}

/// #24 — derived Behavior/Retract ids must be UUID v8 with RFC 4122 variant.
#[test]
fn derived_event_ids_are_rfc9562_uuid_v8() {
    let samples = [
        behavior(1, 0, "pin-a"),
        behavior(2, 7, "pin-b"),
        behavior(99, 3, "pin-c"),
        Event::stable_derived(Op::Retract { event_id: EventId::nil() }, 4),
        Event::stable_derived(Op::Retract { event_id: EventId::nil() }, 11),
    ];
    for ev in &samples {
        assert!(
            is_rfc4122_v8(ev.id),
            "stable_derived must mint UUID v8/RFC4122, got version {} bytes {:02x?}",
            ev.id.get_version_num(),
            ev.id.as_bytes()
        );
    }
}

/// Content-derived Define encodings share the EventId UUID v8 contract.
#[test]
fn stable_define_event_ids_are_rfc9562_uuid_v8() {
    for name in ["inForceAs", "relatedTo", "kutha:allows"] {
        let ev = Event::stable_define(name, 0);
        assert!(
            is_rfc4122_v8(ev.id),
            "stable_define({name}) must mint UUID v8/RFC4122, got version {}",
            ev.id.get_version_num()
        );
    }
}

/// User ingest must keep UUID v7 (must not silently become v8).
#[test]
fn user_ingest_event_ids_stay_uuid_v7() {
    let ev = Event::new(
        Op::Assert {
            subject: 0,
            relation: 1,
            object: 2,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        },
        1,
    );
    assert_eq!(ev.id.get_version_num(), 7, "Event::new must remain UUID v7");
}

/// DET-02: overlay must not break hash-stable derived ids.
#[test]
fn stable_derived_is_deterministic() {
    let a = behavior(5, 9, "same-pin");
    let b = behavior(5, 9, "same-pin");
    assert_eq!(a.id, b.id);
}
