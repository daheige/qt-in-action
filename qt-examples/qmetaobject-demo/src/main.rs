use cstr::cstr;
use qmetaobject::prelude::*;
use qttypes::QString;

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
enum Options {
    Foo = 1,
    Bar = 2,
    Quaz = 3,
}

fn main() {
    // 注册qml名字，版本号，传递给qml文件中rust enum变量
    qml_register_enum::<Options>(cstr!("qRustCode"), 1, 0, cstr!("Options"));

    let mut engine = QmlEngine::new();
    engine.load_file(QString::from("qml/main.qml"));
    engine.exec();
}