use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank1 {
    pub BTCR:               [u32; 8],
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank1E {
    pub BWTR:               [u32; 7],
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank2 {
    pub PCR2:               u32,
    pub SR2:                u32,
    pub PMEM2:              u32,
    pub PATT2:              u32,
    pub RESERVED0:          u32,
    pub ECCR2:              u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank3 {
    pub PCR3:               u32,
    pub SR3:                u32,
    pub PMEM3:              u32,
    pub PATT3:              u32,
    pub RESERVED0:          u32,
    pub ECCR3:              u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FMC_Bank4 {
    pub PCR4:               u32,
    pub SR4:                u32,
    pub PMEM4:              u32,
    pub PATT4:              u32,
    pub PIO4:               u32,
}

//register addresses

registers! {
    const FMC_BANK1:    FMC_Bank1       = FMC_BANK1_R_BASE,
}

registers! {
    const FMC_BANK1E:   FMC_Bank1E      = FMC_BANK1E_R_BASE,
}

registers! {
    const FMC_BANK2:    FMC_Bank2       = FMC_BANK2_R_BASE,
}

registers! {
    const FMC_BANK3:    FMC_Bank3       = FMC_BANK3_R_BASE,
}

registers! {
    const FMC_BANK4:    FMC_Bank4       = FMC_BANK4_R_BASE,
}

//custom

