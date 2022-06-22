SRC_DIR := src
OBJ_DIR := build
ROM_DIR := roms

SRC_FILES := $(wildcard $(SRC_DIR)/*.asm)
OBJ_FILES := $(patsubst $(SRC_DIR)/%.asm,$(OBJ_DIR)/%.o,$(SRC_FILES))
ROM_FILES := $(patsubst $(SRC_DIR)/%.asm,$(ROM_DIR)/%.gb,$(SRC_FILES))

RGBASM_FLAGS := -L
RGBFIX_FLAGS := -v -p 0xFF

all: $(ROM_FILES)

$(ROM_DIR)/%.gb: $(OBJ_DIR)/%.o | $(ROM_DIR)
	rgblink -o $@ $<
	rgbfix $(RGBFIX_FLAGS) $@

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.asm $(SRC_DIR)/hardware.inc | $(OBJ_DIR)
	cd $(SRC_DIR); rgbasm $(RGBASM_FLAGS) -o ../$@ ../$<

$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

$(ROM_DIR):
	mkdir -p $(ROM_DIR)

clean:
	rm -rf $(OBJ_DIR) $(ROM_DIR)
