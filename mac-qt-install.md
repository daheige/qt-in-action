# mac os qt install
```shell
brew install qt
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
