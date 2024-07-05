include!(concat!(env!("OUT_DIR"), "/recursive.rs"));

#[test]
fn decode_resursive_empty_buffer() {
    // https://github.com/tokio-rs/prost/issues/924
    use prost::Message;
    let _ = Foo::decode("".as_bytes());
}
