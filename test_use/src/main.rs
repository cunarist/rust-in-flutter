use test_base::Serialize;
use test_proc::Serialize;

#[derive(Serialize)]
struct MyStruct {
    name: String,
    age: u32,
}

#[derive(Serialize)]
enum MyEnum {
    Unit,
    NewType(String),
    Tuple(String, u32),
    Struct { x: u32, y: u32 },
}

fn main() {
    let my_struct = MyStruct {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{}", my_struct.serialize());

    let my_enum = MyEnum::Struct { x: 1, y: 2 };
    println!("{}", my_enum.serialize());
}
