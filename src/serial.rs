use stm32f30x;
use stm32f30x::gpio::*;
use stm32f30x::irq::*;
use stm32f30x::rcc::*;
use stm32f30x::usart::*;
use hardware::registers::*;

pub fn init(usart: &mut RegPtr<USART>,
             rx_port: &mut RegPtr<GPIO>, rx_pin: u8,
             tx_port: &mut RegPtr<GPIO>, tx_pin: u8) {
    let rcc = &mut *RCC;
    rcc.APB2RSTR |= APB2RSTR::USART1RST; //Usart1 Reset
    rcc.APB2RSTR &= !APB2RSTR::USART1RST;
    rcc.AHBENR |= AHBENR::GPIOAEN; //GPIOA Clock Enable
    rcc.APB2ENR |= APB2ENR::USART1EN; //USART1 Clock Enable
    rcc.CFGR3 |= CFGR3::USART1SW_0; //System clock (SYSCLK) selected as USART1 clock
    rcc.CFGR3 &= !CFGR3::USART1SW_1;

    let gpio = &mut *rx_port;
    let pin = rx_pin;
    gpio.set_ospeed(pin, OSPEEDR::_50MHZ);
    gpio.set_af(pin, 0x7);
    gpio.set_mode(pin, MODER::AF);

    let gpio = &mut *tx_port;
    let pin = tx_pin;
    gpio.set_ospeed(pin, OSPEEDR::_50MHZ);
    gpio.set_af(pin, 0x7);
    gpio.set_mode(pin, MODER::AF);

    let usart = &mut *usart;
    usart.BRR = unsafe { stm32f30x::SystemCoreClock / 115200 } as u16;
    usart.CR1 |= CR1::UE;
    usart.CR1 |= CR1::TE | CR1::RE | CR1::RXNEIE;
    nvic_enable_irq(IRQn::USART1);
    nvic_set_priority(IRQn::USART1, 0x0B);
}

