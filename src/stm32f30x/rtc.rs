use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct RTC {
    pub TR:                 u32,                                        //RTC time register,                                        Address offset: 0x00
    pub DR:                 u32,                                        //RTC date register,                                        Address offset: 0x04
    pub CR:                 u32,                                        //RTC control register,                                     Address offset: 0x08
    pub ISR:                u32,                                        //RTC initialization and status register,                   Address offset: 0x0C
    pub PRER:               u32,                                        //RTC prescaler register,                                   Address offset: 0x10
    pub WUTR:               u32,                                        //RTC wakeup timer register,                                Address offset: 0x14
    pub RESERVED0:          u32,                                        //Reserved, 0x18
    pub ALRMAR:             u32,                                        //RTC alarm A register,                                     Address offset: 0x1C
    pub ALRMBR:             u32,                                        //RTC alarm B register,                                     Address offset: 0x20
    pub WPR:                u32,                                        //RTC write protection register,                            Address offset: 0x24
    pub SSR:                u32,                                        //RTC sub second register,                                  Address offset: 0x28
    pub SHIFTR:             u32,                                        //RTC shift control register,                               Address offset: 0x2C
    pub TSTR:               u32,                                        //RTC time stamp time register,                             Address offset: 0x30
    pub TSDR:               u32,                                        //RTC time stamp date register,                             Address offset: 0x34
    pub TSSSR:              u32,                                        //RTC time-stamp sub second register,                       Address offset: 0x38
    pub CALR:               u32,                                        //RTC calibration register,                                 Address offset: 0x3C
    pub TAFCR:              u32,                                        //RTC tamper and alternate function configuration register, Address offset: 0x40
    pub ALRMASSR:           u32,                                        //RTC alarm A sub second register,                          Address offset: 0x44
    pub ALRMBSSR:           u32,                                        //RTC alarm B sub second register,                          Address offset: 0x48
    pub RESERVED7:          u32,                                        //Reserved, 0x4C
    pub BKP0R:              u32,                                        //RTC backup register 0,                                    Address offset: 0x50
    pub BKP1R:              u32,                                        //RTC backup register 1,                                    Address offset: 0x54
    pub BKP2R:              u32,                                        //RTC backup register 2,                                    Address offset: 0x58
    pub BKP3R:              u32,                                        //RTC backup register 3,                                    Address offset: 0x5C
    pub BKP4R:              u32,                                        //RTC backup register 4,                                    Address offset: 0x60
    pub BKP5R:              u32,                                        //RTC backup register 5,                                    Address offset: 0x64
    pub BKP6R:              u32,                                        //RTC backup register 6,                                    Address offset: 0x68
    pub BKP7R:              u32,                                        //RTC backup register 7,                                    Address offset: 0x6C
    pub BKP8R:              u32,                                        //RTC backup register 8,                                    Address offset: 0x70
    pub BKP9R:              u32,                                        //RTC backup register 9,                                    Address offset: 0x74
    pub BKP10R:             u32,                                        //RTC backup register 10,                                   Address offset: 0x78
    pub BKP11R:             u32,                                        //RTC backup register 11,                                   Address offset: 0x7C
    pub BKP12R:             u32,                                        //RTC backup register 12,                                   Address offset: 0x80
    pub BKP13R:             u32,                                        //RTC backup register 13,                                   Address offset: 0x84
    pub BKP14R:             u32,                                        //RTC backup register 14,                                   Address offset: 0x88
    pub BKP15R:             u32,                                        //RTC backup register 15,                                   Address offset: 0x8C
}

//register addresses

registers! {
    const RTC:              RTC                 = RTC_BASE,
}

//bit definitions

constants! {
    TR: u32 {
        const PM                                = 0x00400000,
        const HT                                = 0x00300000,
        const HT_0                              = 0x00100000,
        const HT_1                              = 0x00200000,
        const HU                                = 0x000F0000,
        const HU_0                              = 0x00010000,
        const HU_1                              = 0x00020000,
        const HU_2                              = 0x00040000,
        const HU_3                              = 0x00080000,
        const MNT                               = 0x00007000,
        const MNT_0                             = 0x00001000,
        const MNT_1                             = 0x00002000,
        const MNT_2                             = 0x00004000,
        const MNU                               = 0x00000F00,
        const MNU_0                             = 0x00000100,
        const MNU_1                             = 0x00000200,
        const MNU_2                             = 0x00000400,
        const MNU_3                             = 0x00000800,
        const ST                                = 0x00000070,
        const ST_0                              = 0x00000010,
        const ST_1                              = 0x00000020,
        const ST_2                              = 0x00000040,
        const SU                                = 0x0000000F,
        const SU_0                              = 0x00000001,
        const SU_1                              = 0x00000002,
        const SU_2                              = 0x00000004,
        const SU_3                              = 0x00000008,
    }

    DR: u32 {
        const YT                                = 0x00F00000,
        const YT_0                              = 0x00100000,
        const YT_1                              = 0x00200000,
        const YT_2                              = 0x00400000,
        const YT_3                              = 0x00800000,
        const YU                                = 0x000F0000,
        const YU_0                              = 0x00010000,
        const YU_1                              = 0x00020000,
        const YU_2                              = 0x00040000,
        const YU_3                              = 0x00080000,
        const WDU                               = 0x0000E000,
        const WDU_0                             = 0x00002000,
        const WDU_1                             = 0x00004000,
        const WDU_2                             = 0x00008000,
        const MT                                = 0x00001000,
        const MU                                = 0x00000F00,
        const MU_0                              = 0x00000100,
        const MU_1                              = 0x00000200,
        const MU_2                              = 0x00000400,
        const MU_3                              = 0x00000800,
        const DT                                = 0x00000030,
        const DT_0                              = 0x00000010,
        const DT_1                              = 0x00000020,
        const DU                                = 0x0000000F,
        const DU_0                              = 0x00000001,
        const DU_1                              = 0x00000002,
        const DU_2                              = 0x00000004,
        const DU_3                              = 0x00000008,
    }

    CR: u32 {
        const COE                               = 0x00800000,
        const OSEL                              = 0x00600000,
        const OSEL_0                            = 0x00200000,
        const OSEL_1                            = 0x00400000,
        const POL                               = 0x00100000,
        const COSEL                             = 0x00080000,
        const BCK                               = 0x00040000,
        const SUB1H                             = 0x00020000,
        const ADD1H                             = 0x00010000,
        const TSIE                              = 0x00008000,
        const WUTIE                             = 0x00004000,
        const ALRBIE                            = 0x00002000,
        const ALRAIE                            = 0x00001000,
        const TSE                               = 0x00000800,
        const WUTE                              = 0x00000400,
        const ALRBE                             = 0x00000200,
        const ALRAE                             = 0x00000100,
        const FMT                               = 0x00000040,
        const BYPSHAD                           = 0x00000020,
        const REFCKON                           = 0x00000010,
        const TSEDGE                            = 0x00000008,
        const WUCKSEL                           = 0x00000007,
        const WUCKSEL_0                         = 0x00000001,
        const WUCKSEL_1                         = 0x00000002,
        const WUCKSEL_2                         = 0x00000004,
    }

    ISR: u32 {
        const RECALPF                           = 0x00010000,
        const TAMP3F                            = 0x00008000,
        const TAMP2F                            = 0x00004000,
        const TAMP1F                            = 0x00002000,
        const TSOVF                             = 0x00001000,
        const TSF                               = 0x00000800,
        const WUTF                              = 0x00000400,
        const ALRBF                             = 0x00000200,
        const ALRAF                             = 0x00000100,
        const INIT                              = 0x00000080,
        const INITF                             = 0x00000040,
        const RSF                               = 0x00000020,
        const INITS                             = 0x00000010,
        const SHPF                              = 0x00000008,
        const WUTWF                             = 0x00000004,
        const ALRBWF                            = 0x00000002,
        const ALRAWF                            = 0x00000001,
    }

    PRER: u32 {
        const PREDIV_A                          = 0x007F0000,
        const PREDIV_S                          = 0x00007FFF,
    }

    WUTR: u32 {
        const WUT                               = 0x0000FFFF,
    }

    ALRMAR: u32 {
        const MSK4                              = 0x80000000,
        const WDSEL                             = 0x40000000,
        const DT                                = 0x30000000,
        const DT_0                              = 0x10000000,
        const DT_1                              = 0x20000000,
        const DU                                = 0x0F000000,
        const DU_0                              = 0x01000000,
        const DU_1                              = 0x02000000,
        const DU_2                              = 0x04000000,
        const DU_3                              = 0x08000000,
        const MSK3                              = 0x00800000,
        const PM                                = 0x00400000,
        const HT                                = 0x00300000,
        const HT_0                              = 0x00100000,
        const HT_1                              = 0x00200000,
        const HU                                = 0x000F0000,
        const HU_0                              = 0x00010000,
        const HU_1                              = 0x00020000,
        const HU_2                              = 0x00040000,
        const HU_3                              = 0x00080000,
        const MSK2                              = 0x00008000,
        const MNT                               = 0x00007000,
        const MNT_0                             = 0x00001000,
        const MNT_1                             = 0x00002000,
        const MNT_2                             = 0x00004000,
        const MNU                               = 0x00000F00,
        const MNU_0                             = 0x00000100,
        const MNU_1                             = 0x00000200,
        const MNU_2                             = 0x00000400,
        const MNU_3                             = 0x00000800,
        const MSK1                              = 0x00000080,
        const ST                                = 0x00000070,
        const ST_0                              = 0x00000010,
        const ST_1                              = 0x00000020,
        const ST_2                              = 0x00000040,
        const SU                                = 0x0000000F,
        const SU_0                              = 0x00000001,
        const SU_1                              = 0x00000002,
        const SU_2                              = 0x00000004,
        const SU_3                              = 0x00000008,
    }

    ALRMBR: u32 {
        const MSK4                              = 0x80000000,
        const WDSEL                             = 0x40000000,
        const DT                                = 0x30000000,
        const DT_0                              = 0x10000000,
        const DT_1                              = 0x20000000,
        const DU                                = 0x0F000000,
        const DU_0                              = 0x01000000,
        const DU_1                              = 0x02000000,
        const DU_2                              = 0x04000000,
        const DU_3                              = 0x08000000,
        const MSK3                              = 0x00800000,
        const PM                                = 0x00400000,
        const HT                                = 0x00300000,
        const HT_0                              = 0x00100000,
        const HT_1                              = 0x00200000,
        const HU                                = 0x000F0000,
        const HU_0                              = 0x00010000,
        const HU_1                              = 0x00020000,
        const HU_2                              = 0x00040000,
        const HU_3                              = 0x00080000,
        const MSK2                              = 0x00008000,
        const MNT                               = 0x00007000,
        const MNT_0                             = 0x00001000,
        const MNT_1                             = 0x00002000,
        const MNT_2                             = 0x00004000,
        const MNU                               = 0x00000F00,
        const MNU_0                             = 0x00000100,
        const MNU_1                             = 0x00000200,
        const MNU_2                             = 0x00000400,
        const MNU_3                             = 0x00000800,
        const MSK1                              = 0x00000080,
        const ST                                = 0x00000070,
        const ST_0                              = 0x00000010,
        const ST_1                              = 0x00000020,
        const ST_2                              = 0x00000040,
        const SU                                = 0x0000000F,
        const SU_0                              = 0x00000001,
        const SU_1                              = 0x00000002,
        const SU_2                              = 0x00000004,
        const SU_3                              = 0x00000008,
    }

    WPR: u32 {
        const KEY                               = 0x000000FF,
    }

    SSR: u32 {
        const SS                                = 0x0000FFFF,
    }

    SHIFTR: u32 {
        const SUBFS                             = 0x00007FFF,
        const ADD1S                             = 0x80000000,
    }

    TSTR: u32 {
        const PM                                = 0x00400000,
        const HT                                = 0x00300000,
        const HT_0                              = 0x00100000,
        const HT_1                              = 0x00200000,
        const HU                                = 0x000F0000,
        const HU_0                              = 0x00010000,
        const HU_1                              = 0x00020000,
        const HU_2                              = 0x00040000,
        const HU_3                              = 0x00080000,
        const MNT                               = 0x00007000,
        const MNT_0                             = 0x00001000,
        const MNT_1                             = 0x00002000,
        const MNT_2                             = 0x00004000,
        const MNU                               = 0x00000F00,
        const MNU_0                             = 0x00000100,
        const MNU_1                             = 0x00000200,
        const MNU_2                             = 0x00000400,
        const MNU_3                             = 0x00000800,
        const ST                                = 0x00000070,
        const ST_0                              = 0x00000010,
        const ST_1                              = 0x00000020,
        const ST_2                              = 0x00000040,
        const SU                                = 0x0000000F,
        const SU_0                              = 0x00000001,
        const SU_1                              = 0x00000002,
        const SU_2                              = 0x00000004,
        const SU_3                              = 0x00000008,
    }

    TSDR: u32 {
        const WDU                               = 0x0000E000,
        const WDU_0                             = 0x00002000,
        const WDU_1                             = 0x00004000,
        const WDU_2                             = 0x00008000,
        const MT                                = 0x00001000,
        const MU                                = 0x00000F00,
        const MU_0                              = 0x00000100,
        const MU_1                              = 0x00000200,
        const MU_2                              = 0x00000400,
        const MU_3                              = 0x00000800,
        const DT                                = 0x00000030,
        const DT_0                              = 0x00000010,
        const DT_1                              = 0x00000020,
        const DU                                = 0x0000000F,
        const DU_0                              = 0x00000001,
        const DU_1                              = 0x00000002,
        const DU_2                              = 0x00000004,
        const DU_3                              = 0x00000008,
    }

    TSSSR: u32 {
        const SS                                = 0x0000FFFF,
    }

    CALR: u32 {
        const CALP                              = 0x00008000,
        const CALW8                             = 0x00004000,
        const CALW16                            = 0x00002000,
        const CALM                              = 0x000001FF,
        const CALM_0                            = 0x00000001,
        const CALM_1                            = 0x00000002,
        const CALM_2                            = 0x00000004,
        const CALM_3                            = 0x00000008,
        const CALM_4                            = 0x00000010,
        const CALM_5                            = 0x00000020,
        const CALM_6                            = 0x00000040,
        const CALM_7                            = 0x00000080,
        const CALM_8                            = 0x00000100,
    }

    TAFCR: u32 {
        const ALARMOUTTYPE                      = 0x00040000,
        const TAMPPUDIS                         = 0x00008000,
        const TAMPPRCH                          = 0x00006000,
        const TAMPPRCH_0                        = 0x00002000,
        const TAMPPRCH_1                        = 0x00004000,
        const TAMPFLT                           = 0x00001800,
        const TAMPFLT_0                         = 0x00000800,
        const TAMPFLT_1                         = 0x00001000,
        const TAMPFREQ                          = 0x00000700,
        const TAMPFREQ_0                        = 0x00000100,
        const TAMPFREQ_1                        = 0x00000200,
        const TAMPFREQ_2                        = 0x00000400,
        const TAMPTS                            = 0x00000080,
        const TAMP3TRG                          = 0x00000040,
        const TAMP3E                            = 0x00000020,
        const TAMP2TRG                          = 0x00000010,
        const TAMP2E                            = 0x00000008,
        const TAMPIE                            = 0x00000004,
        const TAMP1TRG                          = 0x00000002,
        const TAMP1E                            = 0x00000001,
    }

    ALRMASSR: u32 {
        const MASKSS                            = 0x0F000000,
        const MASKSS_0                          = 0x01000000,
        const MASKSS_1                          = 0x02000000,
        const MASKSS_2                          = 0x04000000,
        const MASKSS_3                          = 0x08000000,
        const SS                                = 0x00007FFF,
    }

    ALRMBSSR: u32 {
        const MASKSS                            = 0x0F000000,
        const MASKSS_0                          = 0x01000000,
        const MASKSS_1                          = 0x02000000,
        const MASKSS_2                          = 0x04000000,
        const MASKSS_3                          = 0x08000000,
        const SS                                = 0x00007FFF,
    }
}

//custom

