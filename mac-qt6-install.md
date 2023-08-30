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
brew安装的qt版本也有可能是高版本，更具对应的目录执行 env setting，比如说下面的版本
```shell
% ls
6.5.1_3
```

# env setting
vim ~/.bash_profile

```shell
# 分别对 LDFLAGS CPPFLAGS PKG_CONFIG_PATH 配置

export LDFLAGS="-L/usr/local/Cellar/qt/6.5.1_3/lib"
# 对于 LDFLAGS 如果系统中本身就有设置值的话
# 请通过 export LDFLAGS="$LDFLAGS -L/usr/local/Cellar/qt/6.5.1_3/lib" 方式设置

export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib"
export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib/c++ -Wl,-rpath,/usr/local/opt/llvm/lib/c++"

# 对于 CPPFLAGS 如果系统中本身就有设置值的话
# 请通过 export CPPFLAGS="$CPPFLAGS -I/usr/local/Cellar/qt/6.5.1_3/include" 方式配置
export CPPFLAGS="-I/usr/local/Cellar/qt/6.5.1_3/include"
export CPPFLAGS="$CPPFLAGS -I/usr/local/opt/llvm/include"

# 对于 PKG_CONFIG_PATH 设置，如果系统中本身就有的话
# 请通过 export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:/usr/local/Cellar/qt/6.5.1_3/lib/pkgconfig" 方式设置
export PKG_CONFIG_PATH="/usr/local/Cellar/qt/6.5.1_3/lib/pkgconfig"

export PATH="/usr/local/Cellar/qt/6.5.1_3/bin:$PATH"
export PATH="/usr/local/opt/llvm/bin:$PATH"
```

保存:wq 之后，再执行如下命令生效
```shell
source ~/.bash_profile
```

# qt-creator setting
打开qt-creator填写qt安装路径`/usr/local/Cellar/qt/6.5.1_3`就完成了qt link设置
