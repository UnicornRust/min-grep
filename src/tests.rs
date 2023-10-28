/// 用于测试文本搜索的功能实现
#[cfg(test)]
mod tests {

    use crate::{search, search_case_insensitive};

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // 注意这里的 `\`, 它告诉rust 不要在字符串字面值的内容的开头加入换行符
        let contents = "\
Rust:
safe, fast, productive.
Pick thers.
Duct tape.";
        // 我们断言 search 函数的返回值是我们期望的哪一行的文本
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        // 注意这里的 `\`, 它告诉rust 不要在字符串字面值的内容的开头加入换行符
        let contents = "\
Rust:
safe, fast, productive.
Pick thers.
Trust me.";
        // 我们断言 search 函数的返回值是我们期望的哪一行的文本
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        );
    }
}
