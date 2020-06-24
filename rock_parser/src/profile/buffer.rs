use std::io::{BufReader, Read, Error};
use std::ops::{Shl, Shr};

use flate2::read::GzDecoder;

use crate::profile::Profile;
use rock_utils::errors::RockError;
use std::convert::From;
use std::string::ToString;

// ProfileDecoder is a main trait to decode the profile
pub trait ProfileDecoder {
    fn decode(data: &mut Vec<u8>) -> Result<Profile, RockError>;
}

// Constants that identify the encoding of a value on the wire.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireTypes {
    WireVarint = 0,
    WireFixed64 = 1,
    WireBytes = 2,
    WireFixed32 = 5,
}

impl From<usize> for WireTypes {
    fn from(var: usize) -> Self {
        match var {
            0 => self::WireTypes::WireVarint,
            1 => self::WireTypes::WireFixed64,
            2 => self::WireTypes::WireBytes,
            5 => self::WireTypes::WireFixed32,
            _ => panic!("unknown WireType"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Buffer {
    pub field: usize,
    pub r#type: WireTypes,
    pub u64: u64,
    pub data: Option<Vec<u8>>,
}

impl ProfileDecoder for Buffer {
    fn decode(data: &mut Vec<u8>) -> Result<Profile, RockError> {
        let mut p = Profile::default();
        let mut b = Buffer {
            field: 0,
            // 2 Length-delimited -> string, bytes, embedded messages, packed repeated fields
            r#type: WireTypes::WireBytes,
            u64: 0,
            data: None,
        };

        // check is there data gzipped
        // https://tools.ietf.org/html/rfc1952#page-5
        let is_data_gzipped = data.len() > 2 && data[0] == 0x1f && data[1] == 0x8b;
        // check if data is gzipped
        if is_data_gzipped {
            //
            let mut data = uncompress_gzip(&data).map_err(|e| RockError::ProfileUncompressFailed {
                reason: e.to_string(),
            })?;
            // pass uncompressed data
            decode_message(&mut b, &mut data, &mut p);
        } else {
            // or just RAW if it is not compressed
            decode_message(&mut b, data, &mut p);
        }

        // data not in the buffer, since the data in the buffer used for internal processing
        p.post_decode();
        match p.validate() {
            Ok(_) => Ok(p),
            Err(err) => panic!(err),
        }
    }
}

fn uncompress_gzip(data: &[u8]) -> Result<Vec<u8>, RockError> {
    let reader = BufReader::new(data);
    let mut gz_decoder = GzDecoder::new(reader);
    let mut uncompressed = vec![];
    match gz_decoder.read_to_end(&mut uncompressed) {
        Ok(_) => Ok(uncompressed),

        Err(err) => {
            Err(RockError::ProfileUncompressFailed {
                reason: err.to_string()
            })
        }
    }
}

#[inline]
pub fn decode_message(buf: &mut Buffer, data: &mut Vec<u8>, profile: &mut Profile) {
    if buf.r#type != WireTypes::WireBytes {
        panic!("WireTypes not Equal WireBytes");
    }

    while !data.is_empty() {
        // here we decode data, the algorithm is following:
        // 1. We pass whole data and buffer to the decode_field function
        // 2. As the result we get main data (which drained to the buffer size) and buffer with that drained data filled with other fields
        // 3. We also calculate field, type and u64 fields to pass it to Profile::decode_profile function
        match decode_field(buf, data) {
            Ok(()) => {
                Profile::decode_profile_field(profile, buf);
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

// decode_field is used to decode fields from incoming data
// buf -> buffer with data to allocate
// data -> unparsed data
#[inline]
#[allow(unused_assignments)]
pub fn decode_field(buf: &mut Buffer, data: &mut Vec<u8>) -> Result<(), RockError> {
    let varint = decode_varint(data)?;

    // decode
    // 90 -> 1011010
    // after right shift -> 1011, this is field number in proto
    // then we're doing AND operation and getting 7 bits
    buf.field = varint.shr(3);
    buf.r#type = WireTypes::from(varint & 7);
    buf.u64 = 0;

    // this is returned type
    match buf.r#type {
        //0
        WireTypes::WireVarint => {
            let variant = decode_varint(data)?;
            buf.u64 = variant as u64;
        }
        //1
        WireTypes::WireFixed64 => {
            if data.len() < 8 {
                return Err(RockError::DecodeFieldFailed {
                    reason: "data len less than 8 bytes".to_string(),
                });
            }
            buf.u64 = decode_fixed64(&data[..8]);
            // drain first 8 elements
            data.drain(..8);
        }
        //2
        WireTypes::WireBytes => {
            let varint = decode_varint(data)?;
            if varint > data.len() {
                return Err(RockError::DecodeFieldFailed {
                    reason: "too much data".to_string(),
                });
            }
            // buf.data = Rc::new(RefCell::new(data.borrow_mut()[..varint].into()));
            buf.data = Some(data[..varint].to_vec());
            // draint vec, start index removing decoded data
            data.drain(..varint);
        }
        //5
        WireTypes::WireFixed32 => {
            if data.len() < 4 {
                return Err(RockError::DecodeFieldFailed {
                    reason: "data len less than 8 bytes".to_string(),
                });
            }
            buf.u64 = decode_fixed32(&data[..4]) as u64;
            data.drain(..4);
        }
    }

    Ok(())
}

/// return parameters:
/// u8 --> current decoded varint
/// &[u8] --> subslice of incoming data w/o the decoded varint
/// todo!(https://github.com/golang/protobuf/commit/5d356b9d1c22e345c2ea08432302e82fd02d8a61);
#[inline(always)]
pub fn decode_varint(data: &mut Vec<u8>) -> Result<usize, RockError> {
    let mut u: usize = 0;
    let mut i: usize = 0;

    loop {
        // Message should be no more than 10 bytes
        if i >= 10 || i >= data.len() {
            return Err(RockError::DecodeFieldFailed {
                reason: "bad varint".to_string(),
            });
        }

        // get 7 bits except MSB
        // here is would be a number w/o the sign bit
        // 0x7F --> 127. So, if the number in the self.data[i]
        // is eq to 127 there is probably MSB would be set to 1, and if it is
        // there is would be a second 7 bits of information
        // than we shift like this:
        //  1010 1100 0000 0010
        //  →010 1100  000 0010
        // and doing OR, because OR is like an ADDITION while A & B == 0
        // 86 | 15104 == 15190
        //         01010110         OR
        // 0011101100000000
        // 0011101101010110 = 15190
        u |= (((data[i] & 0x7F) as u64).shl((7 * i) as u64)) as usize; // shl -> safe shift left operation
        // here we check all 8 bits for MSB
        // if all bits are zero, we'are done
        // if not, MSB is set and there is presents next byte to read
        if data[i] & 0x80 == 0 {
            // drain first i-th number of elements
            data.drain(..=i);
            return Ok(u);
        }
        i += 1;
    }
}

/// Decode WireType -- 1, Fixed64
#[inline]
pub fn decode_fixed64(p: &[u8]) -> u64 {
    ((p[0])
        | (p[1].shl(8))
        | (p[2].shl(16))
        | (p[3].shl(24))
        | (p[4].shl(32))
        | (p[5].shl(40))
        | (p[6].shl(48))
        | (p[7].shl(56))) as u64
}

/// Decode WireType -- 5, Fixed32
#[inline]
pub fn decode_fixed32(p: &[u8]) -> u32 {
    (p[0] | p[1].shl(8) | p[2].shl(16) | p[3].shl(24)) as u32
}

#[inline]
pub fn decode_string(v: &[u8]) -> String {
    std::str::from_utf8(v).unwrap().to_string()
}

#[cfg(test)]
mod profile_test {
    use std::collections::HashMap;
    use std::io::Read;

    use crate::profile::buffer::ProfileDecoder;

    #[test]
    fn parse() {
        // key - path to pb
        // value - path to related golden file
        let mut test_data = HashMap::<String, String>::new();
        test_data.insert(
            String::from("tests/HEAP.pb.gz"),
            String::from("tests/HEAP_GOLDEN.string"),
        );
        test_data.insert(
            String::from("tests/CPU.pb.gz"),
            String::from("tests/CPU_GOLDEN.string"),
        );

        test_data.insert(
            String::from("tests/encoded"),
            String::from("tests/encoded.string"),
        );

        test_data.insert(
            String::from("tests/RR_CPU.pb.gz"),
            String::from("tests/RR_CPU_golden.string"),
        );

        for (k, v) in test_data.iter() {
            let r_file_res = std::fs::File::open(k);
            let golden_file = std::fs::read_to_string(v).unwrap();
            match r_file_res {
                Ok(mut file) => {
                    let mut buffer = vec![];
                    let _ = file.read_to_end(&mut buffer);
                    let r = super::Buffer::decode(&mut buffer);
                    match r {
                        Ok(b) => {
                            assert_eq!(b.to_string().trim_end().eq(&golden_file), true);
                        }
                        Err(err) => {
                            panic!(err);
                        }
                    }
                }
                Err(err) => panic!(err),
            }
        }
    }
}
