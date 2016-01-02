PROJ_NAME = stm32_rust_project

# Set toolchain
TC = arm-none-eabi

# Set Tools
CC      = $(TC)-gcc
AR      = $(TC)-ar
OBJCOPY = $(TC)-objcopy
OBJDUMP = $(TC)-objdump
SIZE    = $(TC)-size
NIGHTLY_RUST = multirust run nightly
RUSTC   = $(NIGHTLY_RUST) rustc
CARGO   = $(NIGHTLY_RUST) cargo

# Set Device specific paths and names
STM32_LIB_URL = http://www.st.com/st-web-ui/static/active/en/st_prod_software_internet/resource/technical/software/firmware/stm32f30x_dsp_stdperiph_lib.zip
STM32_DEVICE_NAME = STM32F30X

STM32 = STM32F30x_DSP_StdPeriph_Lib_V1.2.3
STM32_CMSIS = $(STM32)/Libraries/CMSIS
STM32_CMSIS_DEVICE = $(STM32_CMSIS)/Device/ST/STM32F30x
STM32_CPAL_DRIVER = $(STM32)/Libraries/STM32F30x_I2C_CPAL_Driver
STM32_PERIPH_DRIVER = $(STM32)/Libraries/STM32F30x_StdPeriph_Driver
STM32_TEMPLATE = $(STM32)/Projects/STM32F30x_StdPeriph_Templates
STM32_LINKER_SCRIPT = $(STM32_TEMPLATE)/TrueSTUDIO/STM32F303xC/STM32F303VC_FLASH.ld

# Set Sources
LIB_C_SRCS = $(wildcard $(STM32_PERIPH_DRIVER)/src/*.c)
LIB_C_SRCS+= $(wildcard $(STM32_CMSIS)/Source/Templates/*.c)
LIB_S_SRCS = $(STM32_CMSIS_DEVICE)/Source/Templates/TrueSTUDIO/startup_stm32f30x.s
C_SRCS = $(wildcard src/*.c)
RS_SRCS = $(wildcard src/*.rs) $(wildcard src/*/*.rs)

# Set Objects
LIB_OBJS = $(LIB_C_SRCS:.c=.o) $(LIB_S_SRCS:.s=.o)
OBJS     = $(C_SRCS:.c=.o) target/debug/main.o

# Set Include Paths
INCLUDES = -I$(STM32_PERIPH_DRIVER)/inc/ \
           -I$(STM32_CMSIS)/Include/ \
           -I$(STM32_CMSIS_DEVICE)/Include/ \
           -Iinc/
			
# Set Libraries
LIBS = -lm -lc


# Set Board
MCU     = -mthumb -mcpu=cortex-m4
FPU     = -mfpu=fpv4-sp-d16 -mfloat-abi=softfp
DEFINES = -D$(STM32_DEVICE_NAME) -DUSE_STDPERIPH_DRIVER

# Set Compilation and Linking Flags
CFLAGS  = $(MCU) $(FPU) $(DEFINES) $(INCLUDES) \
          -g -Wall -std=gnu90 -O0 -ffunction-sections -fdata-sections
ASFLAGS = $(MCU) $(FPU) -g -Wa,--warn -x assembler-with-cpp
LDFLAGS = $(MCU) $(FPU) -g -gdwarf-2\
          -T$(STM32_LINKER_SCRIPT) \
          -Xlinker --gc-sections -Wl,-Map=$(PROJ_NAME).map \
          $(LIBS) \
          -o $(PROJ_NAME).elf

RUSTFLAGS  = -C opt-level=2 -Z no-landing-pads \
             --target thumbv7em-none-eabi
CARGOFLAGS = -- $(RUSTFLAGS) \
             -g --emit obj \
             -L thumbv7m \
             -L $(shell multirust run nightly sh -c 'echo `dirname $$CARGO_HOME`/lib/rustlib/*/lib')

.PHONY: all download-stm unpack-stm download-rust build-rust template info flash remote-flash clean

# Default target

all: $(PROJ_NAME).bin info

# Setup targets

download-stm:
	rm -f stdperiph_lib.zip
	curl -o stdperiph_lib.zip $(STM32_LIB_URL)

unpack-stm:
	rm -rf $(STM32)
	unzip stdperiph_lib.zip

download-rust:
	test -d rust || git clone git@github.com:rust-lang/rust.git
	cd rust && git pull

build-rust:
	rm -rf thumbv7m
	cd rust && git checkout $(shell multirust run nightly rustc -v --version | grep commit-hash: | sed 's/commit-hash: //')
	mkdir thumbv7m
	$(RUSTC) $(RUSTFLAGS) -g rust/src/libcore/lib.rs --out-dir thumbv7m
	$(RUSTC) $(RUSTFLAGS) -g rust/src/librustc_bitflags/lib.rs --out-dir thumbv7m -L thumbv7m/

template:
	mkdir src/ inc/
	cp -f $(STM32_TEMPLATE)/*.c src/
	cp -f $(STM32_TEMPLATE)/*.h inc/
	@echo "(main.c, main.h are not used)"
	rm src/main.c
	rm inc/main.h

# Build

%.o: %.c
	@$(CC) $(CFLAGS) -c -o $@ $<
	@echo $@

%.o: %.s
	@$(CC) $(ASFLAGS) -c -o $@ $<
	@echo $@

target/debug/main.o: $(RS_SRCS)
	@$(CARGO) rustc --lib $(CARGOFLAGS)
	@echo $@

$(PROJ_NAME).elf: $(LIB_OBJS) $(OBJS)
	@$(CC) $(LIB_OBJS) $(OBJS) $(LDFLAGS)
	@echo $@

$(PROJ_NAME).bin: $(PROJ_NAME).elf
	@$(OBJCOPY) -O binary $(PROJ_NAME).elf $(PROJ_NAME).bin
	@echo $@

info: $(PROJ_NAME).elf
	@$(SIZE) --format=berkeley $(PROJ_NAME).elf

# Deployment

flash:
	stm32flasher $(PROJ_NAME).bin

remote-flash:
	rsync -avz $(PROJ_NAME).bin orangepi@10.42.0.226:/tmp/
	ssh orangepi@10.42.0.226 stm32flasher /tmp/$(PROJ_NAME).bin

# Cleanup

clean:
	rm -f $(LIB_OBJS)
	rm -f $(OBJS)
	rm -f $(PROJ_NAME).elf
	rm -f $(PROJ_NAME).bin
	rm -f $(PROJ_NAME).map
	$(CARGO) clean

