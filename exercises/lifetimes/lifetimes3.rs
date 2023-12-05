/*
 * @Author: wanfeng 1505991024@qq.com
 * @Date: 2023-11-16 09:34:27
 * @LastEditors: wanfeng 1505991024@qq.com
 * @LastEditTime: 2023-12-05 14:55:16
 * @FilePath: /rustlings/exercises/lifetimes/lifetimes3.rs
 * @Description: 
 */
// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
