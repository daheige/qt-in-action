# mac os qt5 install
```shell
brew install qt@5
```

安装好的目录在：`/usr/local/opt/qt@5`中
# env setting
```shell
vim ~/.bash_profile
# 设置 LDFLAGS 和 CPPFLAGS
export LDFLAGS="-L/usr/local/opt/qt@5/lib"
export CPPFLAGS="-I/usr/local/opt/qt@5/include"
# 以下配置如果系统中本来就有，就直接拼接，比如
# export LDFLAGS="-L/usr/local/opt/ruby/lib -L/usr/local/opt/qt@5/lib"
# export CPPFLAGS="-I/usr/local/opt/ruby/include -I/usr/local/opt/qt@5/include"

# 设置 PKG_CONFIG_PATH
export PKG_CONFIG_PATH="/usr/local/opt/qt@5/lib/pkgconfig"
# 如果之前存在该环境变量，请通过下面的方式设置
# export PKG_CONFIG_PATH="/usr/local/opt/qt@5/lib/pkgconfig:$PKG_CONFIG_PATH"

# 将qt加入PATH中
export PATH="/usr/local/opt/qt@5/bin:$PATH"
```

保存:wq 之后，再执行如下命令生效
```shell
source ~/.bash_profile
```

# qt-creator setting
打开qt-creator填写qt安装路径`/usr/local/opt/qt@5`就完成了qt link设置
