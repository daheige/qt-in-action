use cstr::cstr;
use log::info;
use qmetaobject::prelude::*;
// use std::env;

// 配置资源文件qml
// as前面的qml是文件目录，as后面的是虚拟路径
qrc!(qml_resource,
    "qml" as "qml" {
        "cxx_hello.qml",
    },
);

// 定义模块 my_object
mod my_object;

fn main() {
    // 注册资源qml资源
    qml_resource();

    // 日志level 优先级  error > warn > info > debug > trace
    // 如果这一行注释掉，运行时，可以设置  RUST_LOG=debug cargo run --bin cxx_hello
    // env::set_var("RUST_LOG", "debug"); // 手动设置日志级别环境变量
    env_logger::init();

    // 下面的代码注释掉后，就采用qt的js console.log日志输出
    // 如果需要把qml作为前缀加入到日志终端输出，就打开注释
    // qmetaobject::log::init_qt_to_rust();

    qmetaobject::log::init_qt_to_rust();
    info!("exec begin cxx_hello");

    // 注册自定义类型
    qml_register_type::<my_object::Hello>(cstr!("qRustCode"), 1, 0, cstr!("Hello"));
    qml_register_type::<my_object::Rot>(cstr!("qRustCode"), 1, 0, cstr!("Rot"));
    let mut engine = QmlEngine::new();
    engine.load_file(QString::from("qrc:/qml/cxx_hello.qml"));
    engine.exec();
}
