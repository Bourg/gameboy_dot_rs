# Test ROMs

There are two types of test ROMs for this project:
- Static
- Built

The static test ROMs were written with a magnetized needle and have no associated source files.

The built test ROMs have actual RGBDS source files in the `src/` directory.
Their associated built roms are in the `roms/` directory.

To build new test ROMs or update existing ones, simply run `make`.
You will need to have `rgbasm`, `rgblink`, and `rgbfix` on your path for the `Makefile` to work.
See [the RGBDS homepage](https://rgbds.gbdev.io/) for instructions on installing RGBDS.
