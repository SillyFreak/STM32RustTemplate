use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank1 {
    pub BTCR:               [u32; 8],                                   //NOR/PSRAM chip-select control register(BCR) and chip-select timing register(BTR), Address offset: 0x00-1C
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank1E {
    pub BWTR:               [u32; 7],                                   //NOR/PSRAM write timing registers, Address offset: 0x104-0x11C
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank2 {
    pub PCR2:               u32,                                        //NAND Flash control register 2,                       Address offset: 0x60
    pub SR2:                u32,                                        //NAND Flash FIFO status and interrupt register 2,     Address offset: 0x64
    pub PMEM2:              u32,                                        //NAND Flash Common memory space timing register 2,    Address offset: 0x68
    pub PATT2:              u32,                                        //NAND Flash Attribute memory space timing register 2, Address offset: 0x6C
    pub RESERVED0:          u32,                                        //Reserved, 0x70
    pub ECCR2:              u32,                                        //NAND Flash ECC result registers 2,                   Address offset: 0x74
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank3 {
    pub PCR3:               u32,                                        //NAND Flash control register 3,                       Address offset: 0x80
    pub SR3:                u32,                                        //NAND Flash FIFO status and interrupt register 3,     Address offset: 0x84
    pub PMEM3:              u32,                                        //NAND Flash Common memory space timing register 3,    Address offset: 0x88
    pub PATT3:              u32,                                        //NAND Flash Attribute memory space timing register 3, Address offset: 0x8C
    pub RESERVED0:          u32,                                        //Reserved, 0x90
    pub ECCR3:              u32,                                        //NAND Flash ECC result registers 3,                   Address offset: 0x94
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank4 {
    pub PCR4:               u32,                                        //PC Card  control register 4,                       Address offset: 0xA0
    pub SR4:                u32,                                        //PC Card  FIFO status and interrupt register 4,     Address offset: 0xA4
    pub PMEM4:              u32,                                        //PC Card  Common memory space timing register 4,    Address offset: 0xA8
    pub PATT4:              u32,                                        //PC Card  Attribute memory space timing register 4, Address offset: 0xAC
    pub PIO4:               u32,                                        //PC Card  I/O space timing register 4,              Address offset: 0xB0
}

//register addresses

registers! {
    const FMC_BANK1:        FMC_Bank1           = FMC_BANK1_R_BASE,
}

registers! {
    const FMC_BANK1E:       FMC_Bank1E          = FMC_BANK1E_R_BASE,
}

registers! {
    const FMC_BANK2:        FMC_Bank2           = FMC_BANK2_R_BASE,
}

registers! {
    const FMC_BANK3:        FMC_Bank3           = FMC_BANK3_R_BASE,
}

registers! {
    const FMC_BANK4:        FMC_Bank4           = FMC_BANK4_R_BASE,
}

//bit definitions

//custom

