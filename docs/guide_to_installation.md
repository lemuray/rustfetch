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
chmod +x install.sh
./install.sh
```

## Build it from source
You can also build rustfetch directly from source:

> Note: in order to build the code from source, you have to have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed

```bash
cargo install --git https://github.com/lemuray/rustfetch.git
```

## Manual installation
Download the latest binaries for your platform from the [releases page](https://github.com/lemuray/rustfetch/releases/latest).

## None of this works?
In that case, please open an [issue](https://github.com/lemuray/rustfetch/issues/new) and describe your problem. We will be pleased to help you with any complications you might encounter.

**Enjoy your stay!**