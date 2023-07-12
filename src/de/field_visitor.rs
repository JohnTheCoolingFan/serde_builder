use std::{collections::HashMap, marker::PhantomData};

use serde::{de::Visitor, Deserialize};

use super::FinalBuilder;

pub(crate) struct FieldVisitor<T, FBARGS, FB, const FN: usize> {
    field_names: [String; FN],
    field_index: HashMap<String, usize>,
    final_builder: FB,
    target_phantom: PhantomData<T>,
    fields_phantom: PhantomData<FBARGS>,
}

impl<T, T0, FB> FieldVisitor<T, T0, FB, 1>
where
    T0: for<'a> Deserialize<'a>,
    FB: FinalBuilder<T, T0>,
{
    pub(crate) fn new(final_builder: FB, field_names: [String; 1]) -> Self {
        let field_index = field_names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i))
            .collect();
        Self {
            field_names,
            field_index,
            final_builder,
            target_phantom: PhantomData::default(),
            fields_phantom: PhantomData::default(),
        }
    }
}

impl<'de, T, T0, FB> Visitor<'de> for FieldVisitor<T, T0, FB, 1>
where
    T0: for<'a> Deserialize<'a>,
    FB: FinalBuilder<T, T0>,
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

        while let Some(key) = map.next_key()? {
            if self.field_names.contains(&key) {
                match self.field_index.get(&key) {
                    Some(0) => {
                        if field0.is_some() {
                            return Err(serde::de::Error::duplicate_field("field0"));
                        }
                        field0 = Some(map.next_value()?);
                    }
                    // field_index was constructed based on field_names array, so it can't contain
                    // indexes larger than max index of field_names, and cannot contain keys that
                    // are not in field_names
                    _ => unreachable!(),
                }
            }
        }

        let field0 = field0.ok_or_else(|| serde::de::Error::missing_field("field0"))?;

        Ok(self.final_builder.assemble(field0).unwrap())
    }
}

macro_rules! field_visitor_impl {
    ($($len:expr => ($($n:tt $name:ident $fname:ident),+))+) => {
        $(
            impl<T, $($name,)+ FB> FieldVisitor<T, ($($name),+), FB, $len>
            where
                $($name: for<'a> Deserialize<'a>,)+
                FB: FinalBuilder<T, ($($name,)+)>,
            {
                pub(crate) fn new(final_builder: FB, field_names: [String; $len]) -> Self {
                    let field_index = field_names
                        .iter()
                        .enumerate()
                        .map(|(i, n)| (n.clone(), i))
                        .collect();
                    Self {
                        field_names,
                        field_index,
                        final_builder,
                        target_phantom: PhantomData::default(),
                        fields_phantom: PhantomData::default(),
                    }
                }
            }

            impl<'de, T, $($name,)+ FB> Visitor<'de> for FieldVisitor<T, ($($name,)+), FB, $len>
            where
                $($name: for<'a> Deserialize<'a>,)+
                FB: FinalBuilder<T, ($($name),+)>,
            {
                type Value = T;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("struct")
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>
                {
                    $(let mut $fname: Option<$name> = None;)+

                    while let Some(key) = map.next_key()? {
                        if self.field_names.contains(&key) {
                            match self.field_index.get(&key) {
                                $(
                                Some($n) => {
                                    if $fname.is_some() {
                                        #[cfg(not(feature = "leaking"))]
                                        return Err(serde::de::Error::duplicate_field(stringify!($fname)));
                                        #[cfg(feature = "leaking")]
                                        return Err(serde::de::Error::duplicate_field(&*Box::leak(key.clone().into_boxed_str())));
                                    }
                                    $fname = Some(map.next_value()?);
                                },
                                )+
                                // field_index was constructed based on field_names array, so it can't contain
                                // indexes larger than max index of field_names, and cannot contain keys that
                                // are not in field_names
                                _ => unreachable!(),
                            }
                        }
                    }

                    $(
                        #[cfg(not(feature = "leaking"))]
                        let $fname = $fname.ok_or_else(|| serde::de::Error::missing_field(stringify!($fname)))?;
                        #[cfg(feature = "leaking")]
                        let $fname = $fname.ok_or_else(|| serde::de::Error::missing_field(&*Box::leak(self.field_names.get(($n as usize)).unwrap().clone().into_boxed_str())))?;
                    )+

                    Ok(self.final_builder.assemble(($($fname),+)).unwrap())
                }
            }
        )+
    }
}

field_visitor_impl! {
    2 => (0 T0 field0, 1 T1 field1)
    3 => (0 T0 field0, 1 T1 field1, 2 T2 field2)
    4 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3)
    5 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4)
    6 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5)
    7 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6)
    8 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field7)
    9 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field, 8 T8 field8)
    10 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field, 8 T8 field8, 9 T9 field9)
    11 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field, 8 T8 field8, 9 T9 field9, 10 T10 field10)
    12 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field, 8 T8 field8, 9 T9 field9, 10 T10 field10, 11 T11 field11)
    13 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field, 8 T8 field8, 9 T9 field9, 10 T10 field10, 11 T11 field11, 12 T12 field12)
    14 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field, 8 T8 field8, 9 T9 field9, 10 T10 field10, 11 T11 field11, 12 T12 field12, 13 T13 field13)
    15 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field, 8 T8 field8, 9 T9 field9, 10 T10 field10, 11 T11 field11, 12 T12 field12, 13 T13 field13, 14 T14 field14)
    16 => (0 T0 field0, 1 T1 field1, 2 T2 field2, 3 T3 field3, 4 T4 field4, 5 T5 field5, 6 T6 field6, 7 T7 field, 8 T8 field8, 9 T9 field9, 10 T10 field10, 11 T11 field11, 12 T12 field12, 13 T13 field13, 14 T14 field14, 15 T15 field15)
}
