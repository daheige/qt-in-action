use cstr::cstr;
use qmetaobject::prelude::*;

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
    engine.load_data(r##"
        import QtQuick 2.0;
        import QtQuick.Window 2.0;
        import qRustCode 1.0;

        Window {
            visible: true
            title: "Hello App"
            height: 480
            width: 640
            color: "#e4af79"

            Text {
                anchors.centerIn: parent
                text: "Hello! Bar is"+Options.Bar+", Foo is "+Options.Foo+", Quaz is "+Options.Quaz+"."
            }
        }"##.into());
    engine.exec();
}