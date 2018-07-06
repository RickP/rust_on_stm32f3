# rust_on_stm32f3

This is the minimal project I could produce to get starting on embedded rust. Youll need a STM32F3DISCOVERY development board (https://www.digikey.com/product-detail/en/stmicroelectronics/STM32F3DISCOVERY/497-13192-ND)

## Install

This is for ubuntu linux, for other operating system you'll have to find a way to install the gcc-arm toolchain, openocd and git

    sudo apt install gcc-arm-none-eabi gdb-arm-none-eabi openocd git-core # Install dependencies
    echo "set auto-load safe-path /" >> ~/.gdbinit # Allow to load local gdbinit files
   
    curl https://sh.rustup.rs -sSf | sh # Install rustup
  
 log out and in once
 
    rustup default nightly # Switch to rust nightly
    rustup target add thumbv7em-none-eabihf # Intall the standard lib for arm
   
    git clone https://github.com/RickP/rust_on_stm32f3.git # Check out the repo
    cd rust_on_stm32f3 # Change to the project root
    cargo build # Compile to the project

## Run on the STM32F3 discovery board

Start the openocd server on one terminal:

    openocd -f interface/stlink-v2.cfg -f target/stm32f3x.cfg
   
Flash the binary via gdb on another terminal (in project root):

    cargo run
   
The code will be programmed to the board and the controller is halted. Enter 'c' to run the code.
