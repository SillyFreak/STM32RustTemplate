use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct RTC {
    pub TR:                 u32,
    pub DR:                 u32,
    pub CR:                 u32,
    pub ISR:                u32,
    pub PRER:               u32,
    pub WUTR:               u32,
    pub RESERVED0:          u32,
    pub ALRMAR:             u32,
    pub ALRMBR:             u32,
    pub WPR:                u32,
    pub SSR:                u32,
    pub SHIFTR:             u32,
    pub TSTR:               u32,
    pub TSDR:               u32,
    pub TSSSR:              u32,
    pub CALR:               u32,
    pub TAFCR:              u32,
    pub ALRMASSR:           u32,
    pub ALRMBSSR:           u32,
    pub RESERVED7:          u32,
    pub BKP0R:              u32,
    pub BKP1R:              u32,
    pub BKP2R:              u32,
    pub BKP3R:              u32,
    pub BKP4R:              u32,
    pub BKP5R:              u32,
    pub BKP6R:              u32,
    pub BKP7R:              u32,
    pub BKP8R:              u32,
    pub BKP9R:              u32,
    pub BKP10R:             u32,
    pub BKP11R:             u32,
    pub BKP12R:             u32,
    pub BKP13R:             u32,
    pub BKP14R:             u32,
    pub BKP15R:             u32,
}

//register addresses

registers! {
    const RTC:          RTC             = RTC_BASE,
}

//custom

