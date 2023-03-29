#[link(name="foo", kind="static")]
extern { 
    // this is rustified prototype of the function from our C library
    fn testcall(v: f32) -> i32; 
}

fn main() {
    println!("Hello, world from Rust!");

    // calling the function from foo library
    unsafe { 
        let result = testcall(3.14159); 
        println!("Result from C is {}", result);
    };
}
