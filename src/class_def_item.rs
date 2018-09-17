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
use map_list::MapData;
use prelude::*;
use std::io::Cursor;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ClassDefItem {
    pub offset: u64,

    pub class_idx: u32,

    pub access_flags: u32,

    pub superclass_idx: u32,

    pub interfaces_off: u32,

    pub source_file_idx: u32,

    pub annotations_off: u32,

    pub class_data_off: u32,

    pub static_values_off: u32,
}

pub fn read_class_def_item(data: &MapData) -> Vec<ClassDefItem> {
    let mut cursor = Cursor::new(&data.data);
    read_class_defs(&mut cursor, data.size)
}

pub fn diff_class_def2<'a, 'b>(
    def1: &'a Vec<ClassDefItem>,
    def2: &'b Vec<ClassDefItem>,
) -> Vec<(&'a ClassDefItem, &'b ClassDefItem)> {
    def1.iter()
        .zip(def2.iter())
        .filter(|&(a, b)| a != b)
        .collect()
}

fn read_class_defs(data: &mut Cursor<&Vec<u8>>, size: u32) -> Vec<ClassDefItem> {
    let mut cds = Vec::new();
    for _i in 0..size {
        cds.push(ClassDefItem::read(data));
    }
    cds
}

impl ClassDefItem {}

impl ClassDefItem {
    pub fn read(data: &mut Cursor<&Vec<u8>>) -> ClassDefItem {
        ClassDefItem {
            offset: data.position(),
            class_idx: data.u32(),
            access_flags: data.u32(),
            superclass_idx: data.u32(),
            interfaces_off: data.u32(),
            source_file_idx: data.u32(),
            annotations_off: data.u32(),
            class_data_off: data.u32(),
            static_values_off: data.u32(),
        }
    }
}
