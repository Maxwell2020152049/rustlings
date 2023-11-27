/*
 * @Author: wanfeng 1505991024@qq.com
 * @Date: 2023-11-16 09:34:27
 * @LastEditors: wanfeng 1505991024@qq.com
 * @LastEditTime: 2023-11-27 11:02:04
 * @FilePath: /rustlings/exercises/options/options2.rs
 * @Description: 
 */
// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(i) = optional_target {
            assert_eq!(i, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // 最后一个元素是 Some(None)
        while let Some(i) = optional_integers.pop() {
            if let Some(j) = i {
                assert_eq!(j, cursor);
            } else {
                break;
            }

            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
