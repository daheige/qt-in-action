# qt-in-action
qt in action notes

由于rust-qt 暂时只支持qt5，建议安装qt5版本，使用qt5和rust-qt实现绑定

# rust-qt starts fast
1. 参考 mac-qt5-install.md 安装好qt5
2. 安装好rust，并设置好对应的环境变量，参考 rust-install.md
3. 切换到 qt-examples/hello_qml 目录，执行`cargo run`就可以看到效果如下：
![](hello-qml.png)

# rust-qt
官方链接：https://github.com/rust-qt

官方examples: https://github.com/rust-qt/examples

# rust-qt核心组件库
https://github.com/rust-qt/ritual

# rust-qt 练手实战
https://juejin.cn/post/7227624340232208443

# cxx-qt
- 官方地址： https://github.com/KDAB/cxx-qt
- cxx-qt使用手册：https://kdab.github.io/cxx-qt/book/
- cxx-qt中文使用参考：https://github.com/jason-yau/cxx-qt-book/blob/main/src/SUMMARY.md
- cxx-qt实战案例：https://dev.to/logrocket/build-a-desktop-app-with-qt-and-rust-1cfi
- cxx-qt架构设计：
<img width="762" alt="image" src="https://github.com/daheige/qt-in-action/assets/9988859/f24e56f1-e129-46d0-a27b-2608b2ae69b3">

# 关于rust qt绑定选择
- rust-qt目前只支持qt5.15版本，并且暂时没维护了
- cxx-qt 是KDAB出品的，跨平台支持linux,macos,windows等不同的操作系统，它支持cmake和cargo两种构建方式（cargo这种方式，无任何c/c++代码依赖），同时支持c/c++和rust相互调用和代码生成，Safe interop between Rust and Qt
- gtk-rs 相对来说比较复杂，不大建议使用，在跨平台的绑定支持还不是很成熟

综合对比，推荐使用cxx-qt这个库，需要懂一些c/c++基础，比如说header头引入以及c++简单的语法，能看懂一部分c++代码（这块主要是qt接口函数，api调用需要）。
