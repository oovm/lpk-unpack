pub mod cubism_v1;
pub mod cubism_v3;

pub use crate::{cubism_v1::ModelConfigV1, cubism_v3::ModelConfigV3};
use serde::{Deserialize, Serialize};

pub enum Live2DModel {
    V1(ModelConfigV1),
    V3(ModelConfigV3),
}

impl _serde::Serialize for Live2DModel {
    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: _serde::Serializer,
    {
        match *self {
            Live2DModel::V1(ref __field0) => _serde::Serializer::serialize_newtype_variant(__serializer, "Live2DModel", 0u32, "V1", __field0),
            Live2DModel::V3(ref __field0) => _serde::Serializer::serialize_newtype_variant(__serializer, "Live2DModel", 1u32, "V3", __field0),
        }
    }
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::absolute_paths, )]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Live2DModel {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field { __field0, __field1 }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result { _serde::__private::Formatter::write_str(__formatter, "variant identifier") }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value), &"variant index 0 <= i < 2")),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "V1" => _serde::__private::Ok(__Field::__field0),
                        "V3" => _serde::__private::Ok(__Field::__field1),
                        _ => { _serde::__private::Err(_serde::de::Error::unknown_variant(__value, VARIANTS)) }
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"V1" => _serde::__private::Ok(__Field::__field0),
                        b"V3" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_variant(__value, VARIANTS))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                { _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor) }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Live2DModel>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Live2DModel;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result { _serde::__private::Formatter::write_str(__formatter, "enum Live2DModel") }
                fn visit_enum<__A>(self, __data: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => _serde::__private::Result::map(_serde::de::VariantAccess::newtype_variant::<ModelConfigV1>(__variant), Live2DModel::V1),
                        (__Field::__field1, __variant) => _serde::__private::Result::map(_serde::de::VariantAccess::newtype_variant::<ModelConfigV3>(__variant), Live2DModel::V3),
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["V1", "V3"];
            _serde::Deserializer::deserialize_enum(__deserializer, "Live2DModel", VARIANTS, __Visitor { marker: _serde::__private::PhantomData::<Live2DModel>, lifetime: _serde::__private::PhantomData })
        }
    }
};