# ubuntu 18.04 or 20.04 系统安装qt
https://wiki.qt.io/Install_Qt_5_on_Ubuntu

```shell
# 安装cmake make gcc g++ clang++ clang llvm
sudo apt update -y
sudo apt install -y make gcc g++ llvm clang cmake vim
#sudo snap install cmake --classic # 升级到最新的cmake

# 安装qt5依赖的组件
sudo apt install -y pkg-config libssl-dev zlib1g-dev
sudo apt install -y build-essential libgl1-mesa-dev gdb
sudo apt install -y libfontconfig1
sudo apt install -y mesa-common-dev 
sudo apt install -y libglu1-mesa-dev
sudo apt install -y libxkbcommon-dev
sudo apt install -y libvulkan-dev
sudo apt install -y freeglut3-dev
sudo apt install -y wget vim bash curl

# 安装 qt5工具
sudo apt install -y qt5*
sudo apt install -y qt5-default
sudo apt install -y qtbase5-dev qtchooser qt5-qmake qtbase5-dev-tools

# 安装qt5 qml相关的依赖
sudo apt install -y qtdeclarative5-dev
sudo apt install -y qml-module-qtquick-controls
sudo apt install -y qml-module-qtquick-controls2
sudo apt install -y qtmultimedia5-dev
sudo apt install -y libqt5multimedia5-plugins
sudo apt install -y qml-module-qtmultimedia
sudo apt install -y qml-module-qtquick-extras
sudo apt install -y qml-module-qt-labs-platform
sudo apt install -y qtbase5-private-dev
sudo apt install -y qtlocation5-dev qtpositioning5-dev
sudo apt install -y qml qmlscene
sudo apt install -y qml-module-qtquick-dialogs
sudo apt install -y qml-module-qtquick-shapes
sudo apt install -y qml-module-qt-labs-folderlistmodel
sudo apt install -y qml-module-qt-labs-settings
sudo apt install -y qml-module-qtwebengine
sudo apt install -y qml-module-qtwebview
sudo apt install -y qml-module-qtwebchannel
sudo apt install -y qml*
sudo update-mime-database /usr/share/mime
```

# 安装qtcreator (可选)
```shell
sudo apt install qtcreator
```
