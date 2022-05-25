
// 修复错误，不要新增代码行
/*fn main() {
    let s: &str = "hello, world";
}*/


// 使用至少两种方法来修复错误
/*fn main() {
    let mut s: Box<str> =  "hello, world".into();
    greetings(s.as_mut())
}

fn greetings(s: &str) {
    println!("{}",s)
}
*/


// 填空
/*fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}*/


//3 修复所有错误，并且不要新增代码行
/*fn main() {
    let mut  s =  String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}*/

//4 修复所有错误，并且不要新增代码行

//5 填空
/*fn main() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}*/


//6 修复所有错误，不要删除任何一行代码
/*fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3,"hello,world!");
    println!("{}",&s1);
}*/


//7 使用至少两种方法来修复错误
/*fn main() {
    let  s =  "hello, world";
    greetings(s.into())
}

fn greetings(s: String) {
    println!("{}",s)
}*/

/*fn main() {
    // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
    // 填空以输出 "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means {}) {}", "\x3F",byte_escape);

    // 也可以使用 Unicode 形式的转义字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    // 还能使用 \ 来连接多行字符串
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}*/

/* 填空并修复所有错误 */
/*fn main() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");
    let s = "abc
    sfdsfsf
    dsfsfsdfdsfd ";
    print!("{s}")
}*/

/*use std::str;

fn main() {
    // 注意，这并不是 `&str` 类型了！
    let bytestring: &[u8; 21] = b"this is a byte string";


    // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
    println!("A byte string: {:?}", bytestring);

    // 字节数组也可以使用转义
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但是不支持 unicode 转义
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // raw string
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 将字节数组转成 `str` 类型可能会失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节数组可以不是 UTF-8 格式
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // 但是它们未必能转换成 `str` 类型
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}*/



/*fn main() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    assert_eq!(h, "h");

    let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    assert_eq!(h1, "中");
}*/



fn main() {
    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}