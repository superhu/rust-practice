/*// 修复错误
struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arrays = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 3, 5],
        },
        Array {
            data: [1, 2, 3]
        }
    ];
}*/

/*
use std::fmt::Debug;

// 填空
fn print_array<T:Debug, const N:usize>(arr: [T;N]) {
    println!("{:?}", arr);
}
fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}*/

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(x: T)
    where
        Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    println!("size:{}", core::mem::size_of::<T>()  );
}
// 修复 main 函数中的错误
fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // size of &str ?
    check_size(["hello你好"; 31]);  // size of String?
    check_size(['中'; 1]); // size of char ?
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}