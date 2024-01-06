// Rust codes for analysis tool

pub const MAX_SIZE: usize = 100;

pub struct Data {
    pub number: i32,
    pub name: String,
}

pub enum Status {
    Active,
    Inactive,
    Unknown,
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn use_unsafe() {
    let mut num = 5;

    unsafe {
        let num_ptr: *mut i32 = &mut num;
        *num_ptr += 1;
    }

    println!("Number after unsafe modification: {}", num);
}



pub trait Information {
    fn print_info(&self);
}

impl Information for Data {
    fn print_info(&self) {
        println!("Data number is: {}, name is: {}", self.number, self.name);
    }
}

pub fn generic_add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub mod utils {
    pub fn compute_length(s: &str) -> usize {
        s.len()
    }

    pub struct Helper {
        pub value: i32,
    }

    impl Helper {
        pub fn new(value: i32) -> Self {
            Helper { value }
        }

        pub fn is_positive(&self) -> bool {
            self.value > 0
        }
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Example async
pub async fn fetch_data() -> Result<String, ()> {
    // Mock data fetching
    Ok("Data".to_string())
}

pub trait AdvancedInfo {
    fn detailed_info(&self) -> String;
}

impl AdvancedInfo for Data {
    fn detailed_info(&self) -> String {
        format!("Detailed Data: number is {}, name is {}", self.number, self.name)
    }
}