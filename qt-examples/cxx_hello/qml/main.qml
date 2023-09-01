// qml/main.qml

import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// This must match the qml_uri and qml_version
// specified with the #[cxx_qt::qobject] macro in Rust.
// 这里的导入名字必须要 cxxqt_object.rs的 一致
// #[cxx_qt::qobject(qml_uri = "hello", qml_version = "1.0")]
import hello 1.0

Window {
    title: qsTr("Hello App")
    visible: true
    height: 480
    width: 640
    color: "#e4af79"

    Hello {
        id: hello
    }

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        /* space between widget */
        spacing: 10
        Label {
            text: "please click this button"
            font.bold: true
        }

        Button {
            text: "Say Hello!"
            onClicked: {
                // 支持js语法
                var name = "js";
                console.log("name: ",name);
                hello.sayHello();
            }
        }
    }
}
