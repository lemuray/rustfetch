# Guide to installation

Hello and welcome to our comprehensive guide on how to install **rustfetch**!

## Install script
The main way to install rustfetch is through the **bash install script**, you can use it in two main ways:
- Directly run it from your terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/lemuray/rustfetch/main/install.sh | bash
```
- Download it and run it separately (May fix some bugs):
```bash
curl -fsSL https://raw.githubusercontent.com/lemuray/rustfetch/main/install.sh -o install.sh
chmod +x install.sh # Make the script executable
./install.sh
```

## Package managers
Rustfetch is currently available on nixpkgs, to install it system wide run:
```
nix shell nixpkgs#rustfetch
```
Or you can also install it for the current shell only:
```
nix-shell -p rustfetch
```

## Build it from source
You can also build rustfetch directly from source, fetching the latest release:

> Note: in order to build the code from source, [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) must be installed

```bash
cargo install --git https://github.com/lemuray/rustfetch.git --tag v0.3.0
```
You can also build from the dev branch, note that this branch is not tested on and may cause bugs
```bash
cargo install --git https://github.com/lemuray/rustfetch.git --branch dev
```

In case any error that includes ``linking with cc failed`` occurs, make sure to have the following dependencies installed:
- ``libxcb-devel``
- ``libX11-devel``
<details>
  <summary>Click here to see the installation commands for these dependencies on your distro</summary>

  - Debian/Ubuntu based: ``sudo apt update && sudo apt install libxcb1-dev libx11-dev``
  - Arch (Pacman): ``sudo pacman -Syu libxcb libx11``
  - Fedora/RHEL: ``sudo dnf install libxcb-devel libX11-devel``
  - OpenSUSE: ``sudo zypper install libxcb-devel libX11-devel``
  - Gentoo: ``sudo emerge -av x11-libs/libxcb x11-libs/libX11``
</details>

In case installing these dependencies does not work, feel free to [open an issue](https://github.com/lemuray/rustfetch/issues/new), including the error message and/or logs.

## Manual installation
Download the latest binaries for your platform from the [releases page](https://github.com/lemuray/rustfetch/releases/latest).

You can then make the script executable by doing ``chmod +x rustfetch`` and then run it with ``./rustfetch``

## None of this works?
In that case, please open an [issue](https://github.com/lemuray/rustfetch/issues/new) and describe your problem. We will be pleased to help you with any complications you might encounter.

**Enjoy your stay!**
