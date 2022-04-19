#!/bin/bash
cargo build
sudo mv ./target/debug/flatman /usr/bin
mkdir -p ~/.cache/flatman_install
if (pacman -Q paru)
then
echo "paru already installed, skipping installation"
else
cd ~/.cache/flatman_install
sudo pacman -S git
git clone https://aur.archlinux.org/paru
cd paru
makepkg -si
cd ~
fi
