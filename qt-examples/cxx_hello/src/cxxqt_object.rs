#[cxx_qt::bridge]
mod my_object {
    #[cxx_qt::qobject(qml_uri = "hello", qml_version = "1.0")]
    #[derive(Default)]
    pub struct Hello {}
    // 为 Hello 实现say_hello方法
    impl qobject::Hello {
        #[qinvokable]
        pub fn say_hello(&self) {
            println!("Hello world!");
        }
    }

    // 下面的Rot 是实现md5字符串加密功能
    // 引入c++ cxx_qt_lib 包中的rust QString
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "hello", qml_version = "1.0")]
    #[derive(Default)]
    pub struct Rot {
        #[qproperty]
        plain: QString,
        #[qproperty]
        secret: QString,
    }

    impl qobject::Rot {
        // 实现md5加密
        #[qinvokable]
        pub fn md5(&self, plain: &QString) -> QString {
            let b = plain.to_string();
            if b.is_empty() {
                return QString::from("plain is empty");
            }

            let digest = md5::compute(b);
            let md5_str = format!("{:x}", digest); // 生成md5 string
            println!("plain:{} md5 string:{}", plain, md5_str);
            let result = format!("md5 string:{}", md5_str);
            QString::from(&result)
        }
    }
}
