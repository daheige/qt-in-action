# gitconfig配置
```git
[user]
	name = your_name # 改成你的git
	email = you@email.com # 改成你的git email
[core]
    fileMode = false
    autocrlf = false
    safecrlf = true
    crlf     = false
	excludesfile = ~/.gitignore_global
[push]
    default = simple
[branch]
    autosetuprebase = always
[alias]
    co = checkout
    cm = commit -m
    st = status
    pl = pull
    ps = push
    lgst=log --pretty=format:'%H [%an] %ci%ncommit:%s' --stat
    lt =log --stat
    cp = cherry-pick
    cam = commit -am
    br = branch
    df = diff
    fe = fetch -p
    lg=log --graph --pretty=format:'%h %ci [%Cgreen%an%Creset]%s'
    lgf=log --graph --pretty=format:'%h %ci [%Cgreen%an%Creset]%s' -p
    re=reset
    sta=stash
    gcpr=gc --prune=now
    clf=clean -f
    del=branch -D
    slog=show

[http]
	sslBackend = openssl
[remote "origin"]
	proxy =
```
