# mac os qt install
```shell
brew install qt
brew install qt-creator
```
当看到下面这个表示安装好了
```
==> Installing Cask qt-creator
==> Moving App 'Qt Creator.app' to '/Applications/Qt Creator.app'
🍺  qt-creator was successfully installed!
```
安装好的目录在：`/usr/local/Cellar/qt`中
```shell
% ls
6.5.1_2
```
# env setting
```shell
vim ~/.bash_profile
export PATH="/usr/local/Cellar/qt/6.5.1_2/bin:$PATH"
export LDFLAGS="-L/usr/local/Cellar/qt/6.5.1_2/lib"
export CPPFLAGS="-I/usr/local/Cellar/qt/6.5.1_2/include"
```
根据对应的qt帮忙设置即可
```shell
source ~/.bash_profile
```

# qt-creator setting
打开qt-creator填写qt安装路径`/usr/local/Cellar/qt/6.5.1_2`就完成了qt link设置
