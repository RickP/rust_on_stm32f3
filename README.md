# rust_on_stm32f3

This is a minimal project to get starting on rust for microcontrollers. You'll need a STM32F3DISCOVERY development board (https://www.digikey.com/product-detail/en/stmicroelectronics/STM32F3DISCOVERY/497-13192-ND). It's completely based on the work of Jorge Aparicio (http://blog.japaric.io). Read this blog to learn more.

As some stuff described on Jorges blog is deprecated (e.g. the 'xargo' tool is no longer needed) I created something that works today as a starting point. It could break tomorrow.

## Install

This is for ubuntu linux, for other operating systems you'll have to find a way to install the gcc-arm toolchain, openocd and git

    sudo apt install gcc-arm-none-eabi gdb-arm-none-eabi openocd git-core # Install dependencies

    curl https://sh.rustup.rs -sSf | sh # Install rustup

 log out and in once

    rustup default nightly # Switch to rust nightly
    rustup target add thumbv7em-none-eabihf # Intall the standard lib for arm

    git clone https://github.com/RickP/rust_on_stm32f3.git # Check out this repo
    cd rust_on_stm32f3 # Change to the project root
    cargo build # Compile to the project

## Run on the STM32F3 discovery board

To flash the binary to an attached STM32F3 board:

    cargo run

This code flashes all leds and stops when you tilt the board.
