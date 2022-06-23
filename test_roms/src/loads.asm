INCLUDE "hardware.inc"

SECTION "Header", ROM0[$100]

	jp Entrypoint

	ds $150 - @, 0

Entrypoint:
    ld a, $1A
    ld b, $2B
    ld c, $3C
    ld d, $4D
    ld e, $5E
    ld h, $6F
    ld l, $7F

Loop:
    jp Loop
