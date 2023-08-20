# rust安装
- https://www.rust-lang.org/zh-CN/tools/install
- https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html

    建议安装到rust v1.58.0+版本
    shell安装
    curl https://sh.rustup.rs -sSf | sh
    对于centos7安装请看 rust-centos7-install.md
    
    rust升级执行如下操作：
    rustup update
    
    rust版本查看
    cargo --version
    cargo 1.58.0 (7f08ace4f 2021-11-24)
    升级到指定版本 rust update "1.58.0"

# 设置rust国内镜像
    https://mirrors.tuna.tsinghua.edu.cn/help/rustup/

	国内提高访问速度，建议设置环境变量 
    export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
    export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
    export PATH="$HOME/.cargo/bin:$PATH"

	在用户目录.cargo文件夹或在与Cargo.toml同级目录.cargo文件夹下创建config文件
	$cd ~/.cargo/
	$touch config
	添加如下内容：
	# 指定镜像
    #replace-with = 'sjtu'
    replace-with = 'ustc'
    
    # 源码地址
    [source.crates-io]
    registry = "https://github.com/rust-lang/crates.io-index"
    
    # 清华大学
    [source.tuna]
    registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
    
    # 中国科学技术大学
    [source.ustc]
    registry = "https://mirrors.ustc.edu.cn/crates.io-index"
    #registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    
    # 上海交通大学
    [source.sjtu]
    registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
    
    # rustcc社区
    [source.rustcc]
    registry = "git://crates.rustcc.cn/crates.io-index"
    
    [source.aliyun]
    registry = "https://code.aliyun.com/rustcc/crates.io-index"
    [net]
    git-fetch-with-cli=true
    [http]
    check-revoke = false

# rust编辑器选择
使用vscode,clion都可以
