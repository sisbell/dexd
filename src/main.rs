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

extern crate byteorder;
extern crate clap;
extern crate dexd;

use std::fs;
use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

use clap::App;
use clap::Arg;

use dexd::annotation_set_item::*;
use dexd::annotation_set_ref_list::*;
use dexd::annotations_directory_item::*;
use dexd::class_def_item::*;
use dexd::map_list::*;
use dexd::prelude::*;

fn diff(mut df: File, mut df2: File, dexd_dir: PathBuf) {
    let annotations_dir_item_file =
        create_file(&dexd_dir, "annotations_directory_item.txt").unwrap();
    let annotation_set_ref_list_file =
        create_file(&dexd_dir, "annotation_set_ref_list.txt").unwrap();
    let class_def_item_file = create_file(&dexd_dir, "class_def_item.txt").unwrap();
    let annotation_set_item_file = create_file(&dexd_dir, "annotation_set_item.txt").unwrap();

    let d1 = read_dex_file(&mut df);
    let d2 = read_dex_file(&mut df2);

    let map_data_1 = d1.1;
    let map_data_2 = d2.1;
    for i in 0..map_data_1.len() {
        let x: &MapData = map_data_1.get(i).unwrap();
        let y: &MapData = map_data_2.get(i).unwrap();
        if x.data != y.data {
            println!("{} (DIFF)", type_code(x.item_type));
            match x.item_type {
                0x1000 => {}
                0x0006 => {
                    let v1 = read_class_def_item(x);
                    let v2 = read_class_def_item(y);
                    let diff = diff_class_def2(&v1, &v2);
                    print_matches(diff.len(), v1.len());
                    log(format!("{:#?}", diff), &class_def_item_file);
                }
                0x1002 => {
                    let v1 = read_annotation_set_item(x);
                    let v2 = read_annotation_set_item(y);
                    let diff = diff_annotation_set_item(&v1, &v2);
                    print_matches(diff.len(), v1.len());
                    log(format!("{:#?}", diff), &annotation_set_item_file);
                }
                0x1003 => {
                    let v1 = read_annotation_set_ref_list(x);
                    let v2 = read_annotation_set_ref_list(y);
                    let diff = diff_annotation_set_ref_list(&v1, &v2);
                    print_matches(diff.len(), v1.len());
                    log(format!("{:#?}", diff), &annotation_set_ref_list_file);
                }
                0x2006 => {
                    let v1 = read_annotations_directory_item(x);
                    let v2 = read_annotations_directory_item(y);
                    let diff = diff_annotations_directory_item(&v1, &v2);
                    print_matches(diff.len(), v1.len());

                    log(format!("{:#?}", diff), &annotations_dir_item_file);
                }
                _ => {}
            }
        } else {
            println!("{} (OK)\r\n ", type_code(x.item_type));
        }
    }
}

pub fn print_matches(diff_count: usize, total_count: usize) {
    println!(
        "    Matches = {} Mismatch = {}\r\n",
        (total_count - diff_count),
        diff_count
    );
}

fn main() -> Result<(), Error> {
    let matches = App::new("dexd")
        .version("0.1.0")
        .author("Shane Isbell <shane.isbell@gmail.com>")
        .about("Program to diff dex class files")
        .arg(
            Arg::with_name("classes1.dex")
                .help("First dex class file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("classes2.dex")
                .help("Second dex class file")
                .index(2)
                .required(true),
        )
        .get_matches();

    let dexd_dir = default_dexd_dir();
    fs::create_dir_all(&dexd_dir)?;

    let dex1 = matches.value_of("classes1.dex").unwrap();
    let dex2 = matches.value_of("classes2.dex").unwrap();

    let file = File::open(dex1).expect("file not found");
    let file2 = File::open(dex2).expect("file not found");

    diff(file, file2, dexd_dir);

    Ok(())
}
