use std::marker::PhantomData;

use serde::{ser::SerializeStruct, Serialize, Serializer};

pub trait FieldAccessor<T, FT> {
    fn get_field(self, parent: &T) -> &FT;
}

impl<T, FT, FN> FieldAccessor<T, FT> for FN
where
    FN: FnOnce(&T) -> &FT,
{
    fn get_field(self, parent: &T) -> &FT {
        self(parent)
    }
}

pub struct Field<T, FT, FA> {
    name: &'static str,
    accessor: FA,
    target_phantom: PhantomData<T>,
    field_phantom: PhantomData<FT>,
}

impl<T, FT, FA> Field<T, FT, FA> {
    fn new(name: &'static str, field_accessor: FA) -> Self {
        Self {
            name,
            accessor: field_accessor,
            target_phantom: PhantomData::default(),
            field_phantom: PhantomData::default(),
        }
    }
}

pub struct StructSerializer<T, FIELDS = (), const FN: usize = 0> {
    target_phantom: PhantomData<T>,
    fields: FIELDS,
}

impl<T> Default for StructSerializer<T> {
    fn default() -> Self {
        Self {
            target_phantom: PhantomData::default(),
            fields: (),
        }
    }
}

impl<T> StructSerializer<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<FT, FA: FieldAccessor<T, FT>>(
        self,
        name: &'static str,
        field_accessor: FA,
    ) -> StructSerializer<T, (Field<T, FT, FA>,), 1> {
        let StructSerializer {
            target_phantom,
            fields: _,
        } = self;
        StructSerializer {
            target_phantom,
            fields: (Field::new(name, field_accessor),),
        }
    }
}

macro_rules! add_field_impl {
    ($($len:expr => ($($n:tt $fname:ident $faname:ident),+) $fname2:ident $faname2:ident)+) => {
        $(
            impl<T, $($fname, $faname),+> StructSerializer<T, ($(Field<T, $fname, $faname>,)+), $len>
            where
                $($faname: FieldAccessor<T, $fname>,)+
            {
                pub fn field<$fname2, $faname2: FieldAccessor<T, $fname2>>(self, name: &'static str, field_accessor: $faname2) ->
                    StructSerializer<T, ($(Field<T, $fname, $faname>,)+ Field<T, $fname2, $faname2>,), {$len + 1}>
                {
                    let StructSerializer {
                        target_phantom,
                        fields,
                    } = self;
                    StructSerializer {
                        target_phantom,
                        fields: ($(fields.$n,)+ Field::new(name, field_accessor)),
                    }
                }
            }
        )+
    };
}

add_field_impl! {
    1 => (0 F0 F0A) F1 F1A
    2 => (0 F0 F0A, 1 F1 F1A) F2 F2A
    3 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A) F3 F3A
    4 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A) F4 F4A
    5 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A) F5 F5A
    6 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A) F6 F6A
    7 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A) F7 F7A
    8 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A, 7 F7 F7A) F8 F8A
    9 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A, 7 F7 F7A, 8 F8 F8A) F9 F9A
    10 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A, 7 F7 F7A, 8 F8 F8A, 9 F9 F9A) F10 F10A
    11 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A, 7 F7 F7A, 8 F8 F8A, 9 F9 F9A, 10 F10 F10A) F11 F11A
    12 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A, 7 F7 F7A, 8 F8 F8A, 9 F9 F9A, 10 F10 F10A, 11 F11 F11A) F12 F12A
    13 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A, 7 F7 F7A, 8 F8 F8A, 9 F9 F9A, 10 F10 F10A, 11 F11 F11A, 12 F12 F12A) F13 F13A
    14 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A, 7 F7 F7A, 8 F8 F8A, 9 F9 F9A, 10 F10 F10A, 11 F11 F11A, 12 F12 F12A, 13 F13 F13A) F14 F14A
    15 => (0 F0 F0A, 1 F1 F1A, 2 F2 F2A, 3 F3 F3A, 4 F4 F4A, 5 F5 F5A, 6 F6 F6A, 7 F7 F7A, 8 F8 F8A, 9 F9 F9A, 10 F10 F10A, 11 F11 F11A, 12 F12 F12A, 13 F13 F13A, 14 F14 F14A) F15 F15A
}

macro_rules! ser_impl {
    ($($len:expr => $($ftname:ident $faname:ident $fvname:ident),+)+) => {
        $(
            impl<T, $($ftname, $faname),+> StructSerializer<T, ($(Field<T, $ftname, $faname>,)+), $len>
            where
                $(
                $ftname: Serialize,
                $faname: FieldAccessor<T, $ftname>,
                )+
            {
                pub fn serialize<S: Serializer>(self, value: &T, ser: S) -> Result<S::Ok, S::Error> {
                    let mut struct_ser_state = ser.serialize_struct(std::any::type_name::<T>(), $len)?;
                    let ($($fvname,)+) = self.fields;
                    $(
                        struct_ser_state.serialize_field($fvname.name, $fvname.accessor.get_field(value))?;
                    )+
                    struct_ser_state.end()
                }
            }
        )+
    }
}

ser_impl! {
    1 => F0 F0A f0
    2 => F0 F0A f0, F1 F1A f1
    3 => F0 F0A f0, F1 F1A f1, F2 F2A f2
    4 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3
    5 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4
    6 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5
    7 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6
    8 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7
    9 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7, F8 F8A f8
    10 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7, F8 F8A f8, F9 F9A f9
    11 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7, F8 F8A f8, F9 F9A f9, F10 F10A f10
    12 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7, F8 F8A f8, F9 F9A f9, F10 F10A f10, F11 F11A f11
    13 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7, F8 F8A f8, F9 F9A f9, F10 F10A f10, F11 F11A f11, F12 F12A f12
    14 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7, F8 F8A f8, F9 F9A f9, F10 F10A f10, F11 F11A f11, F12 F12A f12, F13 F13A f13
    15 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7, F8 F8A f8, F9 F9A f9, F10 F10A f10, F11 F11A f11, F12 F12A f12, F13 F13A f13, F14 F14A f14
    16 => F0 F0A f0, F1 F1A f1, F2 F2A f2, F3 F3A f3, F4 F4A f4, F5 F5A f5, F6 F6A f6, F7 F7A f7, F8 F8A f8, F9 F9A f9, F10 F10A f10, F11 F11A f11, F12 F12A f12, F13 F13A f13, F14 F14A f14, F15 F15A f15
}
