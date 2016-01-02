use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct EXTI {
    pub IMR:                u32,                                        //EXTI Interrupt mask register,                Address offset: 0x00
    pub EMR:                u32,                                        //EXTI Event mask register,                    Address offset: 0x04
    pub RTSR:               u32,                                        //EXTI Rising trigger selection register,      Address offset: 0x08
    pub FTSR:               u32,                                        //EXTI Falling trigger selection register,     Address offset: 0x0C
    pub SWIER:              u32,                                        //EXTI Software interrupt event register,      Address offset: 0x10
    pub PR:                 u32,                                        //EXTI Pending register,                       Address offset: 0x14
    pub RESERVED1:          u32,                                        //Reserved, 0x18
    pub RESERVED2:          u32,                                        //Reserved, 0x1C
    pub IMR2:               u32,                                        //EXTI Interrupt mask register,                Address offset: 0x20
    pub EMR2:               u32,                                        //EXTI Event mask register,                    Address offset: 0x24
    pub RTSR2:              u32,                                        //EXTI Rising trigger selection register,      Address offset: 0x28
    pub FTSR2:              u32,                                        //EXTI Falling trigger selection register,     Address offset: 0x2C
    pub SWIER2:             u32,                                        //EXTI Software interrupt event register,      Address offset: 0x30
    pub PR2:                u32,                                        //EXTI Pending register,                       Address offset: 0x34
}

//register addresses

registers! {
    const EXTI:             EXTI                = EXTI_BASE,
}

//bit definitions

constants! {
    IMR: u32 {
        const MR0                               = 0x00000001,           //Interrupt Mask on line 0
        const MR1                               = 0x00000002,           //Interrupt Mask on line 1
        const MR2                               = 0x00000004,           //Interrupt Mask on line 2
        const MR3                               = 0x00000008,           //Interrupt Mask on line 3
        const MR4                               = 0x00000010,           //Interrupt Mask on line 4
        const MR5                               = 0x00000020,           //Interrupt Mask on line 5
        const MR6                               = 0x00000040,           //Interrupt Mask on line 6
        const MR7                               = 0x00000080,           //Interrupt Mask on line 7
        const MR8                               = 0x00000100,           //Interrupt Mask on line 8
        const MR9                               = 0x00000200,           //Interrupt Mask on line 9
        const MR10                              = 0x00000400,           //Interrupt Mask on line 10
        const MR11                              = 0x00000800,           //Interrupt Mask on line 11
        const MR12                              = 0x00001000,           //Interrupt Mask on line 12
        const MR13                              = 0x00002000,           //Interrupt Mask on line 13
        const MR14                              = 0x00004000,           //Interrupt Mask on line 14
        const MR15                              = 0x00008000,           //Interrupt Mask on line 15
        const MR16                              = 0x00010000,           //Interrupt Mask on line 16
        const MR17                              = 0x00020000,           //Interrupt Mask on line 17
        const MR18                              = 0x00040000,           //Interrupt Mask on line 18
        const MR19                              = 0x00080000,           //Interrupt Mask on line 19
        const MR20                              = 0x00100000,           //Interrupt Mask on line 20
        const MR21                              = 0x00200000,           //Interrupt Mask on line 21
        const MR22                              = 0x00400000,           //Interrupt Mask on line 22
        const MR23                              = 0x00800000,           //Interrupt Mask on line 23
        const MR24                              = 0x01000000,           //Interrupt Mask on line 24
        const MR25                              = 0x02000000,           //Interrupt Mask on line 25
        const MR26                              = 0x04000000,           //Interrupt Mask on line 26
        const MR27                              = 0x08000000,           //Interrupt Mask on line 27
        const MR28                              = 0x10000000,           //Interrupt Mask on line 28
        const MR29                              = 0x20000000,           //Interrupt Mask on line 29
        const MR30                              = 0x40000000,           //Interrupt Mask on line 30
        const MR31                              = 0x80000000,           //Interrupt Mask on line 31
    }

    EMR: u32 {
        const MR0                               = 0x00000001,           //Event Mask on line 0
        const MR1                               = 0x00000002,           //Event Mask on line 1
        const MR2                               = 0x00000004,           //Event Mask on line 2
        const MR3                               = 0x00000008,           //Event Mask on line 3
        const MR4                               = 0x00000010,           //Event Mask on line 4
        const MR5                               = 0x00000020,           //Event Mask on line 5
        const MR6                               = 0x00000040,           //Event Mask on line 6
        const MR7                               = 0x00000080,           //Event Mask on line 7
        const MR8                               = 0x00000100,           //Event Mask on line 8
        const MR9                               = 0x00000200,           //Event Mask on line 9
        const MR10                              = 0x00000400,           //Event Mask on line 10
        const MR11                              = 0x00000800,           //Event Mask on line 11
        const MR12                              = 0x00001000,           //Event Mask on line 12
        const MR13                              = 0x00002000,           //Event Mask on line 13
        const MR14                              = 0x00004000,           //Event Mask on line 14
        const MR15                              = 0x00008000,           //Event Mask on line 15
        const MR16                              = 0x00010000,           //Event Mask on line 16
        const MR17                              = 0x00020000,           //Event Mask on line 17
        const MR18                              = 0x00040000,           //Event Mask on line 18
        const MR19                              = 0x00080000,           //Event Mask on line 19
        const MR20                              = 0x00100000,           //Event Mask on line 20
        const MR21                              = 0x00200000,           //Event Mask on line 21
        const MR22                              = 0x00400000,           //Event Mask on line 22
        const MR23                              = 0x00800000,           //Event Mask on line 23
        const MR24                              = 0x01000000,           //Event Mask on line 24
        const MR25                              = 0x02000000,           //Event Mask on line 25
        const MR26                              = 0x04000000,           //Event Mask on line 26
        const MR27                              = 0x08000000,           //Event Mask on line 27
        const MR28                              = 0x10000000,           //Event Mask on line 28
        const MR29                              = 0x20000000,           //Event Mask on line 29
        const MR30                              = 0x40000000,           //Event Mask on line 30
        const MR31                              = 0x80000000,           //Event Mask on line 31
    }

    RTSR: u32 {
        const TR0                               = 0x00000001,           //Rising trigger event configuration bit of line 0
        const TR1                               = 0x00000002,           //Rising trigger event configuration bit of line 1
        const TR2                               = 0x00000004,           //Rising trigger event configuration bit of line 2
        const TR3                               = 0x00000008,           //Rising trigger event configuration bit of line 3
        const TR4                               = 0x00000010,           //Rising trigger event configuration bit of line 4
        const TR5                               = 0x00000020,           //Rising trigger event configuration bit of line 5
        const TR6                               = 0x00000040,           //Rising trigger event configuration bit of line 6
        const TR7                               = 0x00000080,           //Rising trigger event configuration bit of line 7
        const TR8                               = 0x00000100,           //Rising trigger event configuration bit of line 8
        const TR9                               = 0x00000200,           //Rising trigger event configuration bit of line 9
        const TR10                              = 0x00000400,           //Rising trigger event configuration bit of line 10
        const TR11                              = 0x00000800,           //Rising trigger event configuration bit of line 11
        const TR12                              = 0x00001000,           //Rising trigger event configuration bit of line 12
        const TR13                              = 0x00002000,           //Rising trigger event configuration bit of line 13
        const TR14                              = 0x00004000,           //Rising trigger event configuration bit of line 14
        const TR15                              = 0x00008000,           //Rising trigger event configuration bit of line 15
        const TR16                              = 0x00010000,           //Rising trigger event configuration bit of line 16
        const TR17                              = 0x00020000,           //Rising trigger event configuration bit of line 17
        const TR18                              = 0x00040000,           //Rising trigger event configuration bit of line 18
        const TR19                              = 0x00080000,           //Rising trigger event configuration bit of line 19
        const TR20                              = 0x00100000,           //Rising trigger event configuration bit of line 20
        const TR21                              = 0x00200000,           //Rising trigger event configuration bit of line 21
        const TR22                              = 0x00400000,           //Rising trigger event configuration bit of line 22
        const TR23                              = 0x00800000,           //Rising trigger event configuration bit of line 23
        const TR24                              = 0x01000000,           //Rising trigger event configuration bit of line 24
        const TR25                              = 0x02000000,           //Rising trigger event configuration bit of line 25
        const TR26                              = 0x04000000,           //Rising trigger event configuration bit of line 26
        const TR27                              = 0x08000000,           //Rising trigger event configuration bit of line 27
        const TR28                              = 0x10000000,           //Rising trigger event configuration bit of line 28
        const TR29                              = 0x20000000,           //Rising trigger event configuration bit of line 29
        const TR30                              = 0x40000000,           //Rising trigger event configuration bit of line 30
        const TR31                              = 0x80000000,           //Rising trigger event configuration bit of line 31
    }

    FTSR: u32 {
        const TR0                               = 0x00000001,           //Falling trigger event configuration bit of line 0
        const TR1                               = 0x00000002,           //Falling trigger event configuration bit of line 1
        const TR2                               = 0x00000004,           //Falling trigger event configuration bit of line 2
        const TR3                               = 0x00000008,           //Falling trigger event configuration bit of line 3
        const TR4                               = 0x00000010,           //Falling trigger event configuration bit of line 4
        const TR5                               = 0x00000020,           //Falling trigger event configuration bit of line 5
        const TR6                               = 0x00000040,           //Falling trigger event configuration bit of line 6
        const TR7                               = 0x00000080,           //Falling trigger event configuration bit of line 7
        const TR8                               = 0x00000100,           //Falling trigger event configuration bit of line 8
        const TR9                               = 0x00000200,           //Falling trigger event configuration bit of line 9
        const TR10                              = 0x00000400,           //Falling trigger event configuration bit of line 10
        const TR11                              = 0x00000800,           //Falling trigger event configuration bit of line 11
        const TR12                              = 0x00001000,           //Falling trigger event configuration bit of line 12
        const TR13                              = 0x00002000,           //Falling trigger event configuration bit of line 13
        const TR14                              = 0x00004000,           //Falling trigger event configuration bit of line 14
        const TR15                              = 0x00008000,           //Falling trigger event configuration bit of line 15
        const TR16                              = 0x00010000,           //Falling trigger event configuration bit of line 16
        const TR17                              = 0x00020000,           //Falling trigger event configuration bit of line 17
        const TR18                              = 0x00040000,           //Falling trigger event configuration bit of line 18
        const TR19                              = 0x00080000,           //Falling trigger event configuration bit of line 19
        const TR20                              = 0x00100000,           //Falling trigger event configuration bit of line 20
        const TR21                              = 0x00200000,           //Falling trigger event configuration bit of line 21
        const TR22                              = 0x00400000,           //Falling trigger event configuration bit of line 22
        const TR23                              = 0x00800000,           //Falling trigger event configuration bit of line 23
        const TR24                              = 0x01000000,           //Falling trigger event configuration bit of line 24
        const TR25                              = 0x02000000,           //Falling trigger event configuration bit of line 25
        const TR26                              = 0x04000000,           //Falling trigger event configuration bit of line 26
        const TR27                              = 0x08000000,           //Falling trigger event configuration bit of line 27
        const TR28                              = 0x10000000,           //Falling trigger event configuration bit of line 28
        const TR29                              = 0x20000000,           //Falling trigger event configuration bit of line 29
        const TR30                              = 0x40000000,           //Falling trigger event configuration bit of line 30
        const TR31                              = 0x80000000,           //Falling trigger event configuration bit of line 31
    }

    SWIER: u32 {
        const SWIER0                            = 0x00000001,           //Software Interrupt on line 0
        const SWIER1                            = 0x00000002,           //Software Interrupt on line 1
        const SWIER2                            = 0x00000004,           //Software Interrupt on line 2
        const SWIER3                            = 0x00000008,           //Software Interrupt on line 3
        const SWIER4                            = 0x00000010,           //Software Interrupt on line 4
        const SWIER5                            = 0x00000020,           //Software Interrupt on line 5
        const SWIER6                            = 0x00000040,           //Software Interrupt on line 6
        const SWIER7                            = 0x00000080,           //Software Interrupt on line 7
        const SWIER8                            = 0x00000100,           //Software Interrupt on line 8
        const SWIER9                            = 0x00000200,           //Software Interrupt on line 9
        const SWIER10                           = 0x00000400,           //Software Interrupt on line 10
        const SWIER11                           = 0x00000800,           //Software Interrupt on line 11
        const SWIER12                           = 0x00001000,           //Software Interrupt on line 12
        const SWIER13                           = 0x00002000,           //Software Interrupt on line 13
        const SWIER14                           = 0x00004000,           //Software Interrupt on line 14
        const SWIER15                           = 0x00008000,           //Software Interrupt on line 15
        const SWIER16                           = 0x00010000,           //Software Interrupt on line 16
        const SWIER17                           = 0x00020000,           //Software Interrupt on line 17
        const SWIER18                           = 0x00040000,           //Software Interrupt on line 18
        const SWIER19                           = 0x00080000,           //Software Interrupt on line 19
        const SWIER20                           = 0x00100000,           //Software Interrupt on line 20
        const SWIER21                           = 0x00200000,           //Software Interrupt on line 21
        const SWIER22                           = 0x00400000,           //Software Interrupt on line 22
        const SWIER23                           = 0x00800000,           //Software Interrupt on line 23
        const SWIER24                           = 0x01000000,           //Software Interrupt on line 24
        const SWIER25                           = 0x02000000,           //Software Interrupt on line 25
        const SWIER26                           = 0x04000000,           //Software Interrupt on line 26
        const SWIER27                           = 0x08000000,           //Software Interrupt on line 27
        const SWIER28                           = 0x10000000,           //Software Interrupt on line 28
        const SWIER29                           = 0x20000000,           //Software Interrupt on line 29
        const SWIER30                           = 0x40000000,           //Software Interrupt on line 30
        const SWIER31                           = 0x80000000,           //Software Interrupt on line 31
    }

    PR: u32 {
        const PR0                               = 0x00000001,           //Pending bit for line 0
        const PR1                               = 0x00000002,           //Pending bit for line 1
        const PR2                               = 0x00000004,           //Pending bit for line 2
        const PR3                               = 0x00000008,           //Pending bit for line 3
        const PR4                               = 0x00000010,           //Pending bit for line 4
        const PR5                               = 0x00000020,           //Pending bit for line 5
        const PR6                               = 0x00000040,           //Pending bit for line 6
        const PR7                               = 0x00000080,           //Pending bit for line 7
        const PR8                               = 0x00000100,           //Pending bit for line 8
        const PR9                               = 0x00000200,           //Pending bit for line 9
        const PR10                              = 0x00000400,           //Pending bit for line 10
        const PR11                              = 0x00000800,           //Pending bit for line 11
        const PR12                              = 0x00001000,           //Pending bit for line 12
        const PR13                              = 0x00002000,           //Pending bit for line 13
        const PR14                              = 0x00004000,           //Pending bit for line 14
        const PR15                              = 0x00008000,           //Pending bit for line 15
        const PR16                              = 0x00010000,           //Pending bit for line 16
        const PR17                              = 0x00020000,           //Pending bit for line 17
        const PR18                              = 0x00040000,           //Pending bit for line 18
        const PR19                              = 0x00080000,           //Pending bit for line 19
        const PR20                              = 0x00100000,           //Pending bit for line 20
        const PR21                              = 0x00200000,           //Pending bit for line 21
        const PR22                              = 0x00400000,           //Pending bit for line 22
        const PR23                              = 0x00800000,           //Pending bit for line 23
        const PR24                              = 0x01000000,           //Pending bit for line 24
        const PR25                              = 0x02000000,           //Pending bit for line 25
        const PR26                              = 0x04000000,           //Pending bit for line 26
        const PR27                              = 0x08000000,           //Pending bit for line 27
        const PR28                              = 0x10000000,           //Pending bit for line 28
        const PR29                              = 0x20000000,           //Pending bit for line 29
        const PR30                              = 0x40000000,           //Pending bit for line 30
        const PR31                              = 0x80000000,           //Pending bit for line 31
    }

    IMR2: u32 {
        const MR32                              = 0x00000001,           //Interrupt Mask on line 32
        const MR33                              = 0x00000002,           //Interrupt Mask on line 33
        const MR34                              = 0x00000004,           //Interrupt Mask on line 34
        const MR35                              = 0x00000008,           //Interrupt Mask on line 35
    }

    EMR2: u32 {
        const MR32                              = 0x00000001,           //Event Mask on line 32
        const MR33                              = 0x00000002,           //Event Mask on line 33
        const MR34                              = 0x00000004,           //Event Mask on line 34
        const MR35                              = 0x00000008,           //Event Mask on line 35
    }

    RTSR2: u32 {
        const TR32                              = 0x00000001,           //Rising trigger event configuration bit of line 32
        const TR33                              = 0x00000002,           //Rising trigger event configuration bit of line 33
    }

    FTSR2: u32 {
        const TR32                              = 0x00000001,           //Falling trigger event configuration bit of line 32
        const TR33                              = 0x00000002,           //Falling trigger event configuration bit of line 32
    }

    SWIER2: u32 {
        const SWIER32                           = 0x00000001,           //Software Interrupt on line 32
        const SWIER33                           = 0x00000002,           //Software Interrupt on line 32
    }

    PR2: u32 {
        const PR32                              = 0x00000001,           //Pending bit for line 32
        const PR33                              = 0x00000002,           //Pending bit for line 32
    }
}

//custom

