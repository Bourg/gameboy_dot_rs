all: roms/hello-world.gb

roms/hello-world.gb: build/hello-world.o roms
	rgblink -o roms/hello-world.gb build/hello-world.o
	rgbfix -v -p 0xFF roms/hello-world.gb

build/hello-world.o: src/hello-world.asm src/hardware.inc build
	cd src; rgbasm -L -o ../build/hello-world.o hello-world.asm

build:
	mkdir -p build

roms:
	mkdir -p roms


clean:
	rm -rf build roms
