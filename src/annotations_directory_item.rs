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
pub struct AnnotationsDirectoryItem {
    pub class_annotations_off: u32,
    pub field_size: u32,
    pub annotated_methods_size: u32,
    pub annotated_parameters_size: u32,
    pub field_annotations: Vec<FieldAnnotation>,
    pub method_annotations: Vec<MethodAnnotation>,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct FieldAnnotation {
    pub field_idx: u32,

    pub annotations_off: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct MethodAnnotation {
    pub method_idx: u32,

    pub annotations_off: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ParameterAnnotation {
    pub method_idx: u32,

    pub annotations_off: u32,
}

pub fn diff_annotations_directory_item<'a, 'b>(
    v1: &'a Vec<AnnotationsDirectoryItem>,
    v2: &'b Vec<AnnotationsDirectoryItem>,
) -> Vec<(&'a AnnotationsDirectoryItem, &'b AnnotationsDirectoryItem)> {
    v1.iter().zip(v2.iter()).filter(|&(a, b)| a != b).collect()
}

pub fn read_annotations_directory_item(data: &MapData) -> Vec<AnnotationsDirectoryItem> {
    let mut cursor = Cursor::new(&data.data);
    let mut list = Vec::new();
    for _i in 0..data.size {
        list.push(AnnotationsDirectoryItem::read(&mut cursor));
    }
    list
}

impl AnnotationsDirectoryItem {
    pub fn read(data: &mut Cursor<&Vec<u8>>) -> AnnotationsDirectoryItem {
        let class_annotations_off = data.u32();
        let field_size = data.u32();
        let annotated_methods_size = data.u32();
        let annotated_parameters_size = data.u32();

        AnnotationsDirectoryItem {
            class_annotations_off,
            field_size,
            annotated_methods_size,
            annotated_parameters_size,
            field_annotations: FieldAnnotation::read_list(data, field_size),
            method_annotations: MethodAnnotation::read_list(data, annotated_methods_size),
            parameter_annotations: ParameterAnnotation::read_list(data, annotated_parameters_size),
        }
    }
}

impl FieldAnnotation {
    pub fn read_list(data: &mut Cursor<&Vec<u8>>, size: u32) -> Vec<FieldAnnotation> {
        if size == 0 {
            return Vec::new();
        }
        let mut items = Vec::new();
        for _i in 0..size {
            items.push(FieldAnnotation::read(data));
        }
        items
    }

    pub fn read(data: &mut Cursor<&Vec<u8>>) -> FieldAnnotation {
        FieldAnnotation {
            field_idx: data.u32(),
            annotations_off: data.u32(),
        }
    }
}

impl MethodAnnotation {
    pub fn read_list(data: &mut Cursor<&Vec<u8>>, size: u32) -> Vec<MethodAnnotation> {
        if size == 0 {
            return Vec::new();
        }
        let mut items = Vec::new();
        for _i in 0..size {
            items.push(MethodAnnotation::read(data));
        }
        items
    }

    pub fn read(data: &mut Cursor<&Vec<u8>>) -> MethodAnnotation {
        MethodAnnotation {
            method_idx: data.u32(),
            annotations_off: data.u32(),
        }
    }
}

impl ParameterAnnotation {
    pub fn read_list(data: &mut Cursor<&Vec<u8>>, size: u32) -> Vec<ParameterAnnotation> {
        if size == 0 {
            return Vec::new();
        }
        let mut items = Vec::new();
        for _i in 0..size {
            items.push(ParameterAnnotation::read(data));
        }
        items
    }

    pub fn read(data: &mut Cursor<&Vec<u8>>) -> ParameterAnnotation {
        ParameterAnnotation {
            method_idx: data.u32(),
            annotations_off: data.u32(),
        }
    }
}
