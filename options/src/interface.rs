use crate::{OptionsErr, Subject};
use atom::prelude::*;
use atom::space::*;
use core::extension::ExtensionDescriptor;
use core::prelude::*;
use std::marker::PhantomData;
use std::slice;
use urid::*;

pub trait Options: Plugin {
    fn get<'a>(
        &self,
        urid: URID,
        subject: Subject,
        writer: &mut OptionWriter<'a>,
    ) -> Result<(), OptionsErr>;

    fn set(
        &self,
        urid: URID,
        subject: Subject,
        payload: UnidentifiedAtom,
    ) -> Result<(), OptionsErr>;
}

pub struct OptionsDescriptor<P: Options> {
    plugin: PhantomData<P>,
}

unsafe impl<P: Options> UriBound for OptionsDescriptor<P> {
    const URI: &'static [u8] = sys::LV2_OPTIONS__interface;
}

impl<P: Options> OptionsDescriptor<P> {
    pub unsafe extern "C" fn extern_get(
        instance: sys::LV2_Handle,
        raw_options: *mut sys::LV2_Options_Option,
    ) -> u32 {
        let plugin: &P = if let Some(plugin) = (instance as *const P).as_ref() {
            plugin
        } else {
            return sys::LV2_State_Status_LV2_STATE_ERR_UNKNOWN;
        };

        let mut ret: u32 = 0;

        let mut ptr = raw_options;
        while let Some(urid) = URID::new((*ptr).key) {
            let mut element = SpaceElement::default();
            let mut writer = OptionWriter {
                head: SpaceHead::new(&mut element),
            };
            let subject = match try_extract_subject(&*ptr) {
                Ok(subject) => subject,
                Err(e) => {
                    ret |= e.bits;
                    ptr = ptr.offset(1);
                    continue;
                }
            };

            plugin
                .get(urid, subject, &mut writer)
                .and_then(|_| {
                    let written_data = element.to_vec();
                    let space = Space::from_slice(written_data.as_ref());
                    let (header, data) = space
                        .split_type::<sys::LV2_Atom>()
                        .ok_or(OptionsErr::BAD_VALUE)?;

                    if header.type_ == (*ptr).type_ {
                        (*ptr).size = header.size;
                        (*ptr).value = data.data().ok_or(OptionsErr::BAD_VALUE)?.as_ptr()
                            as *const std::ffi::c_void;
                        Ok(())
                    } else {
                        Err(OptionsErr::BAD_VALUE)
                    }
                })
                .unwrap_or_else(|e| ret |= e.bits);
            ptr = ptr.offset(1);
        }
        ret
    }

    pub unsafe extern "C" fn extern_set(
        instance: sys::LV2_Handle,
        raw_options: *const sys::LV2_Options_Option,
    ) -> u32 {
        let plugin: &P = if let Some(plugin) = (instance as *const P).as_ref() {
            plugin
        } else {
            return sys::LV2_State_Status_LV2_STATE_ERR_UNKNOWN;
        };

        let mut ret: u32 = 0;

        let mut ptr = raw_options;
        while let Some(urid) = URID::new((*ptr).key) {
            let mut payload_data: Vec<u8> = Vec::new();
            payload_data.extend_from_slice(slice::from_raw_parts(
                &(*ptr).size as *const u32 as *const u8,
                std::mem::size_of::<u32>(),
            ));
            payload_data.extend_from_slice(slice::from_raw_parts(
                &(*ptr).type_ as *const u32 as *const u8,
                std::mem::size_of::<u32>(),
            ));
            payload_data.extend_from_slice(slice::from_raw_parts(
                (*ptr).value as *const u8,
                (*ptr).size as usize,
            ));

            let space = Space::from_slice(&payload_data);

            match try_extract_subject(&*ptr) {
                Ok(subject) => plugin
                    .set(urid, subject, UnidentifiedAtom::new(space))
                    .unwrap_or_else(|e| ret |= e.bits),
                Err(e) => ret |= e.bits,
            };

            ptr = ptr.offset(1);
        }
        ret
    }
}


impl<P: Options> ExtensionDescriptor for OptionsDescriptor<P> {
    type ExtensionInterface = sys::LV2_Options_Interface;

   const INTERFACE: &'static sys::LV2_Options_Interface = &sys::LV2_Options_Interface {
        get: Some(Self::extern_get),
        set: Some(Self::extern_set),
    };
}


fn try_extract_subject(opt: &sys::LV2_Options_Option) -> Result<Subject, OptionsErr> {
    match opt.context {
        sys::LV2_Options_Context_LV2_OPTIONS_INSTANCE => Ok(Subject::Instance),
        sys::LV2_Options_Context_LV2_OPTIONS_RESOURCE => urid::URID::new(opt.subject)
            .ok_or(OptionsErr::BAD_SUBJECT)
            .map(|urid| Subject::Resource(urid)),
        sys::LV2_Options_Context_LV2_OPTIONS_BLANK => Ok(Subject::Blank(opt.subject)),
        sys::LV2_Options_Context_LV2_OPTIONS_PORT => Ok(Subject::Port(opt.subject)),
        _ => Err(OptionsErr::BAD_SUBJECT),
    }
}

pub struct OptionWriter<'a> {
    head: SpaceHead<'a>,
}

impl<'a> OptionWriter<'a> {
    pub fn init<'b, A: Atom<'a, 'b>>(
        &'b mut self,
        urid: URID<A>,
        parameter: A::WriteParameter,
    ) -> Result<A::WriteHandle, OptionsErr> {
        (&mut self.head as &mut dyn MutSpace)
            .init(urid, parameter)
            .ok_or(OptionsErr::UNKNOWN)
    }
}
