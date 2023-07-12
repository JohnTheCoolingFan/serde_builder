use std::{collections::HashMap, marker::PhantomData};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

pub trait FinalBuilder<T, ARGS> {
    fn assemble(self, args: ARGS) -> Option<T>;
}

impl<T, FBARGS> FinalBuilder<T, FBARGS> for () {
    fn assemble(self, _args: FBARGS) -> Option<T> {
        None
    }
}

macro_rules! final_builder_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<T, FN: FnOnce($($name),+) -> T, $($name),+> FinalBuilder<T, ($($name,)+)> for FN {
                fn assemble(self, args: ($($name),+)) -> Option<T> {
                    Some(self($(args.$n),+))
                }
            }
        )+
    }
}

final_builder_impls! {
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

pub trait Validator<T> {
    fn validate(self, value: &T) -> Result<(), String>;
}

impl<T> Validator<T> for () {
    fn validate(self, _value: &T) -> Result<(), String> {
        Ok(())
    }
}

impl<T, FN: FnOnce(&T) -> Result<(), String>> Validator<T> for FN {
    fn validate(self, value: &T) -> Result<(), String> {
        self(value)
    }
}

pub struct StructDeserializer<
    T,
    FBARGS = (),
    FB: FinalBuilder<T, FBARGS> = (),
    V: Validator<T> = (),
> {
    target_phantom: PhantomData<T>,
    fb_args_phantom: PhantomData<FBARGS>,
    final_builder: Option<FB>,
    validator: Option<V>,
    field_names: Vec<String>,
}

impl<T> Default for StructDeserializer<T> {
    fn default() -> Self {
        Self {
            target_phantom: PhantomData::default(),
            fb_args_phantom: PhantomData::default(),
            final_builder: None,
            validator: None,
            field_names: vec![],
        }
    }
}

impl<T, FBARGS, V: Validator<T>> StructDeserializer<T, FBARGS, (), V> {
    pub fn final_builder<FB: FinalBuilder<T, FBARGS>>(
        self,
        final_builder: FB,
    ) -> StructDeserializer<T, FBARGS, FB, V> {
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

impl<T, FBARGS, FB: FinalBuilder<T, FBARGS>> StructDeserializer<T, FBARGS, FB, ()> {
    pub fn validator<V: Validator<T>>(self, validator: V) -> StructDeserializer<T, FBARGS, FB, V> {
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

// Making an impl that woudl go from one element to two conflicted with otehr manual impl and macro
// impls. So I think there is no way to have a method for adding just one first field, sadly.
impl<T, FB: FinalBuilder<T, ()>, V: Validator<T>> StructDeserializer<T, (), FB, V> {
    pub fn first_fields<T0, T1, N0: ToString, N1: ToString>(
        self,
        name0: N0,
        name1: N1,
    ) -> StructDeserializer<T, (T0, T1), (), V> {
        let StructDeserializer {
            target_phantom,
            fb_args_phantom: _,
            final_builder: _,
            validator,
            mut field_names,
        } = self;
        field_names.push(name0.to_string());
        field_names.push(name1.to_string());
        StructDeserializer {
            target_phantom,
            fb_args_phantom: PhantomData::default(),
            final_builder: None,
            validator,
            field_names,
        }
    }
}

macro_rules! add_field_impl {
    ($($len:expr => ($($name1:ident),+), $name2:ident)+) => {
        $(
            impl<T, $($name1,)+ FB: FinalBuilder<T, ($($name1),+)>, V: Validator<T>> StructDeserializer<T, ($($name1),+), FB, V> {
                pub fn field<$name2, N: ToString>(self, name: N) -> StructDeserializer<T, ($($name1,)+ $name2), (), V> {
                    let StructDeserializer {
                        target_phantom,
                        fb_args_phantom: _,
                        final_builder: _,
                        validator,
                        mut field_names,
                    } = self;
                    field_names.push(name.to_string());
                    StructDeserializer {
                        target_phantom,
                        fb_args_phantom: PhantomData::default(),
                        final_builder: None,
                        validator,
                        field_names,
                    }
                }
            }
        )+
    }
}

add_field_impl! {
    2 => (T0, T1), T2
    3 => (T0, T1, T2), T3
    4 => (T0, T1, T2, T3), T4
    5 => (T0, T1, T2, T3, T4), T5
    6 => (T0, T1, T2, T3, T4, T5), T6
    7 => (T0, T1, T2, T3, T4, T5, T6), T7
    8 => (T0, T1, T2, T3, T4, T5, T6, T7), T8
    9 => (T0, T1, T2, T3, T4, T5, T6, T7, T8), T9
    10 => (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T10
    11 => (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T11
    12 => (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T12
    13 => (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), T13
    14 => (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), T14
    15 => (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), T15
}

struct FieldVisitor<T, T0, T1, T2, FB> {
    field_names: Vec<String>,
    field_index: HashMap<String, usize>,
    final_builder: FB,
    target_phantom: PhantomData<T>,
    fields_phantom: PhantomData<(T0, T1, T2)>,
}

impl<'de, T, T0, T1, T2, FB> Visitor<'de> for FieldVisitor<T, T0, T1, T2, FB>
where
    T0: for<'a> Deserialize<'a>,
    T1: for<'a> Deserialize<'a>,
    T2: for<'a> Deserialize<'a>,
    FB: FinalBuilder<T, (T0, T1, T2)>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut field0: Option<T0> = None;
        let mut field1: Option<T1> = None;
        let mut field2: Option<T2> = None;

        while let Some(key) = map.next_key()? {
            if self.field_names.contains(&key) {
                match self.field_index.get(&key) {
                    Some(0) => {
                        if field0.is_some() {
                            return Err(de::Error::duplicate_field("field0"));
                        }
                        field0 = Some(map.next_value()?);
                    }
                    Some(1) => {
                        if field1.is_some() {
                            return Err(de::Error::duplicate_field("field1"));
                        }
                        field1 = Some(map.next_value()?);
                    }
                    Some(2) => {
                        if field2.is_some() {
                            return Err(de::Error::duplicate_field("field2"));
                        }
                        field2 = Some(map.next_value()?);
                    }
                    _ => {}
                }
            }
        }

        let field0 = field0.ok_or_else(|| de::Error::missing_field("field0"))?;
        let field1 = field1.ok_or_else(|| de::Error::missing_field("field1"))?;
        let field2 = field2.ok_or_else(|| de::Error::missing_field("field2"))?;

        Ok(self
            .final_builder
            .assemble((field0, field1, field2))
            .unwrap())
    }
}

impl<T, T0, T1, T2, FB, V> StructDeserializer<T, (T0, T1, T2), FB, V>
where
    T0: for<'a> Deserialize<'a>,
    T1: for<'a> Deserialize<'a>,
    T2: for<'a> Deserialize<'a>,
    FB: FinalBuilder<T, (T0, T1, T2)>,
    V: Validator<T>,
{
    pub fn deserialize<'de, D: Deserializer<'de>>(self, des: D) -> Result<T, D::Error> {
        let StructDeserializer {
            target_phantom: _,
            fb_args_phantom: _,
            final_builder,
            validator,
            field_names,
        } = self;
        let field_names_index: HashMap<String, _> = field_names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i))
            .collect();
        let field_visitor = FieldVisitor {
            field_names: field_names.clone(),
            field_index: field_names_index,
            final_builder: final_builder.unwrap(),
            target_phantom: PhantomData::default(),
            fields_phantom: PhantomData::default(),
        };
        // I don't like this AT ALL
        let field_names_static: &'static [&'static str] = &*field_names
            .into_iter()
            .map(|s| &*Box::leak(s.into_boxed_str()))
            .collect::<Vec<_>>()
            .leak();
        des.deserialize_struct("struct", field_names_static, field_visitor)
    }
}
