import QtQuick 2.12;
import QtQuick.Controls 2.12;
import QtQuick.Window 2.12;
// import QtQuick.Controls.Basic 2.12;

// Import our Rust classes
// 这个版本名称，必须和main.rs qml_register 注册的名称一样
import qRustCode 1.0;

Window {
    visible: true
    title: "Hello App"
    height: 480
    width: 640
    color: "#e4af79"

    // 自定义的Hello类型
    Hello {
        id: hello
    }

    // 自定义的Rot类型
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

                // 调用Hello上面的say_hello方法
                hello.say_hello();
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

        // Label {
        //     text: rot.secret
        // }
        TextArea {
            placeholderText: qsTr("md5 value")
            text: rot.secret
            background: Rectangle {
                implicitWidth: 400
                implicitHeight: 50
                radius: 3
                color: "#e2e8f0"
                border.color: "#21be2b"
            }
        }
    }
}