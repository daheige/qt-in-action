# centos7 qt安装
1. 安装必要的依赖工具
```shell
sudo yum update -y
sudo yum install epel-release -y
sudo yum -y groupinstall "Development Tools"
sudo yum install -y clang gcc gcc-c++ cmake make llvm git openssl-devel zlib-devel automake
sudo yum install -y glibc-static file libstdc++-static diffutils util-linux wget
sudo sudo yum install -y gdb
```

配置好gitconfig，参考 git-config.md
2. 升级cmake到cmake3
```shell
cd /usr/local/src
sudo wget https://github.com/Kitware/CMake/releases/download/v3.27.4/cmake-3.27.4-linux-x86_64.tar.gz
sudo tar -zxvf cmake-3.27.4-linux-x86_64.tar.gz
sudo mv cmake-3.27.4-linux-x86_64 /usr/local/cmake
sudo rm -rf /usr/bin/cmake
sudo ln -s /usr/local/cmake/bin/cmake /usr/bin/cmake
```
添加环境变量 vim ~/.bash_profile
```
export PATH="$PATH:/usr/local/cmake/bin"
```

查看cmake版本
```shell
$ source ~/.bash_profile
$ cmake --version
cmake version 3.27.4

CMake suite maintained and supported by Kitware (kitware.com/cmake).
```

3. 升级gcc
devtoolset-4对应gcc5.x.x版本
devtoolset-6对应gcc6.x.x版本
devtoolset-7对应gcc7.x.x版本
devtoolset-8对应gcc8.x.x版本
devtoolset-9对应gcc9.x.x版本
devtoolset-10对应gcc10.x.x版本
```shell
sudo yum install -y centos-release-scl
sudo yum install -y devtoolset-10-gcc* devtoolset-10-libstdc++*
sudo yum install -y devtoolset-10
```
激活gcc版本，使其生效
```shell
source /opt/rh/devtoolset-10/enable
# 备份原来的gcc
sudo mv /usr/bin/gcc /usr/bin/gcc-4.8.5
# 建立软链接
sudo ln -s /opt/rh/devtoolset-10/root/bin/gcc /usr/bin/gcc
```
安装相关依赖
```shell
yum -y install mesa-libGL-devel mesa-libGLU-devel freeglut-devel 
```

4. 桌面环境安装
(如果已经是centos7桌面环境就不需要安装)
```shell
sudo yum -y groupinstall "GNOME Desktop"
sudo yum -y groupinstall "Development Tools"
```

5. 安装centos必要的依赖
```shell
sudo yum install -y libGL libGL-devel
sudo yum install -y mesa-libGL-devel mesa-libGLU-devel freeglut-devel
sudo yum install -y libxcb libxcb-devel libXrender libXrender-devel xcb-util-wm xcb-util-wm-devel xcb-util xcb-util-devel xcb-util-image xcb-util-image-devel xcb-util-keysyms xcb-util-keysyms-devel
```

reboot 重启centos7
6. 安装qt5
```shell
cd /usr/local/src
sudo wget https://download.qt.io/archive/qt/5.14/5.14.2/qt-opensource-linux-x64-5.14.2.run
sudo chmod +x qt-opensource-linux-x64-5.14.2.run
sudo ./qt-opensource-linux-x64-5.14.2.run
#如果遇到登陆，可以跳过
```

或者直接通过yum安装
```shell
sudo yum install -y qt5-qttools qt5-qttools-devel

sudo yum install -y qt5-qtsvg-devel qt5-qtbase-devel qt5-qtdeclarative-devel qt5-qtxmlpatterns-devel qt5-qtmultimedia-devel qt5-qtscript-devel qt5-qttools-devel qt5-qtwebkit-devel qt5-qtx11extras-devel qt5-qtlocation-devel qt5-qtwebsockets-devel qt5-qtserialport-devel qt5-qtwebchannel-devel

sudo yum install -y qt5-qtbase
sudo yum install -y qt5-qtbase-devel
sudo yum install -y qt5-qtquickcontrols2-devel
sudo yum install qt5-qtquickcontrols qt5-qtdeclarative-devel -y
sudo yum install -y qt5*
```

通过yum安装的qt5目录在 `/usr/lib64/qt5`
查看安装的qmake路径
```shell
which qmake-qt5
/usr/bin/qmake-qt5

# 建立软链接
ln -s /usr/bin/qmake-qt5 /usr/bin/qmake

# 查看是否安装成功
$ qmake -version
QMake version 3.1
Using Qt version 5.9.7 in /usr/lib64
```

7. 配置环境变量
vim ~/.bash_profile
```shell
export QT_HOME=/usr/lib64/qt5
export PATH="$QT_HOME/bin:$PATH"
```
使配置生效 source ~/.bash_profile
