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
pub struct MapData {
    pub item_type: u16,
    pub size: u32,
    pub offset: u32,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct MapItem {
    pub item_type: u16,
    pub unused: u16,
    pub size: u32,
    pub offset: u32,
}

#[derive(Debug)]
pub struct MapList {
    pub size: u32,
    pub list: Vec<MapItem>,
}

pub fn type_code<'a>(value: u16) -> &'a str {
    match value {
        0x0000 => "TYPE_HEADER_ITEM",
        0x0001 => "TYPE_STRING_ID_ITEM",
        0x0002 => "TYPE_TYPE_ID_ITEM",
        0x0003 => "TYPE_PROTO_ID_ITEM",
        0x0004 => "TYPE_FIELD_ID_ITEM",
        0x0005 => "TYPE_METHOD_ID_ITEM",
        0x0006 => "TYPE_CLASS_DEF_ITEM",
        0x0007 => "TYPE_CALL_SITE_ID_ITEM",
        0x0008 => "TYPE_METHOD_HANDLE_ITEM",
        0x1000 => "TYPE_MAP_LIST",
        0x1001 => "TYPE_TYPE_LIST",
        0x1002 => "TYPE_ANNOTATION_SET_REF_LIST",
        0x1003 => "TYPE_ANNOTATION_SET_ITEM",
        0x2000 => "TYPE_CLASS_DATA_ITEM",
        0x2001 => "TYPE_CODE_ITEM",
        0x2002 => "TYPE_STRING_DATA_ITEM",
        0x2003 => "TYPE_DEBUG_INFO_ITEM",
        0x2004 => "TYPE_ANNOTATION_ITEM",
        0x2005 => "TYPE_ENCODED_ARRAY_ITEM",
        0x2006 => "TYPE_ANNOTATIONS_DIRECTORY_ITEM",
        _ => "Unknown type",
    }
}

impl MapData {
    pub fn read(file: &mut File, map_item: &MapItem, len: u32) -> MapData {
        MapData {
            item_type: map_item.item_type,
            size: map_item.size,
            offset: map_item.offset,
            data: get_bytes_range(file, map_item.offset, len as usize),
        }
    }
}

impl MapItem {
    pub fn read(file: &mut File) -> MapItem {
        MapItem {
            item_type: file.u16(),
            unused: file.u16(),
            size: file.u32(),
            offset: file.u32(),
        }
    }
}

impl MapList {
    pub fn read(file: &mut File, offset: u32) -> MapList {
        file.seek(SeekFrom::Start(offset.into()));
        let mut map_items = Vec::new();

        let size = file.u32();
        for _x in 0..size {
            map_items.push(MapItem::read(file));
        }

        MapList {
            size: size,
            list: map_items,
        }
    }
}
