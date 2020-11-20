/// Host feature to communicate options to the plugin
///
/// https://lv2plug.in/ns/ext/options
///
/// Add an `LV2Options` field to your plugin's `Feature` struct. Then use the `::retrieve_option()` method to get the information about the option.

use std::collections::HashMap;
use std::slice;

use core::feature::Feature;
use core::prelude::*;
use std::ffi::c_void;
use urid::*;

pub struct LV2Options {
    slice_map: HashMap<u32, (usize, usize)>,
    data: Vec<u8>,
}

unsafe impl UriBound for LV2Options {
    const URI: &'static [u8] = sys::LV2_OPTIONS__options;
}

unsafe impl Feature for LV2Options {
    unsafe fn from_feature_ptr(feature: *const c_void, class: ThreadingClass) -> Option<Self> {
        if class != ThreadingClass::Audio {
            Self::new(feature as *const sys::LV2_Options_Option)
        } else {
            panic!("The URID mapping feature isn't allowed in the audio threading class");
        }
    }
}

impl LV2Options {
    unsafe fn new(options: *const sys::LV2_Options_Option) -> Option<Self> {
        let mut ptr = options;
        let mut data = Vec::new();
        let mut slice_map = HashMap::new();
        while (*ptr).key != 0 {
            let start = data.len();
            data.extend_from_slice(slice::from_raw_parts(
                &(*ptr).size as *const u32 as *const u8,
                std::mem::size_of::<u32>(),
            ));
            data.extend_from_slice(slice::from_raw_parts(
                &(*ptr).type_ as *const u32 as *const u8,
                std::mem::size_of::<u32>(),
            ));
            data.extend_from_slice(slice::from_raw_parts(
                (*ptr).value as *const u8,
                (*ptr).size as usize,
            ));
            slice_map.insert((*ptr).key, (start, data.len()));

            ptr = ptr.offset(1);
        }

        Some(Self { slice_map, data })
    }

    /// Tries to retrieve the option specified by `urid`.
    ///
    /// Returns an `lv2_atom::UnidentifiedAtom` from which the option can be read using the `::read()` method.
    pub fn retrieve_option<'a, T>(&'a self, urid: URID<T>) -> Option<atom::UnidentifiedAtom<'a>> {
        self.slice_map.get(&urid.get()).and_then(|(start, end)| {
            let space = atom::space::Space::from_slice(&self.data[*start..*end]);
            Some(atom::UnidentifiedAtom::new(space))
        })
    }
}

#[cfg(test)]
mod tests {
    use lv2_atom::prelude::*;
    use lv2_urid::*;
    use std::pin::Pin;
    use urid::*;
    use super::*;

    struct TestOption1;
    struct TestOption2;
    struct TestOption3;

    unsafe impl UriBound for TestOption1 {
        const URI: &'static [u8] = b"urn:lv2_atom:option#TestOption1\0";
    }
    unsafe impl UriBound for TestOption2 {
        const URI: &'static [u8] = b"urn:lv2_atom:option#TestOption2\0";
    }
    unsafe impl UriBound for TestOption3 {
        const URI: &'static [u8] = b"urn:lv2_atom:option#TestOption3\0";
    }

    #[derive(URIDCollection)]
    struct URIDs {
        test_option_1: URID<TestOption1>,
        test_option_2: URID<TestOption2>,
        test_option_3: URID<TestOption3>,
        atom: AtomURIDCollection,
    }

    #[test]
    fn test_option_feature() {

        let mut mapper: Pin<Box<HostMap<HashURIDMapper>>> = Box::pin(HashURIDMapper::new().into());
        let map_interface = Box::pin(mapper.as_mut().make_map_interface());
        let map = LV2Map::new(map_interface.as_ref().get_ref());
        let urids: URIDs = map.populate_collection().unwrap();

        let option_value_1 = 42u32;
        let option_value_2 = 23.42f32;

        let option_data = Box::new([
            lv2_sys::LV2_Options_Option {
                context: 0,
                subject: 0,
                key: urids.test_option_1.get(),
                type_: urids.atom.int.get(),
                size: 4,
                value: &option_value_1 as *const u32 as *const std::ffi::c_void
            },
            lv2_sys::LV2_Options_Option {
                context: 0,
                subject: 0,
                key: urids.test_option_2.get(),
                type_: urids.atom.float.get(),
                size: 4,
                value: &option_value_2 as *const f32 as *const std::ffi::c_void
            },
            lv2_sys::LV2_Options_Option {
                context: 0,
                subject: 0,
                key: 0,
                type_: 0,
                size: 0,
                value: std::ptr::null()
            }
        ]);

        let option_ptr = option_data.as_ptr() as *const lv2_sys::LV2_Options_Option;

        let options = unsafe { LV2Options::new(option_ptr).unwrap() };
        let option_value_1 = options
            .retrieve_option(urids.test_option_1)
            .and_then(|atom| atom.read(urids.atom.int, ()));
        let option_value_2 = options
            .retrieve_option(urids.test_option_2)
            .and_then(|atom| atom.read(urids.atom.float, ()));
        let option_value_3 = options
            .retrieve_option(urids.test_option_3)
            .and_then(|atom| atom.read(urids.atom.int, ()));

        assert_eq!(option_value_1, Some(42));
        assert_eq!(option_value_2, Some(23.42));
        assert_eq!(option_value_3, None);
    }
}
