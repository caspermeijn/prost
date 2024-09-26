mod oneof_name_conflict {
    include!(concat!(env!("OUT_DIR"), "/oneof_name_conflict.rs"));
}

#[test]
/// Check nameing convention by creating an instance
fn test_creation() {
    let _ = oneof_name_conflict::Msg {
        field: Some(oneof_name_conflict::msg::FieldOneOf::B(
            oneof_name_conflict::msg::Field { c: 12 },
        )),
    };
}
