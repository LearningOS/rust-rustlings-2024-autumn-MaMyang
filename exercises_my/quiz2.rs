pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 完整的函数签名：输入是 Vec<(String, Command)>，输出是 Vec<String>
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![]; // 声明一个空的输出向量
        
        // 遍历输入向量
        for (string, command) in input.iter() {
            // 根据命令模式匹配并处理字符串
            let result = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(times) => {
                    let mut s = string.clone(); // 克隆原始字符串
                    for _ in 0..*times {
                        s.push_str("bar"); // 追加 "bar"
                    }
                    s
                }
            };
            output.push(result); // 将结果推入输出
        }
        
        output // 返回输出向量
    }
}

#[cfg(test)]
mod tests {
    // 引入 my_module 中的 transformer 函数
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),        // 转换成大写 HELLO
            (" all roads lead to rome! ".into(), Command::Trim), // 去除空白
            ("foo".into(), Command::Append(1)),          // 追加 'bar' 1 次
            ("bar".into(), Command::Append(5)),          // 追加 'bar' 5 次
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
