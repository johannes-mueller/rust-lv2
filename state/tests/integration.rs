use lv2_atom::prelude::*;
use lv2_core::feature::{FeatureCollection, FeatureContainer, MissingFeatureError};
use lv2_core::prelude::*;
use lv2_state::prelude::*;
use lv2_urid::mapper::*;
use lv2_urid::prelude::*;
use std::path::Path;
use std::pin::Pin;

struct Stateful {
    internal: f32,
    audio: Vec<f32>,

    urids: AtomURIDCache,
}

#[derive(FeatureCollection)]
pub struct Features<'a> {
    map: Map<'a>,
}

unsafe impl UriBound for Stateful {
    const URI: &'static [u8] = b"urn:lv2_atom:stateful\0";
}

impl Plugin for Stateful {
    type Ports = ();
    type Features = Features<'static>;

    fn new(_plugin_info: &PluginInfo, features: Features<'static>) -> Option<Self> {
        Some(Stateful {
            internal: 42.0,
            audio: Vec::new(),
            urids: features.map.populate_cache()?,
        })
    }

    fn run(&mut self, _: &mut ()) {
        self.internal = 17.0;
        self.audio.extend((0..32).map(|f| f as f32));
    }

    fn extension_data(uri: &Uri) -> Option<&'static dyn std::any::Any> {
        match_extensions!(uri, StateDescriptor<Self>)
    }
}

impl State for Stateful {
    type StateFeatures = ();

    fn save(&self, mut store: StoreHandle, _: ()) -> Result<(), StateErr> {
        store
            .draft(URID::new(1000).unwrap())
            .init(self.urids.float, self.internal)
            .unwrap();
        store
            .draft(URID::new(1001).unwrap())
            .init(self.urids.vector(), self.urids.float)
            .unwrap()
            .append(self.audio.as_ref());

        store.commit_all()
    }

    fn restore(&mut self, store: RetrieveHandle, _: ()) {
        self.internal = store
            .retrieve(URID::new(1000).unwrap())
            .unwrap()
            .read(self.urids.float, ())
            .unwrap();
        self.audio = Vec::from(
            store
                .retrieve(URID::new(1001).unwrap())
                .unwrap()
                .read(self.urids.vector(), self.urids.float)
                .unwrap(),
        );
    }
}

lv2_descriptors! {
    Stateful
}

fn create_plugin(mapper: Pin<&mut HashURIDMapper>) -> Stateful {
    let plugin = {
        // Faking the map's lifetime.
        let interface = mapper.make_map_interface();
        let interface = &interface as *const lv2_sys::LV2_URID_Map;
        let interface = unsafe { interface.as_ref().unwrap() };
        let map = Map::new(interface);

        // Constructing the plugin.
        Stateful::new(
            &PluginInfo::new(Stateful::uri(), Path::new("./"), 44100.0),
            Features { map: map },
        )
        .unwrap()
    };

    assert_eq!(42.0, plugin.internal);
    assert_eq!(0, plugin.audio.len());

    plugin
}

#[test]
fn test_save_n_restore() {
    let mut mapper = Box::pin(HashURIDMapper::new());
    let mut storage = lv2_state::Storage::default();

    let (store_fn, restore_fn) = unsafe {
        let extension_data_fn = lv2_descriptor(0).as_ref().unwrap().extension_data;
        let uri = lv2_sys::LV2_STATE__interface.as_ptr() as *const i8;
        let extension = ((extension_data_fn.unwrap())(uri) as *const lv2_sys::LV2_State_Interface)
            .as_ref()
            .unwrap();
        (extension.save.unwrap(), extension.restore.unwrap())
    };
    assert!(store_fn == StateDescriptor::<Stateful>::extern_save);
    assert!(restore_fn == StateDescriptor::<Stateful>::extern_restore);

    let mut first_plugin = create_plugin(mapper.as_mut());

    first_plugin.run(&mut ());

    assert_eq!(17.0, first_plugin.internal);
    assert_eq!(32, first_plugin.audio.len());

    unsafe {
        (store_fn)(
            &mut first_plugin as *mut Stateful as lv2_sys::LV2_Handle,
            Some(lv2_state::Storage::extern_store),
            &mut storage as *mut lv2_state::Storage as lv2_sys::LV2_State_Handle,
            lv2_sys::LV2_State_Flags_LV2_STATE_IS_POD,
            std::ptr::null_mut(),
        )
    };

    let mut second_plugin = create_plugin(mapper.as_mut());

    unsafe {
        (restore_fn)(
            &mut second_plugin as *mut Stateful as lv2_sys::LV2_Handle,
            Some(lv2_state::Storage::extern_retrieve),
            &mut storage as *mut lv2_state::Storage as lv2_sys::LV2_State_Handle,
            lv2_sys::LV2_State_Flags_LV2_STATE_IS_POD,
            std::ptr::null_mut(),
        )
    };

    assert_eq!(17.0, second_plugin.internal);
    assert_eq!(32, second_plugin.audio.len());
}
