fn take_string(s: impl Into<String>) {
	let _s = s.into();
}

fn main() {
    // 文字列リテラル (str)
    take_string("str");

    // String
    let arg = "str".to_string();
    take_string(arg);
}
