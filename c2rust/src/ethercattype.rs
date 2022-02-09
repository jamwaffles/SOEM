/*
 * Licensed under the GNU General Public License version 2 with exceptions. See
 * LICENSE file in the project root for full license information
 */

/** \file
 * \brief
 * General typedefs and defines for EtherCAT.
 *
 * Defines that could need optimisation for specific applications
 * are the EC_TIMEOUTxxx. Assumptions for the standard settings are a
 * standard linux PC or laptop and a wired connection to maximal 100 slaves.
 * For use with wireless connections or lots of slaves the timeouts need
 * increasing. For fast systems running Xenomai and RT-net or alike the
 * timeouts need to be shorter.
 */

/** return value no frame returned */
pub const EC_NOFRAME: i32 = -1;
/** return value unknown frame received */
pub const EC_OTHERFRAME: i32 = -2;
/** return value general error */
pub const EC_ERROR: i32 = -3;
/** return value too many slaves */
pub const EC_SLAVECOUNTEXCEEDED: i32 = -4;
/** return value request timeout */
pub const EC_TIMEOUT: i32 = -5;
/** maximum EtherCAT frame length in bytes */
pub const EC_MAXECATFRAME: usize = 1518;
/** maximum EtherCAT LRW frame length in bytes */
/* MTU - Ethernet header - length - datagram header - WCK - FCS */
pub const EC_MAXLRWDATA: usize = EC_MAXECATFRAME - 14 - 2 - 10 - 2 - 4;
/** size of DC datagram used in first LRW frame */
pub const EC_FIRSTDCDATAGRAM: i32 = 20;
/** standard frame buffer size in bytes */
pub const EC_BUFSIZE: usize = EC_MAXECATFRAME;
/** datagram type EtherCAT */
pub const EC_ECATTYPE: i32 = 0x1000;
/** number of frame buffers per channel (tx, rx1 rx2) */
pub const EC_MAXBUF: i32 = 16;
/** timeout value in us for tx frame to return to rx */
pub const EC_TIMEOUTRET: i32 = 2000;
/** timeout value in us for safe data transfer, max. triple retry */
pub const EC_TIMEOUTRET3: i32 = EC_TIMEOUTRET * 3;
/** timeout value in us for return "safe" variant (f.e. wireless) */
pub const EC_TIMEOUTSAFE: i32 = 20000;
/** timeout value in us for EEPROM access */
pub const EC_TIMEOUTEEP: i32 = 20000;
/** timeout value in us for tx mailbox cycle */
pub const EC_TIMEOUTTXM: i32 = 20000;
/** timeout value in us for rx mailbox cycle */
pub const EC_TIMEOUTRXM: i32 = 700000;
/** timeout value in us for check statechange */
pub const EC_TIMEOUTSTATE: i32 = 2000000;
/** size of EEPROM bitmap cache */
pub const EC_MAXEEPBITMAP: i32 = 128;
/** size of EEPROM cache buffer */
pub const EC_MAXEEPBUF: i32 = EC_MAXEEPBITMAP << 5;
/** default number of retries if wkc <= 0 */
pub const EC_DEFAULTRETRIES: i32 = 3;
/** default group size in 2^x */
pub const EC_LOGGROUPOFFSET: i32 = 16;

/** definition for frame buffers */
pub type ec_bufT = [u8; EC_BUFSIZE];

/** ethernet header definition */
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_etherheadert {
    /** destination MAC */
    da0: u16,
    da1: u16,
    da2: u16,
    /** source MAC */
    sa0: u16,
    sa1: u16,
    sa2: u16,
    /** ethernet type */
    etype: u16,
}

/** ethernet header size */
pub const ETH_HEADERSIZE: usize = size_of::<ec_etherheadert>();

/** EtherCAT datagram header definition */
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_comt {
    /** length of EtherCAT datagram */
    elength: u16,
    /** EtherCAT command, see ec_cmdtype */
    command: u8,
    /** index, used in SOEM for Tx to Rx recombination */
    index: u8,
    /** ADP */
    ADP: u16,
    /** ADO */
    ADO: u16,
    /** length of data portion in datagram */
    dlength: u16,
    /** interrupt, currently unused */
    irpt: u16,
}

/** EtherCAT header size */
pub const EC_HEADERSIZE: usize = size_of::<ec_comt>();
/** size of ec_comt.elength item in EtherCAT header */
pub const EC_ELENGTHSIZE: usize = size_of::<u16>();
/** offset position of command in EtherCAT header */
pub const EC_CMDOFFSET: usize = EC_ELENGTHSIZE;
/** size of workcounter item in EtherCAT datagram */
pub const EC_WKCSIZE: usize = size_of::<u16>();
/** definition of datagram follows bit in ec_comt.dlength */
pub const EC_DATAGRAMFOLLOWS: i32 = 1 << 15;

/** Possible error codes returned. */
pub enum ec_err {
    /** No error */
    EC_ERR_OK = 0,
    /** Library already initialized. */
    EC_ERR_ALREADY_INITIALIZED,
    /** Library not initialized. */
    EC_ERR_NOT_INITIALIZED,
    /** Timeout occurred during execution of the function. */
    EC_ERR_TIMEOUT,
    /** No slaves were found. */
    EC_ERR_NO_SLAVES,
    /** Function failed. */
    EC_ERR_NOK,
}

/** Possible EtherCAT slave states */
pub enum ec_state {
    /** No valid state. */
    EC_STATE_NONE = 0x00,
    /** Init state*/
    EC_STATE_INIT = 0x01,
    /** Pre-operational. */
    EC_STATE_PRE_OP = 0x02,
    /** Boot state*/
    EC_STATE_BOOT = 0x03,
    /** Safe-operational. */
    EC_STATE_SAFE_OP = 0x04,
    /** Operational */
    EC_STATE_OPERATIONAL = 0x08,
    /** Error or ACK error */
    // EC_STATE_ACK = 0x10,
    EC_STATE_ERROR = 0x10,
}

/** Possible buffer states */
pub enum ec_bufstate {
    /** Empty */
    EC_BUF_EMPTY = 0x00,
    /** Allocated, but not filled */
    EC_BUF_ALLOC = 0x01,
    /** Transmitted */
    EC_BUF_TX = 0x02,
    /** Received, but not consumed */
    EC_BUF_RCVD = 0x03,
    /** Cycle completed */
    EC_BUF_COMPLETE = 0x04,
}

/** Ethercat data types */
pub enum ec_datatype {
    ECT_BOOLEAN = 0x0001,
    ECT_INTEGER8 = 0x0002,
    ECT_INTEGER16 = 0x0003,
    ECT_INTEGER32 = 0x0004,
    ECT_UNSIGNED8 = 0x0005,
    ECT_UNSIGNED16 = 0x0006,
    ECT_UNSIGNED32 = 0x0007,
    ECT_REAL32 = 0x0008,
    ECT_VISIBLE_STRING = 0x0009,
    ECT_OCTET_STRING = 0x000A,
    ECT_UNICODE_STRING = 0x000B,
    ECT_TIME_OF_DAY = 0x000C,
    ECT_TIME_DIFFERENCE = 0x000D,
    ECT_DOMAIN = 0x000F,
    ECT_INTEGER24 = 0x0010,
    ECT_REAL64 = 0x0011,
    ECT_INTEGER64 = 0x0015,
    ECT_UNSIGNED24 = 0x0016,
    ECT_UNSIGNED64 = 0x001B,
    ECT_BIT1 = 0x0030,
    ECT_BIT2 = 0x0031,
    ECT_BIT3 = 0x0032,
    ECT_BIT4 = 0x0033,
    ECT_BIT5 = 0x0034,
    ECT_BIT6 = 0x0035,
    ECT_BIT7 = 0x0036,
    ECT_BIT8 = 0x0037,
}

/** Ethercat command types */
pub enum ec_cmdtype {
    /** No operation */
    EC_CMD_NOP = 0x00,
    /** Auto Increment Read */
    EC_CMD_APRD,
    /** Auto Increment Write */
    EC_CMD_APWR,
    /** Auto Increment Read Write */
    EC_CMD_APRW,
    /** Configured Address Read */
    EC_CMD_FPRD,
    /** Configured Address Write */
    EC_CMD_FPWR,
    /** Configured Address Read Write */
    EC_CMD_FPRW,
    /** Broadcast Read */
    EC_CMD_BRD,
    /** Broadcast Write */
    EC_CMD_BWR,
    /** Broadcast Read Write */
    EC_CMD_BRW,
    /** Logical Memory Read */
    EC_CMD_LRD,
    /** Logical Memory Write */
    EC_CMD_LWR,
    /** Logical Memory Read Write */
    EC_CMD_LRW,
    /** Auto Increment Read Multiple Write */
    EC_CMD_ARMW,
    /** Configured Read Multiple Write */
    EC_CMD_FRMW,
}
/** Ethercat EEprom command types */
pub enum ec_ecmdtype {
    /** No operation */
    EC_ECMD_NOP = 0x0000,
    /** Read */
    EC_ECMD_READ = 0x0100,
    /** Write */
    EC_ECMD_WRITE = 0x0201,
    /** Reload */
    EC_ECMD_RELOAD = 0x0300,
}

/** EEprom state machine read size */
pub const EC_ESTAT_R64: i32 = 0x0040;
/** EEprom state machine busy flag */
pub const EC_ESTAT_BUSY: i32 = 0x8000;
/** EEprom state machine error flag mask */
pub const EC_ESTAT_EMASK: i32 = 0x7800;
/** EEprom state machine error acknowledge */
pub const EC_ESTAT_NACK: i32 = 0x2000;

/* Ethercat SSI (Slave Information Interface) */

use std::mem::size_of;

use crate::osal::linux::osal::ec_timet;

/** Start address SII sections in Eeprom */
pub const ECT_SII_START: i32 = 0x0040;

pub enum SIICategory {
    /** SII category strings */
    ECT_SII_STRING = 10,
    /** SII category general */
    ECT_SII_GENERAL = 30,
    /** SII category FMMU */
    ECT_SII_FMMU = 40,
    /** SII category SM */
    ECT_SII_SM = 41,
    /** SII category PDO */
    ECT_SII_PDO = 50,
}

/** Item offsets in SII general section */
pub enum SIIGeneral {
    ECT_SII_MANUF = 0x0008,
    ECT_SII_ID = 0x000a,
    ECT_SII_REV = 0x000c,
    ECT_SII_BOOTRXMBX = 0x0014,
    ECT_SII_BOOTTXMBX = 0x0016,
    ECT_SII_MBXSIZE = 0x0019,
    ECT_SII_TXMBXADR = 0x001a,
    ECT_SII_RXMBXADR = 0x0018,
    ECT_SII_MBXPROTO = 0x001c,
}

/** Mailbox types definitions */
pub enum MailboxType {
    /** Error mailbox type */
    ECT_MBXT_ERR = 0x00,
    /** ADS over EtherCAT mailbox type */
    ECT_MBXT_AOE = 0x01,
    /** Ethernet over EtherCAT mailbox type */
    ECT_MBXT_EOE = 0x02,
    /** CANopen over EtherCAT mailbox type */
    ECT_MBXT_COE = 0x03,
    /** File over EtherCAT mailbox type */
    ECT_MBXT_FOE = 0x04,
    /** Servo over EtherCAT mailbox type */
    ECT_MBXT_SOE = 0x05,
    /** Vendor over EtherCAT mailbox type */
    ECT_MBXT_VOE = 0x0f,
}

/** CoE mailbox types */
pub enum CoEMailboxType {
    ECT_COES_EMERGENCY = 0x01,
    ECT_COES_SDOREQ,
    ECT_COES_SDORES,
    ECT_COES_TXPDO,
    ECT_COES_RXPDO,
    ECT_COES_TXPDO_RR,
    ECT_COES_RXPDO_RR,
    ECT_COES_SDOINFO,
}

/** CoE SDO commands */
pub enum CoESDOCommand {
    ECT_SDO_DOWN_INIT = 0x21,
    ECT_SDO_DOWN_EXP = 0x23,
    ECT_SDO_DOWN_INIT_CA = 0x31,
    ECT_SDO_UP_REQ = 0x40,
    ECT_SDO_UP_REQ_CA = 0x50,
    ECT_SDO_SEG_UP_REQ = 0x60,
    ECT_SDO_ABORT = 0x80,
}

/** CoE Object Description commands */
pub enum CoEObjectDescription {
    ECT_GET_ODLIST_REQ = 0x01,
    ECT_GET_ODLIST_RES = 0x02,
    ECT_GET_OD_REQ = 0x03,
    ECT_GET_OD_RES = 0x04,
    ECT_GET_OE_REQ = 0x05,
    ECT_GET_OE_RES = 0x06,
    ECT_SDOINFO_ERROR = 0x07,
}

/** FoE opcodes */
pub enum FoEOpCode {
    ECT_FOE_READ = 0x01,
    ECT_FOE_WRITE,
    ECT_FOE_DATA,
    ECT_FOE_ACK,
    ECT_FOE_ERROR,
    ECT_FOE_BUSY,
}

/** SoE opcodes */
pub enum SoEOpCode {
    ECT_SOE_READREQ = 0x01,
    ECT_SOE_READRES = 0x02,
    ECT_SOE_WRITEREQ = 0x03,
    ECT_SOE_WRITERES = 0x04,
    ECT_SOE_NOTIFICATION = 0x05,
    ECT_SOE_EMERGENCY = 0x06,
}

/** Ethercat registers */
pub enum EthercatRegister {
    ECT_REG_TYPE = 0x0000,
    ECT_REG_PORTDES = 0x0007,
    ECT_REG_ESCSUP = 0x0008,
    ECT_REG_STADR = 0x0010,
    ECT_REG_ALIAS = 0x0012,
    ECT_REG_DLCTL = 0x0100,
    ECT_REG_DLPORT = 0x0101,
    ECT_REG_DLALIAS = 0x0103,
    ECT_REG_DLSTAT = 0x0110,
    ECT_REG_ALCTL = 0x0120,
    ECT_REG_ALSTAT = 0x0130,
    ECT_REG_ALSTATCODE = 0x0134,
    ECT_REG_PDICTL = 0x0140,
    ECT_REG_IRQMASK = 0x0200,
    ECT_REG_RXERR = 0x0300,
    ECT_REG_FRXERR = 0x0308,
    ECT_REG_EPUECNT = 0x030C,
    ECT_REG_PECNT = 0x030D,
    ECT_REG_PECODE = 0x030E,
    ECT_REG_LLCNT = 0x0310,
    ECT_REG_WDCNT = 0x0442,
    ECT_REG_EEPCFG = 0x0500,
    ECT_REG_EEPCTL = 0x0502,
    ECT_REG_EEPSTAT = 0x0503,
    ECT_REG_EEPADR = 0x0504,
    ECT_REG_EEPDAT = 0x0508,
    ECT_REG_FMMU0 = 0x0600,
    ECT_REG_FMMU1 = EthercatRegister::ECT_REG_FMMU0 as isize + 0x10,
    ECT_REG_FMMU2 = EthercatRegister::ECT_REG_FMMU1 as isize + 0x10,
    ECT_REG_FMMU3 = EthercatRegister::ECT_REG_FMMU2 as isize + 0x10,
    ECT_REG_SM0 = 0x0800,
    ECT_REG_SM1 = EthercatRegister::ECT_REG_SM0 as isize + 0x08,
    ECT_REG_SM2 = EthercatRegister::ECT_REG_SM1 as isize + 0x08,
    ECT_REG_SM3 = EthercatRegister::ECT_REG_SM2 as isize + 0x08,
    ECT_REG_SM0STAT = EthercatRegister::ECT_REG_SM0 as isize + 0x05,
    ECT_REG_SM1STAT = EthercatRegister::ECT_REG_SM1 as isize + 0x05,
    ECT_REG_SM1ACT = EthercatRegister::ECT_REG_SM1 as isize + 0x06,
    ECT_REG_SM1CONTR = EthercatRegister::ECT_REG_SM1 as isize + 0x07,
    ECT_REG_DCTIME0 = 0x0900,
    ECT_REG_DCTIME1 = 0x0904,
    ECT_REG_DCTIME2 = 0x0908,
    ECT_REG_DCTIME3 = 0x090C,
    ECT_REG_DCSYSTIME = 0x0910,
    ECT_REG_DCSOF = 0x0918,
    ECT_REG_DCSYSOFFSET = 0x0920,
    ECT_REG_DCSYSDELAY = 0x0928,
    ECT_REG_DCSYSDIFF = 0x092C,
    ECT_REG_DCSPEEDCNT = 0x0930,
    ECT_REG_DCTIMEFILT = 0x0934,
    ECT_REG_DCCUC = 0x0980,
    ECT_REG_DCSYNCACT = 0x0981,
    ECT_REG_DCSTART0 = 0x0990,
    ECT_REG_DCCYCLE0 = 0x09A0,
    ECT_REG_DCCYCLE1 = 0x09A4,
}

/** standard SDO Sync Manager Communication Type */
pub const ECT_SDO_SMCOMMTYPE: i32 = 0x1c00;
/** standard SDO PDO assignment */
pub const ECT_SDO_PDOASSIGN: i32 = 0x1c10;
/** standard SDO RxPDO assignment */
pub const ECT_SDO_RXPDOASSIGN: i32 = 0x1c12;
/** standard SDO TxPDO assignment */
pub const ECT_SDO_TXPDOASSIGN: i32 = 0x1c13;

/** Ethercat packet type */
pub const ETH_P_ECAT: i32 = 0x88A4;

/** Error types */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ec_err_type {
    EC_ERR_TYPE_SDO_ERROR = 0,
    EC_ERR_TYPE_EMERGENCY = 1,
    EC_ERR_TYPE_PACKET_ERROR = 3,
    EC_ERR_TYPE_SDOINFO_ERROR = 4,
    EC_ERR_TYPE_FOE_ERROR = 5,
    EC_ERR_TYPE_FOE_BUF2SMALL = 6,
    EC_ERR_TYPE_FOE_PACKETNUMBER = 7,
    EC_ERR_TYPE_SOE_ERROR = 8,
    EC_ERR_TYPE_MBX_ERROR = 9,
    EC_ERR_TYPE_FOE_FILE_NOTFOUND = 10,
    EC_ERR_TYPE_EOE_INVALID_RX_DATA = 11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_errort {
    pub Time: ec_timet,
    pub Signal: bool,
    pub Slave: u16,
    pub Index: u16,
    pub SubIdx: u8,
    pub Etype: ec_err_type,
    pub c2rust_unnamed: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub AbortCode: i32,
    pub c2rust_unnamed: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub ErrorCode: u16,
    pub ErrorReg: u8,
    pub b1: u8,
    pub w1: u16,
    pub w2: u16,
}

///
/// NOOP functions as we're only going to support LE systems
///

pub fn htoes<T>(input: T) -> T {
    input
}
pub fn htoel<T>(input: T) -> T {
    input
}
pub fn htoell<T>(input: T) -> T {
    input
}
pub fn etohs<T>(input: T) -> T {
    input
}
pub fn etohl<T>(input: T) -> T {
    input
}
pub fn etohll<T>(input: T) -> T {
    input
}

pub fn htons(u: u16) -> u16 {
    u.to_be()
}
pub fn ntohs(u: u16) -> u16 {
    u16::from_be(u)
}
