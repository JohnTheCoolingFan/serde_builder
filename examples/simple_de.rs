use serde_builder::de::StructDeserializer;
use serde_json::json;

#[derive(Debug)]
#[allow(dead_code)]
struct TestStruct {
    a_number: u32,
    string: String,
    array_of_strings: Vec<String>,
}

fn main() {
    let test_data = json!({
        "a_number": 20,
        "string": "foobar",
        "array_of_strings": ["foo", "bar", "baz"]
    });
    let test_data_string = serde_json::to_string(&test_data).unwrap();
    let mut test_data_deser = serde_json::Deserializer::from_str(&test_data_string);
    let struct_deserializer = StructDeserializer::default()
        .field("a_number")
        .field("string")
        .field("array_of_strings")
        .final_builder(|num, string, arr| TestStruct {
            a_number: num,
            string,
            array_of_strings: arr,
        });
    let deser_result = struct_deserializer.deserialize(&mut test_data_deser);
    println!("{:?}", deser_result);
}
