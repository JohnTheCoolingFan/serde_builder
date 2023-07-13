pub mod error;
mod field_visitor;
mod final_builder;
mod validator;

use error::Error;
use field_visitor::*;
pub use final_builder::*;
pub use validator::*;

use std::marker::PhantomData;

use serde::{Deserialize, Deserializer};

use concat_arrays::concat_arrays;

pub struct StructDeserializer<
    T,
    FBARGS = (),
    FB: FinalBuilder<T, FBARGS> = (),
    V: Validator<T> = (),
    const FN: usize = 0,
> {
    target_phantom: PhantomData<T>,
    fb_args_phantom: PhantomData<FBARGS>,
    final_builder: Option<FB>,
    validator: Option<V>,
    field_names: [String; FN],
}

impl<T> Default for StructDeserializer<T> {
    fn default() -> Self {
        Self {
            target_phantom: PhantomData::default(),
            fb_args_phantom: PhantomData::default(),
            final_builder: None,
            validator: None,
            field_names: [],
        }
    }
}

impl<T> StructDeserializer<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, FBARGS, V: Validator<T>, const FN: usize> StructDeserializer<T, FBARGS, (), V, FN> {
    pub fn final_builder<FB: FinalBuilder<T, FBARGS>>(
        self,
        final_builder: FB,
    ) -> StructDeserializer<T, FBARGS, FB, V, FN> {
        let StructDeserializer {
            target_phantom,
            fb_args_phantom: _,
            final_builder: _,
            validator,
            field_names,
        } = self;
        StructDeserializer {
            target_phantom,
            fb_args_phantom: PhantomData::default(),
            final_builder: Some(final_builder),
            validator,
            field_names,
        }
    }
}

impl<T, FBARGS, FB: FinalBuilder<T, FBARGS>, const FN: usize>
    StructDeserializer<T, FBARGS, FB, (), FN>
{
    pub fn validator<V: Validator<T>>(
        self,
        validator: V,
    ) -> StructDeserializer<T, FBARGS, FB, V, FN> {
        let StructDeserializer {
            target_phantom,
            fb_args_phantom,
            final_builder,
            validator: _,
            field_names,
        } = self;
        StructDeserializer {
            target_phantom,
            fb_args_phantom,
            final_builder,
            validator: Some(validator),
            field_names,
        }
    }
}

impl<T, FB: FinalBuilder<T, ()>, V: Validator<T>> StructDeserializer<T, (), FB, V, 0> {
    pub fn field<T0, N: ToString>(self, name: N) -> StructDeserializer<T, (T0,), (), V, 1> {
        let StructDeserializer {
            target_phantom,
            fb_args_phantom: _,
            final_builder: _,
            validator,
            field_names: _,
        } = self;
        StructDeserializer {
            target_phantom,
            fb_args_phantom: PhantomData::default(),
            final_builder: None,
            validator,
            field_names: [name.to_string()],
        }
    }
}

macro_rules! add_field_impl {
    ($($len:expr => ($($n:tt $name1:ident),+), $name2:ident)+) => {
        $(
            impl<T, $($name1,)+ FB: FinalBuilder<T, ($($name1,)+)>, V: Validator<T>> StructDeserializer<T, ($($name1,)+), FB, V, $len> {
                pub fn field<$name2, N: ToString>(self, name: N) -> StructDeserializer<T, ($($name1,)+ $name2), (), V, {$len+1}> {
                    let StructDeserializer {
                        target_phantom,
                        fb_args_phantom: _,
                        final_builder: _,
                        validator,
                        field_names,
                    } = self;
                    #[allow(clippy::drop_non_drop)]
                    StructDeserializer {
                        target_phantom,
                        fb_args_phantom: PhantomData::default(),
                        final_builder: None,
                        validator,
                        field_names: concat_arrays!(field_names, [name.to_string()]),
                    }
                }
            }
        )+
    }
}

add_field_impl! {
    1 => (0 T0), T1
    2 => (0 T0, 1 T1), T2
    3 => (0 T0, 1 T1, 2 T2), T3
    4 => (0 T0, 1 T1, 2 T2, 3 T3), T4
    5 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4), T5
    6 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5), T6
    7 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6), T7
    8 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7), T8
    9 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8), T9
    10 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9), T10
    11 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10), T11
    12 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11), T12
    13 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12), T13
    14 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13), T14
    15 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13, 14 T14), T15
}

impl<T, T0, FB, V> StructDeserializer<T, T0, FB, V, 1>
where
    T0: for<'a> Deserialize<'a>,
    FB: FinalBuilder<T, T0>,
    V: Validator<T>,
{
    pub fn deserialize<'de, D: Deserializer<'de>>(self, des: D) -> Result<T, Error<'de, D>> {
        let StructDeserializer {
            target_phantom: _,
            fb_args_phantom: _,
            final_builder,
            validator,
            field_names,
        } = self;
        #[cfg_attr(not(feature = "leaking"), allow(clippy::redundant_clone))]
        let field_visitor =
            FieldVisitor::<T, T0, FB, 1>::new(final_builder.unwrap(), field_names.clone());
        // I don't like this AT ALL
        #[cfg(feature = "leaking")]
        let field_names_static: &'static [&'static str] = &*field_names
            .into_iter()
            .map(|s| &*Box::leak(s.into_boxed_str()))
            .collect::<Vec<_>>()
            .leak();
        #[cfg(not(feature = "leaking"))]
        let field_names_static = &["field 0"];
        let value = des
            .deserialize_struct(
                std::any::type_name::<T>(),
                field_names_static,
                field_visitor,
            )
            .map_err(|deerr| Error::Deserialization(deerr))?;
        if let Some(validator) = validator {
            validator
                .validate(&value)
                .map_err(|vaerr| Error::Validation(vaerr))?;
        }
        Ok(value)
    }
}

macro_rules! deserialize_impl {
    ($($len:expr => $($name:ident),+)+) => {
        $(
            impl<T, $($name,)+ FB, V> StructDeserializer<T, ($($name),+), FB, V, $len>
            where
                $($name: for<'a> Deserialize<'a>,)+
                FB: FinalBuilder<T, ($($name),+)>,
                V: Validator<T>,
            {
                pub fn deserialize<'de, D: Deserializer<'de>>(self, des: D) -> Result<T, Error<'de, D>> {
                    let StructDeserializer {
                        target_phantom: _,
                        fb_args_phantom: _,
                        final_builder,
                        validator,
                        field_names,
                    } = self;
                    #[cfg_attr(not(feature = "leaking"), allow(clippy::redundant_clone))]
                    let field_visitor = FieldVisitor::<T, ($($name,)+), FB, $len>::new(
                        final_builder.unwrap(),
                        field_names.clone(),
                    );
                    // I don't like this AT ALL
                    #[cfg(feature = "leaking")]
                    let field_names_static: &'static [&'static str] = &*field_names
                        .into_iter()
                        .map(|s| &*Box::leak(s.into_boxed_str()))
                        .collect::<Vec<_>>()
                        .leak();
                    #[cfg(not(feature = "leaking"))]
                    let field_names_static = &[$(stringify!(field $name)),+];
                    let value = des.deserialize_struct(std::any::type_name::<T>(), field_names_static, field_visitor).map_err(|e| Error::Deserialization(e))?;
                    if let Some(validator) = validator {
                        validator.validate(&value).map_err(|e| Error::Validation(e))?;
                    }
                    Ok(value)
                }
            }
        )+
    }
}

deserialize_impl! {
    2 => T0, T1
    3 => T0, T1, T2
    4 => T0, T1, T2, T3
    5 => T0, T1, T2, T3, T4
    6 => T0, T1, T2, T3, T4, T5
    7 => T0, T1, T2, T3, T4, T5, T6
    8 => T0, T1, T2, T3, T4, T5, T6, T7
    9 => T0, T1, T2, T3, T4, T5, T6, T7, T8
    10 => T0, T1, T2, T3, T4, T5, T6, T7, T8, T9
    11 => T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10
    12 => T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11
    13 => T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12
    14 => T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13
    15 => T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
    16 => T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
}
