use std::io::Cursor;

use serde_builder::ser::StructSerializer;
use serde_json::json;

#[derive(Debug)]
struct TestStruct {
    a_number: u32,
    string: String,
    array_of_strings: Vec<String>,
}

fn main() {
    let test_data = TestStruct {
        a_number: 42,
        string: "foobar".into(),
        array_of_strings: vec!["foo".into(), "bar".into(), "baz".into()],
    };
    let mut output_buf = Vec::new();
    let mut ser = serde_json::Serializer::new(Cursor::new(&mut output_buf));
    StructSerializer::new()
        .field("a_number", |v: &TestStruct| &v.a_number)
        .field("string", |v: &TestStruct| &v.string)
        .field("array_of_strings", |v: &TestStruct| &v.array_of_strings)
        .serialize(&test_data, &mut ser)
        .expect("Serialization failed");
    let test_json_data = json!({
        "a_number": 42,
        "string": "foobar",
        "array_of_strings": ["foo", "bar", "baz"]
    });
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            &String::from_utf8(output_buf).expect("String parsing from utf8 failed")
        )
        .expect("Deserialization of serialized data failed"),
        test_json_data
    );
}
