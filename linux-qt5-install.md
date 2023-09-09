# linux qt5安装
https://doc.qt.io/qt-5/linux-building.html

下载安装qt6
```shell
cd /usr/local/src
sudo wget https://download.qt.io/archive/qt/5.15/5.15.5/single/qt-everywhere-opensource-src-5.15.5.tar.xz

sudo tar -vxf qt-everywhere-opensource-src-5.15.5.tar.xz
cd ./qt-everywhere-opensource-src-5.15.5
sudo ./configure
sudo make && make install
```

# set env
vim ~/.bash_profile
```shell
export PATH=/usr/local/Qt-5.15.5/bin:$PATH
```
