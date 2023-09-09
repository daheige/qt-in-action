import QtQuick 2.6
import QtQuick.Window 2.0
// Import our Rust classes
// 这个版本名称，必须和main.rs
import qRustCode 1.0

Window {
    visible: true
    title: "Hello App"
    height: 480
    width: 640
    color: "#e4af79"

    Text {
        anchors.centerIn: parent
        text: `Hello! Bar is ${Options.Bar}, Foo is ${Options.Foo},Quaz is ${Options.Quaz}.`
    }
}