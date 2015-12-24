#include "common.h"

// SysTick_Config is a static inline function,
// and thus not present in any .o file.
uint32_t SysTick_Config_wrapper(uint32_t ticks) {
    return SysTick_Config(ticks);
}

