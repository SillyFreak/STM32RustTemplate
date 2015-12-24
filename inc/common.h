/*
 * common.h
 *
 *  Created on: Nov 14, 2012
 *      Author: andrei
 */

#ifndef COMMON_H_
#define COMMON_H_

#include "stm32f30x.h"
#include <stdio.h>
#include <stdint.h>

#include "math.h"


extern void _delay_ms(__IO uint32_t ms);
extern void _delay_us(__IO uint32_t us);

#endif /* COMMON_H_ */
