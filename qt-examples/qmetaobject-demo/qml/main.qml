import QtQuick 2.12;
import QtQuick.Window 2.12;

// Import our Rust classes
// 这个版本名称，必须和main.rs qml_register 注册的名称一样
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
}