use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct COMP {
    pub CSR:                u32,                                        //Comparator control Status register, Address offset: 0x00
}

//register addresses

registers! {
    const COMP:             COMP                = COMP_BASE,
    const COMP1:            COMP                = COMP1_BASE,
    const COMP2:            COMP                = COMP2_BASE,
    const COMP3:            COMP                = COMP3_BASE,
    const COMP4:            COMP                = COMP4_BASE,
    const COMP5:            COMP                = COMP5_BASE,
    const COMP6:            COMP                = COMP6_BASE,
    const COMP7:            COMP                = COMP7_BASE,
}

//bit definitions

constants! {
    CSR: u32 {
        const COMP1EN                           = 0x00000001,           //COMP1 enable
        const COMP1SW1                          = 0x00000002,           //COMP1 SW1 switch control
        const COMP1MODE                         = 0x0000000C,           //COMP1 power mode
        const COMP1MODE_0                       = 0x00000004,           //COMP1 power mode bit 0
        const COMP1MODE_1                       = 0x00000008,           //COMP1 power mode bit 1
        const COMP1INSEL                        = 0x00000070,           //COMP1 inverting input select
        const COMP1INSEL_0                      = 0x00000010,           //COMP1 inverting input select bit 0
        const COMP1INSEL_1                      = 0x00000020,           //COMP1 inverting input select bit 1
        const COMP1INSEL_2                      = 0x00000040,           //COMP1 inverting input select bit 2
        const COMP1NONINSEL                     = 0x00000080,           //COMP1 non inverting input select
        const COMP1OUTSEL                       = 0x00003C00,           //COMP1 output select
        const COMP1OUTSEL_0                     = 0x00000400,           //COMP1 output select bit 0
        const COMP1OUTSEL_1                     = 0x00000800,           //COMP1 output select bit 1
        const COMP1OUTSEL_2                     = 0x00001000,           //COMP1 output select bit 2
        const COMP1OUTSEL_3                     = 0x00002000,           //COMP1 output select bit 3
        const COMP1POL                          = 0x00008000,           //COMP1 output polarity
        const COMP1HYST                         = 0x00030000,           //COMP1 hysteresis
        const COMP1HYST_0                       = 0x00010000,           //COMP1 hysteresis bit 0
        const COMP1HYST_1                       = 0x00020000,           //COMP1 hysteresis bit 1
        const COMP1BLANKING                     = 0x000C0000,           //COMP1 blanking
        const COMP1BLANKING_0                   = 0x00040000,           //COMP1 blanking bit 0
        const COMP1BLANKING_1                   = 0x00080000,           //COMP1 blanking bit 1
        const COMP1BLANKING_2                   = 0x00100000,           //COMP1 blanking bit 2
        const COMP1OUT                          = 0x40000000,           //COMP1 output level
        const COMP1LOCK                         = 0x80000000,           //COMP1 lock
        const COMP2EN                           = 0x00000001,           //COMP2 enable
        const COMP2MODE                         = 0x0000000C,           //COMP2 power mode
        const COMP2MODE_0                       = 0x00000004,           //COMP2 power mode bit 0
        const COMP2MODE_1                       = 0x00000008,           //COMP2 power mode bit 1
        const COMP2INSEL                        = 0x00000070,           //COMP2 inverting input select
        const COMP2INSEL_0                      = 0x00000010,           //COMP2 inverting input select bit 0
        const COMP2INSEL_1                      = 0x00000020,           //COMP2 inverting input select bit 1
        const COMP2INSEL_2                      = 0x00000040,           //COMP2 inverting input select bit 2
        const COMP2NONINSEL                     = 0x00000080,           //COMP2 non inverting input select
        const COMP2WNDWEN                       = 0x00000200,           //COMP2 window mode enable
        const COMP2OUTSEL                       = 0x00003C00,           //COMP2 output select
        const COMP2OUTSEL_0                     = 0x00000400,           //COMP2 output select bit 0
        const COMP2OUTSEL_1                     = 0x00000800,           //COMP2 output select bit 1
        const COMP2OUTSEL_2                     = 0x00001000,           //COMP2 output select bit 2
        const COMP2OUTSEL_3                     = 0x00002000,           //COMP2 output select bit 3
        const COMP2POL                          = 0x00008000,           //COMP2 output polarity
        const COMP2HYST                         = 0x00030000,           //COMP2 hysteresis
        const COMP2HYST_0                       = 0x00010000,           //COMP2 hysteresis bit 0
        const COMP2HYST_1                       = 0x00020000,           //COMP2 hysteresis bit 1
        const COMP2BLANKING                     = 0x000C0000,           //COMP2 blanking
        const COMP2BLANKING_0                   = 0x00040000,           //COMP2 blanking bit 0
        const COMP2BLANKING_1                   = 0x00080000,           //COMP2 blanking bit 1
        const COMP2BLANKING_2                   = 0x00100000,           //COMP2 blanking bit 2
        const COMP2OUT                          = 0x40000000,           //COMP2 output level
        const COMP2LOCK                         = 0x80000000,           //COMP2 lock
        const COMP3EN                           = 0x00000001,           //COMP3 enable
        const COMP3MODE                         = 0x0000000C,           //COMP3 power mode
        const COMP3MODE_0                       = 0x00000004,           //COMP3 power mode bit 0
        const COMP3MODE_1                       = 0x00000008,           //COMP3 power mode bit 1
        const COMP3INSEL                        = 0x00000070,           //COMP3 inverting input select
        const COMP3INSEL_0                      = 0x00000010,           //COMP3 inverting input select bit 0
        const COMP3INSEL_1                      = 0x00000020,           //COMP3 inverting input select bit 1
        const COMP3INSEL_2                      = 0x00000040,           //COMP3 inverting input select bit 2
        const COMP3NONINSEL                     = 0x00000080,           //COMP3 non inverting input select
        const COMP3OUTSEL                       = 0x00003C00,           //COMP3 output select
        const COMP3OUTSEL_0                     = 0x00000400,           //COMP3 output select bit 0
        const COMP3OUTSEL_1                     = 0x00000800,           //COMP3 output select bit 1
        const COMP3OUTSEL_2                     = 0x00001000,           //COMP3 output select bit 2
        const COMP3OUTSEL_3                     = 0x00002000,           //COMP3 output select bit 3
        const COMP3POL                          = 0x00008000,           //COMP3 output polarity
        const COMP3HYST                         = 0x00030000,           //COMP3 hysteresis
        const COMP3HYST_0                       = 0x00010000,           //COMP3 hysteresis bit 0
        const COMP3HYST_1                       = 0x00020000,           //COMP3 hysteresis bit 1
        const COMP3BLANKING                     = 0x000C0000,           //COMP3 blanking
        const COMP3BLANKING_0                   = 0x00040000,           //COMP3 blanking bit 0
        const COMP3BLANKING_1                   = 0x00080000,           //COMP3 blanking bit 1
        const COMP3BLANKING_2                   = 0x00100000,           //COMP3 blanking bit 2
        const COMP3OUT                          = 0x40000000,           //COMP3 output level
        const COMP3LOCK                         = 0x80000000,           //COMP3 lock
        const COMP4EN                           = 0x00000001,           //COMP4 enable
        const COMP4MODE                         = 0x0000000C,           //COMP4 power mode
        const COMP4MODE_0                       = 0x00000004,           //COMP4 power mode bit 0
        const COMP4MODE_1                       = 0x00000008,           //COMP4 power mode bit 1
        const COMP4INSEL                        = 0x00000070,           //COMP4 inverting input select
        const COMP4INSEL_0                      = 0x00000010,           //COMP4 inverting input select bit 0
        const COMP4INSEL_1                      = 0x00000020,           //COMP4 inverting input select bit 1
        const COMP4INSEL_2                      = 0x00000040,           //COMP4 inverting input select bit 2
        const COMP4NONINSEL                     = 0x00000080,           //COMP4 non inverting input select
        const COMP4WNDWEN                       = 0x00000200,           //COMP4 window mode enable
        const COMP4OUTSEL                       = 0x00003C00,           //COMP4 output select
        const COMP4OUTSEL_0                     = 0x00000400,           //COMP4 output select bit 0
        const COMP4OUTSEL_1                     = 0x00000800,           //COMP4 output select bit 1
        const COMP4OUTSEL_2                     = 0x00001000,           //COMP4 output select bit 2
        const COMP4OUTSEL_3                     = 0x00002000,           //COMP4 output select bit 3
        const COMP4POL                          = 0x00008000,           //COMP4 output polarity
        const COMP4HYST                         = 0x00030000,           //COMP4 hysteresis
        const COMP4HYST_0                       = 0x00010000,           //COMP4 hysteresis bit 0
        const COMP4HYST_1                       = 0x00020000,           //COMP4 hysteresis bit 1
        const COMP4BLANKING                     = 0x000C0000,           //COMP4 blanking
        const COMP4BLANKING_0                   = 0x00040000,           //COMP4 blanking bit 0
        const COMP4BLANKING_1                   = 0x00080000,           //COMP4 blanking bit 1
        const COMP4BLANKING_2                   = 0x00100000,           //COMP4 blanking bit 2
        const COMP4OUT                          = 0x40000000,           //COMP4 output level
        const COMP4LOCK                         = 0x80000000,           //COMP4 lock
        const COMP5EN                           = 0x00000001,           //COMP5 enable
        const COMP5MODE                         = 0x0000000C,           //COMP5 power mode
        const COMP5MODE_0                       = 0x00000004,           //COMP5 power mode bit 0
        const COMP5MODE_1                       = 0x00000008,           //COMP5 power mode bit 1
        const COMP5INSEL                        = 0x00000070,           //COMP5 inverting input select
        const COMP5INSEL_0                      = 0x00000010,           //COMP5 inverting input select bit 0
        const COMP5INSEL_1                      = 0x00000020,           //COMP5 inverting input select bit 1
        const COMP5INSEL_2                      = 0x00000040,           //COMP5 inverting input select bit 2
        const COMP5NONINSEL                     = 0x00000080,           //COMP5 non inverting input select
        const COMP5OUTSEL                       = 0x00003C00,           //COMP5 output select
        const COMP5OUTSEL_0                     = 0x00000400,           //COMP5 output select bit 0
        const COMP5OUTSEL_1                     = 0x00000800,           //COMP5 output select bit 1
        const COMP5OUTSEL_2                     = 0x00001000,           //COMP5 output select bit 2
        const COMP5OUTSEL_3                     = 0x00002000,           //COMP5 output select bit 3
        const COMP5POL                          = 0x00008000,           //COMP5 output polarity
        const COMP5HYST                         = 0x00030000,           //COMP5 hysteresis
        const COMP5HYST_0                       = 0x00010000,           //COMP5 hysteresis bit 0
        const COMP5HYST_1                       = 0x00020000,           //COMP5 hysteresis bit 1
        const COMP5BLANKING                     = 0x000C0000,           //COMP5 blanking
        const COMP5BLANKING_0                   = 0x00040000,           //COMP5 blanking bit 0
        const COMP5BLANKING_1                   = 0x00080000,           //COMP5 blanking bit 1
        const COMP5BLANKING_2                   = 0x00100000,           //COMP5 blanking bit 2
        const COMP5OUT                          = 0x40000000,           //COMP5 output level
        const COMP5LOCK                         = 0x80000000,           //COMP5 lock
        const COMP6EN                           = 0x00000001,           //COMP6 enable
        const COMP6MODE                         = 0x0000000C,           //COMP6 power mode
        const COMP6MODE_0                       = 0x00000004,           //COMP6 power mode bit 0
        const COMP6MODE_1                       = 0x00000008,           //COMP6 power mode bit 1
        const COMP6INSEL                        = 0x00000070,           //COMP6 inverting input select
        const COMP6INSEL_0                      = 0x00000010,           //COMP6 inverting input select bit 0
        const COMP6INSEL_1                      = 0x00000020,           //COMP6 inverting input select bit 1
        const COMP6INSEL_2                      = 0x00000040,           //COMP6 inverting input select bit 2
        const COMP6NONINSEL                     = 0x00000080,           //COMP6 non inverting input select
        const COMP6WNDWEN                       = 0x00000200,           //COMP6 window mode enable
        const COMP6OUTSEL                       = 0x00003C00,           //COMP6 output select
        const COMP6OUTSEL_0                     = 0x00000400,           //COMP6 output select bit 0
        const COMP6OUTSEL_1                     = 0x00000800,           //COMP6 output select bit 1
        const COMP6OUTSEL_2                     = 0x00001000,           //COMP6 output select bit 2
        const COMP6OUTSEL_3                     = 0x00002000,           //COMP6 output select bit 3
        const COMP6POL                          = 0x00008000,           //COMP6 output polarity
        const COMP6HYST                         = 0x00030000,           //COMP6 hysteresis
        const COMP6HYST_0                       = 0x00010000,           //COMP6 hysteresis bit 0
        const COMP6HYST_1                       = 0x00020000,           //COMP6 hysteresis bit 1
        const COMP6BLANKING                     = 0x000C0000,           //COMP6 blanking
        const COMP6BLANKING_0                   = 0x00040000,           //COMP6 blanking bit 0
        const COMP6BLANKING_1                   = 0x00080000,           //COMP6 blanking bit 1
        const COMP6BLANKING_2                   = 0x00100000,           //COMP6 blanking bit 2
        const COMP6OUT                          = 0x40000000,           //COMP6 output level
        const COMP6LOCK                         = 0x80000000,           //COMP6 lock
        const COMP7EN                           = 0x00000001,           //COMP7 enable
        const COMP7MODE                         = 0x0000000C,           //COMP7 power mode
        const COMP7MODE_0                       = 0x00000004,           //COMP7 power mode bit 0
        const COMP7MODE_1                       = 0x00000008,           //COMP7 power mode bit 1
        const COMP7INSEL                        = 0x00000070,           //COMP7 inverting input select
        const COMP7INSEL_0                      = 0x00000010,           //COMP7 inverting input select bit 0
        const COMP7INSEL_1                      = 0x00000020,           //COMP7 inverting input select bit 1
        const COMP7INSEL_2                      = 0x00000040,           //COMP7 inverting input select bit 2
        const COMP7NONINSEL                     = 0x00000080,           //COMP7 non inverting input select
        const COMP7OUTSEL                       = 0x00003C00,           //COMP7 output select
        const COMP7OUTSEL_0                     = 0x00000400,           //COMP7 output select bit 0
        const COMP7OUTSEL_1                     = 0x00000800,           //COMP7 output select bit 1
        const COMP7OUTSEL_2                     = 0x00001000,           //COMP7 output select bit 2
        const COMP7OUTSEL_3                     = 0x00002000,           //COMP7 output select bit 3
        const COMP7POL                          = 0x00008000,           //COMP7 output polarity
        const COMP7HYST                         = 0x00030000,           //COMP7 hysteresis
        const COMP7HYST_0                       = 0x00010000,           //COMP7 hysteresis bit 0
        const COMP7HYST_1                       = 0x00020000,           //COMP7 hysteresis bit 1
        const COMP7BLANKING                     = 0x000C0000,           //COMP7 blanking
        const COMP7BLANKING_0                   = 0x00040000,           //COMP7 blanking bit 0
        const COMP7BLANKING_1                   = 0x00080000,           //COMP7 blanking bit 1
        const COMP7BLANKING_2                   = 0x00100000,           //COMP7 blanking bit 2
        const COMP7OUT                          = 0x40000000,           //COMP7 output level
        const COMP7LOCK                         = 0x80000000,           //COMP7 lock
        const COMPXEN                           = 0x00000001,           //COMPx enable
        const COMPXSW1                          = 0x00000002,           //COMP1 SW1 switch control
        const COMPXMODE                         = 0x0000000C,           //COMPx power mode
        const COMPXMODE_0                       = 0x00000004,           //COMPx power mode bit 0
        const COMPXMODE_1                       = 0x00000008,           //COMPx power mode bit 1
        const COMPXINSEL                        = 0x00000070,           //COMPx inverting input select
        const COMPXINSEL_0                      = 0x00000010,           //COMPx inverting input select bit 0
        const COMPXINSEL_1                      = 0x00000020,           //COMPx inverting input select bit 1
        const COMPXINSEL_2                      = 0x00000040,           //COMPx inverting input select bit 2
        const COMPXNONINSEL                     = 0x00000080,           //COMPx non inverting input select
        const COMPXWNDWEN                       = 0x00000200,           //COMPx window mode enable
        const COMPXOUTSEL                       = 0x00003C00,           //COMPx output select
        const COMPXOUTSEL_0                     = 0x00000400,           //COMPx output select bit 0
        const COMPXOUTSEL_1                     = 0x00000800,           //COMPx output select bit 1
        const COMPXOUTSEL_2                     = 0x00001000,           //COMPx output select bit 2
        const COMPXOUTSEL_3                     = 0x00002000,           //COMPx output select bit 3
        const COMPXPOL                          = 0x00008000,           //COMPx output polarity
        const COMPXHYST                         = 0x00030000,           //COMPx hysteresis
        const COMPXHYST_0                       = 0x00010000,           //COMPx hysteresis bit 0
        const COMPXHYST_1                       = 0x00020000,           //COMPx hysteresis bit 1
        const COMPXBLANKING                     = 0x000C0000,           //COMPx blanking
        const COMPXBLANKING_0                   = 0x00040000,           //COMPx blanking bit 0
        const COMPXBLANKING_1                   = 0x00080000,           //COMPx blanking bit 1
        const COMPXBLANKING_2                   = 0x00100000,           //COMPx blanking bit 2
        const COMPXINSEL_3                      = 0x00400000,           //COMPx inverting input select bit 3
        const COMPXOUT                          = 0x40000000,           //COMPx output level
        const COMPXLOCK                         = 0x80000000,           //COMPx lock
    }
}

//custom

