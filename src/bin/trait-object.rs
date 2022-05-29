/*trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn main() {
    // 填空
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
    // bird.swim();
    // 但它依然可以叫唤
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // 这只鸟儿忘了如何飞翔，因此以下代码会报错
    // bird.fly();
    // 但它也可以叫唤
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!")
}

// 实现以下函数
// fn hatch_a_bird(n: i8) -> Box<dyn Bird> {
//     match n {
//         1 => Box::new(Swan),
//         2 => Box::new(Duck),
//         _ => Box::new(Duck),
//     }
// }
// // 实现以下函数
fn hatch_a_bird(n: i8) -> &'static dyn Bird {
    match n {
        1 => &Swan,
        2 => &Duck,
        _ => &Duck,
    }
}*/

/*
trait Bird {
    fn quack(&self);
}

struct Duck;

impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}

struct Swan;

impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    // 填空
    // let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];
    let birds: [&dyn Bird; 2] = [&Duck {}, &Swan {}];

    for bird in birds {
        bird.quack();
        // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
        // 因此，以下代码会报错
        // bird.fly();
    }
}*/

/*// 填空
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn main() {
    let x = 1.1f64;
    let y = 8u8;

    // draw x
    draw_with_box(Box::new(x));

    // draw y
    draw_with_ref(&y);

    println!("Success!")
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}
*/

// trait Foo {
//     fn method(&self) -> String;
// }
//
// impl Foo for u8 {
//     fn method(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }
//
// impl Foo for String {
//     fn method(&self) -> String {
//         format!("string: {}", *self)
//     }
// }
//
// // 通过泛型实现以下函数
// fn static_dispatch(x: impl Foo) -> String {
//     x.method()
// }
// fn static_dispatch2<T: Foo>(x: T) -> String {
//     x.method()
// }
//
// // 通过特征对象实现以下函数
// fn dynamic_dispatch(x: &dyn Foo) -> String {
//     x.method()
// }
//
// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();
//
//     static_dispatch(x);
//     static_dispatch2(x);
//     dynamic_dispatch(&y);
//
//     println!("Success!")
// }

// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self {
        42
    }
}

impl MyTrait for String {
    fn f(&self) -> Self {
        self.clone()
    }
}

fn my_function(x: impl MyTrait) -> impl MyTrait {
    x.f()
}

fn main() {
    my_function(13_u32);
    my_function(String::from("abc"));

    println!("Success!")
}
