use crate::utils;
use crate::Base16384;
pub struct Base16384Utf8;
use crate::error::Base16384DecodeError;
use crate::utils::slice_as_chunks_exact;

impl Base16384Utf8 {
    /// Returns the minimum number of bytes needed to encode a `data_len`-byte
    #[inline]
    pub const fn encode_len(data_len: usize) -> usize {
        Base16384::encode_len(data_len) * 3
    }

    const START_HI: u16 = Base16384::START >> 6;
    const START_LO: u8 = (Base16384::START & 0x3F) as u8;

    const START_UTF8_HI: u8 = 0xE0 | (Self::START_HI >> 6) as u8;
    const START_UTF8_MD: u8 = 0x80 | (Self::START_HI & 0x3F) as u8;
    const START_UTF8_LO: u8 = 0x80 | Self::START_LO;

    #[allow(dead_code)]
    #[allow(clippy::assertions_on_constants)]
    const START_LO_MUST_BE_ZERO: () = assert!(Self::START_LO == 0, "START_LO must be 0");

    const PADDING_OFFSET_HI: u8 = 0xE0 | (Base16384::PADDING_OFFSET >> 12) as u8;
    const PADDING_OFFSET_MD: u8 = 0x80 | ((Base16384::PADDING_OFFSET >> 6) & 0x3F) as u8;
    const PADDING_OFFSET_LO: u8 = 0x80 | (Base16384::PADDING_OFFSET & 0x3F) as u8;

    /// Encodes the given data as Base16384 in a new allocated [`String`].
    ///
    /// # Examples
    /// ```
    /// use base16384::Base16384Utf8;
    ///
    /// let data = b"12345678";
    /// let encoded = Base16384Utf8::encode(data);
    ///
    /// assert_eq!(encoded, "婌焳廔萷尀㴁");
    /// ```
    ///
    /// [`String`]: alloc::string::String
    #[cfg(any(feature = "std", test, feature = "alloc"))]
    pub fn encode(data: &[u8]) -> alloc::string::String {
        let capacity = Self::encode_len(data.len());
        let mut result = alloc::vec::Vec::with_capacity(capacity);

        // SAFETY: `encode_chunk` guarantees that N is non-zero.
        let (chunks, remainder) = unsafe { utils::slice_as_chunks(data) };
        for chunk in chunks {
            let mut buf = [0u8; 12];
            result.extend_from_slice(Self::encode_chunk(chunk, &mut buf));
        }
        if !remainder.is_empty() {
            let mut buf = [0u8; 12];
            result.extend_from_slice(Self::encode_remainder(remainder, &mut buf));
            result.extend([
                Self::PADDING_OFFSET_HI,
                Self::PADDING_OFFSET_MD,
                Self::PADDING_OFFSET_LO | (remainder.len() as u8),
            ]);
        }
        unsafe { alloc::string::String::from_utf8_unchecked(result) }
    }

    /// Encodes the given data as Base16384 into the given buffer.
    ///
    /// # Panics
    /// Panics if the buffer is too small. Use [`Base16384::encode_len`] to get the required capacity.
    ///
    /// # Examples
    /// ```
    /// use base16384::Base16384Utf8;
    ///
    /// let data = b"12345678";
    /// let mut buf = "A".repeat(18);
    /// let encoded = Base16384Utf8::encode_to_slice(data, &mut buf);
    ///
    /// assert_eq!(encoded, "婌焳廔萷尀㴁");
    /// ```
    pub fn encode_to_slice<'a>(data: &[u8], buf: &'a mut str) -> &'a str {
        let buf = unsafe { buf.as_bytes_mut() };
        let capacity = Self::encode_len(data.len());
        assert!(buf.len() >= capacity, "buffer is too small");

        // SAFETY: `encode_chunk` guarantees that N is non-zero.
        let (chunks, remainder) = unsafe { utils::slice_as_chunks(data) };
        let mut i = 0;
        for chunk in chunks {
            let mut tmp = [0u8; 12];
            buf[i..i + 12].copy_from_slice(Self::encode_chunk(chunk, &mut tmp));
            i += 12;
        }
        if !remainder.is_empty() {
            let mut tmp = [0u8; 12];
            let encoded = Self::encode_remainder(remainder, &mut tmp);
            buf[i..i + encoded.len()].copy_from_slice(encoded);
            i += encoded.len();
            buf[i] = Self::PADDING_OFFSET_HI;
            buf[i + 1] = Self::PADDING_OFFSET_MD;
            buf[i + 2] = Self::PADDING_OFFSET_LO | (remainder.len() as u8);
            i += 3;
        }
        unsafe { core::str::from_utf8_unchecked(&buf[..i]) }
    }

    #[inline]
    fn encode_chunk<'a>(chunk: &[u8; 7], buf: &'a mut [u8; 12]) -> &'a [u8; 12] {
        let b0_hi = chunk[0] as u16 + Self::START_HI;
        let b0_lo = chunk[1] >> 2;
        buf[0] = 0xE0 | (b0_hi >> 6) as u8;
        buf[1] = 0x80 | (b0_hi & 0x3F) as u8;
        buf[2] = 0x80 | b0_lo;

        let b1_hi = ((chunk[1] & 0x03) << 6 | (chunk[2] >> 2)) as u16 + Self::START_HI;
        let b1_lo = (chunk[2] & 0x03) << 4 | (chunk[3] >> 4);
        buf[3] = 0xE0 | (b1_hi >> 6) as u8;
        buf[4] = 0x80 | (b1_hi & 0x3F) as u8;
        buf[5] = 0x80 | b1_lo;

        let b2_hi = ((chunk[3] & 0x0F) << 4 | (chunk[4] >> 4)) as u16 + Self::START_HI;
        let b2_lo = (chunk[4] & 0x0F) << 2 | (chunk[5] >> 6);
        buf[6] = 0xE0 | (b2_hi >> 6) as u8;
        buf[7] = 0x80 | (b2_hi & 0x3F) as u8;
        buf[8] = 0x80 | b2_lo;

        let b3_hi = ((chunk[5] & 0x3F) << 2 | (chunk[6] >> 6)) as u16 + Self::START_HI;
        let b3_lo = chunk[6] & 0x3F;
        buf[9] = 0xE0 | (b3_hi >> 6) as u8;
        buf[10] = 0x80 | (b3_hi & 0x3F) as u8;
        buf[11] = 0x80 | b3_lo;

        buf
    }

    #[inline]
    fn encode_remainder<'a>(remainder: &[u8], buf: &'a mut [u8; 12]) -> &'a [u8] {
        let mut chunk = [0u8; 7];
        chunk[..remainder.len()].copy_from_slice(remainder);
        Self::encode_chunk(&chunk, buf);
        &buf[..(remainder.len() / 2 + 1) * 3]
    }

    /// Returns the minimum number of bytes needed to decode the given number of bytes of utf8 data.
    /// The given offset is the padding code point of the last chunk (if exists).
    ///
    /// # Panics
    /// Panics if the given offset is out of the Base16384 padding code points range
    /// (see [`Base16384::PADDING_OFFSET`]).
    #[inline]
    pub fn decode_len(data_len: usize, padding: Option<u16>) -> usize {
        assert!(data_len % 3 == 0, "data_len must be a multiple of 3");
        Base16384::decode_len(data_len / 3, padding)
    }

    /// Gets the padding code point of the last chunk (if exists).
    #[inline]
    pub fn padding(last: [u8; 3]) -> Option<u16> {
        if last[0] & 0xF0 != 0xE0 || last[1] & 0xC0 != 0x80 || last[2] & 0xC0 != 0x80 {
            return None;
        }
        let b0 = (last[0] & 0x0F) as u16;
        let b1 = (last[1] & 0x3F) as u16;
        let b2 = (last[2] & 0x3F) as u16;
        Base16384::padding(b0 << 12 | b1 << 6 | b2)
    }

    /// Decodes the given utf8 data as Base16384 in a new allocated vector.
    ///
    /// # Examples
    /// ```
    /// use base16384::Base16384Utf8;
    ///
    /// let data = "婌焳廔萷尀㴁";
    /// let decoded = Base16384Utf8::decode(data).unwrap();
    /// assert_eq!(decoded, b"12345678");
    /// ```
    #[cfg(any(feature = "std", test, feature = "alloc"))]
    pub fn decode(data: &str) -> Result<alloc::vec::Vec<u8>, Base16384DecodeError> {
        if data.is_empty() {
            return Ok(alloc::vec::Vec::new());
        }
        if data.len() % 3 != 0 {
            return Err(Base16384DecodeError::InvalidLength);
        }
        if data.len() < 3 {
            return Err(Base16384DecodeError::InvalidLength);
        }

        let data = data.as_bytes();
        let padding = &data[data.len() - 3..];
        let padding = Self::padding(padding.try_into().unwrap());
        let capacity = Self::decode_len(data.len(), padding);
        let mut result = alloc::vec::Vec::with_capacity(capacity);

        let padding_size = padding.map(|padding| padding - Base16384::PADDING_OFFSET);
        let last_chunk_size = padding_size.map(|padding_size| match padding_size {
            0 => 1,
            1 => 2,
            2 | 3 => 3,
            4 | 5 => 4,
            6 => 5,
            _ => unreachable!(),
        });

        let (data, remainder) = if let Some(last_chunk_size) = last_chunk_size {
            let (data, remainder) = data.split_at(data.len() - last_chunk_size * 3);
            (data, &remainder[..remainder.len() - 3])
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
        if let Some(padding_size) = padding_size {
            let mut buf = [0u8; 7];
            result.extend_from_slice(Self::decode_remainder(remainder, &mut buf, padding_size)?);
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
    /// use base16384::Base16384Utf8;
    ///
    /// let data = "婌焳廔萷尀㴁";
    /// let mut buf = [0u8; 8];
    /// let decoded = Base16384Utf8::decode_to_slice(&data, &mut buf).unwrap();
    /// assert_eq!(decoded, b"12345678");
    /// ```
    pub fn decode_to_slice<'a>(
        data: &str,
        buf: &'a mut [u8],
    ) -> Result<&'a [u8], Base16384DecodeError> {
        if data.is_empty() {
            return Ok(&buf[..0]);
        }
        if data.len() % 3 != 0 {
            return Err(Base16384DecodeError::InvalidLength);
        }
        if data.len() < 3 {
            return Err(Base16384DecodeError::InvalidLength);
        }

        let data = data.as_bytes();
        let padding = &data[data.len() - 3..];
        let padding = Self::padding(padding.try_into().unwrap());
        let capacity = Self::decode_len(data.len(), padding);
        assert!(buf.len() >= capacity, "buffer is too small");

        let padding_size = padding.map(|padding| padding - Base16384::PADDING_OFFSET);
        let last_chunk_size = padding_size.map(|padding_size| match padding_size {
            0 => 1,
            1 => 2,
            2 | 3 => 3,
            4 | 5 => 4,
            6 => 5,
            _ => unreachable!(),
        });

        let (data, remainder) = if let Some(last_chunk_size) = last_chunk_size {
            let (data, remainder) = data.split_at(data.len() - last_chunk_size * 3);
            (data, &remainder[..remainder.len() - 3])
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
        if let Some(padding_size) = padding_size {
            let mut tmp = [0u8; 7];
            let decoded = Self::decode_remainder(remainder, &mut tmp, padding_size)?;
            buf[i..i + decoded.len()].copy_from_slice(decoded);
            i += decoded.len();
        }
        Ok(&buf[..i])
    }

    #[inline]
    fn valid_char(c: [u8; 3]) -> Option<u16> {
        let b0 = (c[0] & 0x0F) as u16;
        let b1 = (c[1] & 0x3F) as u16;
        let b2 = (c[2] & 0x3F) as u16;
        let c = b0 << 12 | b1 << 6 | b2;
        if (Base16384::START..Base16384::START + 0x3FFF).contains(&c) {
            Some(c)
        } else {
            None
        }
    }

    #[inline]
    fn decode_chunk<'a>(
        chunk: &[u8; 12],
        buf: &'a mut [u8; 7],
    ) -> Result<&'a [u8; 7], Base16384DecodeError> {
        let mut chars = [0u16; 4];
        unsafe {
            for (i, c) in slice_as_chunks_exact(chunk).iter().enumerate() {
                let c = Self::valid_char(*c)
                    .ok_or(Base16384DecodeError::InvalidCharacter { index: i * 3 })?;
                chars[i] = c;
            }
        };
        let b0 = chars[0] - Base16384::START;
        let b1 = chars[1] - Base16384::START;
        let b2 = chars[2] - Base16384::START;
        let b3 = chars[3] - Base16384::START;

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
        remainder: &[u8],
        buf: &'a mut [u8; 7],
        padding_size: u16,
    ) -> Result<&'a [u8], Base16384DecodeError> {
        let mut chunk = [
            Self::START_UTF8_HI,
            Self::START_UTF8_MD,
            Self::START_UTF8_LO,
            Self::START_UTF8_HI,
            Self::START_UTF8_MD,
            Self::START_UTF8_LO,
            Self::START_UTF8_HI,
            Self::START_UTF8_MD,
            Self::START_UTF8_LO,
            Self::START_UTF8_HI,
            Self::START_UTF8_MD,
            Self::START_UTF8_LO,
        ];
        chunk[..remainder.len()].copy_from_slice(remainder);
        Self::decode_chunk(&chunk, buf)?;
        Ok(&buf[..padding_size as usize])
    }
}
