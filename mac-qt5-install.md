# mac os qt5 install
```shell
brew install qt@5
```

安装好的目录在：`/usr/local/opt/qt@5`中，对应安装的版本 5.15.10
# 安装qt必要的工具链
```shell
brew install llvm cmake make gcc mold clang-format
```

# qt 环境变量配置
vim ~/.bash_profile

```shell
# 分别对 LDFLAGS CPPFLAGS PKG_CONFIG_PATH 配置
export QT_HOME=/usr/local/opt/qt@5 # qt安装目录
export LDFLAGS="-L$QT_HOME/lib"
# 对于 LDFLAGS 如果系统中本身就有设置值的话
# 请通过 export LDFLAGS="$LDFLAGS -L$QT_HOME/lib" 方式设置

export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib"
export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib/c++ -Wl,-rpath,/usr/local/opt/llvm/lib/c++"

# 对于 CPPFLAGS 如果系统中本身就有设置值的话
# 请通过 export CPPFLAGS="$CPPFLAGS -I$QT_HOME/include" 方式配置
export CPPFLAGS="-I$QT_HOME/include"
export CPPFLAGS="$CPPFLAGS -I/usr/local/opt/llvm/include"

# 对于 PKG_CONFIG_PATH 设置，如果系统中本身就有的话
# 请通过 export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$QT_HOME/lib/pkgconfig" 方式设置
export PKG_CONFIG_PATH="$QT_HOME/lib/pkgconfig"

export PATH="$QT_HOME/bin:$PATH"
export PATH="/usr/local/opt/llvm/bin:$PATH"
```

保存:wq 之后，再执行如下命令生效
```shell
source ~/.bash_profile
```

# qt-creator 安装(可选）
如果使用c++开发qt项目，就需要安装qt-creator
```shell
brew install qt-creator
```

打开qt-creator填写qt安装路径`/usr/local/opt/qt@5`就完成了qt link设置
