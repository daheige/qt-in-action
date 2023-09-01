#[cxx_qt::bridge]
mod my_object {

    #[cxx_qt::qobject(qml_uri = "hello", qml_version = "1.0")]
    #[derive(Default)]
    pub struct Hello {}

    // 为 Hello 实现say_hello方法
    impl qobject::Hello {
        #[qinvokable]
        pub fn say_hello(&self) {
            println!("Hello world!")
        }
    }
}
