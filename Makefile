# For non-musl, use: armv7-unknown-linux-gnueabihf
TARGET ?= armv7-unknown-linux-musleabihf

DEVICE_IP ?= '10.11.99.1'
DEVICE_HOST ?= root@$(DEVICE_IP)

all: build

.PHONY: all

run: deploy
	ssh $(DEVICE_HOST) 'killall -q -9 harmony || true; systemctl stop xochitl || true'
	ssh $(DEVICE_HOST) './harmonizers'

build: test
	cargo build --release

deploy: build
	ssh $(DEVICE_HOST) 'killall -q -9 demo || true; systemctl stop xochitl || true'
	scp ./target/$(TARGET)/release/examples/demo $(DEVICE_HOST):
	ssh $(DEVICE_HOST) 'RUST_BACKTRACE=1 RUST_LOG=debug ./demo'

test:
	# Notice we aren't using the armv7 target here
	cargo test

start-xochitl:
	ssh $(DEVICE_HOST) 'killall -q -9 demo || true; systemctl start xochitl'
