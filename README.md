# pudding

Dead simple mouse jiggler for Linux. Both X11 and Wayland are supported.

## Installation

### Pre-built binaries

You can download pre-built binaries from the [releases](https://github.com/mucahitkurtlar/pudding/releases) page.

### From source

You can also build `pudding` from source. You need to have [Rust](https://www.rust-lang.org/tools/install) installed

```bash
git clone https://github.com/mucahitkurtlar/pudding
cd pudding
cargo build --release
```

The binary will be located at `target/release/pudding`.

### Arch Linux

For Arch Linux users, `pudding` is not available yet on the AUR but you can use the following PKGBUILD to build and install `pudding`.

```bash
git clone https://github.com/mucahitkurtlar/pudding
cd pudding
cd build
makepkg -si
```

## Usage

Before using `pudding`, you need to be sure that you have the necessary permissions. You can do this by running the following command:

```bash
sudo usermod -a -G input $USER
```

After that, you can run `pudding` by simply executing the binary.

```bash
pudding
```

Also you can run `pudding` with some options:

```bash
pudding --period 1 --duration 10 # Jiggles the mouse cursor every second for 10 minutes
```

```bash
pudding --help
```

Prints the help information.

```
pudding
Jiggles the mouse cursor.

USAGE:
    pudding [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --period <jiggling-period-secs>          Jiggling period in seconds [default: 1]
    -d, --duration <running-duration-minutes>
            Program running duration in minutes (if not specified, runs indefinitely)

```
