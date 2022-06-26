INCLUDE "hardware.inc"

SECTION "Header", ROM0[$100]

	jp Entrypoint

	ds $150 - @, 0

Entrypoint:
    ld a, 123
    ld [_RAM], a
    ld a, $FF
    ld a, [_RAM]
    ld e, a

Loop:
	jp Loop
