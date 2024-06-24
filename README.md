# WakaFarm
Simple and intuitive Wakatime farmer.

### Warning: read the [how to use](#start) before running.

## Table of Contents

1. [How to install](#install)
2. [How to use](#start)
2. [Adding another language](#language)
3. [Stacks and libraries](#stacks)

## <a name="install">How to install</a>
    
To run this need to have Rust installed.

<details>
<summary> Rust instalation </summary>

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After the instalation is done run

```sh
rustc -V
# rustc 1.xx.x
cargo -V
# cargo 1.xx.x
```
</details>

<details>
<summary> Libxdo </summary>

You may run into the error "cannot find -lxdo" (missing xdo library) when running the program if you just installed Rust.
To fix this you need to install libxdo-dev.

```sh
# mint:
sudo apt install libxdo-dev

# ubuntu/debian:
sudo apt-get update
sudo apt-get install libxdo-dev

# arch
sudo pacman -S xdotool
```
</details>

## <a name="start">How to use</a>

After the installation is done, run

```sh
cargo run
```
And type the extension of the language you wanna farm and open the file where you want the farmer to type.

```s
Select language you want to farm.
Type the extension only (eg. go, js, rs).

"rs" # <- your input

Found "../wakafarm/src/rs.rs"
The typing will start in 7 seconds.
The program will continue running until you press ctrl+c to stop.
```
To stop the program, hit ctrl+c on the console.

## <a name="language">Adding another language</a>

This program works by searching for a file with the name inputted and reading its content, then writing it over and over until you stop it.
In order to add another language you just need to add a file with the language's extension in the ```language``` folder and write the code you want to be read.

Note that the program reads ```_``` as ```\n```, so you should put _ on the end of the line you want to copy (and an empty line after if you want to write in the same file).

js.js:
```js
console.log("Hello world!");_

```
*Ps: it will read comments as well.*
I would recommend adding a simple code.

## <a name="stacks">Stacks and libraries</a>

- [Rust](https://www.rust-lang.org/learn)
- [Enigo](https://docs.rs/enigo/latest/enigo/)