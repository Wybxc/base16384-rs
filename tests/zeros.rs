use base16384::{Base16384, Base16384Utf8};

#[test]
fn zeros_32() {
    let data = [0u8; 32];
    let mut buf = vec![0u16; Base16384::encode_len(data.len())];
    let buf = Base16384::encode_to_slice(&data, &mut buf);

    let mut data = [0x4e00u16; 20];
    data[19] = 0x3d04;

    assert_eq!(buf.len(), data.len());
    assert_eq!(buf, &data[..]);
}

#[test]
fn zeros_7x() {
    for i in 0..7 {
        let data = vec![0u8; 7 + i];
        let mut buf = vec![0u16; Base16384::encode_len(data.len())];
        let buf = Base16384::encode_to_slice(&data, &mut buf);

        let mut data = vec![0x4e00u16; Base16384::encode_len(data.len())];
        if i != 0 {
            let last = data.len() - 1;
            data[last] = 0x3d00 + i as u16;
        }
        assert_eq!(buf, data);
    }
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

#[test]
fn zeros_100k_utf8() {
    let data = vec![0u8; 1024000];
    let buf = Base16384Utf8::encode(&data);

    let mut data = std::iter::repeat([0xE4, 0xB8, 0x80])
        .take(585143)
        .flatten()
        .collect::<Vec<_>>();
    data.extend([0xE3, 0xB4, 0x85]);
    let data = String::from_utf8(data).unwrap();

    assert_eq!(buf.len(), data.len());
    assert_eq!(buf, &data[..]);
}
