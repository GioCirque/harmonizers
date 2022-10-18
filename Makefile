.SILENT:

TARGET ?= armv7-unknown-linux-gnueabihf
DEVICE_IP ?= '10.11.99.1'
DEVICE_HOST ?= root@$(DEVICE_IP)
BIN_NAME ?= harmonizers

all: test build
.PHONY: all

reboot:
	ssh $(DEVICE_HOST) '/sbin/reboot || true;'

run: deploy
	ssh $(DEVICE_HOST) 'killall -q -9 $(BIN_NAME) || true; systemctl stop xochitl || true'
	ssh $(DEVICE_HOST) './$(BIN_NAME)'

stop:
	ssh $(DEVICE_HOST) 'killall -q -9 $(BIN_NAME) || true; systemctl start xochitl'

build: deps
	cross build --release

deploy: build
	ssh $(DEVICE_HOST) 'killall -q -9 $(BIN_NAME) || true; systemctl stop xochitl || true'
	scp ./target/$(TARGET)/release/$(BIN_NAME) $(DEVICE_HOST):
	ssh $(DEVICE_HOST) 'RUST_BACKTRACE=1 RUST_LOG=debug ./$(BIN_NAME)'

deps:
	cargo install cross

test: deps
# Notice we aren't using the armv7 target here
	cross test
