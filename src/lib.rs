use std::error::Error;
use std::{env, fs};

pub mod tests;

// 通过将真实的业务逻辑抽离到 run 函数中，这个函数会返回一个Result<T,E> 对象
// 这样就可以进一步的用用户友好的方式来统一 main 中的错误处理。
//
// Box<dyn Error> : 是 trait 对象, 这是一个实现了 Error trait 的类型.
// 不过无需指定具体的返回值的类型，这在不同的错误场景可能不同类型的
// 错误提供了返回值的灵活性. 这也就是 dyn, (dynamic 的缩写).
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 引入 fs  模块来读取文件内容
    // expect() 来处理可能遇到的错误
    let contents = fs::read_to_string(config.file_path)?;

    // 我们需要通过配置环境变量来转换我们匹配时的行为(大小写是否敏感)
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// 功能的核心代码，在文本中搜索包含我们查找文本的特定语句
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // --------------------
    // 使用迭代器适配器来重写这段代码，可以避免一个可变的中间 result vector 的是哟个
    // 函数式编程风格倾向于最小化可变状态的数量来使代码尽可能简洁，去掉可变状态可能会
    // 使得将来进行并行搜索增强变得容易。因为我们不必管理 result vector 的并发访问
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    // 迭代器版本
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// 构建配置结构体，这将有利于我们清晰的表达 query 与 file_path 之间
// 的关联关系，在代码的维护中可以通过字段名清晰的表达每个字段的功能
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // 参数个数校验，防止后续取值出现的异常
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // 我们收到的时一个引用，但是我们返回的是一个具有所有权的
        // Config, 因此使用 clone 方法得到字符串的完整拷贝，这回耗费
        // 一些时间与性能，后续会有更高效的方式来处理字符串
        //
        // 获取到的第一个参数为当前执行的文件的名称，这在根据名称做一些
        // 行为处理的时时很有效的，这里我们的程序并不使用它。
        let query = args[1].clone();
        let file_path = args[2].clone();

        // 实际处理环境变量的函数位于标准库 env 模块中，引入这个到当前作用域后
        // 我们来获取这个环境变量, 传入环境变量的 key 以查找. 返回 Result<T, E>
        // 这里我们的规则是关注 IGNORE_CASE 变量是否被设置，无论他被设置了什么值
        // 因此使用了 is_ok(), 而不是 unwrap，expect 或者 Result 的其他方法
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    // 我们来使用一个迭代器的版本
    // 这里使用 trait 作为参数，限定任何实现了 Iterator trait 的类型并返回 String
    // 都可以作为入参, 同时我们已经获取了迭代器的所有权，因此使用 mut 使得引用可变
    //
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 第一个参数是程序的名称,忽略
        args.next();

        // 我们使用 match 来获取数据赋值, 这里直接就可以获得args中的值，而不需要 clone
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
