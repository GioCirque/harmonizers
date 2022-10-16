#!/bin/sh

VER_RM="rm11x"
VER_TOOL="3.1.15"
VER_CORTEX="a7hf"
TARGET_ARCH="x86_64"
BUILD_TARGET="armv7-unknown-linux-gnueabihf"
TARGET_INSTALLER="codex-$TARGET_ARCH-cortex$VER_CORTEX-neon-$VER_RM-toolchain-$VER_TOOL.sh"

if [ ! -f "$TARGET_INSTALLER" ]; then
  sudo wget "https://storage.googleapis.com/remarkable-codex-toolchain/$TARGET_INSTALLER"
  chmod +x "$TARGET_INSTALLER"
fi

"./$TARGET_INSTALLER" -y
rm -f $TARGET_INSTALLER

rustup target add $BUILD_TARGET

for env_setup_script in `ls /opt/codex/$VER_RM/$VER_TOOL/environment-setup-*`; do
  set ENVIRONMENT_SOURCE=$env_setup_script
  sudo chmod +x $env_setup_script;
  . $env_setup_script
  echo "
# Source the libremarkable environment
source $env_setup_script
" >>~/.profile
  echo "
# Source the libremarkable environment
source $env_setup_script
" >>~/.zshrc
done

sudo wget https://raw.githubusercontent.com/canselcik/libremarkable/master/gen_cargo_config.py
sudo chmod +x ./gen_cargo_config.py

if [ ! -f ".cargo/config" ]; then
  mkdir -p .cargo
  touch .cargo/config
fi

. $ENVIRONMENT_SOURCE && ./gen_cargo_config.py && rm -f ./gen_cargo_config.py

echo "
[build]
# Set the default --target flag
target = \"armv7-unknown-linux-gnueabihf\"" >>.cargo/config

if [ ! -f "Makefile" ]; then
  wget https://raw.githubusercontent.com/canselcik/libremarkable/master/Makefile -O Makefile
fi
