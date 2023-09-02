# ubuntu系统安装qt
```shell
sudo apt install -y make cmake gcc g++ llvm molds
sudo apt install -y libssl-dev zlib1g-dev
sudo apt install -y build-essential libgl1-mesa-dev
sudo apt install -y libxkbcommon-dev
sudo apt install -y libvulkan-dev

# Fedora
# sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel

sudo apt install -y qt6-base-dev qt6-declarative-dev

# If you are unsure. Just install all Qt dependencies
# It is no more than 200 MB
sudo apt install -y qt6*
sudo dnf install -y qt6*
```
