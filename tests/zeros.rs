use base16384::Base16384;

#[test]
fn zeros() {
    let data = [0u8; 32];
    let mut buf = vec![0u16; Base16384::encode_len(data.len())];
    let buf = Base16384::encode_to_slice(&data, &mut buf);

    let mut data = [0x4e00u16; 20];
    data[19] = 0x3d04;

    assert_eq!(buf.len(), data.len());
    assert_eq!(buf, &data[..]);
}

#[test]
fn zeros_100k() {
    let data = vec![0u8; 1024000];
    let mut buf = vec![0u16; Base16384::encode_len(data.len())];
    let buf = Base16384::encode_to_slice(&data, &mut buf);

    let mut data = vec![0x4e00u16; 585144];
    data[585143] = 0x3d05;

    assert_eq!(buf.len(), data.len());
    assert_eq!(buf, &data[..]);
}
