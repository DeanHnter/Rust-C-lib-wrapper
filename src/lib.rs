#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use libc::*;
include!("./build.rs");



#[cfg(test)]
mod tests {
    extern {
        fn add(inputa: i32,inputb: i32) -> i32;
    }
    #[test]
    fn it_works() {
        let mut result = 0;
        unsafe {
            result = add(4,4);
            println!("{}", add(5,7));
        } 

        assert_eq!(result, 8);
       
    }
}
