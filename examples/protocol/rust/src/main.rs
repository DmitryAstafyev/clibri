pub mod protocol;

use protocol::{PackingStruct, StructDecode, StructEncode};

fn create() {
    let struct_example_a = protocol::StructExampleA {
        field_str: String::from("test"),
        field_str_empty: String::from(""),
        field_u8: 1,
        field_u16: 2,
        field_u32: 3,
        field_u64: 4,
        field_i8: -1,
        field_i16: -2,
        field_i32: -3,
        field_i64: -4,
        field_f32: 0.1,
        field_f64: 0.2,
        field_bool: true,
    };
}

fn reading() -> Result<(), String> {
    // Create a couple of examples
    let mut struct_example_a = protocol::StructExampleA::defaults();
    let mut struct_example_b = protocol::StructExampleB::defaults();
    let mut struct_example_c = protocol::StructExampleC::defaults();

    // Create reader
    let mut reader = protocol::Buffer::new();
    // Create a buffer and fill it with example data
    let buffer: Vec<u8> = [
        struct_example_a.pack(1, None).map_err(|e| e.to_string())?,
        struct_example_b.pack(2, None).map_err(|e| e.to_string())?,
        struct_example_c.pack(3, None).map_err(|e| e.to_string())?,
    ]
    .concat();

    // Put data into reader
    reader
        .chunk(&buffer, None)
        .map_err(|e| format!("Fail to read data: {:?}", e))?;
    // Reading messages
    while let Some(msg) = reader.next() {
        match msg.msg {
            protocol::AvailableMessages::StructExampleA(struct_a) => {
                println!("{:?}", struct_a);
            }
            protocol::AvailableMessages::StructExampleB(struct_b) => {
                println!("{:?}", struct_b);
            }
            protocol::AvailableMessages::StructExampleC(struct_c) => {
                println!("{:?}", struct_c);
            }
            _ => {}
        }
    }
    Ok(())
}
fn main() {
    reading().expect("Oops!");
}
