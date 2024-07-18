#!/bin/env bash

function setup {
    if [ ! -x "$(command -v cargo)" ]; then
        echo -e "\033[0;31mERROR\033[0m: Rust does not seem to be installed."
        read -p "Do you want to install Rust (y/n): " rs
        if [[ $rs = "y" ]]; then
          if [[ ! -z "$(echo $PREFIX | grep 'com.termux')" ]]; then
              pkg update;
              pkg install rust -y;
          else
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
          fi
        else
            exit 1
        fi
    fi

    if [ ! -x "$(command -v cargo)" ]; then
        exit 1
    fi
}

function build {
    cargo build --release
    if [[ $? -ne 0 ]]; then
      echo "Faild to build"
      exit 1
    fi
    echo "Build success..."
}
setup;
build;

