#!/bin/sh

# This is needed because cargo's runner option can only append the binary name to a command

openocd -f interface/stlink-v2.cfg -f target/stm32f3x.cfg -c "program $1 verify reset exit"
