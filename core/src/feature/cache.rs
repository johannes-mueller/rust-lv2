use crate::feature::*;
use std::collections::{hash_map, HashMap};
use std::ffi::{c_void, CStr};
use std::iter::Map;

/// Cache for host features, used in the feature discovery stage.
///
/// At initialization time, a raw LV2 plugin receives a null-terminated array containing all requested host features. Obviously, this is not suited for safe Rust code and therefore, it needs an abstraction layer.
///
/// Internally, this struct contains a hash map which is filled the raw LV2 feature descriptors. Using this map, methods are defined to identify and retrieve features.
#[derive(Clone)]
pub struct FeatureCache<'a> {
    internal: HashMap<&'a CStr, *const c_void>,
}

impl<'a> FeatureCache<'a> {
    /// Construct a cache from the raw features array.
    ///
    /// It basically populates a hash map by walking through the array and then creates a `FeatureContainer` with it. However, this method is unsafe since it dereferences a C string to a URI. Also, this method should only be used with the features list supplied by the host since the soundness of the whole module depends on that assumption.
    ///
    /// # Safety
    ///
    /// This method is unsafe since it needs to dereference the raw feature pointers.
    pub unsafe fn from_raw(raw: *const *const ::sys::LV2_Feature) -> Self {
        let mut internal_map = HashMap::new();
        let mut feature_ptr = raw;

        if !raw.is_null() {
            while !(*feature_ptr).is_null() {
                let uri = CStr::from_ptr((**feature_ptr).URI);
                let data = (**feature_ptr).data as *const c_void;
                internal_map.insert(uri, data);
                feature_ptr = feature_ptr.add(1);
            }
        }

        Self {
            internal: internal_map,
        }
    }

    /// Evaluate whether this object contains the requested feature.
    pub fn contains<T: Feature>(&self) -> bool {
        self.internal.contains_key(T::uri())
    }

    /// Try to retrieve a feature.
    ///
    /// If the feature is not found, this method will return `None`. Since the resulting feature object may have mutable access to the raw data, it will be removed from the cache to avoid aliasing.
    ///
    /// You also have to provide the threading class of the feature you want to retrieve.
    pub fn retrieve_feature<F: Feature, T: FromResolvedFeature<F>>(
        &mut self,
        class: ThreadingClass,
    ) -> Result<T, MissingFeatureError> {
        T::from_resolved_feature(
            self.internal
                .remove(F::uri())
                .and_then(|ptr| unsafe { F::from_feature_ptr(ptr, class) }),
        )
    }
}

type HashMapIterator<'a> = hash_map::IntoIter<&'a CStr, *const c_void>;
type DescriptorBuildFn<'a> = fn((&'a CStr, *const c_void)) -> FeatureDescriptor<'a>;

impl<'a> std::iter::IntoIterator for FeatureCache<'a> {
    type Item = FeatureDescriptor<'a>;
    type IntoIter = Map<HashMapIterator<'a>, DescriptorBuildFn<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.internal.into_iter().map(|element| {
            let uri = element.0;
            let data = element.1;
            FeatureDescriptor { uri, data }
        })
    }
}

impl<'a> FeatureCollection<'a> for FeatureCache<'a> {
    fn from_cache(
        cache: &mut FeatureCache<'a>,
        _: ThreadingClass,
    ) -> Result<Self, MissingFeatureError> {
        Ok(FeatureCache {
            internal: cache.internal.clone(),
        })
    }
}

/// A trait to allow arbitrary types to be potentially created from feature resolution.
///
/// Any type present in a `FeatureCollection` must implement this trait.
///
/// For more information, see `FeatureCollection`.
///
/// For now this only covers `&T` and `Option<&T>` (where T is a `Feature`), but this may be
/// extended in the future.
pub trait FromResolvedFeature<F: Feature>: Sized {
    fn from_resolved_feature(feature: Option<F>) -> Result<Self, MissingFeatureError>;
}

impl<F: Feature> FromResolvedFeature<F> for F {
    fn from_resolved_feature(feature: Option<F>) -> Result<Self, MissingFeatureError> {
        feature.ok_or_else(|| MissingFeatureError { uri: F::uri() })
    }
}

impl<F: Feature> FromResolvedFeature<F> for Option<F> {
    #[inline]
    fn from_resolved_feature(feature: Option<F>) -> Result<Self, MissingFeatureError> {
        Ok(feature)
    }
}
