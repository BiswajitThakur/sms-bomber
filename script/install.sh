#!/bin/env bash

function install {
  if [[ -f ./target/release/sms-bomber ]]; then
    if [[ -d /usr/bin/ ]]; then
      mv ./target/release/sms-bomber /usr/bin/ 2>/dev/null 
      if [[ $? -ne 0 ]]; then
        echo -e "\033[0;31mFaild to install (Administrator privileges required to install)\033[0m"
        exit 1
      fi
      echo -e "\033[0;32mInstall success\033[0m"
      exit 0
    elif [[ -d /data/data/com.termux/files/usr/bin/ ]]; then
      mv ./target/release/sms-bomber /data/data/com.termux/files/usr/bin/ 2>/dev/null
      if [[ $? -ne 0 ]]; then
          echo -e "\033[0;31mFaild to install\033[0m"
          exit 1
      fi
      echo  -e "\033[0;32mInstall success\033[0m"
      exit 0
    fi
  else
    echo "$(pwd)/target/release/sms-bomber   NOT FOUND"
    echo "Please rebuild, then try to install"
    exit 1
  fi
}

install;

