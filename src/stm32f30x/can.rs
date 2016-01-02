use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CAN_TxMailBox {
    pub TIR:                u32,                                        //CAN TX mailbox identifier register
    pub TDTR:               u32,                                        //CAN mailbox data length control and time stamp register
    pub TDLR:               u32,                                        //CAN mailbox data low register
    pub TDHR:               u32,                                        //CAN mailbox data high register
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CAN_FIFOMailBox {
    pub RIR:                u32,                                        //CAN receive FIFO mailbox identifier register
    pub RDTR:               u32,                                        //CAN receive FIFO mailbox data length control and time stamp register
    pub RDLR:               u32,                                        //CAN receive FIFO mailbox data low register
    pub RDHR:               u32,                                        //CAN receive FIFO mailbox data high register
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CAN_FilterRegister {
    pub FR1:                u32,                                        //CAN Filter bank register 1
    pub FR2:                u32,                                        //CAN Filter bank register 1
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CAN {
    pub MCR:                u32,                                        //CAN master control register,         Address offset: 0x00
    pub MSR:                u32,                                        //CAN master status register,          Address offset: 0x04
    pub TSR:                u32,                                        //CAN transmit status register,        Address offset: 0x08
    pub RF0R:               u32,                                        //CAN receive FIFO 0 register,         Address offset: 0x0C
    pub RF1R:               u32,                                        //CAN receive FIFO 1 register,         Address offset: 0x10
    pub IER:                u32,                                        //CAN interrupt enable register,       Address offset: 0x14
    pub ESR:                u32,                                        //CAN error status register,           Address offset: 0x18
    pub BTR:                u32,                                        //CAN bit timing register,             Address offset: 0x1C
    pub RESERVED0:          [u32; 88],                                  //Reserved, 0x020 - 0x17F
    pub sTxMailBox:         [CAN_TxMailBox; 3],                         //CAN Tx MailBox,                      Address offset: 0x180 - 0x1AC
    pub sFIFOMailBox:       [CAN_FIFOMailBox; 2],                       //CAN FIFO MailBox,                    Address offset: 0x1B0 - 0x1CC
    pub RESERVED1:          [u32; 12],                                  //Reserved, 0x1D0 - 0x1FF
    pub FMR:                u32,                                        //CAN filter master register,          Address offset: 0x200
    pub FM1R:               u32,                                        //CAN filter mode register,            Address offset: 0x204
    pub RESERVED2:          u32,                                        //Reserved, 0x208
    pub FS1R:               u32,                                        //CAN filter scale register,           Address offset: 0x20C
    pub RESERVED3:          u32,                                        //Reserved, 0x210
    pub FFA1R:              u32,                                        //CAN filter FIFO assignment register, Address offset: 0x214
    pub RESERVED4:          u32,                                        //Reserved, 0x218
    pub FA1R:               u32,                                        //CAN filter activation register,      Address offset: 0x21C
    pub RESERVED5:          [u32; 8],                                   //Reserved, 0x220-0x23F
    pub sFilterRegister:    [CAN_FilterRegister; 28],                   //CAN Filter Register,                 Address offset: 0x240-0x31C
}

//register addresses

registers! {
    const CAN1:             CAN                 = CAN1_BASE,
}

//bit definitions

constants! {
    MCR: u32 {
        const INRQ                              = 0x0001,               //Initialization Request
        const SLEEP                             = 0x0002,               //Sleep Mode Request
        const TXFP                              = 0x0004,               //Transmit FIFO Priority
        const RFLM                              = 0x0008,               //Receive FIFO Locked Mode
        const NART                              = 0x0010,               //No Automatic Retransmission
        const AWUM                              = 0x0020,               //Automatic Wakeup Mode
        const ABOM                              = 0x0040,               //Automatic Bus-Off Management
        const TTCM                              = 0x0080,               //Time Triggered Communication Mode
        const RESET                             = 0x8000,               //bxCAN software master reset
    }

    MSR: u32 {
        const INAK                              = 0x0001,               //Initialization Acknowledge
        const SLAK                              = 0x0002,               //Sleep Acknowledge
        const ERRI                              = 0x0004,               //Error Interrupt
        const WKUI                              = 0x0008,               //Wakeup Interrupt
        const SLAKI                             = 0x0010,               //Sleep Acknowledge Interrupt
        const TXM                               = 0x0100,               //Transmit Mode
        const RXM                               = 0x0200,               //Receive Mode
        const SAMP                              = 0x0400,               //Last Sample Point
        const RX                                = 0x0800,               //CAN Rx Signal
    }

    TSR: u32 {
        const RQCP0                             = 0x00000001,           //Request Completed Mailbox0
        const TXOK0                             = 0x00000002,           //Transmission OK of Mailbox0
        const ALST0                             = 0x00000004,           //Arbitration Lost for Mailbox0
        const TERR0                             = 0x00000008,           //Transmission Error of Mailbox0
        const ABRQ0                             = 0x00000080,           //Abort Request for Mailbox0
        const RQCP1                             = 0x00000100,           //Request Completed Mailbox1
        const TXOK1                             = 0x00000200,           //Transmission OK of Mailbox1
        const ALST1                             = 0x00000400,           //Arbitration Lost for Mailbox1
        const TERR1                             = 0x00000800,           //Transmission Error of Mailbox1
        const ABRQ1                             = 0x00008000,           //Abort Request for Mailbox 1
        const RQCP2                             = 0x00010000,           //Request Completed Mailbox2
        const TXOK2                             = 0x00020000,           //Transmission OK of Mailbox 2
        const ALST2                             = 0x00040000,           //Arbitration Lost for mailbox 2
        const TERR2                             = 0x00080000,           //Transmission Error of Mailbox 2
        const ABRQ2                             = 0x00800000,           //Abort Request for Mailbox 2
        const CODE                              = 0x03000000,           //Mailbox Code
        const TME                               = 0x1C000000,           //TME[2:0] bits
        const TME0                              = 0x04000000,           //Transmit Mailbox 0 Empty
        const TME1                              = 0x08000000,           //Transmit Mailbox 1 Empty
        const TME2                              = 0x10000000,           //Transmit Mailbox 2 Empty
        const LOW                               = 0xE0000000,           //LOW[2:0] bits
        const LOW0                              = 0x20000000,           //Lowest Priority Flag for Mailbox 0
        const LOW1                              = 0x40000000,           //Lowest Priority Flag for Mailbox 1
        const LOW2                              = 0x80000000,           //Lowest Priority Flag for Mailbox 2
    }

    RF0R: u32 {
        const FMP0                              = 0x03,                 //FIFO 0 Message Pending
        const FULL0                             = 0x08,                 //FIFO 0 Full
        const FOVR0                             = 0x10,                 //FIFO 0 Overrun
        const RFOM0                             = 0x20,                 //Release FIFO 0 Output Mailbox
    }

    RF1R: u32 {
        const FMP1                              = 0x03,                 //FIFO 1 Message Pending
        const FULL1                             = 0x08,                 //FIFO 1 Full
        const FOVR1                             = 0x10,                 //FIFO 1 Overrun
        const RFOM1                             = 0x20,                 //Release FIFO 1 Output Mailbox
    }

    IER: u32 {
        const TMEIE                             = 0x00000001,           //Transmit Mailbox Empty Interrupt Enable
        const FMPIE0                            = 0x00000002,           //FIFO Message Pending Interrupt Enable
        const FFIE0                             = 0x00000004,           //FIFO Full Interrupt Enable
        const FOVIE0                            = 0x00000008,           //FIFO Overrun Interrupt Enable
        const FMPIE1                            = 0x00000010,           //FIFO Message Pending Interrupt Enable
        const FFIE1                             = 0x00000020,           //FIFO Full Interrupt Enable
        const FOVIE1                            = 0x00000040,           //FIFO Overrun Interrupt Enable
        const EWGIE                             = 0x00000100,           //Error Warning Interrupt Enable
        const EPVIE                             = 0x00000200,           //Error Passive Interrupt Enable
        const BOFIE                             = 0x00000400,           //Bus-Off Interrupt Enable
        const LECIE                             = 0x00000800,           //Last Error Code Interrupt Enable
        const ERRIE                             = 0x00008000,           //Error Interrupt Enable
        const WKUIE                             = 0x00010000,           //Wakeup Interrupt Enable
        const SLKIE                             = 0x00020000,           //Sleep Interrupt Enable
    }

    ESR: u32 {
        const EWGF                              = 0x00000001,           //Error Warning Flag
        const EPVF                              = 0x00000002,           //Error Passive Flag
        const BOFF                              = 0x00000004,           //Bus-Off Flag
        const LEC                               = 0x00000070,           //LEC[2:0] bits (Last Error Code)
        const LEC_0                             = 0x00000010,           //Bit 0
        const LEC_1                             = 0x00000020,           //Bit 1
        const LEC_2                             = 0x00000040,           //Bit 2
        const TEC                               = 0x00FF0000,           //Least significant byte of the 9-bit Transmit Error Counter
        const REC                               = 0xFF000000,           //Receive Error Counter
    }

    BTR: u32 {
        const BRP                               = 0x000003FF,           //Baud Rate Prescaler
        const TS1                               = 0x000F0000,           //Time Segment 1
        const TS2                               = 0x00700000,           //Time Segment 2
        const SJW                               = 0x03000000,           //Resynchronization Jump Width
        const LBKM                              = 0x40000000,           //Loop Back Mode (Debug)
        const SILM                              = 0x80000000,           //Silent Mode
    }

    FMR: u32 {
        const FINIT                             = 0x01,                 //Filter Init Mode
    }

    FM1R: u32 {
        const FBM                               = 0x3FFF,               //Filter Mode
        const FBM0                              = 0x0001,               //Filter Init Mode bit 0
        const FBM1                              = 0x0002,               //Filter Init Mode bit 1
        const FBM2                              = 0x0004,               //Filter Init Mode bit 2
        const FBM3                              = 0x0008,               //Filter Init Mode bit 3
        const FBM4                              = 0x0010,               //Filter Init Mode bit 4
        const FBM5                              = 0x0020,               //Filter Init Mode bit 5
        const FBM6                              = 0x0040,               //Filter Init Mode bit 6
        const FBM7                              = 0x0080,               //Filter Init Mode bit 7
        const FBM8                              = 0x0100,               //Filter Init Mode bit 8
        const FBM9                              = 0x0200,               //Filter Init Mode bit 9
        const FBM10                             = 0x0400,               //Filter Init Mode bit 10
        const FBM11                             = 0x0800,               //Filter Init Mode bit 11
        const FBM12                             = 0x1000,               //Filter Init Mode bit 12
        const FBM13                             = 0x2000,               //Filter Init Mode bit 13
    }

    FS1R: u32 {
        const FSC                               = 0x3FFF,               //Filter Scale Configuration
        const FSC0                              = 0x0001,               //Filter Scale Configuration bit 0
        const FSC1                              = 0x0002,               //Filter Scale Configuration bit 1
        const FSC2                              = 0x0004,               //Filter Scale Configuration bit 2
        const FSC3                              = 0x0008,               //Filter Scale Configuration bit 3
        const FSC4                              = 0x0010,               //Filter Scale Configuration bit 4
        const FSC5                              = 0x0020,               //Filter Scale Configuration bit 5
        const FSC6                              = 0x0040,               //Filter Scale Configuration bit 6
        const FSC7                              = 0x0080,               //Filter Scale Configuration bit 7
        const FSC8                              = 0x0100,               //Filter Scale Configuration bit 8
        const FSC9                              = 0x0200,               //Filter Scale Configuration bit 9
        const FSC10                             = 0x0400,               //Filter Scale Configuration bit 10
        const FSC11                             = 0x0800,               //Filter Scale Configuration bit 11
        const FSC12                             = 0x1000,               //Filter Scale Configuration bit 12
        const FSC13                             = 0x2000,               //Filter Scale Configuration bit 13
    }

    FFA1R: u32 {
        const FFA                               = 0x3FFF,               //Filter FIFO Assignment
        const FFA0                              = 0x0001,               //Filter FIFO Assignment for Filter 0
        const FFA1                              = 0x0002,               //Filter FIFO Assignment for Filter 1
        const FFA2                              = 0x0004,               //Filter FIFO Assignment for Filter 2
        const FFA3                              = 0x0008,               //Filter FIFO Assignment for Filter 3
        const FFA4                              = 0x0010,               //Filter FIFO Assignment for Filter 4
        const FFA5                              = 0x0020,               //Filter FIFO Assignment for Filter 5
        const FFA6                              = 0x0040,               //Filter FIFO Assignment for Filter 6
        const FFA7                              = 0x0080,               //Filter FIFO Assignment for Filter 7
        const FFA8                              = 0x0100,               //Filter FIFO Assignment for Filter 8
        const FFA9                              = 0x0200,               //Filter FIFO Assignment for Filter 9
        const FFA10                             = 0x0400,               //Filter FIFO Assignment for Filter 10
        const FFA11                             = 0x0800,               //Filter FIFO Assignment for Filter 11
        const FFA12                             = 0x1000,               //Filter FIFO Assignment for Filter 12
        const FFA13                             = 0x2000,               //Filter FIFO Assignment for Filter 13
    }

    FA1R: u32 {
        const FACT                              = 0x3FFF,               //Filter Active
        const FACT0                             = 0x0001,               //Filter 0 Active
        const FACT1                             = 0x0002,               //Filter 1 Active
        const FACT2                             = 0x0004,               //Filter 2 Active
        const FACT3                             = 0x0008,               //Filter 3 Active
        const FACT4                             = 0x0010,               //Filter 4 Active
        const FACT5                             = 0x0020,               //Filter 5 Active
        const FACT6                             = 0x0040,               //Filter 6 Active
        const FACT7                             = 0x0080,               //Filter 7 Active
        const FACT8                             = 0x0100,               //Filter 8 Active
        const FACT9                             = 0x0200,               //Filter 9 Active
        const FACT10                            = 0x0400,               //Filter 10 Active
        const FACT11                            = 0x0800,               //Filter 11 Active
        const FACT12                            = 0x1000,               //Filter 12 Active
        const FACT13                            = 0x2000,               //Filter 13 Active
    }
}

//custom

