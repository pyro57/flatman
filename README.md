# flatman
Wrapper for Flatpak, pacman, and the AUR

# build and install
## automatic
```
cd flatman
chmod +x ./install.sh
./install.sh
```
this will autmatically compile the flatman binary and move it to /usr/bin

## manual
```
cd flatman
cargo build
sudo cp ./target/debug/flatman /usr/bin
```


# Info
I recently started using flatpak as my primary source of packages and wanted a one stop shop for all my package sources, flatpak, pacman, and the AUR.  Similiar to how yay works for pacman and the AUR.
