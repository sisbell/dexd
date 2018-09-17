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
use prelude::DexReader;
use std::io::Cursor;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AnnotationSetRefList {
    size: u32,

    list: Vec<AnnotationSetRefItem>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AnnotationSetRefItem {
    annotations_off: u32,
}

pub fn diff_annotation_set_ref_list<'a, 'b>(
    v1: &'a Vec<AnnotationSetRefList>,
    v2: &'b Vec<AnnotationSetRefList>,
) -> Vec<(&'a AnnotationSetRefList, &'b AnnotationSetRefList)> {
    v1.iter().zip(v2.iter()).filter(|&(a, b)| a != b).collect()
}

pub fn read_annotation_set_ref_list(data: &MapData) -> Vec<AnnotationSetRefList> {
    let mut cursor = Cursor::new(&data.data);
    let mut list = Vec::new();
    for _i in 0..data.size {
        list.push(AnnotationSetRefList::read(&mut cursor));
    }
    list
}

impl AnnotationSetRefList {
    pub fn read(data: &mut Cursor<&Vec<u8>>) -> AnnotationSetRefList {
        let size = data.u32();
        AnnotationSetRefList {
            size: size,
            list: AnnotationSetRefItem::read_list(data, size),
        }
    }
}

impl AnnotationSetRefItem {
    pub fn read_list(data: &mut Cursor<&Vec<u8>>, size: u32) -> Vec<AnnotationSetRefItem> {
        let mut items = Vec::new();
        for _i in 0..size {
            items.push(AnnotationSetRefItem::read(data));
        }
        items
    }

    pub fn read(data: &mut Cursor<&Vec<u8>>) -> AnnotationSetRefItem {
        AnnotationSetRefItem {
            annotations_off: data.u32(),
        }
    }
}
