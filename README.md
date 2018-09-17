# Dex Diff
A program to diff Android dex class files. The purpose of this project is to assist in creating reproducible Android builds. Two builds with the same source code may have subtle differences in the resulting dex file, causing the hashes to be different. This program will help identify those differences

### Command Line
To run, type the following command
```
dexd classes.dex classes2.dex
```
replacing classes.dex and classes2.dex with your dex file names.

### Summary Output
The dex classfile is divided into 18 encoded blocks. dexd will check whether each block between two different dex files are the same. This can help narrow down where your problem is.

The following output is a real example where the annotation sections are not matching for a build that should be the same.

<pre>
TYPE_HEADER_ITEM (DIFF)
TYPE_STRING_ID_ITEM (OK)

TYPE_TYPE_ID_ITEM (OK)

TYPE_PROTO_ID_ITEM (OK)

TYPE_FIELD_ID_ITEM (OK)

TYPE_METHOD_ID_ITEM (OK)

TYPE_CLASS_DEF_ITEM (DIFF)
    Matches = 2757 Mismatch = 3391

TYPE_MAP_LIST (OK)

TYPE_TYPE_LIST (OK)

TYPE_ANNOTATION_SET_REF_LIST (DIFF)
    Matches = 266 Mismatch = 613

TYPE_ANNOTATION_SET_ITEM (DIFF)
    Matches = 3491 Mismatch = 6108

TYPE_CLASS_DATA_ITEM (OK)

TYPE_CODE_ITEM (OK)

TYPE_STRING_DATA_ITEM (OK)

TYPE_DEBUG_INFO_ITEM (OK)

TYPE_ANNOTATION_ITEM (OK)

TYPE_ENCODED_ARRAY_ITEM (DIFF)
TYPE_ANNOTATIONS_DIRECTORY_ITEM (DIFF)
    Matches = 2057 Mismatch = 3182
</pre>
**Matches/Mismatches gives the count of items that are matching, not the number of bytes.**
### Detailed Logs
For the following blocks, dexd writes out the exact differences it finds between elements.

* CLASS_DEF_ITEM
* ANNOTATION_SET_REF_LIST
* ANNOTATION_SET_ITEM
* ANNOTATIONS_DIRECTORY_ITEM

Each of these blocks has an associated txt file under the _./target/dexd_ directory.

Output will contain pairs of items _({item1}, {item2})_ where item1 (from first dex file) was expected to be the same as item2 (from second dex file).

In the case below, we see that the AnnotationSetItem in the first pair has a different number of items. The second pair has the same number of items but point to different offsets.

<pre>
  (
        AnnotationSetItem {
            size: 1,
            entries: [
                AnnotationOffItem {
                    annotation_off: 1333860
                }
            ]
        },
        AnnotationSetItem {
            size: 3,
            entries: [
                AnnotationOffItem {
                    annotation_off: 1333400
                },
                AnnotationOffItem {
                    annotation_off: 1333408
                },
                AnnotationOffItem {
                    annotation_off: 1333408
                }
            ]
        }
    ),
    (
        AnnotationSetItem {
            size: 1,
            entries: [
                AnnotationOffItem {
                    annotation_off: 1333860
                }
            ]
        },
        AnnotationSetItem {
            size: 1,
            entries: [
                AnnotationOffItem {
                    annotation_off: 1335352
                }
            ]
        }
    ),
</pre>

### Future Work
In future releases, I plan to add additional blocks, eventually covering all 18 blocks. There will also be additional features to determine if the dex class differences are unimportant (like ordering) or important like changes in code or resources.

### Additional info
* [Dex Format](https://source.android.com/devices/tech/dalvik/dex-format)
* [Reproducible Builds](https://reproducible-builds.org/)