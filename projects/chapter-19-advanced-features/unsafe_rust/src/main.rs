use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}


fn main() {

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous()
    }

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
       (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
       )
    }
}

unsafe fn dangerous() {

}


