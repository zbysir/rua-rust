fn main() {
    print!("ok")
}

// 字符串拼接
fn add(a: &mut String) {
    a.push_str("!");
}

fn add2(mut a: String) -> String {
    a.push_str("!");
    a
}

// 写测试
#[cfg(test)]
mod test {
    use crate::{add, add2};

    #[test]
    fn append_str() {
        let mut s = "Hello, world".to_string();
        add(&mut s);
        let b = add2(s.clone());
        assert_eq!(s, "Hello, world!");
        assert_eq!(b, "Hello, world!!");
    }
}