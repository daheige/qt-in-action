# ubuntu 22.04 系统安装qt
```shell
# 安装cmake make gcc g++ clang++ clang llvm
sudo apt update -y
sudo apt install -y make gcc g++ clang++ llvm clang cmake
sudo snap install cmake --classic # 升级到最新的cmake

# 安装相关依赖
sudo apt install -y pkg-config libssl-dev zlib1g-dev
sudo apt install -y build-essential libgl1-mesa-dev gdb
sudo apt install -y libxkbcommon-dev
sudo apt install -y libvulkan-dev
sudo apt install -y wget vim bash curl

# Fedora
# sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel

sudo apt install -y qt6-base-dev qt6-declarative-dev

# 安装qt6
# If you are unsure. Just install all Qt dependencies
# It is no more than 200 MB
sudo apt install -y qt6*
sudo dnf install -y qt6*
```
