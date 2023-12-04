/*
 * @Author: wanfeng 1505991024@qq.com
 * @Date: 2023-11-16 09:34:27
 * @LastEditors: wanfeng 1505991024@qq.com
 * @LastEditTime: 2023-12-04 11:54:45
 * @FilePath: /rustlings/exercises/generics/generics1.rs
 * @Description: 
 */
// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
