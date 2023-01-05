#!/usr/bin/env python3

import json
import logging
import shutil
import sys
from typing import List, Optional, Tuple
import subprocess
from pathlib import Path
import os

try:
    import typer
except ImportError:
    logging.error(
        "Please install the required libraries using the following command:\n - python -m pip install typer"
    )

    sys.exit(0)

os.environ["KEYBOARD_LAYOUT"] = "qwerty"

cli = typer.Typer()

LIMINE_TEMPLATE = """
TIMEOUT=3
VERBOSE=yes

: Prestige
PROTOCOL=limine
KASLR=no
KERNEL_PATH=boot:///prestige.elf
"""
LIMINE_GIT_URL = "https://github.com/limine-bootloader/limine.git"
LIMINE_DIR = Path("limine")


def exec_command(args, **kwargs) -> Tuple[int, str, str]:
    """Conveinent wrapper around executing a command with the `subprocess` module."""
    output = subprocess.run(args, **kwargs)

    return output.returncode, output.stdout, output.stderr


def ensure_latest_limine(limine_repo_path: Path) -> None:
    """Ensure that the version of the Limine bootloader is up to date."""
    exec_command(f"cd {limine_repo_path}; git fetch; make; cd -", shell=True)


def determine_cargo_args(*args) -> List[str]:
    """Determine the arguments to pass to Cargo based on the arguments passed to the subcommands."""
    target, features, debug = args

    cargo_args = [
        "--bin",
        "prestige",
        "--no-default-features",
        "--target",
        f".cargo/targets/{target}.json",
    ]

    if not debug:
        cargo_args.append("--release")

    if features:
        cargo_args.extend(["--features", ",".join(features)])

    return cargo_args


def extract_executable(cmd: str, args: List[str]) -> Optional[List[str]]:
    code, _, _ = exec_command(["cargo", cmd, *args])

    if code != 0:
        return None

    _, stdout, _ = exec_command(
        ["cargo", cmd, *args, "--message-format=json"],
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    )

    res = []
    lines = stdout.splitlines()

    for line in lines:
        data = json.loads(line)
        executable = data["executable"] if "executable" in data else None

        if executable:
            res.append(data["executable"])

    return res


def generate_iso(kernel_bin: str, limine_repo_path: Path) -> Optional[Path]:
    """Generate the ISO file."""
    iso_root = LIMINE_DIR.joinpath("iso_root")
    iso_path = LIMINE_DIR.joinpath("prestige.iso")

    if iso_path.exists():
        shutil.rmtree(iso_root)

    Path.mkdir(iso_root, parents=True)

    shutil.copy(kernel_bin, iso_root.joinpath("prestige.elf"))
    shutil.copy(limine_repo_path.joinpath("limine.sys"), iso_root)
    shutil.copy(limine_repo_path.joinpath("limine-cd.bin"), iso_root)
    shutil.copy(limine_repo_path.joinpath("limine-cd-efi.bin"), iso_root)

    efi_boot = iso_root.joinpath("EFI", "BOOT")
    Path.mkdir(efi_boot, parents=True)

    shutil.copy(limine_repo_path.joinpath("BOOTX64.EFI"), efi_boot)
    shutil.copy(limine_repo_path.joinpath("BOOTAA64.EFI"), efi_boot)

    with open(iso_root.joinpath("limine.cfg"), "w") as limine_cfg:
        limine_cfg.write(LIMINE_TEMPLATE)

    code, _, stderr = exec_command(
        [
            "xorriso",
            "-as",
            "mkisofs",
            "-b",
            "limine-cd.bin",
            "-no-emul-boot",
            "-boot-load-size",
            "4",
            "-boot-info-table",
            "--efi-boot",
            "limine-cd-efi.bin",
            "-efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            iso_root,
            "-o",
            iso_path,
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    if code != 0:
        logging.error("Failed to create ISO image.")
        logging.error(stderr.decode("utf-8"))

        return None

    limine_deploy = limine_repo_path.joinpath("limine-deploy")

    code, _, stderr = exec_command(
        [limine_deploy, iso_path], stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )

    if code != 0:
        logging.error("Failed to run `limine-deploy` on the ISO image.")
        logging.error(stderr)

        return None

    return iso_path


def run_in_qemu(*cmd_args, additional_qemu_args: List[str], iso_path: str) -> None:
    """Run in QEMU."""
    qemu_args = [
        "-no-reboot",
        "-no-shutdown",
        "-machine",
        "q35",
        "-M",
        "smm=off",
        "-serial",
        "stdio",
        "-drive",
        f"file={iso_path},format=raw",
    ]

    target_arch, _, debug = cmd_args
    qemu_bin = f"qemu-system-{target_arch}"

    if debug:
        qemu_args.extend(["-s", "-S"])

    if target_arch == "x86_64":
        qemu_args.extend(["-cpu", "qemu64"])
    elif target_arch == "aarch64":
        qemu_args.extend(["-device", "ramfb", "-M", "virt", "-cpu", "cortex-a72"])
    else:
        logging.error(f"The {target_arch} architecture is not supported.")
        exit(1)

    if additional_qemu_args:
        qemu_args.extend(additional_qemu_args)

    exec_command([qemu_bin, *qemu_args])


@cli.command()
def build(
    target: str = typer.Option(
        "x86_64-prestige",
        help="Specify the target architecture to build the OS with.",
        rich_help_panel="Customizations",
    ),
    features: List[str] = typer.Option(
        [],
        help="Specify a **single comma-separated** list of crate features.",
        rich_help_panel="Customizations",
    ),
    firmware: str = typer.Option(
        "bios",
        help="Specify the firmware to boot the OS with.",
        rich_help_panel="Customizations",
    ),
    keyboard: str = typer.Option(
        "qwerty",
        help="Specify the desired keyboard layout.",
        rich_help_panel="Customizations",
    ),
    debug: bool = typer.Option(
        False,
        "--debug",
        is_flag=True,
        help="Build the OS in debug mode instead of the default release mode.",
        rich_help_panel="Customizations",
    ),
    no_qemu: bool = typer.Option(
        False,
        "--no-qemu",
        is_flag=True,
        help="Prevent running the disk image in QEMU.",
        rich_help_panel="Customizations",
    ),
) -> None:
    """Build the operating system."""
    target_arch = target.split("-")[0]

    if target_arch == "aarch64":
        logging.error("aarch64 is currently not supported.")
        return

    # if target_arch == "aarch64" and not firmware == "uefi":
    #     logging.error(
    #         "aarch64 must be booted with the UEFI firmware (help: run again with `--firmware uefi`)"
    #     )
    #     return

    if keyboard not in ("qwerty", "dvorak", "azerty"):
        logging.error(f"The {keyboard} keyboard layout is not supported.")
        return

    os.environ["KEYBOARD_LAYOUT"] = keyboard

    cargo_cmd = "build"
    cargo_args = determine_cargo_args(target, features, debug)

    limine_repo_path = LIMINE_DIR.joinpath("limine")

    if not limine_repo_path.exists():
        exec_command(
            [
                "git",
                "clone",
                "--depth",
                "1",
                "--branch",
                "v4.x-branch-binary",
                LIMINE_GIT_URL,
                limine_repo_path,
            ]
        )

    ensure_latest_limine(limine_repo_path)

    kernel_bin = extract_executable(cargo_cmd, cargo_args)[0]
    iso_path = generate_iso(kernel_bin, limine_repo_path)

    if not no_qemu:
        run_in_qemu(
            target_arch, firmware, debug, additional_qemu_args=[], iso_path=iso_path
        )


@cli.command()
def test(
    target: str = typer.Option(
        "x86_64-prestige",
        help="Specify the target architecture to build the OS with.",
        rich_help_panel="Customizations",
    ),
    features: List[str] = typer.Option(
        [],
        help="Specify a **single comma-separated** list of crate features.",
        rich_help_panel="Customizations",
    ),
    firmware: str = typer.Option(
        "bios",
        help="Specify the firmware to boot the OS with.",
        rich_help_panel="Customizations",
    ),
    debug: bool = typer.Option(
        False,
        "--debug",
        is_flag=True,
        help="Build the OS in debug mode instead of the default release mode.",
        rich_help_panel="Customizations",
    ),
) -> None:
    """Run the test suite."""
    target_arch = target.split("-")[0]

    if target_arch == "aarch64":
        logging.error("aarch64 is currently not supported.")
        return

    # if target_arch == "aarch64" and not firmware == "uefi":
    #     logging.error(
    #         "aarch64 must be booted with the UEFI firmware (help: run again with `--firmware uefi`)"
    #     )
    #     return

    cargo_cmd = "test"
    cargo_args = determine_cargo_args(target, features, debug)

    limine_repo_path = LIMINE_DIR.joinpath("limine")

    if not limine_repo_path.exists():
        exec_command(
            [
                "git",
                "clone",
                "--depth",
                "1",
                "--branch",
                "v4.x-branch-binary",
                LIMINE_GIT_URL,
                limine_repo_path,
            ]
        )

    ensure_latest_limine(limine_repo_path)

    kernel_bin = extract_executable(cargo_cmd, cargo_args)[0]
    iso_path = generate_iso(kernel_bin, limine_repo_path)

    run_in_qemu(
        target_arch,
        firmware,
        debug,
        additional_qemu_args=[
            "-display",
            "none",
            "-device",
            "isa-debug-exit,iobase=0xf4,iosize=0x04",
        ],
        iso_path=iso_path,
    )


if __name__ == "__main__":
    try:
        cli()
    except KeyboardInterrupt:
        pass
