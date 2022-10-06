use std::fs::File;

#[test]
fn test_decode_pico_txt() {
    let txt = deco8::decode_pico_txt(File::open("tests/data/hello.p8.png").unwrap()).unwrap();
    let expected: Vec<u8> = vec![1, 2, 3];
    assert_eq!(txt, expected);
}
