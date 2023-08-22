# mac os qt5 install
```shell
brew install qt@5
```

安装好的目录在：`/usr/local/opt/qt@5`中
# env setting
vim ~/.bash_profile
```shell
# LDFLAGS CPPFLAGS PKG_CONFIG_PATH 如果系统中本身就有设置值的话，请通过export LDFLAGS="$LDFLAGS -L/usr/local/opt/qt@5/lib" 方式设置
export LDFLAGS="-L/usr/local/opt/qt@5/lib"
export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib"
export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib/c++ -Wl,-rpath,/usr/local/opt/llvm/lib/c++"

export CPPFLAGS="-I/usr/local/opt/qt@5/include"
export CPPFLAGS="$CPPFLAGS -I/usr/local/opt/llvm/include"

# PKG_CONFIG_PATH 设置，如果系统中本身就有的话，请通过 export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:/usr/local/opt/qt@5/lib/pkgconfig" 方式设置
export PKG_CONFIG_PATH="/usr/local/opt/qt@5/lib/pkgconfig"

export PATH="/usr/local/opt/qt@5/bin:$PATH"
export PATH="/usr/local/opt/llvm/bin:$PATH"
```

保存:wq 之后，再执行如下命令生效
```shell
source ~/.bash_profile
```

# qt-creator install
```shell
brew install qt-creator
```

打开qt-creator填写qt安装路径`/usr/local/opt/qt@5`就完成了qt link设置
