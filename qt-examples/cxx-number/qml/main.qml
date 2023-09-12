import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// This must match the qml_uri and qml_version
// specified with the #[qobject] macro in Rust.
import cxx_demo 1.0

Window {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640

    MyObject {
        id: myObject
        number: 1
        string: "My String with my number: " + myObject.number
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: "current number: " + myObject.number
        }

        Label {
            text: "current string: " + myObject.string
        }

        Button {
            text: "Increment Number"
            onClicked: myObject.incrementNumber()
        }

        Button {
            text: "Say Hi!"
            onClicked: myObject.sayHi(myObject.string, myObject.number)
        }
    }
}