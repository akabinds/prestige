# Prestige
Prestige is an operating system written for fun and educational purposes in Rust. It targets the x86-64 architecture and can run on common emulators like QEMU. 

The Prestige project began from an interest to develop an operating system. The development of Prestige is attributed to the second edition of [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann, reading the [OSDev wiki](https://wiki.osdev.org/Main_Page), and by inspecting the source code of other open source operating systems and kernels.

## Building Prestige
**Clone the repository**

*With git*:
```
$ git clone https://github.com/akabinds/prestige.git
$ cd prestige
```

*With GitHub CLI*:
```
$ gh repo clone akabinds/prestige
$ cd prestige
```

**Installed Required Tools**

*Through `make`*:
```
$ make setup
```

*Raw Commands*:
```
$ curl https://sh.rustup.rs -sSf | sh
$ rustup install nightly
$ rustup default nightly
$ cargo install bootimage
```

**Build image**
```
$ make image
```

**Run in QEMU**
```
$ make qemu
```

**All at Once**
```
$ make
```

**Argument/Compilation Options**
- output: *vga*, *serial* (defaults to *vga*)
- kbd_layout: *qwerty*, *azerty*, *dvorak* (defaults to *qwerty*)
- build_mode: *release*, *debug* (defaults to *release*)

*Specify or override options by typing `OPT=VAL` after `make` or `make TARGET`*