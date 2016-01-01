#include "stm32f30x.h"

// wrappers for static inline function, not present in any .o file.

uint32_t SysTick_Config_wrapper(uint32_t ticks) {
    return SysTick_Config(ticks);
}

void NVIC_EnableIRQ_wrapper(IRQn_Type IRQn) {
    NVIC_EnableIRQ(IRQn);
}

void NVIC_SetPriority_wrapper(IRQn_Type IRQn, uint32_t priority) {
    NVIC_SetPriority(IRQn, priority);
}

