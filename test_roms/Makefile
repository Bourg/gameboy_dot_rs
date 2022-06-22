all: roms/hello-world.gb

roms/%.gb: build/%.o roms
	rgblink -o $@ $<
	rgbfix -v -p 0xFF $@

build/%.o: src/%.asm src/hardware.inc build
	cd src; rgbasm -L -o ../$@ ../$<

build:
	mkdir -p build

roms:
	mkdir -p roms


clean:
	rm -rf build roms
