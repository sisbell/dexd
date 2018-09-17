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

#[derive(Debug)]
pub struct CallSiteIdItem {
    pub offset: u64,

    pub call_site_off: u32,
}

#[derive(Debug)]
pub struct FieldIdItem {
    pub offset: u64,

    pub class_idx: u16,

    pub type_idx: u16,

    pub name_idx: u32,
}

#[derive(Debug)]
pub struct MethodHandleItem {
    pub method_handle_type: u16,
    pub unused_1: u16,
    field_or_method_id: u16,
    pub unused_2: u16,
}

#[derive(Debug)]
pub struct MethodIdItem {
    pub offset: u64,

    pub class_idx: u16,

    pub proto_idx: u16,

    pub name_idx: u32,
}

#[derive(Debug)]
pub struct ProtoIdItem {
    pub offset: u64,

    pub shorty_idx: u32,

    pub return_type_idx: u32,

    pub parameters_off: u32,
}

#[derive(Debug)]
pub struct StringIdItem {
    pub offset: u64,
    pub string_id_off: u32,
}

#[derive(Debug)]
pub struct TypeIdItem {
    pub offset: u64,
    pub descriptor_idx: u32,
}

impl CallSiteIdItem {}

impl FieldIdItem {}

impl MethodIdItem {}

impl ProtoIdItem {}
