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
use encoded_value::EncodedValue;
use std::io::Cursor;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EncodedArrayItem {
    value: Vec<EncodedArray>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EncodedArray {
    pub size: (u64, u16),

    pub values: Vec<EncodedValue>,
}

impl EncodedArrayItem {
    pub fn read(data: &mut Cursor<&Vec<u8>>) -> EncodedArrayItem {
        EncodedArrayItem { value: Vec::new() }
    }
}

impl EncodedArray {
    pub fn read(data: &mut Cursor<&Vec<u8>>) -> EncodedArray {
        EncodedArray {
            size: (0, 0),
            values: Vec::new(),
        }
    }
}
