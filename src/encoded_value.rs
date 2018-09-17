// Shane Isbell licenses this file to you under the Apache License, Version 2.0
// (the "License"); you may not use this file except in compliance with the License.
//
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License. See the NOTICE file distributed with this work for
// additional information regarding copyright ownership.
use byteorder::ReadBytesExt;
use std::io::Cursor;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EncodedValue {
    pub value_type: u8,
    pub value: Vec<u8>,
}

pub fn number_of_bytes(encoded_type: u8) -> u32 {
    match encoded_type {
        0x00 => 1,
        0x02 => 2,
        0x03 => 2,
        0x04 => 4,
        0x06 => 8,
        0x10 => 4,
        0x11 => 8,
        0x15 => 4,
        0x16 => 4,
        0x17 => 4,
        0x18 => 4,
        0x19 => 4,
        0x1a => 4,
        0x1b => 4, //enum
        0x1c => 0, //encoded array
        0x1d => 0, //annotation
        0x1e => 0, //NULL
        0x1f => 0, //boolean - bit in arg value
        _ => 0,
    }
}

impl EncodedValue {
    pub fn read(data: &mut Cursor<&Vec<u8>>) -> EncodedValue {
        let value_type = data.read_u8().unwrap();
        let value = number_of_bytes(value_type << 5);
        EncodedValue {
            value_type,
            value: Vec::new(),
        }
    }
}
