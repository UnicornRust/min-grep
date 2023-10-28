// 使用 env 库来获取命令行参数
use std::{env, process};

// 引入 mgrep 库模块
use mgrep::Config;
///
/// env::args() : 只能处理有效的 Unicode 字符的输入，在任何参数中
///              包含了无效Unicode字符的时候会 panic, 返回 String
/// env::args_os() : 可以处理无效的 Unicode 字符，返回的是OsString
///              这个类型于操作系统相关，因此更为复杂

fn main() {
    // 将命令行传递的参数放入道一个 vector 中, 这里我们声明了 args
    // 的的类型，collect() 函数可以用来创建多样的集合，这里rust 无
    // 法推断你想要什么类型的集合，因此需要显示地声明
    // let args: Vec<String> = env::args().collect();

    // 接收返回的配置信息，或者处理异常,
    // let config = Config::new(&args).unwrap_or_else(|error| {
    //
    // ---------------------
    //
    // 使用迭代器对程序进行改造, 这里 env:args() 返回就是一个迭代器
    // std:env:Args() 类型,
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("problem parsing arguments: {error}");
        // 立即终止进程并将提供的入参作为退出的状态码
        process::exit(1);
    });

    // 这里我们不不关注 run 函数的返回值，因为它是 ()
    // 而是关注它是否返回了错误，因此，使用 if let 而不是 unwrap_or_else
    if let Err(e) = mgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
