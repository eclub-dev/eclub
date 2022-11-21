#!/usr/bin/env bash

set -e

init_depenencies() {
  echo "------------Set up rust development environment--------------"
  rustup -v
  if [ $? -ne 0 ]; then
    curl https://sh.rustup.rs -sSf | sh
    if [ $? -ne 0 ]; then
      echo "set rustup development  failed"
      exit 1
    fi
  fi

  echo "------------Set up docker development environment--------------"
  docker -v
  if [ $? -ne 0 ]; then
    curl -fsSL https://get.docker.com | bash -s docker
    if [ $? -ne 0 ]; then
      echo "Install docker failed"
      exit 1
    fi
  fi

}

init_depenencies
