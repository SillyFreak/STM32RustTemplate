use stm32f30x;
use stm32f30x::gpio::*;
use stm32f30x::irq::*;
use stm32f30x::rcc::*;
use stm32f30x::usart::*;
use hardware::registers::*;

use ringbuffer::{self, Ringbuffer};

pub struct Serial {
    usart: RegPtr<USART>,
    rx_port: RegPtr<GPIO>,
    tx_port: RegPtr<GPIO>,
    rx_pin: u8,
    tx_pin: u8,
    pub rx: Ringbuffer<u8>,
    pub tx: Ringbuffer<u8>,
}

impl Serial {
    pub const fn new(usart: RegPtr<USART>,
                     rx_port: RegPtr<GPIO>, rx_pin: u8,
                     tx_port: RegPtr<GPIO>, tx_pin: u8) -> Self {
        Serial {
            usart: usart,
            rx_port: rx_port,
            tx_port: tx_port,
            rx_pin: rx_pin,
            tx_pin: tx_pin,
            rx: Ringbuffer::new([0; ringbuffer::SIZE]),
            tx: Ringbuffer::new([0; ringbuffer::SIZE]),
        }
    }

    pub fn init(&mut self) {
        let rcc = &mut *RCC;
        rcc.APB2RSTR |= APB2RSTR::USART1RST; //Usart1 Reset
        rcc.APB2RSTR &= !APB2RSTR::USART1RST;
        rcc.AHBENR |= AHBENR::GPIOAEN; //GPIOA Clock Enable
        rcc.APB2ENR |= APB2ENR::USART1EN; //USART1 Clock Enable
        rcc.CFGR3 |= CFGR3::USART1SW_0; //System clock (SYSCLK) selected as USART1 clock
        rcc.CFGR3 &= !CFGR3::USART1SW_1;

        let gpio = &mut *self.rx_port;
        let pin = self.rx_pin;
        gpio.set_ospeed(pin, OSPEEDR::_50MHZ);
        gpio.set_af(pin, 0x7);
        gpio.set_mode(pin, MODER::AF);

        let gpio = &mut *self.tx_port;
        let pin = self.tx_pin;
        gpio.set_ospeed(pin, OSPEEDR::_50MHZ);
        gpio.set_af(pin, 0x7);
        gpio.set_mode(pin, MODER::AF);

        let usart = &mut *self.usart;
        usart.BRR = unsafe { stm32f30x::SystemCoreClock / 115200 } as u16;
        usart.CR1 |= CR1::UE;
        usart.CR1 |= CR1::TE | CR1::RE | CR1::RXNEIE;
        nvic_enable_irq(IRQn::USART1);
        nvic_set_priority(IRQn::USART1, 0x0B);
    }

    pub fn check(&mut self) {
        if self.tx.filled_slots() > 0 {
            self.usart.CR1 |= CR1::TCIE;
        }
    }
    
    pub fn handler(&mut self) {
        if (self.usart.ISR & ISR::RXNE) != 0 {
            //always read RDR to clear RXNE
            let data = self.usart.RDR as u8;
            if self.rx.free_slots() > 0 {
                self.rx.push(data);
            }
        } else if (self.usart.ISR & ISR::TXE) != 0 {
            if self.tx.filled_slots() == 0 {
                self.usart.CR1 &= !CR1::TCIE;
            } else {
                self.usart.TDR = self.tx.pop() as u16;
            }
        }
    }
}

