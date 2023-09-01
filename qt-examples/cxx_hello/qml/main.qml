// qml/main.qml

// 引入qt基本组件
import QtQuick 2.12;
import QtQuick.Controls 2.12;
import QtQuick.Window 2.12;
import QtQuick.Controls.Basic;

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

    Rot {
        id: rot // 唯一标识
        plain: ""
        secret: ""
    }

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        /* space between widget */
        spacing: 10

        // 实现say hello功能
        Label {
            text: "please click this button"
            font.bold: true
        }

        Button {
            text: "Say Hello!"
            onClicked: {
                // 支持js es5/es6语法
                let name = "js";
                console.log("name: ", name);

                // 生成m-n的随机数字
                let m = 1;
                let n = 100;
                let rnd = Math.floor(Math.random() * (n - m)) + m;
                console.log("gen js random number: ", rnd);

                // 调用Hello上面的say_hello方法，这里的say_hello需要采用驼峰格式
                hello.sayHello();
            }
        }

        // 实现md5加密功能
        Label {
            text: "please input text"
            font.bold: true
        }
        TextArea {
            placeholderText: qsTr("origin string")
            text: rot.plain
            onTextChanged: rot.plain = text
            background: Rectangle {
                implicitWidth: 400
                implicitHeight: 50
                radius: 3
                color: "#e2e8f0"
                border.color: "#21be2b"
            }
        }

        Button {
            text: "Md5 Encrypt"
            onClicked: {
                // js语法赋值操作
                // console.log("plain: ", rot.plain);
                let secret = rot.md5(rot.plain);
                // console.log("secret: ", rot.secret);

                // 赋值后，就会自动填充 Label 中的text
                rot.secret = secret;
            }
        }

        Label {
            text: rot.secret
        }
    }
}
