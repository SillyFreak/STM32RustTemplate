use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CAN_TxMailBox {
    pub TIR:                u32,
    pub TDTR:               u32,
    pub TDLR:               u32,
    pub TDHR:               u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CAN_FIFOMailBox {
    pub RIR:                u32,
    pub RDTR:               u32,
    pub RDLR:               u32,
    pub RDHR:               u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CAN_FilterRegister {
    pub FR1:                u32,
    pub FR2:                u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CAN {
    pub MCR:                u32,
    pub MSR:                u32,
    pub TSR:                u32,
    pub RF0R:               u32,
    pub RF1R:               u32,
    pub IER:                u32,
    pub ESR:                u32,
    pub BTR:                u32,
    pub RESERVED0:          [u32; 88],
    pub sTxMailBox:         [CAN_TxMailBox; 3],
    pub sFIFOMailBox:       [CAN_FIFOMailBox; 2],
    pub RESERVED1:          [u32; 12],
    pub FMR:                u32,
    pub FM1R:               u32,
    pub RESERVED2:          u32,
    pub FS1R:               u32,
    pub RESERVED3:          u32,
    pub FFA1R:              u32,
    pub RESERVED4:          u32,
    pub FA1R:               u32,
    pub RESERVED5:          [u32; 8],
    pub sFilterRegister:    [CAN_FilterRegister; 28],
}

//register addresses

registers! {
    const CAN1:         CAN             = CAN1_BASE,
}

//custom

