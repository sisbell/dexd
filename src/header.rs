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
use byteorder::{LittleEndian, ReadBytesExt};
use prelude::get_bytes;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct DexFileHeader {
    pub magic: Vec<u8>,

    pub checksum: u32,

    pub signature: Vec<u8>,

    pub file_size: u32,

    pub header_size: u32,

    pub endian_tag: u32,

    pub link_size: u32,

    pub link_off: u32,

    pub map_off: u32,

    pub string_ids_size: u32,

    pub string_ids_off: u32,

    pub type_ids_size: u32,

    pub type_ids_off: u32,

    pub proto_ids_size: u32,

    pub proto_ids_off: u32,

    pub field_ids_size: u32,

    pub field_ids_off: u32,

    pub method_ids_size: u32,

    pub method_ids_off: u32,

    pub class_defs_size: u32,

    pub class_defs_off: u32,

    pub data_size: u32,

    pub data_off: u32,
}

pub trait HeaderDexer {
    fn dex_header(&mut self) -> DexFileHeader;

    fn u32(&mut self) -> u32;
}

impl HeaderDexer for File {
    fn u32(&mut self) -> u32 {
        self.read_u32::<LittleEndian>().unwrap()
    }

    fn dex_header(&mut self) -> DexFileHeader {
        DexFileHeader {
            magic: get_bytes(&self, 8),
            checksum: self.u32(),
            signature: get_bytes(&self, 20),
            file_size: self.u32(),
            header_size: self.u32(),
            endian_tag: self.u32(),
            link_size: self.u32(),
            link_off: self.u32(),
            map_off: self.u32(),
            string_ids_size: self.u32(),
            string_ids_off: self.u32(),
            type_ids_size: self.u32(),
            type_ids_off: self.u32(),
            proto_ids_size: self.u32(),
            proto_ids_off: self.u32(),
            field_ids_size: self.u32(),
            field_ids_off: self.u32(),
            method_ids_size: self.u32(),
            method_ids_off: self.u32(),
            class_defs_size: self.u32(),
            class_defs_off: self.u32(),
            data_size: self.u32(),
            data_off: self.u32(),
        }
    }
}
