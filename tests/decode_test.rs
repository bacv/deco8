use std::fs::File;

#[test]
fn test_decode_pico_txt() {
    let c = deco8::decode_pico_txt(File::open("tests/data/hello.p8.png").unwrap()).unwrap();
    assert_eq!(c.version(), deco8::Version::V0);
    assert_eq!(c.lua.to_string(), "hello".to_string());
}
