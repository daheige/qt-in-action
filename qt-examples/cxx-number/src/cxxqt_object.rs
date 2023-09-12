/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod my_object {
    // 引入QString
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;
    }

    /// The Rust struct for the QObject
    /// 这里的qml_uri,qml_version就是qml/main.qml中的 import cxx_demo 1.0 相对应
    #[cxx_qt::qobject(qml_uri = "cxx_demo", qml_version = "1.0")]
    #[derive(Default)]
    pub struct MyObject {
        #[qproperty]
        number: i32,

        #[qproperty]
        string: QString,
    }

    impl qobject::MyObject {
        /// Increment the number Q_PROPERTY
        /// 自增实现，这里采用了pin特征将内存地址固定住
        #[qinvokable]
        pub fn increment_number(self: Pin<&mut Self>) {
            let previous = *self.as_ref().number();
            self.set_number(previous + 1);
        }

        /// Print a log message with the given string and number
        #[qinvokable]
        pub fn say_hi(&self, s: &QString, number: i32) {
            println!("Hi from cxx_demo! string is {} and number is {}",s,number);
        }
    }
}