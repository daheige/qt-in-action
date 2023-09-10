use cstr::cstr;
use qmetaobject::prelude::*;

// 定义模块 my_object
mod my_object;

fn main() {
    // 注册自定义类型
    qml_register_type::<my_object::Hello>(cstr!("qRustCode"), 1, 0, cstr!("Hello"));
    qml_register_type::<my_object::Rot>(cstr!("qRustCode"), 1, 0, cstr!("Rot"));
    let mut engine = QmlEngine::new();
    engine.load_file(QString::from("qml/cxx_hello.qml"));
    engine.exec();
}