//! Encode binary files to printable utf16

#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[cfg(all(feature = "alloc", not(any(feature = "std", test))))]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std as alloc;

pub mod error;
pub mod utils;

use error::Base16384DecodeError;

pub struct Base16384;

impl Base16384 {
    /// Returns the minimum number of u16s needed to encode the given number of bytes.
    #[inline]
    pub const fn encode_len(data_len: usize) -> usize {
        let n = data_len / 7 * 4;
        let r = data_len % 7;
        match r {
            0 => n,
            1 => n + 2,
            2 | 3 => n + 3,
            4 | 5 => n + 4,
            6 => n + 5,
            _ => unreachable!(),
        }
    }

    /// The first code point of Base16384. It is the character '一' (U+4E00).
    pub const START: u16 = 0x4E00;

    /// The start of the Base16384 padding code points. The padding code points are
    /// "㴀㴁㴂㴃㴄㴅㴆" (U+3D00 to U+3D06).
    pub const PADDING_OFFSET: u16 = 0x3D00;

    /// Encodes the given data as Base16384 in a new allocated vector.
    ///
    /// # Examples
    /// ```
    /// use base16384::Base16384;
    ///
    /// let data = b"12345678";
    /// let encoded = Base16384::encode(data);
    ///
    /// let text = String::from_utf16(&encoded).unwrap();
    /// assert_eq!(text, "婌焳廔萷尀㴁");
    /// ```
    #[cfg(any(feature = "std", test, feature = "alloc"))]
    pub fn encode(data: &[u8]) -> alloc::vec::Vec<u16> {
        let capacity = Self::encode_len(data.len());
        let mut result = alloc::vec::Vec::with_capacity(capacity);

        // SAFETY: `encode_chunk` guarantees that N is non-zero.
        let (chunks, remainder) = unsafe { utils::slice_as_chunks(data) };
        for chunk in chunks {
            let mut buf = [0u16; 4];
            result.extend_from_slice(Self::encode_chunk(chunk, &mut buf));
        }
        if !remainder.is_empty() {
            let mut buf = [0u16; 4];
            result.extend_from_slice(Self::encode_remainder(remainder, &mut buf));
            result.push(0x3D00 | remainder.len() as u16)
        }
        result
    }

    /// Encodes the given data as Base16384 into the given buffer.
    ///
    /// # Panics
    /// Panics if the buffer is too small. Use [`Base16384::encode_len`] to get the required capacity.
    ///
    /// # Examples
    /// ```
    /// use base16384::Base16384;
    ///
    /// let data = b"12345678";
    /// let mut buf = [0u16; 6];
    /// let encoded = Base16384::encode_to_slice(data, &mut buf);
    ///
    /// let text = String::from_utf16(encoded).unwrap();
    /// assert_eq!(text, "婌焳廔萷尀㴁");
    /// ```
    pub fn encode_to_slice<'a>(data: &[u8], buf: &'a mut [u16]) -> &'a [u16] {
        let capacity = Self::encode_len(data.len());
        assert!(buf.len() >= capacity);

        // SAFETY: `encode_chunk` guarantees that N is non-zero.
        let (chunks, remainder) = unsafe { utils::slice_as_chunks(data) };
        let mut i = 0;
        for chunk in chunks {
            let mut tmp = [0u16; 4];
            let encoded = Self::encode_chunk(chunk, &mut tmp);
            buf[i..i + 4].copy_from_slice(encoded);
            i += 4;
        }
        if !remainder.is_empty() {
            let mut tmp = [0u16; 4];
            let encoded = Self::encode_remainder(remainder, &mut tmp);
            buf[i..i + encoded.len()].copy_from_slice(encoded);
            i += encoded.len();
            buf[i] = 0x3D00 | remainder.len() as u16;
            i += 1;
        }
        &buf[..i]
    }

    #[inline]
    fn encode_chunk<'a>(chunk: &[u8; 7], buf: &'a mut [u16; 4]) -> &'a [u16; 4] {
        let b0_hi = chunk[0] as u16;
        let b0_lo = chunk[1] as u16;
        buf[0] = Self::START + ((b0_hi << 6) | (b0_lo >> 2));

        let b1_hi = (chunk[1] & 0x03) as u16;
        let b1_md = chunk[2] as u16;
        let b1_lo = chunk[3] as u16;
        buf[1] = Self::START + ((b1_hi << 12) | (b1_md << 4) | (b1_lo >> 4));

        let b2_hi = (chunk[3] & 0x0F) as u16;
        let b2_md = chunk[4] as u16;
        let b2_lo = chunk[5] as u16;
        buf[2] = Self::START + ((b2_hi << 10) | (b2_md << 2) | (b2_lo >> 6));

        let b3_hi = (chunk[5] & 0x3F) as u16;
        let b3_lo = chunk[6] as u16;
        buf[3] = Self::START + ((b3_hi << 8) | b3_lo);

        buf
    }

    #[inline]
    fn encode_remainder<'a>(remainder: &[u8], buf: &'a mut [u16; 4]) -> &'a [u16] {
        let mut chunk = [0u8; 7];
        chunk[..remainder.len()].copy_from_slice(remainder);
        Self::encode_chunk(&chunk, buf);
        &buf[..remainder.len() / 2 + 1]
    }

    /// Returns the minimum number of bytes needed to decode the given number of u16s.
    /// The given offset is the padding code point of the last chunk (if exists).
    ///
    /// # Panics
    /// Panics if the given offset is out of the Base16384 padding code points range
    /// (see [`Base16384::PADDING_OFFSET`]).
    #[inline]
    pub fn decode_len(mut data_len: usize, padding: Option<u16>) -> usize {
        let r = if let Some(offset) = padding {
            let offset = offset - Self::PADDING_OFFSET;
            assert!((0..7).contains(&offset));
            match offset {
                0 => data_len -= 1,
                1 => data_len -= 2,
                2 | 3 => data_len -= 3,
                4 | 5 => data_len -= 4,
                6 => data_len -= 5,
                _ => unreachable!(),
            };
            offset
        } else {
            0
        };
        data_len / 4 * 7 + r as usize
    }

    /// Gets the padding code point of the last chunk (if exists).
    #[inline]
    pub fn padding(last: u16) -> Option<u16> {
        if (Self::PADDING_OFFSET..Self::PADDING_OFFSET + 7).contains(&last) {
            Some(last)
        } else {
            None
        }
    }

    /// Decodes the given Base16384 data into a new allocated vector.
    ///
    /// # Examples
    /// ```
    /// use base16384::Base16384;
    ///
    /// let data = "婌焳廔萷尀㴁".encode_utf16().collect::<Vec<_>>();
    /// let decoded = Base16384::decode(&data).unwrap();
    /// assert_eq!(decoded, b"12345678");
    /// ```
    #[cfg(any(feature = "std", test, feature = "alloc"))]
    pub fn decode(data: &[u16]) -> Result<alloc::vec::Vec<u8>, Base16384DecodeError> {
        let padding = data.last().cloned().and_then(Self::padding);
        let capacity = Self::decode_len(data.len(), padding);
        let mut result = alloc::vec::Vec::with_capacity(capacity);

        let (data, remainder) = if let Some(padding) = padding {
            let (data, remainder) = data.split_at(
                data.len()
                    - match padding - Self::PADDING_OFFSET {
                        0 => 1,
                        1 => 2,
                        2 | 3 => 3,
                        4 | 5 => 4,
                        6 => 5,
                        _ => unreachable!(),
                    },
            );
            (data, &remainder[..remainder.len() - 1])
        } else {
            (data, &[][..])
        };
        if data.len() % 4 != 0 {
            return Err(Base16384DecodeError::InvalidLength);
        }

        // SAFETY: `decode_chunk` guarantees that N is non-zero,
        // and length of data is checked to be a multiple of N.
        let chunks = unsafe { utils::slice_as_chunks_exact(data) };
        for chunk in chunks {
            let mut buf = [0u8; 7];
            result.extend_from_slice(Self::decode_chunk(chunk, &mut buf)?);
        }
        if let Some(padding) = padding {
            let mut buf = [0u8; 7];
            result.extend_from_slice(Self::decode_remainder(remainder, &mut buf, padding)?);
        }
        Ok(result)
    }

    /// Decodes the given Base16384 data into the given buffer.
    ///
    /// # Panics
    /// Panics if the buffer is too small. Use [`Base16384::decode_len`] to get the required capacity.
    ///
    /// # Examples
    /// ```
    /// use base16384::Base16384;
    ///
    /// let data = "婌焳廔萷尀㴁".encode_utf16().collect::<Vec<_>>();
    /// let mut buf = [0u8; 8];
    /// let decoded = Base16384::decode_to_slice(&data, &mut buf).unwrap();
    /// assert_eq!(decoded, b"12345678");
    /// ```
    pub fn decode_to_slice<'a>(
        data: &[u16],
        buf: &'a mut [u8],
    ) -> Result<&'a [u8], Base16384DecodeError> {
        let padding = data.last().cloned().and_then(Self::padding);
        let capacity = Self::decode_len(data.len(), padding);
        assert!(buf.len() >= capacity);

        let (data, remainder) = if let Some(padding) = padding {
            let (data, remainder) = data.split_at(
                data.len()
                    - match padding - Self::PADDING_OFFSET {
                        0 => 1,
                        1 => 2,
                        2 | 3 => 3,
                        4 | 5 => 4,
                        6 => 5,
                        _ => unreachable!(),
                    },
            );
            (data, &remainder[..remainder.len() - 1])
        } else {
            (data, &[][..])
        };
        if data.len() % 4 != 0 {
            return Err(Base16384DecodeError::InvalidLength);
        }

        // SAFETY: `decode_chunk` guarantees that N is non-zero,
        // and length of data is checked to be a multiple of N.
        let chunks = unsafe { utils::slice_as_chunks_exact(data) };
        let mut i = 0;
        for chunk in chunks {
            let mut tmp = [0u8; 7];
            let decoded = Self::decode_chunk(chunk, &mut tmp)?;
            buf[i..i + 7].copy_from_slice(decoded);
            i += 7;
        }
        if let Some(padding) = padding {
            let mut tmp = [0u8; 7];
            let decoded = Self::decode_remainder(remainder, &mut tmp, padding)?;
            buf[i..i + decoded.len()].copy_from_slice(decoded);
            i += decoded.len();
        }
        Ok(&buf[..i])
    }

    #[inline]
    fn is_valid_char(c: u16) -> bool {
        (Self::START..Self::START + 0x3FFF).contains(&c)
    }

    #[inline]
    fn decode_chunk<'a>(
        chunk: &[u16; 4],
        buf: &'a mut [u8; 7],
    ) -> Result<&'a [u8; 7], Base16384DecodeError> {
        if let Some(index) = chunk.iter().position(|&c| !Self::is_valid_char(c)) {
            return Err(Base16384DecodeError::InvalidCharacter { index });
        }
        let b0 = chunk[0] - Self::START;
        let b1 = chunk[1] - Self::START;
        let b2 = chunk[2] - Self::START;
        let b3 = chunk[3] - Self::START;

        buf[0] = (b0 >> 6) as u8;
        buf[1] = ((b0 & 0x3F) << 2 | (b1 >> 12)) as u8;
        buf[2] = (b1 >> 4) as u8;
        buf[3] = ((b1 & 0x0F) << 4 | (b2 >> 10)) as u8;
        buf[4] = (b2 >> 2) as u8;
        buf[5] = ((b2 & 0x03) << 6 | (b3 >> 8)) as u8;
        buf[6] = b3 as u8;

        Ok(buf)
    }

    #[inline]
    fn decode_remainder<'a>(
        remainder: &[u16],
        buf: &'a mut [u8; 7],
        padding: u16,
    ) -> Result<&'a [u8], Base16384DecodeError> {
        let mut chunk = [Self::START; 4];
        chunk[..remainder.len()].copy_from_slice(remainder);
        Self::decode_chunk(&chunk, buf)?;
        Ok(&buf[..(padding - Self::PADDING_OFFSET) as usize])
    }
}
