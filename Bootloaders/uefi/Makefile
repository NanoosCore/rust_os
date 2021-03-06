
-include ../../common.mk

BIN := ../bootloader-efi.pe
TARGET_FILE := target.json
RUSTFLAGS := --target=$(TARGET_FILE) --out-dir=.obj/ -Z no-landing-pads -L .obj/ -C opt-level=s
#RUSTFLAGS += -C lto

OBJS := main.o core.o uefi.o stubs.o
OBJS := $(addprefix .obj/,$(OBJS))

all: $(BIN)

.obj/bootloader-efi.o: $(OBJS)
	x86_64-efi-pe-ld -g -r -o $@ -e efi_main $(OBJS) --strip-discarded --gc-sections --allow-multiple-definition

$(BIN): .obj/bootloader-efi.o
	x86_64-efi-pe-ld -dll -shared --subsystem 10 -e efi_main -o $@ .obj/bootloader-efi.o -Map map.txt --gc-sections 2>&1 | (grep -v 'has no section'; true)
	x86_64-efi-pe-objdump -d $@ > $@.dsm
	x86_64-efi-pe-strip $@

.obj/uefi.o .obj/libuefi.rlib: ../libuefi/src/lib.rs $(TARGET_FILE) .obj/libcore.rlib
	@mkdir -p .obj/
	$(RUSTC) $< --emit=link,obj,dep-info $(RUSTFLAGS)
.obj/stubs.o: stubs.S
	@mkdir -p .obj/
	x86_64-efi-pe-as $< -o $@

.obj/core.o .obj/libcore.rlib: ../../libcore/lib.rs $(TARGET_FILE)
	@mkdir -p .obj/
	$(RUSTC) $< --emit=link,obj,dep-info $(RUSTFLAGS) --cfg disable_float

.obj/main.o: main.rs .obj/libcore.rlib .obj/libuefi.rlib $(TARGET_FILE)
	mkdir -p .obj/
	$(RUSTC) --emit=obj,dep-info,asm $< $(RUSTFLAGS)

-include .obj/main.d
-include .obj/core.d
-include .obj/uefi.d

