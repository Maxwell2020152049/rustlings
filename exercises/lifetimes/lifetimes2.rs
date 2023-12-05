/*
 * @Author: wanfeng 1505991024@qq.com
 * @Date: 2023-11-16 09:34:27
 * @LastEditors: wanfeng 1505991024@qq.com
 * @LastEditTime: 2023-12-05 14:54:33
 * @FilePath: /rustlings/exercises/lifetimes/lifetimes2.rs
 * @Description: 
 */
// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
