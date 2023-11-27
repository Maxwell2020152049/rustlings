/*
 * @Author: wanfeng 1505991024@qq.com
 * @Date: 2023-11-16 09:34:27
 * @LastEditors: wanfeng 1505991024@qq.com
 * @LastEditTime: 2023-11-27 11:04:34
 * @FilePath: /rustlings/exercises/options/options3.rs
 * @Description: 
 */
// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
