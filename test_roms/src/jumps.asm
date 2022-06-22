INCLUDE "hardware.inc"

SECTION "Header", ROM0[$100]

	jp Entrypoint

	ds $150 - @, 0

Entrypoint:
	jp Entrypoint
