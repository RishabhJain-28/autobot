include!(r"D:\Projects\Rust\Rust Projects\autobot\target\debug\build\autobot-6872681b8f31f8ac\out\runtime\mod.rs");
         

fn main(){ 
            match run() { 
                Ok(_) => (), 
                Err(err) => println!("Error:{}",err)
            }
        }

fn run() -> Result<(), String> {
	let mut _a : f64 = 0.0;
	let mut _b : f64 = 0.0;
	_a = input_number();
	_b = input_number();
	println!("{}", _a + _b);
	let mut _c : String = String::from("");
	_c = String::from("Hello world");
	println!("{}", _c);
	println!("{}", String::from("Enter a string"));
	_c = input_string();
	println!("{}", _c);

Ok(())
}