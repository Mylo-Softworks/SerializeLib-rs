use crate::streams::readablestream::ReadableByteStream;
use crate::streams::writablestream::WritableStream;

pub trait Serializable {
    fn serialize(&self, writer: &mut impl WritableStream<u8>);
    
    fn deserialize(reader: &mut impl ReadableByteStream) -> Self;
}

#[macro_export]
macro_rules! macro_if_value_exists {
    ($_:ty => $tt:tt) => {$tt};
    ( => $tt:tt) => {};
}

#[macro_export]
macro_rules! macro_if_not_value_exists {
    ($_:ty => $tt:tt) => {};
    ( => $tt:tt) => {$tt};
}

#[macro_export]
macro_rules! get_enum_param_part {
    ($val_name:ident $(($($val:ty)?))? -> $id:literal -> id) => {
        $id
    };
    
    ($val_name:ident $(($($val:ty)?))? -> $id:literal -> name) => {
        $val_name
    };
    
    ($val_name:ident $(($($val:ty)?))? -> $id:literal -> value) => {
        $val
    };
    
    ($val_name:ident $(($($val:ty)?))? -> $id:literal -> if_value => $tt:tt) => {
        macro_if_value_exists!($($($val)?)? => $tt);
    };
    
    ($val_name:ident $(($($val:ty)?))? -> $id:literal -> if_not_value => $tt:tt) => {
        macro_if_not_value_exists!($($($val)?)? => $tt);
    };
}

#[macro_export]
macro_rules! serializable_base {
    (struct $name:ident {
        $($field_vis:vis $field_name:ident: $field_type:ty,)*
    }) => {
        pub struct $name {
            $($field_vis $field_name: $field_type,)*
        }

        impl Serializable for $name {
            fn serialize(&self, writer: &mut impl WritableStream<u8>) {
                $(
                    self.$field_name.serialize(writer);
                )*
            }

            fn deserialize(reader: &mut impl ReadableByteStream) -> Self {
                $name {
                    $(
                        $field_name: <$field_type>::deserialize(reader),
                    )*
                }
            }
        }
    };
    
    ($vis:vis enum $name:ident {
        $( // Parameters
            $val_name:ident $(($($val:ident)?))? -> $id:literal
        )*
    }) => {
        enum $name {
            $(
                $val_name $(($($val)?))?,
            )*
        }

        impl Serializable for $name {
            fn serialize(&self, writer: &mut impl WritableStream<u8>) {
                let id = match self { 
                    $(
                        $name::$val_name$(($($val)?))? => $id as u8,
                    )*
                };
                
                id.serialize(writer); // Write id header
                
                match self {
                    $(
                        $name::$val_name$(($($val)?))? => {
                            get_enum_param_part!($val_name $(($($val)?))? -> $id -> if_value => {
                                if let $name::$val_name(v) = self {
                                    v.serialize(writer);
                                }
                            });
                        },
                    )*
                }
            }

            fn deserialize(reader: &mut impl ReadableByteStream) -> Self {
                let id = u8::deserialize(reader);
                
                match id {
                    $(
                        $id => {
                            get_enum_param_part!($val_name $(($($val)?))? -> $id -> if_not_value => {return $name::$val_name});
                            get_enum_param_part!($val_name $(($($val)?))? -> $id -> if_value => {
                                return $name::$val_name($($($val::deserialize(reader))?)?)
                            });
                        }
                    )*
                    
                    _ => { panic!("Failed to deserialize.") }
                }
            }
        }
    };
}


#[macro_export]
macro_rules! serializable {
    ($($t1:tt $name:ident $tt:tt)*) => {
        $(
            serializable_base! {
                $t1 $name $tt
            }
        )*
    };
}

serializable! {
    // struct TestStruct {
    //     pub a: u8,
    //     pub b: i32,
    // }
    
    enum TestEnum {
        None -> 0_u8
        Some(i32) -> 1_u8
    }
}

fn test() {
    match TestEnum::None { 
        TestEnum::None => 0,
        TestEnum::Some(i32) => 1
    };
}