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
use std::io::Cursor;
use std::io::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::SeekFrom::Current;
use std::io::Write;
use std::path::PathBuf;

use byteorder::{LittleEndian, ReadBytesExt};
use header::DexFileHeader;
use header::HeaderDexer;
use map_list::MapData;
use map_list::MapItem;
use map_list::MapList;

pub fn create_file(path: &PathBuf, filename: &str) -> Result<File, Error> {
    let x = PathBuf::new().join(&path).join(filename);
    File::create(x)
}

pub fn default_dexd_dir() -> PathBuf {
    PathBuf::new().join("target").join("dexd")
}

pub fn get_bytes(mut dexfile: &File, len: usize) -> Vec<u8> {
    let mut buffer = vec![0; len];
    dexfile.read(&mut buffer).expect("Unable to read");
    buffer
}

pub fn get_bytes_range(mut dexfile: &File, start: u32, len: usize) -> Vec<u8> {
    dexfile.seek(SeekFrom::Start(start.into()));
    let mut buffer = vec![0; len];
    dexfile.read(&mut buffer).expect("Unable to read");
    buffer
}

pub fn read_dex_file(dexfile: &mut File) -> (DexFileHeader, Vec<MapData>) {
    let header = dexfile.dex_header();
    let map_list = dexfile.map_list(header.map_off);

    let mut data = Vec::new();
    for i in 0..map_list.list.len() {
        let m1: &MapItem = map_list.list.get(i).unwrap();
        let m2: Option<&MapItem> = map_list.list.get(i + 1);
        match m2 {
            Some(item) => {
                let len = item.offset - m1.offset;
                data.push(MapData::read(dexfile, &m1, len));
            }
            None => {
                let len = header.file_size - m1.offset;
                data.push(MapData::read(dexfile, &m1, len));
            }
        }
    }
    (header, data)
}

pub fn read_uleb128(file: &mut File) -> (u64, u16) {
    let mut byte_count = 0;
    let mut shift: usize = 0;
    let mut result: u64 = 0;
    let mut byte: u8;
    loop {
        byte_count = byte_count + 1;
        byte = file.read_u8().unwrap();
        result |= ((byte & 0x7F) as u64) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            break;
        }
    }
    (result, byte_count)
}

pub fn log(line: String, mut file: &File) {
    file.write_all(line.as_bytes());
}

pub trait DexReader {
    fn u32(&mut self) -> u32;
}

impl<'a> DexReader for Cursor<&'a Vec<u8>> {
    fn u32(&mut self) -> u32 {
        self.read_u32::<LittleEndian>().unwrap()
    }
}
pub trait DataDexer {
    fn map_list(&mut self, offset: u32) -> MapList;
}

impl DataDexer for File {
    fn map_list(&mut self, offset: u32) -> MapList {
        MapList::read(self, offset)
    }
}

pub trait DexFileReader {
    fn position(&mut self) -> u64;

    fn u16(&mut self) -> u16;

    fn u32(&mut self) -> u32;
}

impl DexFileReader for File {
    fn position(&mut self) -> u64 {
        self.seek(Current(0)).unwrap()
    }

    fn u16(&mut self) -> u16 {
        self.read_u16::<LittleEndian>().unwrap()
    }

    fn u32(&mut self) -> u32 {
        self.read_u32::<LittleEndian>().unwrap()
    }
}
