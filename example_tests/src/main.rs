use std::i32;
use SerializeLib_rs::{get_enum_param_part, serializable, serializable_base, macro_if_value_exists, macro_if_not_value_exists};
use SerializeLib_rs::serializable::Serializable;
use SerializeLib_rs::streams::{readablestream::ReadableByteStream, writablestream::WritableStream};
use SerializeLib_rs::streams::implementations::memorystream::MemoryStream;
use crate::TestEnum::{Empty, Something};

serializable! {
    struct Test {
        pub a: u8,
        pub b: i32,
    }
    
    enum TestEnum {
        Empty -> 0
        Something(Test) -> 1
    }
}

fn main() {
    
    
    // let mut file = File::options().create(true).write(true).open("test.bin").unwrap();
    // let test = Test {
    //     a: 10,
    //     b: -666
    // };
    // let test = TestEnum::Something(Test {
    //     a: 100,
    //     b: -666
    // });
    let test = vec![Empty, Something(Test {a:0, b:10}), Something(Test {a: 50, b: 3610})];
    // test.serialize(&mut file);
    
    // drop(file);
    
    // let mut file = File::open("test.bin").unwrap();
    // let test = Vec::<TestEnum>::deserialize(&mut file);
    
    let mut buffer: Vec<u8> = vec![];
    let mut buffer_stream = MemoryStream::from_vec(&mut buffer);
    
    test.serialize(&mut buffer_stream);
    
    let test = Vec::<TestEnum>::deserialize(&mut buffer_stream);
    
    // let v = test.b;
    // println!("{v}");
    // match test { 
    //     TestEnum::Empty => {
    //         println!("Empty!")
    //     },
    //     TestEnum::Something(val) => {
    //         let b = val.b;
    //         println!("Something! {b}")
    //     }
    // }

    for test_enum in test {
        match test_enum { 
            Empty => {
                println!("Empty!")
            },
            Something(val) => {
                let a = val.a;
                let b = val.b;
                println!("Something! a: {a}, b: {b}")
            }
        }
    }
}
