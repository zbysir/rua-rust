use std::borrow::Cow;

#[macro_use]
extern crate serde_derive;

fn main() {
    let a: Address = get_into();
    println!("{:?}", a);

    // 报错：`AddressLife<'_>` must implement `Deserialize<'0>`, for any lifetime `'0`...
    // let a: AddressLife = get_with_life();
    // println!("{:?}", a);
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

// 错误的做法：x 被借用了，方法结束就会被销毁，但 T 的生命周期比 x 更大。
#[cfg(error)]
fn get_into<'a, T: serde::Deserialize<'a>>() -> T
{
    let x = String::from(r#"{"street": "1", "city": "2"}"#);
    let k: T = serde_json::from_str(&x).unwrap();

    k
}

// 正确的做法：使用 for <'a> 告诉编译器，T 不会引用 x（或者说，T 不需要任何生命周期）
// 参考资料：https://serde.rs/lifetimes.html，https://www.reddit.com/r/rust/comments/6uobit/fora_lifetime_syntax/
fn get_into<T:for <'a> serde::Deserialize<'a>>() -> T
{
    let x = String::from(r#"{"street": "1", "city": "2"}"#);
    let k: T = serde_json::from_str(&x).unwrap();

    k
}

#[derive(Debug, Serialize, Deserialize)]
struct AddressLife<'a> {
    street: &'a str,
    city: String,
}

// 这样也编译不过，因为 T 拥有一个生命周期 'a，所以 x 的生命周期必须大于等于 'a，而 x 被 T 借用，但函数退出时就销毁了，所以 x 的生命周期 < a。
// rust 必须避免这样的情况发生，否则会出现悬垂引用。
fn get_with_life<T:for <'a> serde::Deserialize<'a>>() -> T
{
    let x = String::from(r#"{"street": "1", "city": "2"}"#);
    let k: T = serde_json::from_str(&x).unwrap();

    k
}


// 探索生命周期
fn get_max<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b { a } else { b }
}


// 探索生命周期
fn get_default(a: &str) -> Cow<str> {
    let x = String::from("a");
    if a == "" { Cow::Owned(x) } else { Cow::Borrowed(a) }
}

// 探索生命周期
// String 可以被返回
fn life_get_str<'a >() -> String {
    let x = String::from("a");
    x
}

// 而 引用 始终不能被返回
#[cfg(error)]
fn life_get_str_err<'a >() -> &'a str{
    let x = String::from("a");
    x.as_str()
}

