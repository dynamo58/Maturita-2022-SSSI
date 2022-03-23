#!/usr/bin/python

from typing import List, Union

import os
import shutil
from sys import argv, exit
from time import time
from subprocess import PIPE, DEVNULL, run

SRC_FOLDER = os.path.join(os.getcwd(), "code")
OUT_FOLDER = os.path.join(os.getcwd(), "dest")
OUT_NAME = "GameOfLife"

# vrátí spec. CLI flag
def flag(args: List[str], flag: str) -> Union[str, None]:
    try:    return args[args.index("-"+flag)+1]
    except: return None

# úspěšný výstup
def exit_success(time, os, arch) -> None:
    os = os if (os != None) else ""
    arch = arch if (arch != None) else ""
    
    if (not (os == "" and arch == "")):
        arch = ", " + arch

    exit("[BUILDER]\t Úspěšně dokončeno za {}s pro {}{}".format(time, os, arch))

# neúspěšný výstup
def exit_fail() -> None:
    exit("Parametry nebyly rozpoznány (nebo chybí), nebo nastala chyba")

# export pro gnu/linux
def build_linux(arch) -> bool:
    supported_archs = [
        "aarch64-linux-android",
        "aarch64-unknown-linux-gnu",
        "aarch64-unknown-linux-gnu_ilp32",
        "aarch64-unknown-linux-musl",
        "aarch64_be-unknown-linux-gnu",
        "aarch64_be-unknown-linux-gnu_ilp32",
        "arm-linux-androideabi",
        "arm-unknown-linux-gnueabi",
        "arm-unknown-linux-gnueabihf",
        "arm-unknown-linux-musleabi",
        "arm-unknown-linux-musleabihf",
        "armv4t-unknown-linux-gnueabi",
        "armv5te-unknown-linux-gnueabi",
        "armv5te-unknown-linux-musleabi",
        "armv5te-unknown-linux-uclibceabi",
        "armv7-linux-androideabi",
        "armv7-unknown-linux-gnueabi",
        "armv7-unknown-linux-gnueabihf",
        "armv7-unknown-linux-musleabi",
        "armv7-unknown-linux-musleabihf",
        "armv7-unknown-linux-uclibceabihf",
        "hexagon-unknown-linux-musl",
        "i586-unknown-linux-gnu",
        "i586-unknown-linux-musl",
        "i686-linux-android",
        "i686-unknown-linux-gnu",
        "i686-unknown-linux-musl",
        "m68k-unknown-linux-gnu",
        "mips-unknown-linux-gnu",
        "mips-unknown-linux-musl",
        "mips-unknown-linux-uclibc",
        "mips64-unknown-linux-gnuabi64",
        "mips64-unknown-linux-muslabi64",
        "mips64el-unknown-linux-gnuabi64",
        "mips64el-unknown-linux-muslabi64",
        "mipsel-unknown-linux-gnu",
        "mipsel-unknown-linux-musl",
        "mipsel-unknown-linux-uclibc",
        "mipsisa32r6-unknown-linux-gnu",
        "mipsisa32r6el-unknown-linux-gnu",
        "mipsisa64r6-unknown-linux-gnuabi64",
        "mipsisa64r6el-unknown-linux-gnuabi64",
        "powerpc-unknown-linux-gnu",
        "powerpc-unknown-linux-gnuspe",
        "powerpc-unknown-linux-musl",
        "powerpc64-unknown-linux-gnu",
        "powerpc64-unknown-linux-musl",
        "powerpc64le-unknown-linux-gnu",
        "powerpc64le-unknown-linux-musl",
        "riscv32gc-unknown-linux-gnu",
        "riscv32gc-unknown-linux-musl",
        "riscv64gc-unknown-linux-gnu",
        "riscv64gc-unknown-linux-musl",
        "s390x-unknown-linux-gnu",
        "s390x-unknown-linux-musl",
        "sparc-unknown-linux-gnu",
        "sparc64-unknown-linux-gnu",
        "thumbv7neon-linux-androideabi",
        "thumbv7neon-unknown-linux-gnueabihf",
        "thumbv7neon-unknown-linux-musleabihf",
        "x86_64-linux-android",
        "x86_64-unknown-linux-gnu",
        "x86_64-unknown-linux-gnux32",
        "x86_64-unknown-linux-musl",
        "x86_64-unknown-none-linuxkernel"
    ]

    if arch not in supported_archs:
        arch = "x86_64-unknown-linux-gnu"

    for file in os.listdir(OUT_FOLDER):
        if file.endswith(arch+".AppImage"):
            os.remove(os.path.join(OUT_FOLDER, file))

    if os.path.isdir("temp.AppDir"):
        shutil.rmtree("temp.AppDir")

    run(["cargo", "build", "--release", "--target", arch], stdout=DEVNULL, stderr=PIPE, universal_newlines=True)

    os.mkdir("temp.AppDir")
    shutil.copytree("resources", "temp.AppDir/resources")
    shutil.copy("target/{}/release/gol".format(arch), "temp.AppDir/AppRun")
    shutil.move("temp.AppDir/resources/logo.png", "temp.AppDir/logo.png")

    GoLmanifest = [
        "[Desktop Entry]",
        "Name={}".format(OUT_NAME),
        "Exec=gol",
        "Icon=logo",
        "Type=Application",
        "Categories=Game;"
    ]

    GoLdesktop = open("temp.AppDir/GoL.desktop", "w")
    for row in GoLmanifest:
        GoLdesktop.write(row + "\n")
    GoLdesktop.close()

    result = run(["appimagetool", "temp.AppDir"], stdout=PIPE, stderr=PIPE, universal_newlines=True)

    if os.path.isdir("temp.AppDir"):
        shutil.rmtree("temp.AppDir")

    if result.stdout.split("\n")[-2].endswith("AppImage"):
        shutil.move(OUT_NAME+"-"+arch.split("-")[0]+".AppImage", os.path.join(OUT_FOLDER, OUT_NAME + "__" + arch + ".AppImage"))

        return True
    
    return False

# export pro windows
def build_win(arch) -> bool:
    supported_archs = [
        "aarch64-pc-windows-msvc",
        "aarch64-uwp-windows-msvc",
        "i586-pc-windows-msvc",
        "i686-pc-windows-gnu",
        "i686-pc-windows-msvc",
        "i686-uwp-windows-gnu",
        "i686-uwp-windows-msvc",
        "thumbv7a-pc-windows-msvc",
        "thumbv7a-uwp-windows-msvc",
        "x86_64-pc-windows-gnu",
        "x86_64-pc-windows-msvc",
        "x86_64-uwp-windows-gnu",
        "x86_64-uwp-windows-msvc"
    ]

    if arch not in supported_archs:
        arch = "x86_64-pc-windows-gnu"

    for file in os.listdir(OUT_FOLDER):
        if file.endswith(arch+".exe"):
            os.remove(os.path.join(OUT_FOLDER, file))

    if os.path.isdir("temp_win"):
        shutil.rmtree("temp_win")

    run(["cargo", "build", "--release", "--target", arch], stdout=DEVNULL, stderr=PIPE, universal_newlines=True)

    os.mkdir("temp_win")
    shutil.copytree("resources", "temp_win/resources")
    shutil.copy("target/{}/release/gol.exe".format(arch), "temp_win/GameOfLife.exe")
    shutil.move("temp_win/resources/logo.ico", "temp_win/logo.ico")
    os.chdir("temp_win")
    run(["wine", "~/bin/rcedit.exe", "GameOfLife.exe", "--set-icon", "\"logo.ico\""], stdout=DEVNULL, stderr=PIPE, universal_newlines=True)
    shutil.move("GameOfLife.exe", os.path.join(OUT_FOLDER, OUT_NAME + "__" + arch + ".exe"))
    os.chdir("..")
    shutil.rmtree("temp_win")


    return True

# export pro WebAssembly
def build_wasm() -> bool:
    run(["cargo", "build", "--release", "--lib", "-p", "gol", "--target", "wasm32-unknown-unknown"], stdout=DEVNULL, stderr=PIPE, universal_newlines=True)

    run(["wasm-bindgen", "target/wasm32-unknown-unknown/release/gol.wasm", "--out-dir", os.path.join(OUT_FOLDER, "web"), "--no-modules", "--no-typescript"], stdout=DEVNULL, stderr=PIPE, universal_newlines=True)

    inp = input("Spustit server? [a/n] ")

    if inp == "a" or inp == "ano" or inp == "":
        os.system("basic-http-server --addr 127.0.0.1:3000 {}".format(os.path.join(OUT_FOLDER, "web")))

    return True


def main(args) -> None:
    TARGET_OS   = flag(args, "os")
    TARGET_ARCH = flag(args, "arch")
    success = False

    if (TARGET_OS == None and TARGET_ARCH == None):
        exit_fail()

    timestamp = time()

    os.chdir(SRC_FOLDER)

    if (TARGET_OS == "linux"):
        success = build_linux(TARGET_ARCH)

    if (TARGET_OS  == "windows" or TARGET_OS == "win"):
        success = build_win(TARGET_ARCH)

    if (TARGET_ARCH == "wasm"):
        success = build_wasm()

    if success:
        time_elapsed = round(time() - timestamp, 4)
        exit_success(time_elapsed, TARGET_OS, TARGET_ARCH)
    
    exit_fail()

if __name__ == "__main__":
    main(argv)