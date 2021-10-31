# Default to a serial device name that is common in Linux.
DEVICE_NAME ?= /dev/ttyUSB0

# Default to the Raspberry Pi 4.
BOARD ?= rpi4

BIN_NAME = miniterm
INSTALL_PATH = /usr/local/bin

ifeq ($(BOARD),rpi3)
	BAUD_RATE = 921_600
else ifeq ($(BOARD),rpi4)
	BAUD_RATE = 921_600
endif


.PHONY: all clean install uninstall run

all:
	@cargo build --release

clean:
	@cargo clean

install: all
	sudo cp target/release/$(BIN_NAME) $(INSTALL_PATH)

uninstall:
	sudo rm $(INSTALL_PATH)/$(BIN_NAME)

run: install
	sudo $(BIN_NAME)