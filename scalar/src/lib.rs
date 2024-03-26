pub fn sum(a: i32, b: i32) -> i32 {

    if a + b>= 0 && a+ b <= 255 {
    
    a + b
    
    } else {
    
    panic!("numbers out of scope")
    
    }
    
    }

pub fn diff(a: i32, b: i32) -> i32 {

    if a-b >= -32768 && a-b <= 32767 {
    
    a-b
    
    } else {
    
    panic!("numbers out of scope")
    
    }
    
    }
    
      
    
    pub fn pro(a: i32, b: i32) -> i32 {
    
    if a * b>= -128 && a* b<= 127 {
    
    a * b
    
    } else {
    
    panic!("numbers out of scope")
    
    }
    
    }
    
      
    
    pub fn quo(a: f32, b: f32) -> f32 {
    
    a / b
    
    }
    
      
    
    pub fn rem(a: f32, b: f32) -> f32 {
    
    a % b
    
    }