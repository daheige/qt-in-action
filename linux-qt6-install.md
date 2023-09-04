# linux qt6 安装
参考文档： https://doc.qt.io/qt-6/linux-building.html
工具依赖：https://doc.qt.io/qt-6/linux-building.html#step-2-install-build-requirements

下载安装qt6
```shell
cd /usr/local/src
sudo wget https://download.qt.io/archive/qt/6.5/6.5.2/single/qt-everywhere-src-6.5.2.tar.xz

sudo tar -vxf qt-everywhere-opensource-src-6.5.2.tar.xz
cd ./qt-everywhere-opensource-src-6.5.2
sudo ./configure
sudo make && make install
```

# set env
vim ~/.bash_profile
```shell
export PATH=/usr/local/Qt-6.5.2/bin:$PATH
```