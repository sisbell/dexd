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
use std::fs::File;
use std::io::{Seek, SeekFrom};

use prelude::*;

#[derive(Debug)]
pub struct StringDataItem {
    pub offset: u32,
    pub utf16_size: (u64, u16),
    pub data: Vec<u8>,
}

impl StringDataItem {
    pub fn read(file: &mut File, offset: u32) -> StringDataItem {
        file.seek(SeekFrom::Start(offset.into()));
        let size = read_uleb128(file);
        StringDataItem {
            offset: offset,
            utf16_size: size,
            data: get_bytes(file, size.0 as usize),
        }
    }
}
