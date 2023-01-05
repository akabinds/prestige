# Prestige
Prestige is an operating system written for fun and educational purposes in Rust. It targets the x86_64 architecture (for now)
and can run on emulators like QEMU.

The Prestige project began from an interest to develop an operating system. The initial development of Prestige began by reading the second edition of [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann. Other sources used were reading the [OSDev wiki](https://wiki.osdev.org/Main_Page) and inspecting the source code of other open source operating systems and kernels.

## Building Prestige
**Dependencies**

To build Prestige, you ***must*** have the following dependencies:
- `rust` (**latest nightly**)
- `nasm`
- `qemu` (if you'd like to run the OS in the QEMU emulator)
- `python` (3.6 or higher)


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

**Build System**

Prestige uses a custom build system that takes care of all the tedious tasks, like building the kernel and disk image, for you. The build system is written in Python and lives in the `prestige.py` file, at the root of the repository. You can view all the subcommands by running the script with the `--help` option.

*Subcommands:*

`build` - build the OS.

- *Default Behavior:*

    By default, the `build` subcommand will build the OS in release mode and run it in the QEMU emulator.

- *Customizations:*

    - `--target`: This option allows you to specify the target architecture to build the OS with. This defaults to `x86_64-prestige`. Ensure to maintain the format of `arch-prestige`.
    - `--features`: This option allows you to specify a **single comma-separated** list of crate features (view the `[features]` section in `Cargo.toml`).
    - `--firmware`: This options allows you to specify the firmware to boot the OS with. This defaults to `bios`.
    - `--keyboard`: This options allows you to specify the desired keyboard layout. This defaults to `qwerty`.
    - `--debug`: This flag allows you to build the OS in **debug** mode instead of the default **release** mode.
    - `--no-qemu`: This flag allows you to prevent running the disk image in QEMU.

`test` - run the test suite.