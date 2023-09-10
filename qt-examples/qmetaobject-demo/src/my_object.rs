use qmetaobject::prelude::*;
use qttypes::QString;

// 需要添加QObject trait
#[derive(Default, QObject)]
pub struct Hello {
    // Specify the base class with the qt_base_class macro
    base: qt_base_class!(trait QObject),
    say_hello: qt_method!(fn(&self) -> ()), // 定义的say_hello方法
}

// 为 Hello 实现say_hello方法
impl Hello {
    pub fn say_hello(&self) {
        println!("Hello world!");
    }
}

#[derive(Default, QObject)]
pub struct Rot {
    // Specify the base class with the qt_base_class macro
    base: qt_base_class!(trait QObject),

    // 属性用 qt_property!包裹起来
    name: qt_property!(QString; NOTIFY name_changed),
    // Declare a signal
    name_changed: qt_signal!(),

    plain: qt_property!(QString; NOTIFY name_changed),
    // Declare a signal
    plain_changed: qt_signal!(),

    secret: qt_property!(QString; NOTIFY secret_changed),
    // Declare a signal
    secret_changed: qt_signal!(),
    md5: qt_method!(fn(&self, plain: String) -> QString),
}

impl Rot {
    // 实现md5加密
    pub fn md5(&self, plain: String) -> QString {
        if plain.is_empty() {
            return QString::from("plain is empty");
        }

        let digest = md5::compute(&plain);
        let md5_str = format!("{:x}", digest); // 生成md5 string
        println!("plain:{} md5 string:{}", plain, md5_str);
        let result = format!("md5 string:{}", md5_str);
        QString::from(result.as_str())
    }
}
