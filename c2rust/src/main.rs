use crate::{
    base::{
        ecx_APRD, ecx_APWR, ecx_BRD, ecx_BWR, ecx_FPRD, ecx_FPWR, ecx_FPWRw, ecx_adddatagram,
        ecx_setupdatagram,
    },
    osal::linux::osal::{
        ec_timet, osal_current_time, osal_timer_is_expired, osal_timer_start, osal_timert,
        osal_usleep,
    },
    oshw::linux::{
        nicdrv::{
            ec_stackT, ecx_closenic, ecx_getindex, ecx_outframe_red, ecx_portt, ecx_redportt,
            ecx_setbufstat, ecx_setupnic, ecx_srconfirm, ecx_waitinframe, secMAC,
        },
        oshw::{oshw_find_adapters, oshw_free_adapters, oshw_htons},
    },
    types::{
        ec_bufT, ec_err_type, ec_errort, BufferState, C2RustUnnamed_0, Command, EepromCommand,
        EthercatHeader, EthercatRegister, EthernetHeader, MailboxType, SiiCategory, SlaveState,
        EC_DEFAULTRETRIES, EC_ESTAT_EMASK, EC_ESTAT_NACK, EC_ESTAT_R64, EC_TIMEOUTEEP,
        EC_TIMEOUTRET, EC_TIMEOUTRET3,
    },
};
use libc::{memcpy, memset, pthread_mutex_t};
use std::mem;

/** max. entries in EtherCAT error list */
pub const EC_MAXELIST: u16 = 64;
/** max. length of readable name in slavelist and Object Description List */
pub const EC_MAXNAME: usize = 40;
/** max. number of slaves in array */
pub const EC_MAXSLAVE: u16 = 200;
/** max. number of groups */
pub const EC_MAXGROUP: u16 = 2;
/** max. number of IO segments per group */
pub const EC_MAXIOSEGMENTS: u16 = 64;
/** max. mailbox size */
pub const EC_MAXMBX: usize = 1486;
/** max. eeprom PDO entries */
pub const EC_MAXEEPDO: u16 = 0x200;
/** max. SM used */
pub const EC_MAXSM: usize = 8;
/** max. FMMU used */
pub const EC_MAXFMMU: usize = 4;
/** max. Adapter */
pub const EC_MAXLEN_ADAPTERNAME: u16 = 128;
/** define maximum number of concurrent threads in mapping */
pub const EC_MAX_MAPT: u16 = 1;
/** delay in us for eeprom ready loop */
pub const EC_LOCALDELAY: u32 = 200;

pub const ECT_MBXPROT_AOE: i32 = 0x0001;
pub const ECT_MBXPROT_EOE: i32 = 0x0002;
pub const ECT_MBXPROT_COE: i32 = 0x0004;
pub const ECT_MBXPROT_FOE: i32 = 0x0008;
pub const ECT_MBXPROT_SOE: i32 = 0x0010;
pub const ECT_MBXPROT_VOE: i32 = 0x0020;

pub const ECT_COEDET_SDO: i32 = 0x01;
pub const ECT_COEDET_SDOINFO: i32 = 0x02;
pub const ECT_COEDET_PDOASSIGN: i32 = 0x04;
pub const ECT_COEDET_PDOCONFIG: i32 = 0x08;
pub const ECT_COEDET_UPLOAD: i32 = 0x10;
pub const ECT_COEDET_SDOCA: i32 = 0x20;

pub const EC_SMENABLEMASK: u32 = 0xfffeffff;

#[derive(Copy, Clone)]
pub struct ec_adapter {
    pub name: [libc::c_char; 128],
    pub desc: [libc::c_char; 128],
    pub next: *mut ec_adaptert,
}
pub type ec_adaptert = ec_adapter;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct ec_fmmu {
    pub LogStart: u32,
    pub LogLength: u16,
    pub LogStartbit: u8,
    pub LogEndbit: u8,
    pub PhysStart: u16,
    pub PhysStartBit: u8,
    pub FMMUtype: u8,
    pub FMMUactive: u8,
    pub unused1: u8,
    pub unused2: u16,
}
pub type ec_fmmut = ec_fmmu;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct ec_sm {
    pub StartAddr: u16,
    pub SMlength: u16,
    pub SMflags: u32,
}
pub type ec_smt = ec_sm;

#[derive(Clone)]
pub struct ecx_context {
    /// port reference, may include red_port
    pub port: *mut ecx_portt,
    /// slavelist reference
    pub slavelist: heapless::Vec<ec_slave, { EC_MAXSLAVE as usize }>,
    /// number of slaves found in configuration
    pub slavecount: u16,
    /// maximum number of slaves allowed in slavelist
    pub maxslave: libc::c_int,
    /// grouplist reference
    pub grouplist: *mut ec_groupt,
    /// maximum number of groups allowed in grouplist
    pub maxgroup: libc::c_int,
    /// internal, reference to eeprom cache buffer
    pub esibuf: *mut u8,
    /// internal, reference to eeprom cache map
    pub esimap: *mut u32,
    /// internal, current slave for eeprom cache
    pub esislave: u16,
    /// internal, reference to error list
    // TODO: Use a ringbuffer instead, looks like that was the previous implementation
    pub elist: heapless::Vec<ec_errort, { EC_MAXELIST as usize }>,
    /// internal, reference to processdata stack buffer info
    pub idxstack: *mut ec_idxstackT,
    /// reference to ecaterror state
    pub ecaterror: bool,
    /// reference to last DC time from slaves
    pub DCtime: *mut i64,
    /// internal, SM buffer
    pub SMcommtype: *mut ec_SMcommtypet,
    /// internal, PDO assign list
    pub PDOassign: *mut ec_PDOassignt,
    /// internal, PDO description list
    pub PDOdesc: *mut ec_PDOdesct,
    /// internal, SM list from eeprom
    pub eepSM: *mut ec_eepromSMt,
    /// internal, FMMU list from eeprom
    pub eepFMMU: *mut ec_eepromFMMUt,
    /// registered FoE hook
    pub FOEhook: Option<unsafe fn(_: u16, _: libc::c_int, _: libc::c_int) -> libc::c_int>,
    /// registered EoE hook
    pub EOEhook:
        Option<unsafe fn(_: *mut ecx_contextt, _: u16, _: *mut libc::c_void) -> libc::c_int>,
    /// flag to control legacy automatic state change or manual state change
    pub manualstatechange: libc::c_int,
    /// userdata, promotes application configuration esp. in EC_VER2 with multiple *ec_context
    /// instances. Note: userdata memory is managed by application, not SOEM
    pub userdata: *mut libc::c_void,
}
pub type ecx_contextt = ecx_context;
pub type ec_eepromFMMUt = ec_eepromFMMU;

#[derive(Copy, Clone)]
pub struct ec_eepromFMMU {
    pub Startpos: u16,
    pub nFMMU: u8,
    pub FMMU0: u8,
    pub FMMU1: u8,
    pub FMMU2: u8,
    pub FMMU3: u8,
}
pub type ec_eepromSMt = ec_eepromSM;

#[derive(Copy, Clone)]
pub struct ec_eepromSM {
    pub Startpos: u16,
    pub nSM: u8,
    pub PhStart: u16,
    pub Plength: u16,
    pub Creg: u8,
    pub Sreg: u8,
    pub Activate: u8,
    pub PDIctrl: u8,
}

pub type ec_PDOdesct = ec_PDOdesc;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_PDOdesc {
    pub n: u8,
    pub nu1: u8,
    pub PDO: [u32; 256],
}

pub type ec_PDOassignt = ec_PDOassign;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_PDOassign {
    pub n: u8,
    pub nu1: u8,
    pub index: [u16; 256],
}

pub type ec_SMcommtypet = ec_SMcommtype;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_SMcommtype {
    pub n: u8,
    pub nu1: u8,
    pub SMtype: [u8; 8],
}

pub type ec_idxstackT = ec_idxstack;

#[derive(Copy, Clone)]
pub struct ec_idxstack {
    pub pushed: u8,
    pub pulled: u8,
    pub idx: [u8; 16],
    pub data: [*mut libc::c_void; 16],
    pub length: [u16; 16],
    pub dcoffset: [u16; 16],
}

pub type ec_eringt = ec_ering;

#[derive(Copy, Clone)]
pub struct ec_ering {
    pub head: i16,
    pub tail: i16,
    pub Error: [ec_errort; EC_MAXELIST as usize],
}

pub type ec_groupt = ec_group;

#[derive(Copy, Clone)]
pub struct ec_group {
    pub logstartaddr: u32,
    pub Obytes: u32,
    pub outputs: *mut u8,
    pub Ibytes: u32,
    pub inputs: *mut u8,
    pub hasdc: bool,
    pub DCnext: u16,
    pub Ebuscurrent: i16,
    pub blockLRW: u8,
    pub nsegments: u16,
    pub Isegment: u16,
    pub Ioffset: u16,
    pub outputsWKC: u16,
    pub inputsWKC: u16,
    pub docheckstate: bool,
    pub IOsegment: [u32; 64],
}

pub type ec_slavet = ec_slave;

#[derive(Copy, Clone)]
pub struct ec_slave {
    /// state of slave
    pub state: u16,
    /// AL status code
    pub ALstatuscode: u16,
    /// Configured address
    pub configadr: u16,
    /// Alias address
    pub aliasadr: u16,
    /// Manufacturer from EEprom
    pub eep_man: u32,
    /// ID from EEprom
    pub eep_id: u32,
    /// revision from EEprom
    pub eep_rev: u32,
    /// Interface type
    pub Itype: u16,
    /// Device type
    pub Dtype: u16,
    /// output bits
    pub Obits: u16,
    /// output bytes, if Obits < 8 then Obytes = 0
    pub Obytes: u32,
    /// output pointer in IOmap buffer
    pub outputs: *mut u8,
    /// startbit in first output byte
    pub Ostartbit: u8,
    /// input bits
    pub Ibits: u16,
    /// input bytes, if Ibits < 8 then Ibytes = 0
    pub Ibytes: u32,
    /// input pointer in IOmap buffer
    pub inputs: *mut u8,
    /// startbit in first input byte
    pub Istartbit: u8,
    /// SM structure
    pub SM: [ec_smt; EC_MAXSM],
    /// SM type 0=unused 1=MbxWr 2=MbxRd 3=Outputs 4=Inputs
    pub SMtype: [u8; EC_MAXSM],
    /// FMMU structure
    pub FMMU: [ec_fmmut; EC_MAXFMMU],
    /// FMMU0 function
    pub FMMU0func: u8,
    /// FMMU1 function
    pub FMMU1func: u8,
    /// FMMU2 function
    pub FMMU2func: u8,
    /// FMMU3 function
    pub FMMU3func: u8,
    /// length of write mailbox in bytes, if no mailbox then 0
    pub mbx_l: u16,
    /// mailbox write offset
    pub mbx_wo: u16,
    /// length of read mailbox in bytes
    pub mbx_rl: u16,
    /// mailbox read offset
    pub mbx_ro: u16,
    /// mailbox supported protocols
    pub mbx_proto: u16,
    /// Counter value of mailbox link layer protocol 1..7
    pub mbx_cnt: u8,
    /// has DC capability
    pub hasdc: bool,
    /// Physical type; Ebus, EtherNet combinations
    pub ptype: u8,
    /// topology: 1 to 3 links
    pub topology: u8,
    /// active ports bitmap : ....3210 , set if respective port is active *
    pub activeports: u8,
    /// consumed ports bitmap : ....3210, used for internal delay measurement *
    pub consumedports: u8,
    /// slave number for parent, 0=master
    pub parent: u16,
    /// port number on parent this slave is connected to *
    pub parentport: u8,
    /// port number on this slave the parent is connected to *
    pub entryport: u8,
    /// DC receivetimes on port A
    pub DCrtA: i32,
    /// DC receivetimes on port B
    pub DCrtB: i32,
    /// DC receivetimes on port C
    pub DCrtC: i32,
    /// DC receivetimes on port D
    pub DCrtD: i32,
    /// propagation delay
    pub pdelay: i32,
    /// next DC slave
    pub DCnext: u16,
    /// previous DC slave
    pub DCprevious: u16,
    /// DC cycle time in ns
    pub DCcycle: i32,
    /// DC shift from clock modulus boundary
    pub DCshift: i32,
    /// DC sync activation, 0=off, 1=on
    pub DCactive: bool,
    /// link to config table
    pub configindex: u16,
    /// link to SII config
    pub SIIindex: u16,
    /// 1 = 8 bytes per read, 0 = 4 bytes per read
    pub eep_8byte: u8,
    /// 0 = eeprom to master , 1 = eeprom to PDI
    pub eep_pdi: u8,
    /// CoE details
    pub CoEdetails: u8,
    /// FoE details
    pub FoEdetails: u8,
    /// EoE details
    pub EoEdetails: u8,
    /// SoE details
    pub SoEdetails: u8,
    /// E-bus current
    pub Ebuscurrent: i16,
    /// if >0 block use of LRW in processdata
    pub blockLRW: u8,
    /// group
    pub group: u8,
    /// first unused FMMU
    pub FMMUunused: u8,
    /// Boolean for tracking whether the slave is (not) responding, not used/set by the SOEM library
    pub islost: bool,
    /// registered configuration function PO->SO, (DEPRECATED)
    pub PO2SOconfig: Option<unsafe fn(_: u16) -> libc::c_int>,
    /// registered configuration function PO->SO
    pub PO2SOconfigx: Option<unsafe fn(_: *mut ecx_contextt, _: u16) -> libc::c_int>,
    /// readable name
    pub name: [libc::c_char; EC_MAXNAME + 1],
}

// TODO: Remove for derive when all the raw pointers are gone
impl Default for ec_slave {
    fn default() -> Self {
        Self {
            state: Default::default(),
            ALstatuscode: Default::default(),
            configadr: Default::default(),
            aliasadr: Default::default(),
            eep_man: Default::default(),
            eep_id: Default::default(),
            eep_rev: Default::default(),
            Itype: Default::default(),
            Dtype: Default::default(),
            Obits: Default::default(),
            Obytes: Default::default(),
            outputs: 0 as *mut u8,
            Ostartbit: Default::default(),
            Ibits: Default::default(),
            Ibytes: Default::default(),
            inputs: 0 as *mut u8,
            Istartbit: Default::default(),
            SM: [ec_sm::default(); EC_MAXSM],
            SMtype: Default::default(),
            FMMU: [ec_fmmu::default(); EC_MAXFMMU],
            FMMU0func: Default::default(),
            FMMU1func: Default::default(),
            FMMU2func: Default::default(),
            FMMU3func: Default::default(),
            mbx_l: Default::default(),
            mbx_wo: Default::default(),
            mbx_rl: Default::default(),
            mbx_ro: Default::default(),
            mbx_proto: Default::default(),
            mbx_cnt: Default::default(),
            hasdc: Default::default(),
            ptype: Default::default(),
            topology: Default::default(),
            activeports: Default::default(),
            consumedports: Default::default(),
            parent: Default::default(),
            parentport: Default::default(),
            entryport: Default::default(),
            DCrtA: Default::default(),
            DCrtB: Default::default(),
            DCrtC: Default::default(),
            DCrtD: Default::default(),
            pdelay: Default::default(),
            DCnext: Default::default(),
            DCprevious: Default::default(),
            DCcycle: Default::default(),
            DCshift: Default::default(),
            DCactive: Default::default(),
            configindex: Default::default(),
            SIIindex: Default::default(),
            eep_8byte: Default::default(),
            eep_pdi: Default::default(),
            CoEdetails: Default::default(),
            FoEdetails: Default::default(),
            EoEdetails: Default::default(),
            SoEdetails: Default::default(),
            Ebuscurrent: Default::default(),
            blockLRW: Default::default(),
            group: Default::default(),
            FMMUunused: Default::default(),
            islost: Default::default(),
            PO2SOconfig: Default::default(),
            PO2SOconfigx: Default::default(),
            name: [0; EC_MAXNAME + 1],
        }
    }
}

#[derive(Copy, Clone)]
pub struct ec_eepromPDO {
    pub Startpos: u16,
    pub Length: u16,
    pub nPDO: u16,
    pub Index: [u16; 512],
    pub SyncM: [u16; 512],
    pub BitSize: [u16; 512],
    pub SMbitsize: [u16; 8],
}
pub type ec_eepromPDOt = ec_eepromPDO;
pub type ec_mbxbuft = [u8; 1487];

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_mbxheader {
    pub length: u16,
    pub address: u16,
    pub priority: u8,
    pub mbxtype: u8,
}
pub type ec_mbxheadert = ec_mbxheader;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_alstatus {
    pub alstatus: u16,
    pub unused: u16,
    pub alstatuscode: u16,
}
pub type ec_alstatust = ec_alstatus;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_eepromt {
    pub comm: u16,
    pub addr: u16,
    pub d2: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_EOEt {
    pub mbxheader: ec_mbxheadert,
    pub frameinfo1: u16,
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub data: [u8; 1476],
}

#[derive(Copy, Clone)]
pub union C2RustUnnamed_8 {
    pub frameinfo2: u16,
    pub result: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_emcyt {
    pub MbxHeader: ec_mbxheadert,
    pub CANOpen: u16,
    pub ErrorCode: u16,
    pub ErrorReg: u8,
    pub bData: u8,
    pub w1: u16,
    pub w2: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_mbxerrort {
    pub MbxHeader: ec_mbxheadert,
    pub Type: u16,
    pub Detail: u16,
}
/* * Main slave data array.
 *  Each slave found on the network gets its own record.
 *  ec_slave[0] is reserved for the master. Structure gets filled
 *  in by the configuration function ec_config().
 */
#[no_mangle]
pub static mut ec_slave: [ec_slavet; 200] = [ec_slavet {
    state: 0,
    ALstatuscode: 0,
    configadr: 0,
    aliasadr: 0,
    eep_man: 0,
    eep_id: 0,
    eep_rev: 0,
    Itype: 0,
    Dtype: 0,
    Obits: 0,
    Obytes: 0,
    outputs: 0 as *mut u8,
    Ostartbit: 0,
    Ibits: 0,
    Ibytes: 0,
    inputs: 0 as *mut u8,
    Istartbit: 0,
    SM: [ec_smt {
        StartAddr: 0,
        SMlength: 0,
        SMflags: 0,
    }; 8],
    SMtype: [0; 8],
    FMMU: [ec_fmmut {
        LogStart: 0,
        LogLength: 0,
        LogStartbit: 0,
        LogEndbit: 0,
        PhysStart: 0,
        PhysStartBit: 0,
        FMMUtype: 0,
        FMMUactive: 0,
        unused1: 0,
        unused2: 0,
    }; 4],
    FMMU0func: 0,
    FMMU1func: 0,
    FMMU2func: 0,
    FMMU3func: 0,
    mbx_l: 0,
    mbx_wo: 0,
    mbx_rl: 0,
    mbx_ro: 0,
    mbx_proto: 0,
    mbx_cnt: 0,
    hasdc: false,
    ptype: 0,
    topology: 0,
    activeports: 0,
    consumedports: 0,
    parent: 0,
    parentport: 0,
    entryport: 0,
    DCrtA: 0,
    DCrtB: 0,
    DCrtC: 0,
    DCrtD: 0,
    pdelay: 0,
    DCnext: 0,
    DCprevious: 0,
    DCcycle: 0,
    DCshift: 0,
    DCactive: false,
    configindex: 0,
    SIIindex: 0,
    eep_8byte: 0,
    eep_pdi: 0,
    CoEdetails: 0,
    FoEdetails: 0,
    EoEdetails: 0,
    SoEdetails: 0,
    Ebuscurrent: 0,
    blockLRW: 0,
    group: 0,
    FMMUunused: 0,
    islost: false,
    PO2SOconfig: None,
    PO2SOconfigx: None,
    name: [0; 41],
}; 200];
/* * number of slaves found on the network */
#[no_mangle]
pub static mut ec_slavecount: libc::c_int = 0;
/* * slave group structure */
#[no_mangle]
pub static mut ec_group: [ec_groupt; 2] = [ec_groupt {
    logstartaddr: 0,
    Obytes: 0,
    outputs: 0 as *mut u8,
    Ibytes: 0,
    inputs: 0 as *mut u8,
    hasdc: false,
    DCnext: 0,
    Ebuscurrent: 0,
    blockLRW: 0,
    nsegments: 0,
    Isegment: 0,
    Ioffset: 0,
    outputsWKC: 0,
    inputsWKC: 0,
    docheckstate: false,
    IOsegment: [0; 64],
}; 2];
/* * cache for EEPROM read functions */
static mut EC_ESI_BUF: [u8; 4096] = [0; 4096];
/* * bitmap for filled cache buffer bytes */
static mut EC_ESI_MAP: [u32; 128] = [0; 128];
/* * current slave for EEPROM cache buffer */
static mut EC_ELIST: heapless::Vec<ec_errort, { EC_MAXELIST as usize }> = heapless::Vec::new();
static mut EC_IDX_STACK: ec_idxstackT = ec_idxstackT {
    pushed: 0,
    pulled: 0,
    idx: [0; 16],
    data: [0 as *mut libc::c_void; 16],
    length: [0; 16],
    dcoffset: [0; 16],
};
/* * SyncManager Communication Type struct to store data of one slave */
static mut EC_SM_COMMTYPE: [ec_SMcommtypet; 1] = [ec_SMcommtypet {
    n: 0,
    nu1: 0,
    SMtype: [0; 8],
}; 1];
/* * PDO assign struct to store data of one slave */
static mut EC_PDO_ASSIGN: [ec_PDOassignt; 1] = [ec_PDOassignt {
    n: 0,
    nu1: 0,
    index: [0; 256],
}; 1];
/* * PDO description struct to store data of one slave */
static mut EC_PDO_DESC: [ec_PDOdesct; 1] = [ec_PDOdesct {
    n: 0,
    nu1: 0,
    PDO: [0; 256],
}; 1];
/* * buffer for EEPROM SM data */
static mut EC_SM: ec_eepromSMt = ec_eepromSMt {
    Startpos: 0,
    nSM: 0,
    PhStart: 0,
    Plength: 0,
    Creg: 0,
    Sreg: 0,
    Activate: 0,
    PDIctrl: 0,
};
/* * buffer for EEPROM FMMU data */
static mut EC_FMMU: ec_eepromFMMUt = ec_eepromFMMUt {
    Startpos: 0,
    nFMMU: 0,
    FMMU0: 0,
    FMMU1: 0,
    FMMU2: 0,
    FMMU3: 0,
};
/* * Global variable TRUE if error available in error stack */
#[no_mangle]
pub static mut EcatError: bool = false;
#[no_mangle]
pub static mut ec_DCtime: i64 = 0;
#[no_mangle]
pub static mut ecx_port: ecx_portt = ecx_portt {
    stack: ec_stackT {
        sock: 0 as *mut libc::c_int,
        txbuf: 0 as *mut [ec_bufT; 16],
        txbuflength: 0 as *mut [libc::c_int; 16],
        tempbuf: 0 as *mut ec_bufT,
        rxbuf: 0 as *mut [ec_bufT; 16],
        rxbufstat: 0 as *mut [libc::c_int; 16],
        rxsa: 0 as *mut [libc::c_int; 16],
    },
    sockhandle: 0,
    rxbuf: [[0; 1518]; 16],
    rxbufstat: [0; 16],
    rxsa: [0; 16],
    tempinbuf: [0; 1518],
    tempinbufs: 0,
    txbuf: [[0; 1518]; 16],
    txbuflength: [0; 16],
    txbuf2: [0; 1518],
    txbuflength2: 0,
    lastidx: 0,
    redport: None,
    getindex_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
    tx_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
    rx_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
};
#[no_mangle]
pub static mut ecx_redport: ecx_redportt = ecx_redportt {
    stack: ec_stackT {
        sock: 0 as *mut libc::c_int,
        txbuf: 0 as *mut [ec_bufT; 16],
        txbuflength: 0 as *mut [libc::c_int; 16],
        tempbuf: 0 as *mut ec_bufT,
        rxbuf: 0 as *mut [ec_bufT; 16],
        rxbufstat: 0 as *mut [libc::c_int; 16],
        rxsa: 0 as *mut [libc::c_int; 16],
    },
    sockhandle: 0,
    rxbuf: [[0; 1518]; 16],
    rxbufstat: [0; 16],
    rxsa: [0; 16],
    tempinbuf: [0; 1518],
};
// Initialized in run_static_initializers
#[no_mangle]
pub static mut ecx_context: ecx_contextt = ecx_contextt {
    port: 0 as *mut ecx_portt,
    slavelist: heapless::Vec::new(),
    slavecount: 0,
    maxslave: 0,
    grouplist: 0 as *mut ec_groupt,
    maxgroup: 0,
    esibuf: 0 as *mut u8,
    esimap: 0 as *mut u32,
    esislave: 0,
    elist: heapless::Vec::new(),
    idxstack: 0 as *mut ec_idxstackT,
    ecaterror: false,
    DCtime: 0 as *mut i64,
    SMcommtype: 0 as *mut ec_SMcommtypet,
    PDOassign: 0 as *mut ec_PDOassignt,
    PDOdesc: 0 as *mut ec_PDOdesct,
    eepSM: 0 as *mut ec_eepromSMt,
    eepFMMU: 0 as *mut ec_eepromFMMUt,
    FOEhook: None,
    EOEhook: None,
    manualstatechange: 0,
    userdata: 0 as *mut libc::c_void,
};
/* * Create list over available network adapters.
 *
 * @return First element in list over available network adapters.
 */
#[no_mangle]
pub unsafe fn ec_find_adapters() -> *mut ec_adaptert {
    let mut ret_adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
    ret_adapter = oshw_find_adapters();
    return ret_adapter;
}
/* * Free dynamically allocated list over available network adapters.
 *
 * @param[in] adapter = Struct holding adapter name, description and pointer to next.
 */
#[no_mangle]
pub unsafe fn ec_free_adapters(adapter: *mut ec_adaptert) {
    oshw_free_adapters(adapter);
}
/* * Pushes an error on the error list.
 *
 * @param[in] context        = context struct
 * @param[in] Ec pointer describing the error.
 */
#[no_mangle]
pub fn ecx_pusherror(context: &mut ecx_contextt, Ec: ec_errort) {
    context.ecaterror = true;

    // TODO: Handle case where error list is full
    // TODO: Use a ringbuffer like original implementation
    context.elist.push(Ec);
}
/* * Pops an error from the list.
 *
 * @param[in] context        = context struct
 * @param[out] Ec = Struct describing the error.
 * @return TRUE if an error was popped.
 */
#[no_mangle]
pub fn ecx_poperror(context: &mut ecx_contextt, Ec: &mut ec_errort) -> bool {
    if let Some(error) = context.elist.pop() {
        *Ec = error;

        true
    } else {
        false
    }
}
/* * Check if error list has entries.
 *
 * @param[in] context        = context struct
 * @return TRUE if error list contains entries.
 */
#[no_mangle]
pub fn ecx_iserror(context: &ecx_contextt) -> bool {
    !context.elist.is_empty()
}
/* * Report packet error
 *
 * @param[in]  context        = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index that generated error
 * @param[in]  SubIdx     = Subindex that generated error
 * @param[in]  ErrorCode  = Error code
 */
#[no_mangle]
pub fn ecx_packeterror(
    context: &mut ecx_contextt,
    Slave: u16,
    Index: u16,
    SubIdx: u8,
    ErrorCode: u16,
) {
    let Ec: ec_errort = ec_errort {
        Signal: false,
        Time: osal_current_time(),
        Slave: Slave,
        Index: Index,
        SubIdx: SubIdx,
        Etype: ec_err_type::EC_ERR_TYPE_PACKET_ERROR,
        c2rust_unnamed: C2RustUnnamed_0 {
            c2rust_unnamed: crate::types::C2RustUnnamed_1 {
                ErrorCode,
                ..crate::types::C2RustUnnamed_1::default()
            },
        },
    };

    context.ecaterror = true;

    ecx_pusherror(context, Ec);
}
/* * Report Mailbox Error
 *
 * @param[in]  context        = context struct
 * @param[in]  Slave        = Slave number
 * @param[in]  Detail       = Following EtherCAT specification
 */
fn ecx_mbxerror(context: &mut ecx_contextt, Slave: u16, Detail: u16) {
    let Ec: ec_errort = ec_errort {
        Signal: false,
        Time: osal_current_time(),
        Slave: Slave,
        Index: 0,
        SubIdx: 0,
        Etype: ec_err_type::EC_ERR_TYPE_MBX_ERROR,
        c2rust_unnamed: C2RustUnnamed_0 {
            c2rust_unnamed: crate::types::C2RustUnnamed_1 {
                ErrorCode: Detail,
                ..crate::types::C2RustUnnamed_1::default()
            },
        },
    };

    context.ecaterror = true;

    ecx_pusherror(context, Ec);
}
/* * Report Mailbox Emergency Error
 *
 * @param[in]  context        = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  ErrorCode  = Following EtherCAT specification
 * @param[in]  ErrorReg
 * @param[in]  b1
 * @param[in]  w1
 * @param[in]  w2
 */
fn ecx_mbxemergencyerror(
    context: &mut ecx_contextt,
    Slave: u16,
    ErrorCode: u16,
    ErrorReg: u16,
    b1: u8,
    w1: u16,
    w2: u16,
) {
    let Ec: ec_errort = ec_errort {
        Signal: false,
        Time: osal_current_time(),
        Slave: Slave,
        Index: 0,
        SubIdx: 0,
        Etype: ec_err_type::EC_ERR_TYPE_EMERGENCY,
        c2rust_unnamed: C2RustUnnamed_0 {
            c2rust_unnamed: crate::types::C2RustUnnamed_1 {
                ErrorCode: ErrorCode,
                ErrorReg: ErrorReg as u8,
                b1: b1,
                w1: w1,
                w2: w2,
            },
        },
    };

    ecx_pusherror(context, Ec);
}
/* * Initialise lib in single NIC mode
 * @param[in]  context = context struct
 * @param[in] ifname   = Dev name, f.e. "eth0"
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe fn ecx_init(context: *mut ecx_contextt, ifname: *const libc::c_char) -> libc::c_int {
    return ecx_setupnic((*context).port.as_mut().unwrap(), ifname, 0i32);
}
/* * Initialise lib in redundant NIC mode
 * @param[in]  context  = context struct
 * @param[in]  redport  = pointer to redport, redundant port data
 * @param[in]  ifname   = Primary Dev name, f.e. "eth0"
 * @param[in]  if2name  = Secondary Dev name, f.e. "eth1"
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe fn ecx_init_redundant(
    mut context: *mut ecx_contextt,
    redport: ecx_redportt,
    ifname: *const libc::c_char,
    if2name: *mut libc::c_char,
) -> libc::c_int {
    let mut rval: libc::c_int = 0;
    let mut zbuf: libc::c_int = 0;
    let mut ehp: *mut EthernetHeader = 0 as *mut EthernetHeader;
    (*(*context).port).redport = Some(redport);
    ecx_setupnic((*context).port.as_mut().unwrap(), ifname, 0i32);
    rval = ecx_setupnic((*context).port.as_mut().unwrap(), if2name, 1i32);
    /* prepare "dummy" BRD tx frame for redundant operation */
    ehp = &mut (*(*context).port).txbuf2 as *mut ec_bufT as *mut EthernetHeader;
    (*ehp).sa1 = oshw_htons(secMAC[0usize]);
    zbuf = 0i32;
    ecx_setupdatagram(
        (*context).port.as_mut().unwrap(),
        &mut (*(*context).port).txbuf2 as *mut ec_bufT as *mut libc::c_void,
        Command::Brd,
        0u8,
        0u16,
        0u16,
        2,
        &mut zbuf as *mut libc::c_int as *mut libc::c_void,
    );
    (*(*context).port).txbuflength2 = core::mem::size_of::<EthernetHeader>()
        .wrapping_add(core::mem::size_of::<EthercatHeader>())
        .wrapping_add(core::mem::size_of::<u16>())
        .wrapping_add(2usize) as libc::c_int;
    return rval;
}
/* * Close lib.
 * @param[in]  context        = context struct
 */
#[no_mangle]
pub unsafe fn ecx_close(context: *mut ecx_contextt) {
    ecx_closenic((*context).port.as_mut().unwrap());
}
/* * Read one byte from slave EEPROM via cache.
 *  If the cache location is empty then a read request is made to the slave.
 *  Depending on the slave capabilities the request is 4 or 8 bytes.
 *  @param[in] context = context struct
 *  @param[in] slave   = slave number
 *  @param[in] address = eeprom address in bytes (slave uses words)
 *  @return requested byte, if not available then 0xff
 */
#[no_mangle]
pub unsafe fn ecx_siigetbyte(mut context: *mut ecx_contextt, slave: u16, address: u16) -> u8 {
    let mut configadr: u16 = 0;
    let mut eadr: u16 = 0;
    let mut edat64: u64 = 0;
    let mut edat32: u32 = 0;
    let mut mapw: u16 = 0;
    let mut mapb: u16 = 0;
    let mut lp: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut retval: u8 = 0;
    retval = 0xffu8;
    if slave as libc::c_int != (*context).esislave as libc::c_int {
        /* not the same slave? */
        memset(
            (*context).esimap as *mut libc::c_void,
            0i32,
            (128usize).wrapping_mul(core::mem::size_of::<u32>()),
        ); /* clear esibuf cache map */
        (*context).esislave = slave
    }
    if (address as libc::c_int) < (128i32) << 5i32 {
        mapw = (address as libc::c_int >> 5i32) as u16;
        mapb = (address as libc::c_int - ((mapw as libc::c_int) << 5i32)) as u16;
        if *(*context).esimap.offset(mapw as isize) & (1u32) << mapb as libc::c_int != 0 {
            /* byte is already in buffer */
            retval = *(*context).esibuf.offset(address as isize)
        } else {
            /* byte is not in buffer, put it there */
            configadr = (*context).slavelist[slave as usize].configadr; /* set eeprom control to master */
            ecx_eeprom2master(context, slave);
            eadr = (address as libc::c_int >> 1i32) as u16;
            edat64 = ecx_readeepromFP(context, configadr, eadr, EC_TIMEOUTEEP);
            /* 8 byte response */
            if (*context).slavelist[slave as usize].eep_8byte != 0 {
                memcpy(
                    &mut *(*context)
                        .esibuf
                        .offset(((eadr as libc::c_int) << 1i32) as isize)
                        as *mut u8 as *mut libc::c_void,
                    &mut edat64 as *mut u64 as *const libc::c_void,
                    8usize,
                );
                cnt = 8i32
            } else {
                /* 4 byte response */
                edat32 = edat64 as u32;
                memcpy(
                    &mut *(*context)
                        .esibuf
                        .offset(((eadr as libc::c_int) << 1i32) as isize)
                        as *mut u8 as *mut libc::c_void,
                    &mut edat32 as *mut u32 as *const libc::c_void,
                    4usize,
                );
                cnt = 4i32
            }
            /* find bitmap location */
            mapw = (eadr as libc::c_int >> 4i32) as u16;
            mapb = (((eadr as libc::c_int) << 1i32) - ((mapw as libc::c_int) << 5i32)) as u16;
            lp = 0i32;
            while lp < cnt {
                /* set bitmap for each byte that is read */
                let ref mut fresh0 = *(*context).esimap.offset(mapw as isize);
                *fresh0 |= (1u32) << mapb as libc::c_int;
                mapb = mapb.wrapping_add(1);
                if mapb as libc::c_int > 31i32 {
                    mapb = 0u16;
                    mapw = mapw.wrapping_add(1)
                }
                lp += 1
            }
            retval = *(*context).esibuf.offset(address as isize)
        }
    }
    return retval;
}
/* * Find SII section header in slave EEPROM.
 *  @param[in]  context        = context struct
 *  @param[in] slave   = slave number
 *  @param[in] cat     = section category
 *  @return byte address of section at section length entry, if not available then 0
 */
#[no_mangle]
pub unsafe fn ecx_siifind(context: *mut ecx_contextt, slave: u16, cat: u16) -> i16 {
    let mut a: i16 = 0;
    let mut p: u16 = 0;
    let eectl: u8 = (*context).slavelist[slave as usize].eep_pdi;
    a = ((0x40i32) << 1i32) as i16;
    /* read first SII section category */
    let fresh1 = a;
    a = a + 1;
    p = ecx_siigetbyte(context, slave, fresh1 as u16) as u16;
    let fresh2 = a;
    a = a + 1;
    p = (p as libc::c_int
        + ((ecx_siigetbyte(context, slave, fresh2 as u16) as libc::c_int) << 8i32)) as u16;
    /* traverse SII while category is not found and not EOF */
    while p as libc::c_int != cat as libc::c_int && p as libc::c_int != 0xffffi32 {
        /* read section length */
        let fresh3 = a;
        a = a + 1;
        p = ecx_siigetbyte(context, slave, fresh3 as u16) as u16;
        let fresh4 = a;
        a = a + 1;
        p = (p as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh4 as u16) as libc::c_int) << 8i32))
            as u16;
        /* locate next section category */
        a = (a as libc::c_int + ((p as libc::c_int) << 1i32)) as i16;
        /* read section category */
        let fresh5 = a;
        a = a + 1;
        p = ecx_siigetbyte(context, slave, fresh5 as u16) as u16;
        let fresh6 = a;
        a = a + 1;
        p = (p as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh6 as u16) as libc::c_int) << 8i32))
            as u16
    }
    if p as libc::c_int != cat as libc::c_int {
        a = 0i16
    }
    if eectl != 0 {
        ecx_eeprom2pdi(context, slave);
        /* if eeprom control was previously pdi then restore */
    }
    return a;
}
/* * Get string from SII string section in slave EEPROM.
 *  @param[in]  context = context struct
 *  @param[out] str     = requested string, 0x00 if not found
 *  @param[in]  slave   = slave number
 *  @param[in]  Sn      = string number
 */
#[no_mangle]
pub unsafe fn ecx_siistring(
    context: *mut ecx_contextt,
    str: *mut libc::c_char,
    slave: u16,
    Sn: u16,
) {
    let mut a: u16 = 0; /* find string section */
    let mut i: u16 = 0; /* skip SII section header */
    let mut j: u16 = 0; /* read number of strings in section */
    let mut l: u16 = 0;
    let mut n: u16 = 0;
    let mut ba: u16 = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let eectl: u8 = (*context).slavelist[slave as usize].eep_pdi;
    ptr = str;
    a = ecx_siifind(context, slave, SiiCategory::String as u16) as u16;
    if a as libc::c_int > 0i32 {
        ba = (a as libc::c_int + 2i32) as u16;
        let fresh7 = ba;
        ba = ba.wrapping_add(1);
        n = ecx_siigetbyte(context, slave, fresh7) as u16;
        if Sn as libc::c_int <= n as libc::c_int {
            /* is req string available? */
            i = 1u16;
            while i as libc::c_int <= Sn as libc::c_int {
                /* walk through strings */
                let fresh8 = ba; /* length of this string */
                ba = ba.wrapping_add(1);
                l = ecx_siigetbyte(context, slave, fresh8) as u16;
                if (i as libc::c_int) < Sn as libc::c_int {
                    ba = (ba as libc::c_int + l as libc::c_int) as u16
                } else {
                    ptr = str;
                    j = 1u16;
                    while j as libc::c_int <= l as libc::c_int {
                        /* copy one string */
                        if j as libc::c_int <= 40i32 {
                            let fresh9 = ba;
                            ba = ba.wrapping_add(1);
                            *ptr = ecx_siigetbyte(context, slave, fresh9) as libc::c_char;
                            ptr = ptr.offset(1)
                        } else {
                            ba = ba.wrapping_add(1)
                        }
                        j = j.wrapping_add(1)
                    }
                }
                i = i.wrapping_add(1)
            }
            *ptr = 0;
        /* add zero terminator */
        } else {
            ptr = str;
            *ptr = 0;
            /* empty string */
        }
    }
    if eectl != 0 {
        ecx_eeprom2pdi(context, slave);
        /* if eeprom control was previously pdi then restore */
    };
}
/* * Get FMMU data from SII FMMU section in slave EEPROM.
 *  @param[in]  context = context struct
 *  @param[in]  slave   = slave number
 *  @param[out] FMMU    = FMMU struct from SII, max. 4 FMMU's
 *  @return number of FMMU's defined in section
 */
#[no_mangle]
pub unsafe fn ecx_siiFMMU(
    context: *mut ecx_contextt,
    slave: u16,
    FMMU: *mut ec_eepromFMMUt,
) -> u16 {
    let mut a: u16 = 0;
    let eectl: u8 = (*context).slavelist[slave as usize].eep_pdi;
    (*FMMU).nFMMU = 0u8;
    (*FMMU).FMMU0 = 0u8;
    (*FMMU).FMMU1 = 0u8;
    (*FMMU).FMMU2 = 0u8;
    (*FMMU).FMMU3 = 0u8;
    (*FMMU).Startpos = ecx_siifind(context, slave, SiiCategory::Fmmu as u16) as u16;
    if (*FMMU).Startpos as libc::c_int > 0i32 {
        a = (*FMMU).Startpos;
        let fresh10 = a;
        a = a.wrapping_add(1);
        (*FMMU).nFMMU = ecx_siigetbyte(context, slave, fresh10);
        let fresh11 = a;
        a = a.wrapping_add(1);
        (*FMMU).nFMMU = ((*FMMU).nFMMU as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh11) as libc::c_int) << 8i32))
            as u8;
        (*FMMU).nFMMU = ((*FMMU).nFMMU as libc::c_int * 2i32) as u8;
        let fresh12 = a;
        a = a.wrapping_add(1);
        (*FMMU).FMMU0 = ecx_siigetbyte(context, slave, fresh12);
        let fresh13 = a;
        a = a.wrapping_add(1);
        (*FMMU).FMMU1 = ecx_siigetbyte(context, slave, fresh13);
        if (*FMMU).nFMMU as libc::c_int > 2i32 {
            let fresh14 = a;
            a = a.wrapping_add(1);
            (*FMMU).FMMU2 = ecx_siigetbyte(context, slave, fresh14);
            let fresh15 = a;
            a = a.wrapping_add(1);
            (*FMMU).FMMU3 = ecx_siigetbyte(context, slave, fresh15)
        }
    }
    if eectl != 0 {
        ecx_eeprom2pdi(context, slave);
        /* if eeprom control was previously pdi then restore */
    }
    return (*FMMU).nFMMU as u16;
}
/* * Get SM data from SII SM section in slave EEPROM.
 *  @param[in]  context = context struct
 *  @param[in]  slave   = slave number
 *  @param[out] SM      = first SM struct from SII
 *  @return number of SM's defined in section
 */
#[no_mangle]
pub unsafe fn ecx_siiSM(context: *mut ecx_contextt, slave: u16, mut SM: *mut ec_eepromSMt) -> u16 {
    let mut a: u16 = 0;
    let mut w: u16 = 0;
    let eectl: u8 = (*context).slavelist[slave as usize].eep_pdi;
    (*SM).nSM = 0u8;
    (*SM).Startpos = ecx_siifind(context, slave, SiiCategory::Sm as u16) as u16;
    if (*SM).Startpos as libc::c_int > 0i32 {
        a = (*SM).Startpos;
        let fresh16 = a;
        a = a.wrapping_add(1);
        w = ecx_siigetbyte(context, slave, fresh16) as u16;
        let fresh17 = a;
        a = a.wrapping_add(1);
        w = (w as libc::c_int + ((ecx_siigetbyte(context, slave, fresh17) as libc::c_int) << 8i32))
            as u16;
        (*SM).nSM = (w as libc::c_int / 4i32) as u8;
        let fresh18 = a;
        a = a.wrapping_add(1);
        (*SM).PhStart = ecx_siigetbyte(context, slave, fresh18) as u16;
        let fresh19 = a;
        a = a.wrapping_add(1);
        (*SM).PhStart = ((*SM).PhStart as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh19) as libc::c_int) << 8i32))
            as u16;
        let fresh20 = a;
        a = a.wrapping_add(1);
        (*SM).Plength = ecx_siigetbyte(context, slave, fresh20) as u16;
        let fresh21 = a;
        a = a.wrapping_add(1);
        (*SM).Plength = ((*SM).Plength as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh21) as libc::c_int) << 8i32))
            as u16;
        let fresh22 = a;
        a = a.wrapping_add(1);
        (*SM).Creg = ecx_siigetbyte(context, slave, fresh22);
        let fresh23 = a;
        a = a.wrapping_add(1);
        (*SM).Sreg = ecx_siigetbyte(context, slave, fresh23);
        let fresh24 = a;
        a = a.wrapping_add(1);
        (*SM).Activate = ecx_siigetbyte(context, slave, fresh24);
        let fresh25 = a;
        a = a.wrapping_add(1);
        (*SM).PDIctrl = ecx_siigetbyte(context, slave, fresh25)
    }
    if eectl != 0 {
        ecx_eeprom2pdi(context, slave);
        /* if eeprom control was previously pdi then restore */
    }
    return (*SM).nSM as u16;
}
/* * Get next SM data from SII SM section in slave EEPROM.
 *  @param[in]  context = context struct
 *  @param[in]  slave   = slave number
 *  @param[out] SM      = first SM struct from SII
 *  @param[in]  n       = SM number
 *  @return >0 if OK
 */
#[no_mangle]
pub unsafe fn ecx_siiSMnext(
    context: *mut ecx_contextt,
    slave: u16,
    mut SM: *mut ec_eepromSMt,
    n: u16,
) -> u16 {
    let mut a: u16 = 0;
    let mut retVal: u16 = 0u16;
    let eectl: u8 = (*context).slavelist[slave as usize].eep_pdi;
    if (n as libc::c_int) < (*SM).nSM as libc::c_int {
        a = ((*SM).Startpos as libc::c_int + 2i32 + n as libc::c_int * 8i32) as u16;
        let fresh26 = a;
        a = a.wrapping_add(1);
        (*SM).PhStart = ecx_siigetbyte(context, slave, fresh26) as u16;
        let fresh27 = a;
        a = a.wrapping_add(1);
        (*SM).PhStart = ((*SM).PhStart as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh27) as libc::c_int) << 8i32))
            as u16;
        let fresh28 = a;
        a = a.wrapping_add(1);
        (*SM).Plength = ecx_siigetbyte(context, slave, fresh28) as u16;
        let fresh29 = a;
        a = a.wrapping_add(1);
        (*SM).Plength = ((*SM).Plength as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh29) as libc::c_int) << 8i32))
            as u16;
        let fresh30 = a;
        a = a.wrapping_add(1);
        (*SM).Creg = ecx_siigetbyte(context, slave, fresh30);
        let fresh31 = a;
        a = a.wrapping_add(1);
        (*SM).Sreg = ecx_siigetbyte(context, slave, fresh31);
        let fresh32 = a;
        a = a.wrapping_add(1);
        (*SM).Activate = ecx_siigetbyte(context, slave, fresh32);
        let fresh33 = a;
        a = a.wrapping_add(1);
        (*SM).PDIctrl = ecx_siigetbyte(context, slave, fresh33);
        retVal = 1u16
    }
    if eectl != 0 {
        ecx_eeprom2pdi(context, slave);
        /* if eeprom control was previously pdi then restore */
    }
    return retVal;
}
/* * Get PDO data from SII PDO section in slave EEPROM.
 *  @param[in]  context = context struct
 *  @param[in]  slave   = slave number
 *  @param[out] PDO     = PDO struct from SII
 *  @param[in]  t       = 0=RXPDO 1=TXPDO
 *  @return mapping size in bits of PDO
 */
#[no_mangle]
pub unsafe fn ecx_siiPDO(
    context: *mut ecx_contextt,
    slave: u16,
    mut PDO: *mut ec_eepromPDOt,
    mut t: u8,
) -> u32 {
    let mut a: u16 = 0;
    let mut w: u16 = 0;
    let mut c: u16 = 0;
    let mut e: u16 = 0;
    let mut er: u16 = 0;
    let mut Size: u16 = 0;
    let eectl: u8 = (*context).slavelist[slave as usize].eep_pdi;
    Size = 0u16;
    (*PDO).nPDO = 0u16;
    (*PDO).Length = 0u16;
    (*PDO).Index[1usize] = 0u16;
    c = 0u16;
    while (c as libc::c_int) < 8i32 {
        (*PDO).SMbitsize[c as usize] = 0u16;
        c = c.wrapping_add(1)
    }
    if t as libc::c_int > 1i32 {
        t = 1u8
    }
    (*PDO).Startpos = ecx_siifind(
        context,
        slave,
        (SiiCategory::Pdo as libc::c_int + t as libc::c_int) as u16,
    ) as u16;
    if (*PDO).Startpos as libc::c_int > 0i32 {
        a = (*PDO).Startpos;
        let fresh34 = a;
        a = a.wrapping_add(1);
        w = ecx_siigetbyte(context, slave, fresh34) as u16;
        let fresh35 = a;
        a = a.wrapping_add(1);
        w = (w as libc::c_int + ((ecx_siigetbyte(context, slave, fresh35) as libc::c_int) << 8i32))
            as u16;
        (*PDO).Length = w;
        c = 1u16;
        loop
        /* traverse through all PDOs */
        {
            (*PDO).nPDO = (*PDO).nPDO.wrapping_add(1);
            let fresh36 = a;
            a = a.wrapping_add(1);
            (*PDO).Index[(*PDO).nPDO as usize] = ecx_siigetbyte(context, slave, fresh36) as u16;
            let fresh37 = a;
            a = a.wrapping_add(1);
            (*PDO).Index[(*PDO).nPDO as usize] = ((*PDO).Index[(*PDO).nPDO as usize] as libc::c_int
                + ((ecx_siigetbyte(context, slave, fresh37) as libc::c_int) << 8i32))
                as u16;
            (*PDO).BitSize[(*PDO).nPDO as usize] = 0u16;
            c = c.wrapping_add(1);
            let fresh38 = a;
            a = a.wrapping_add(1);
            e = ecx_siigetbyte(context, slave, fresh38) as u16;
            let fresh39 = a;
            a = a.wrapping_add(1);
            (*PDO).SyncM[(*PDO).nPDO as usize] = ecx_siigetbyte(context, slave, fresh39) as u16;
            a = (a as libc::c_int + 4i32) as u16;
            c = (c as libc::c_int + 2i32) as u16;
            if ((*PDO).SyncM[(*PDO).nPDO as usize] as libc::c_int) < 8i32 {
                /* active and in range SM? */
                /* read all entries defined in PDO */
                er = 1u16;
                while er as libc::c_int <= e as libc::c_int {
                    c = (c as libc::c_int + 4i32) as u16;
                    a = (a as libc::c_int + 5i32) as u16;
                    let fresh40 = a;
                    a = a.wrapping_add(1);
                    (*PDO).BitSize[(*PDO).nPDO as usize] = ((*PDO).BitSize[(*PDO).nPDO as usize]
                        as libc::c_int
                        + ecx_siigetbyte(context, slave, fresh40) as libc::c_int)
                        as u16;
                    a = (a as libc::c_int + 2i32) as u16;
                    er = er.wrapping_add(1)
                }
                (*PDO).SMbitsize[(*PDO).SyncM[(*PDO).nPDO as usize] as usize] =
                    ((*PDO).SMbitsize[(*PDO).SyncM[(*PDO).nPDO as usize] as usize] as libc::c_int
                        + (*PDO).BitSize[(*PDO).nPDO as usize] as libc::c_int)
                        as u16;
                Size = (Size as libc::c_int + (*PDO).BitSize[(*PDO).nPDO as usize] as libc::c_int)
                    as u16;
                c = c.wrapping_add(1)
            } else {
                /* PDO deactivated because SM is 0xff or > EC_MAXSM */
                c = (c as libc::c_int + 4i32 * e as libc::c_int) as u16;
                a = (a as libc::c_int + 8i32 * e as libc::c_int) as u16;
                c = c.wrapping_add(1)
            }
            if (*PDO).nPDO as libc::c_int >= 0x200i32 - 1i32 {
                c = (*PDO).Length
                /* limit number of PDO entries in buffer */
            }
            if !((c as libc::c_int) < (*PDO).Length as libc::c_int) {
                break;
            }
        }
    }
    if eectl != 0 {
        ecx_eeprom2pdi(context, slave);
        /* if eeprom control was previously pdi then restore */
    }
    return Size as u32;
}
#[no_mangle]
pub unsafe fn ecx_FPRD_multi(
    context: *mut ecx_contextt,
    n: libc::c_int,
    configlst: *mut u16,
    slstatlst: *mut ec_alstatust,
    timeout: u32,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: u8 = 0;
    let mut port: *mut ecx_portt = 0 as *mut ecx_portt;
    let mut sldatapos: [u16; 64] = [0; 64];
    let mut slcnt: libc::c_int = 0;
    port = (*context).port;
    idx = ecx_getindex(port.as_mut().unwrap());
    slcnt = 0i32;
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Fprd,
        idx,
        *configlst.offset(slcnt as isize),
        EthercatRegister::ECT_REG_ALSTAT as u16,
        ::core::mem::size_of::<ec_alstatust>(),
        slstatlst.offset(slcnt as isize) as *mut libc::c_void,
    );
    sldatapos[slcnt as usize] = ::core::mem::size_of::<EthercatHeader>() as u16;
    loop {
        slcnt += 1;
        if !(slcnt < n - 1i32) {
            break;
        }
        sldatapos[slcnt as usize] = ecx_adddatagram(
            port,
            &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                as *mut libc::c_void,
            Command::Fprd,
            idx,
            true,
            *configlst.offset(slcnt as isize),
            EthercatRegister::ECT_REG_ALSTAT as u16,
            ::core::mem::size_of::<ec_alstatust>(),
            slstatlst.offset(slcnt as isize) as *mut libc::c_void,
        )
    }
    if slcnt < n {
        sldatapos[slcnt as usize] = ecx_adddatagram(
            port,
            &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                as *mut libc::c_void,
            Command::Fprd,
            idx,
            false,
            *configlst.offset(slcnt as isize),
            EthercatRegister::ECT_REG_ALSTAT as u16,
            ::core::mem::size_of::<ec_alstatust>(),
            slstatlst.offset(slcnt as isize) as *mut libc::c_void,
        )
    }
    wkc = ecx_srconfirm(port.as_mut().unwrap(), idx, timeout);
    if wkc >= 0i32 {
        slcnt = 0i32;
        while slcnt < n {
            memcpy(
                slstatlst.offset(slcnt as isize) as *mut libc::c_void,
                &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                    .as_mut_ptr()
                    .offset(*sldatapos.as_mut_ptr().offset(slcnt as isize) as isize)
                    as *mut u8 as *const libc::c_void,
                ::core::mem::size_of::<ec_alstatust>(),
            );
            slcnt += 1
        }
    }
    ecx_setbufstat(port.as_mut().unwrap(), idx, BufferState::Empty);
    return wkc;
}
/* * Read all slave states in ec_slave.
 * @param[in] context = context struct
 * @return lowest state found
 */
#[no_mangle]
pub unsafe fn ecx_readstate(context: *mut ecx_contextt) -> libc::c_int {
    let mut slave: u16 = 0;
    let mut fslave: u16 = 0;
    let mut lslave: u16 = 0;
    let mut configadr: u16 = 0;
    let mut lowest: u16 = 0;
    let mut rval: u16 = 0;
    let mut bitwisestate: u16 = 0;
    let mut sl: [ec_alstatust; 64] = [ec_alstatust {
        alstatus: 0,
        unused: 0,
        alstatuscode: 0,
    }; 64];
    let mut slca: [u16; 64] = [0; 64];
    let mut noerrorflag: bool = false;
    let mut allslavessamestate: bool = false;
    let mut allslavespresent: bool = false;
    let mut wkc: libc::c_int = 0;
    /* Try to establish the state of all slaves sending only one broadcast datagram.
     * This way a number of datagrams equal to the number of slaves will be sent only if needed.*/
    rval = 0u16;
    wkc = ecx_BRD(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_ALSTAT as u16,
        ::core::mem::size_of::<u16>(),
        &mut rval as *mut u16 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    if wkc >= (*context).slavelist.len() as i32 {
        allslavespresent = true;
    }
    rval = rval;
    bitwisestate = (rval as libc::c_int & 0xfi32) as u16;
    if rval as libc::c_int & SlaveState::Error as libc::c_int == 0i32 {
        noerrorflag = true;
        (*context).slavelist[0].ALstatuscode = 0u16
    } else {
        noerrorflag = false;
    }
    match SlaveState::from_repr(bitwisestate as usize).unwrap() {
        SlaveState::Init
        | SlaveState::PreOp
        | SlaveState::Boot
        | SlaveState::SafeOp
        | SlaveState::Op => {
            allslavessamestate = true;
            (*context).slavelist[0].state = bitwisestate
        }
        _ => allslavessamestate = false,
    }
    if noerrorflag as libc::c_int != 0
        && allslavessamestate as libc::c_int != 0
        && allslavespresent as libc::c_int != 0
    {
        /* No slave has toggled the error flag so the alstatuscode
         * (even if different from 0) should be ignored and
         * the slaves have reached the same state so the internal state
         * can be updated without sending any datagram. */
        slave = 1u16;
        while slave as libc::c_int <= (*context).slavelist.len() as i32 {
            (*context).slavelist[slave as usize].ALstatuscode = 0u16;
            (*context).slavelist[slave as usize].state = bitwisestate;
            slave = slave.wrapping_add(1)
        }
        lowest = bitwisestate
    } else {
        /* Not all slaves have the same state or at least one is in error so one datagram per slave
         * is needed. */
        (*context).slavelist[0].ALstatuscode = 0u16;
        lowest = 0xffu16;
        fslave = 1u16;
        loop {
            lslave = (*context).slavelist.len() as u16;
            if lslave as libc::c_int - fslave as libc::c_int >= 64i32 {
                lslave = (fslave as libc::c_int + 64i32 - 1i32) as u16
            }
            slave = fslave;
            while slave as libc::c_int <= lslave as libc::c_int {
                let zero: ec_alstatust = {
                    let init = ec_alstatus {
                        alstatus: 0u16,
                        unused: 0u16,
                        alstatuscode: 0u16,
                    };
                    init
                };
                configadr = (*context).slavelist[slave as usize].configadr;
                slca[(slave as libc::c_int - fslave as libc::c_int) as usize] = configadr;
                sl[(slave as libc::c_int - fslave as libc::c_int) as usize] = zero;
                slave = slave.wrapping_add(1)
            }
            ecx_FPRD_multi(
                context,
                lslave as libc::c_int - fslave as libc::c_int + 1i32,
                &mut *slca.as_mut_ptr().offset(0isize),
                &mut *sl.as_mut_ptr().offset(0isize),
                EC_TIMEOUTRET3,
            );
            slave = fslave;
            while slave as libc::c_int <= lslave as libc::c_int {
                configadr = (*context).slavelist[slave as usize].configadr;
                rval = sl[(slave as libc::c_int - fslave as libc::c_int) as usize].alstatus;
                (*context).slavelist[slave as usize].ALstatuscode =
                    sl[(slave as libc::c_int - fslave as libc::c_int) as usize].alstatuscode;
                if (rval as libc::c_int & 0xfi32) < lowest as libc::c_int {
                    lowest = (rval as libc::c_int & 0xfi32) as u16
                }
                (*context).slavelist[slave as usize].state = rval;
                let ref mut fresh41 = (*context).slavelist[0].ALstatuscode;
                *fresh41 = (*fresh41 as libc::c_int
                    | (*context).slavelist[slave as usize].ALstatuscode as libc::c_int)
                    as u16;
                slave = slave.wrapping_add(1)
            }
            fslave = (lslave as libc::c_int + 1i32) as u16;
            if !((lslave as libc::c_int) < (*context).slavelist.len() as i32) {
                break;
            }
        }
        (*context).slavelist[0].state = lowest
    }
    return lowest as libc::c_int;
}
/* * Write slave state, if slave = 0 then write to all slaves.
 * The function does not check if the actual state is changed.
 * @param[in]  context        = context struct
 * @param[in] slave    = Slave number, 0 = master
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe fn ecx_writestate(context: *mut ecx_contextt, slave: u16) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut configadr: u16 = 0;
    let mut slstate: u16 = 0;
    if slave as libc::c_int == 0i32 {
        slstate = (*context).slavelist[slave as usize].state;
        ret = ecx_BWR(
            (*context).port.as_mut().unwrap(),
            0u16,
            EthercatRegister::ECT_REG_ALCTL as u16,
            ::core::mem::size_of::<u16>(),
            &mut slstate as *mut u16 as *mut libc::c_void,
            EC_TIMEOUTRET3,
        )
    } else {
        configadr = (*context).slavelist[slave as usize].configadr;
        ret = ecx_FPWRw(
            (*context).port.as_mut().unwrap(),
            configadr,
            EthercatRegister::ECT_REG_ALCTL as u16,
            (*context).slavelist[slave as usize].state,
            EC_TIMEOUTRET3,
        )
    }
    return ret;
}
/* * Check actual slave state.
 * This is a blocking function.
 * To refresh the state of all slaves ecx_readstate()should be called
 * @param[in] context     = context struct
 * @param[in] slave       = Slave number, 0 = all slaves (only the "slavelist[0].state" is refreshed)
 * @param[in] reqstate    = Requested state
 * @param[in] timeout     = Timeout value in us
 * @return Requested state, or found state after timeout.
 */
#[no_mangle]
pub unsafe fn ecx_statecheck(
    context: *mut ecx_contextt,
    slave: u16,
    reqstate: u16,
    timeout: u32,
) -> u16 {
    let mut configadr: u16 = 0; /* read slave status */
    let mut state: u16 = 0;
    let mut rval: u16 = 0;
    let mut slstat: ec_alstatust = ec_alstatust {
        alstatus: 0,
        unused: 0,
        alstatuscode: 0,
    };
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    if slave > (*context).slavelist.len() as u16 {
        return 0u16;
    }
    osal_timer_start(&mut timer, timeout);
    configadr = (*context).slavelist[slave as usize].configadr;
    loop {
        if slave < 1 {
            rval = 0u16;
            ecx_BRD(
                (*context).port.as_mut().unwrap(),
                0u16,
                EthercatRegister::ECT_REG_ALSTAT as u16,
                ::core::mem::size_of::<u16>(),
                &mut rval as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            rval = rval
        } else {
            slstat.alstatus = 0u16;
            slstat.alstatuscode = 0u16;
            ecx_FPRD(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_ALSTAT as u16,
                ::core::mem::size_of::<ec_alstatust>(),
                &mut slstat as *mut ec_alstatust as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            rval = slstat.alstatus;
            (*context).slavelist[slave as usize].ALstatuscode = slstat.alstatuscode
        }
        state = (rval as libc::c_int & 0xfi32) as u16;
        if state as libc::c_int != reqstate as libc::c_int {
            osal_usleep(1000u32);
        }
        if !(state as libc::c_int != reqstate as libc::c_int
            && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
        {
            break;
        }
    }
    (*context).slavelist[slave as usize].state = rval;
    return state;
}
/* * Get index of next mailbox counter value.
 * Used for Mailbox Link Layer.
 * @param[in] cnt     = Mailbox counter value [0..7]
 * @return next mailbox counter value
 */
#[no_mangle]
pub unsafe fn ec_nextmbxcnt(mut cnt: u8) -> u8 {
    cnt = cnt.wrapping_add(1);
    if cnt as libc::c_int > 7i32 {
        cnt = 1u8
        /* wrap around to 1, not 0 */
    }
    return cnt;
}
/* * Clear mailbox buffer.
 * @param[out] Mbx     = Mailbox buffer to clear
 */
#[no_mangle]
pub unsafe fn ec_clearmbx(Mbx: *mut ec_mbxbuft) {
    memset(Mbx as *mut libc::c_void, 0i32, EC_MAXMBX);
}
/* * Check if IN mailbox of slave is empty.
 * @param[in] context  = context struct
 * @param[in] slave    = Slave number
 * @param[in] timeout  = Timeout in us
 * @return >0 is success
 */
#[no_mangle]
pub unsafe fn ecx_mbxempty(context: *mut ecx_contextt, slave: u16, timeout: u32) -> libc::c_int {
    let mut configadr: u16 = 0;
    let mut SMstat: u8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer, timeout);
    configadr = (*context).slavelist[slave as usize].configadr;
    loop {
        SMstat = 0u8;
        wkc = ecx_FPRD(
            (*context).port.as_mut().unwrap(),
            configadr,
            EthercatRegister::ECT_REG_SM0STAT as u16,
            ::core::mem::size_of::<u8>(),
            &mut SMstat as *mut u8 as *mut libc::c_void,
            EC_TIMEOUTRET,
        );
        SMstat = SMstat;
        if SMstat as libc::c_int & 0x8i32 != 0i32 && timeout > EC_LOCALDELAY {
            osal_usleep(EC_LOCALDELAY);
        }
        if !((wkc <= 0i32 || SMstat as libc::c_int & 0x8i32 != 0i32)
            && osal_timer_is_expired(&mut timer) == false)
        {
            break;
        }
    }
    if wkc > 0i32 && SMstat as libc::c_int & 0x8i32 == 0i32 {
        return 1i32;
    }
    return 0i32;
}
/* * Write IN mailbox to slave.
 * @param[in]  context    = context struct
 * @param[in]  slave      = Slave number
 * @param[out] mbx        = Mailbox data
 * @param[in]  timeout    = Timeout in us
 * @return Work counter (>0 is success)
 */
#[no_mangle]
pub unsafe fn ecx_mbxsend(
    context: *mut ecx_contextt,
    slave: u16,
    mbx: *mut ec_mbxbuft,
    timeout: u32,
) -> libc::c_int {
    let mut mbxwo: u16 = 0;
    let mut mbxl: usize = 0;
    let mut configadr: u16 = 0;
    let mut wkc: libc::c_int = 0;
    wkc = 0i32;
    configadr = (*context).slavelist[slave as usize].configadr;
    mbxl = (*context).slavelist[slave as usize].mbx_l as usize;
    if mbxl > 0 && mbxl <= EC_MAXMBX {
        if ecx_mbxempty(context, slave, timeout) != 0 {
            mbxwo = (*context).slavelist[slave as usize].mbx_wo;
            /* write slave in mailbox */
            wkc = ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                mbxwo,
                mbxl,
                mbx as *mut libc::c_void,
                EC_TIMEOUTRET3,
            )
        } else {
            wkc = 0i32
        }
    }
    return wkc;
}
/* * Read OUT mailbox from slave.
 * Supports Mailbox Link Layer with repeat requests.
 * @param[in]  context    = context struct
 * @param[in]  slave      = Slave number
 * @param[out] mbx        = Mailbox data
 * @param[in]  timeout    = Timeout in us
 * @return Work counter (>0 is success)
 */
#[no_mangle]
pub unsafe fn ecx_mbxreceive(
    context: *mut ecx_contextt,
    slave: u16,
    mbx: *mut ec_mbxbuft,
    timeout: u32,
) -> libc::c_int {
    let mut mbxro: u16 = 0;
    let mut mbxl: usize = 0;
    let mut configadr: u16 = 0;
    let mut wkc: libc::c_int = 0i32;
    let mut wkc2: libc::c_int = 0;
    let mut SMstat: u16 = 0;
    let mut SMcontr: u8 = 0;
    let mut mbxh: *mut ec_mbxheadert = 0 as *mut ec_mbxheadert;
    let mut EMp: *mut ec_emcyt = 0 as *mut ec_emcyt;
    let mut MBXEp: *mut ec_mbxerrort = 0 as *mut ec_mbxerrort;
    configadr = (*context).slavelist[slave as usize].configadr;
    mbxl = (*context).slavelist[slave as usize].mbx_rl as usize;
    if mbxl as libc::c_int > 0i32 && mbxl <= EC_MAXMBX {
        let mut timer: osal_timert = osal_timert {
            stop_time: ec_timet { sec: 0, usec: 0 },
        };
        osal_timer_start(&mut timer, timeout);
        wkc = 0i32;
        loop {
            /* wait for read mailbox available */
            SMstat = 0u16;
            wkc = ecx_FPRD(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_SM1STAT as u16,
                ::core::mem::size_of::<u16>(),
                &mut SMstat as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            SMstat = SMstat;
            if SMstat as libc::c_int & 0x8i32 == 0i32 && timeout > EC_LOCALDELAY {
                osal_usleep(EC_LOCALDELAY);
            }
            if !((wkc <= 0i32 || SMstat as libc::c_int & 0x8i32 == 0i32)
                && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
            {
                break;
            }
        }
        if wkc > 0i32 && SMstat as libc::c_int & 0x8i32 > 0i32 {
            /* read mailbox available ? */
            mbxro = (*context).slavelist[slave as usize].mbx_ro;
            mbxh = mbx as *mut ec_mbxheadert;
            loop {
                /* if WKC<=0 repeat */
                wkc = ecx_FPRD(
                    (*context).port.as_mut().unwrap(),
                    configadr,
                    mbxro,
                    mbxl,
                    mbx as *mut libc::c_void,
                    EC_TIMEOUTRET,
                ); /* get mailbox */
                if wkc > 0i32 && (*mbxh).mbxtype as libc::c_int & 0xfi32 == 0i32 {
                    /* Mailbox error response? */
                    MBXEp = mbx as *mut ec_mbxerrort;
                    ecx_mbxerror(context.as_mut().unwrap(), slave, (*MBXEp).Detail);
                    wkc = 0i32
                /* prevent emergency to cascade up, it is already handled. */
                } else if wkc > 0i32
                    && (*mbxh).mbxtype as libc::c_int & 0xfi32 == MailboxType::Coe as libc::c_int
                {
                    /* CoE response? */
                    EMp = mbx as *mut ec_emcyt;
                    if (*EMp).CANOpen as libc::c_int >> 12i32 == 0x1i32 {
                        /* Emergency request? */
                        ecx_mbxemergencyerror(
                            context.as_mut().unwrap(),
                            slave,
                            (*EMp).ErrorCode,
                            (*EMp).ErrorReg as u16,
                            (*EMp).bData,
                            (*EMp).w1,
                            (*EMp).w2,
                        );
                        wkc = 0i32
                        /* prevent emergency to cascade up, it is already handled. */
                    }
                } else if wkc > 0i32
                    && (*mbxh).mbxtype as libc::c_int & 0xfi32 == MailboxType::Eoe as libc::c_int
                {
                    /* EoE response? */
                    let eoembx: *mut ec_EOEt = mbx as *mut ec_EOEt;
                    let frameinfo1: u16 = (*eoembx).frameinfo1;
                    /* All non fragment data frame types are expected to be handled by
                     * slave send/receive API if the EoE hook is set
                     */
                    if frameinfo1 as libc::c_int >> 0i32 & 0xfi32 == 0i32 {
                        if (*context).EOEhook.is_some() {
                            if (*context).EOEhook.expect("non-null function pointer")(
                                context,
                                slave,
                                eoembx as *mut libc::c_void,
                            ) > 0i32
                            {
                                /* Fragment handled by EoE hook */
                                wkc = 0i32
                            }
                        }
                    }
                } else if wkc <= 0i32 {
                    /* read mailbox lost */
                    SMstat = (SMstat as libc::c_int ^ 0x200i32) as u16; /* toggle repeat request */
                    SMstat = SMstat;
                    wkc2 = ecx_FPWR(
                        (*context).port.as_mut().unwrap(),
                        configadr,
                        EthercatRegister::ECT_REG_SM1STAT as u16,
                        ::core::mem::size_of::<u16>(),
                        &mut SMstat as *mut u16 as *mut libc::c_void,
                        EC_TIMEOUTRET,
                    );
                    SMstat = SMstat;
                    loop {
                        /* wait for toggle ack */
                        wkc2 = ecx_FPRD(
                            (*context).port.as_mut().unwrap(),
                            configadr,
                            EthercatRegister::ECT_REG_SM1CONTR as u16,
                            ::core::mem::size_of::<u8>(),
                            &mut SMcontr as *mut u8 as *mut libc::c_void,
                            EC_TIMEOUTRET,
                        );
                        if !((wkc2 <= 0i32
                            || SMcontr as libc::c_int & 0x2i32
                                != SMstat as libc::c_int >> 8i32 & 0x2i32)
                            && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
                        {
                            break;
                        }
                    }
                    loop {
                        /* wait for read mailbox available */
                        wkc2 = ecx_FPRD(
                            (*context).port.as_mut().unwrap(),
                            configadr,
                            EthercatRegister::ECT_REG_SM1STAT as u16,
                            ::core::mem::size_of::<u16>(),
                            &mut SMstat as *mut u16 as *mut libc::c_void,
                            EC_TIMEOUTRET,
                        );
                        SMstat = SMstat;
                        if SMstat as libc::c_int & 0x8i32 == 0i32 && timeout > EC_LOCALDELAY {
                            osal_usleep(EC_LOCALDELAY);
                        }
                        if !((wkc2 <= 0i32 || SMstat as libc::c_int & 0x8i32 == 0i32)
                            && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
                        {
                            break;
                        }
                    }
                }
                if !(wkc <= 0i32 && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32) {
                    break;
                }
            }
        } else if wkc > 0i32 {
            wkc = -(5i32)
        }
    }
    return wkc;
}
/* no read mailbox available */
/* * Dump complete EEPROM data from slave in buffer.
 * @param[in]  context  = context struct
 * @param[in]  slave    = Slave number
 * @param[out] esibuf   = EEPROM data buffer, make sure it is big enough.
 */
#[no_mangle]
pub unsafe fn ecx_esidump(context: *mut ecx_contextt, slave: u16, esibuf: *mut u8) {
    let mut configadr: u16 = 0; /* set eeprom control to master */
    let mut address: u16 = 0;
    let mut incr: u16 = 0;
    let mut p64: *mut u64 = 0 as *mut u64;
    let mut p16: *mut u16 = 0 as *mut u16;
    let mut edat: u64 = 0;
    let eectl: u8 = (*context).slavelist[slave as usize].eep_pdi;
    ecx_eeprom2master(context, slave);
    configadr = (*context).slavelist[slave as usize].configadr;
    address = 0x40u16;
    p16 = esibuf as *mut u16;
    if (*context).slavelist[slave as usize].eep_8byte != 0 {
        incr = 4u16
    } else {
        incr = 2u16
    }
    loop {
        edat = ecx_readeepromFP(context, configadr, address, EC_TIMEOUTEEP);
        p64 = p16 as *mut u64;
        *p64 = edat;
        p16 = p16.offset(incr as libc::c_int as isize);
        address = (address as libc::c_int + incr as libc::c_int) as u16;
        if !(address as libc::c_int <= (128i32) << 5i32 >> 1i32 && edat as u32 != 0xffffffffu32) {
            break;
        }
    }
    if eectl != 0 {
        ecx_eeprom2pdi(context, slave);
        /* if eeprom control was previously pdi then restore */
    };
}
/* * Read EEPROM from slave bypassing cache.
 * @param[in] context   = context struct
 * @param[in] slave     = Slave number
 * @param[in] eeproma   = (WORD) Address in the EEPROM
 * @param[in] timeout   = Timeout in us.
 * @return EEPROM data 32bit
 */
#[no_mangle]
pub unsafe fn ecx_readeeprom(
    context: *mut ecx_contextt,
    slave: u16,
    eeproma: u16,
    timeout: u32,
) -> u32 {
    let mut configadr: u16 = 0; /* set eeprom control to master */
    ecx_eeprom2master(context, slave);
    configadr = (*context).slavelist[slave as usize].configadr;
    return ecx_readeepromFP(context, configadr, eeproma, timeout) as u32;
}
/* * Write EEPROM to slave bypassing cache.
 * @param[in] context   = context struct
 * @param[in] slave     = Slave number
 * @param[in] eeproma   = (WORD) Address in the EEPROM
 * @param[in] data      = 16bit data
 * @param[in] timeout   = Timeout in us.
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe fn ecx_writeeeprom(
    context: *mut ecx_contextt,
    slave: u16,
    eeproma: u16,
    data: u16,
    timeout: u32,
) -> libc::c_int {
    let mut configadr: u16 = 0; /* set eeprom control to master */
    ecx_eeprom2master(context, slave);
    configadr = (*context).slavelist[slave as usize].configadr;
    return ecx_writeeepromFP(context, configadr, eeproma, data, timeout);
}
/* * Set eeprom control to master. Only if set to PDI.
 * @param[in] context   = context struct
 * @param[in] slave     = Slave number
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe fn ecx_eeprom2master(context: *mut ecx_contextt, slave: u16) -> libc::c_int {
    let mut wkc: libc::c_int = 1i32;
    let mut cnt: libc::c_int = 0i32;
    let mut configadr: u16 = 0;
    let mut eepctl: u8 = 0;
    if (*context).slavelist[slave as usize].eep_pdi != 0 {
        configadr = (*context).slavelist[slave as usize].configadr;
        eepctl = 2u8;
        loop {
            wkc = ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPCFG as u16,
                ::core::mem::size_of::<u8>(),
                &mut eepctl as *mut u8 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            if !(wkc <= 0i32 && {
                let fresh42 = cnt;
                cnt = cnt + 1;
                (fresh42) < EC_DEFAULTRETRIES
            }) {
                break;
            }
            /* force Eeprom from PDI */
        }
        eepctl = 0u8;
        cnt = 0i32;
        loop {
            wkc = ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPCFG as u16,
                ::core::mem::size_of::<u8>(),
                &mut eepctl as *mut u8 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            if !(wkc <= 0i32 && {
                let fresh43 = cnt;
                cnt = cnt + 1;
                (fresh43) < EC_DEFAULTRETRIES
            }) {
                break;
            }
            /* set Eeprom to master */
        }
        (*context).slavelist[slave as usize].eep_pdi = 0u8
    }
    return wkc;
}
/* * Set eeprom control to PDI. Only if set to master.
 * @param[in]  context        = context struct
 * @param[in] slave     = Slave number
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe fn ecx_eeprom2pdi(context: *mut ecx_contextt, slave: u16) -> libc::c_int {
    let mut wkc: libc::c_int = 1i32;
    let mut cnt: libc::c_int = 0i32;
    let mut configadr: u16 = 0;
    let mut eepctl: u8 = 0;
    if (*context).slavelist[slave as usize].eep_pdi == 0 {
        configadr = (*context).slavelist[slave as usize].configadr;
        eepctl = 1u8;
        loop {
            wkc = ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPCFG as u16,
                ::core::mem::size_of::<u8>(),
                &mut eepctl as *mut u8 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            if !(wkc <= 0i32 && {
                let fresh44 = cnt;
                cnt = cnt + 1;
                (fresh44) < EC_DEFAULTRETRIES
            }) {
                break;
            }
            /* set Eeprom to PDI */
        } /* wait for eeprom ready */
        (*context).slavelist[slave as usize].eep_pdi = 1u8
    }
    return wkc;
}
#[no_mangle]
pub unsafe fn ecx_eeprom_waitnotbusyAP(
    context: *mut ecx_contextt,
    aiadr: u16,
    estat: *mut u16,
    timeout: u32,
) -> u16 {
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0i32;
    let mut retval: u16 = 0u16;
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer, timeout);
    loop {
        let fresh45 = cnt;
        cnt = cnt + 1;
        if fresh45 != 0 {
            osal_usleep(EC_LOCALDELAY);
        }
        *estat = 0u16;
        wkc = ecx_APRD(
            (*context).port.as_mut().unwrap(),
            aiadr,
            EthercatRegister::ECT_REG_EEPSTAT as u16,
            ::core::mem::size_of::<u16>(),
            estat as *mut libc::c_void,
            EC_TIMEOUTRET,
        );
        *estat = *estat;
        if !((wkc <= 0i32 || *estat as libc::c_int & 0x8000i32 > 0i32)
            && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
        {
            break;
        }
    }
    if *estat as libc::c_int & 0x8000i32 == 0i32 {
        retval = 1u16
    }
    return retval;
}
/* * Read EEPROM from slave bypassing cache. APRD method.
 * @param[in] context     = context struct
 * @param[in] aiadr       = auto increment address of slave
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 * @param[in] timeout     = Timeout in us.
 * @return EEPROM data 64bit or 32bit
 */
#[no_mangle]
pub unsafe fn ecx_readeepromAP(
    context: *mut ecx_contextt,
    aiadr: u16,
    eeproma: u16,
    timeout: u32,
) -> u64 {
    let mut estat: u16 = 0;
    let mut edat32: u32 = 0;
    let mut edat64: u64 = 0;
    let mut ed: ec_eepromt = ec_eepromt {
        comm: 0,
        addr: 0,
        d2: 0,
    };
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut nackcnt: libc::c_int = 0i32;
    edat64 = 0u64;
    edat32 = 0u32;
    if ecx_eeprom_waitnotbusyAP(context, aiadr, &mut estat, timeout) != 0 {
        if estat as libc::c_int & EC_ESTAT_EMASK != 0 {
            /* error bits are set */
            estat = EepromCommand::Nop as u16; /* clear error bits */
            wkc = ecx_APWR(
                (*context).port.as_mut().unwrap(),
                aiadr,
                EthercatRegister::ECT_REG_EEPCTL as u16,
                ::core::mem::size_of::<u16>(),
                &mut estat as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRET3,
            )
        }
        loop {
            ed.comm = EepromCommand::Read as u16;
            ed.addr = eeproma;
            ed.d2 = 0u16;
            cnt = 0i32;
            loop {
                wkc = ecx_APWR(
                    (*context).port.as_mut().unwrap(),
                    aiadr,
                    EthercatRegister::ECT_REG_EEPCTL as u16,
                    ::core::mem::size_of::<ec_eepromt>(),
                    &mut ed as *mut ec_eepromt as *mut libc::c_void,
                    EC_TIMEOUTRET,
                );
                if !(wkc <= 0i32 && {
                    let fresh46 = cnt;
                    cnt = cnt + 1;
                    (fresh46) < EC_DEFAULTRETRIES
                }) {
                    break;
                }
            }
            if wkc != 0 {
                osal_usleep(EC_LOCALDELAY);
                estat = 0u16;
                if ecx_eeprom_waitnotbusyAP(context, aiadr, &mut estat, timeout) != 0 {
                    if estat as libc::c_int & 0x2000i32 != 0 {
                        nackcnt += 1;
                        osal_usleep(EC_LOCALDELAY * 5);
                    } else {
                        nackcnt = 0i32;
                        if estat as libc::c_int & 0x40i32 != 0 {
                            cnt = 0i32;
                            loop {
                                wkc = ecx_APRD(
                                    (*context).port.as_mut().unwrap(),
                                    aiadr,
                                    EthercatRegister::ECT_REG_EEPDAT as u16,
                                    ::core::mem::size_of::<u64>(),
                                    &mut edat64 as *mut u64 as *mut libc::c_void,
                                    EC_TIMEOUTRET,
                                );
                                if !(wkc <= 0i32 && {
                                    let fresh47 = cnt;
                                    cnt = cnt + 1;
                                    (fresh47) < EC_DEFAULTRETRIES
                                }) {
                                    break;
                                }
                            }
                        } else {
                            cnt = 0i32;
                            loop {
                                wkc = ecx_APRD(
                                    (*context).port.as_mut().unwrap(),
                                    aiadr,
                                    EthercatRegister::ECT_REG_EEPDAT as u16,
                                    ::core::mem::size_of::<u32>(),
                                    &mut edat32 as *mut u32 as *mut libc::c_void,
                                    EC_TIMEOUTRET,
                                );
                                if !(wkc <= 0i32 && {
                                    let fresh48 = cnt;
                                    cnt = cnt + 1;
                                    (fresh48) < EC_DEFAULTRETRIES
                                }) {
                                    break;
                                }
                            }
                            edat64 = edat32 as u64
                        }
                    }
                }
            }
            if !(nackcnt > 0i32 && nackcnt < 3i32) {
                break;
            }
        }
    }
    return edat64;
}
/* * Write EEPROM to slave bypassing cache. APWR method.
 * @param[in] context   = context struct
 * @param[in] aiadr     = configured address of slave
 * @param[in] eeproma   = (WORD) Address in the EEPROM
 * @param[in] data      = 16bit data
 * @param[in] timeout   = Timeout in us.
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe fn ecx_writeeepromAP(
    context: *mut ecx_contextt,
    aiadr: u16,
    eeproma: u16,
    mut data: u16,
    timeout: u32,
) -> libc::c_int {
    let mut estat: u16 = 0;
    let mut ed: ec_eepromt = ec_eepromt {
        comm: 0,
        addr: 0,
        d2: 0,
    };
    let mut wkc: libc::c_int = 0;
    let mut rval: libc::c_int = 0i32;
    let mut cnt: libc::c_int = 0i32;
    let mut nackcnt: libc::c_int = 0i32;
    if ecx_eeprom_waitnotbusyAP(context, aiadr, &mut estat, timeout) != 0 {
        if estat as libc::c_int & EC_ESTAT_EMASK != 0 {
            /* error bits are set */
            estat = EepromCommand::Nop as u16; /* clear error bits */
            wkc = ecx_APWR(
                (*context).port.as_mut().unwrap(),
                aiadr,
                EthercatRegister::ECT_REG_EEPCTL as u16,
                ::core::mem::size_of::<u16>(),
                &mut estat as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRET3,
            )
        } /* wait for eeprom ready */
        loop {
            cnt = 0i32;
            loop {
                wkc = ecx_APWR(
                    (*context).port.as_mut().unwrap(),
                    aiadr,
                    EthercatRegister::ECT_REG_EEPDAT as u16,
                    ::core::mem::size_of::<u16>(),
                    &mut data as *mut u16 as *mut libc::c_void,
                    EC_TIMEOUTRET,
                );
                if !(wkc <= 0i32 && {
                    let fresh49 = cnt;
                    cnt = cnt + 1;
                    (fresh49) < EC_DEFAULTRETRIES
                }) {
                    break;
                }
            }
            ed.comm = EepromCommand::Write as u16;
            ed.addr = eeproma;
            ed.d2 = 0u16;
            cnt = 0i32;
            loop {
                wkc = ecx_APWR(
                    (*context).port.as_mut().unwrap(),
                    aiadr,
                    EthercatRegister::ECT_REG_EEPCTL as u16,
                    ::core::mem::size_of::<ec_eepromt>(),
                    &mut ed as *mut ec_eepromt as *mut libc::c_void,
                    EC_TIMEOUTRET,
                );
                if !(wkc <= 0i32 && {
                    let fresh50 = cnt;
                    cnt = cnt + 1;
                    (fresh50) < EC_DEFAULTRETRIES
                }) {
                    break;
                }
            }
            if wkc != 0 {
                osal_usleep((200i32 * 2i32) as u32);
                estat = 0u16;
                if ecx_eeprom_waitnotbusyAP(context, aiadr, &mut estat, timeout) != 0 {
                    if estat as libc::c_int & 0x2000i32 != 0 {
                        nackcnt += 1;
                        osal_usleep(EC_LOCALDELAY * 5);
                    } else {
                        nackcnt = 0i32;
                        rval = 1i32
                    }
                }
            }
            if !(nackcnt > 0i32 && nackcnt < 3i32) {
                break;
            }
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe fn ecx_eeprom_waitnotbusyFP(
    context: *mut ecx_contextt,
    configadr: u16,
    estat: *mut u16,
    timeout: u32,
) -> u16 {
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0i32;
    let mut retval: u16 = 0u16;
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer, timeout);
    loop {
        let fresh51 = cnt;
        cnt = cnt + 1;
        if fresh51 != 0 {
            osal_usleep(EC_LOCALDELAY);
        }
        *estat = 0u16;
        wkc = ecx_FPRD(
            (*context).port.as_mut().unwrap(),
            configadr,
            EthercatRegister::ECT_REG_EEPSTAT as u16,
            ::core::mem::size_of::<u16>(),
            estat as *mut libc::c_void,
            EC_TIMEOUTRET,
        );
        *estat = *estat;
        if !((wkc <= 0i32 || *estat as libc::c_int & 0x8000i32 > 0i32)
            && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
        {
            break;
        }
    }
    if *estat as libc::c_int & 0x8000i32 == 0i32 {
        retval = 1u16
    }
    return retval;
}
/* * Read EEPROM from slave bypassing cache. FPRD method.
 * @param[in] context     = context struct
 * @param[in] configadr   = configured address of slave
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 * @param[in] timeout     = Timeout in us.
 * @return EEPROM data 64bit or 32bit
 */
#[no_mangle]
pub unsafe fn ecx_readeepromFP(
    context: *mut ecx_contextt,
    configadr: u16,
    eeproma: u16,
    timeout: u32,
) -> u64 {
    let mut estat: u16 = 0;
    let mut edat32: u32 = 0;
    let mut edat64: u64 = 0;
    let mut ed: ec_eepromt = ec_eepromt {
        comm: 0,
        addr: 0,
        d2: 0,
    };
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut nackcnt: libc::c_int = 0i32;
    edat64 = 0u64;
    edat32 = 0u32;
    if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, timeout) != 0 {
        if estat as libc::c_int & EC_ESTAT_EMASK != 0 {
            /* error bits are set */
            estat = EepromCommand::Nop as u16; /* clear error bits */
            wkc = ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPCTL as u16,
                ::core::mem::size_of::<u16>(),
                &mut estat as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRET3,
            )
        }
        loop {
            ed.comm = EepromCommand::Read as u16;
            ed.addr = eeproma;
            ed.d2 = 0u16;
            cnt = 0i32;
            loop {
                wkc = ecx_FPWR(
                    (*context).port.as_mut().unwrap(),
                    configadr,
                    EthercatRegister::ECT_REG_EEPCTL as u16,
                    ::core::mem::size_of::<ec_eepromt>(),
                    &mut ed as *mut ec_eepromt as *mut libc::c_void,
                    EC_TIMEOUTEEP,
                );
                if !(wkc <= 0i32 && {
                    let fresh52 = cnt;
                    cnt = cnt + 1;
                    (fresh52) < EC_DEFAULTRETRIES
                }) {
                    break;
                }
            }
            if wkc != 0 {
                osal_usleep(EC_LOCALDELAY);
                estat = 0u16;
                if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, timeout) != 0 {
                    if estat as libc::c_int & EC_ESTAT_NACK != 0 {
                        nackcnt += 1;
                        osal_usleep(EC_LOCALDELAY * 5);
                    } else {
                        nackcnt = 0i32;
                        if estat as libc::c_int & EC_ESTAT_R64 != 0 {
                            cnt = 0i32;
                            loop {
                                wkc = ecx_FPRD(
                                    (*context).port.as_mut().unwrap(),
                                    configadr,
                                    EthercatRegister::ECT_REG_EEPDAT as u16,
                                    ::core::mem::size_of::<u64>(),
                                    &mut edat64 as *mut u64 as *mut libc::c_void,
                                    EC_TIMEOUTRET,
                                );
                                if !(wkc <= 0i32 && {
                                    let fresh53 = cnt;
                                    cnt = cnt + 1;
                                    (fresh53) < EC_DEFAULTRETRIES
                                }) {
                                    break;
                                }
                            }
                        } else {
                            cnt = 0i32;
                            loop {
                                wkc = ecx_FPRD(
                                    (*context).port.as_mut().unwrap(),
                                    configadr,
                                    EthercatRegister::ECT_REG_EEPDAT as u16,
                                    ::core::mem::size_of::<u32>(),
                                    &mut edat32 as *mut u32 as *mut libc::c_void,
                                    EC_TIMEOUTRET,
                                );
                                if !(wkc <= 0i32 && {
                                    let fresh54 = cnt;
                                    cnt = cnt + 1;
                                    (fresh54) < EC_DEFAULTRETRIES
                                }) {
                                    break;
                                }
                            }
                            edat64 = edat32 as u64
                        }
                    }
                }
            }
            if !(nackcnt > 0i32 && nackcnt < 3i32) {
                break;
            }
        }
    }
    return edat64;
}
/* * Write EEPROM to slave bypassing cache. FPWR method.
 * @param[in]  context        = context struct
 * @param[in] configadr   = configured address of slave
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 * @param[in] data        = 16bit data
 * @param[in] timeout     = Timeout in us.
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe fn ecx_writeeepromFP(
    context: *mut ecx_contextt,
    configadr: u16,
    eeproma: u16,
    mut data: u16,
    timeout: u32,
) -> libc::c_int {
    let mut estat: u16 = 0;
    let mut ed: ec_eepromt = ec_eepromt {
        comm: 0,
        addr: 0,
        d2: 0,
    };
    let mut wkc: libc::c_int = 0;
    let mut rval: libc::c_int = 0i32;
    let mut cnt: libc::c_int = 0i32;
    let mut nackcnt: libc::c_int = 0i32;
    if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, timeout) != 0 {
        if estat as libc::c_int & EC_ESTAT_EMASK != 0 {
            /* error bits are set */
            estat = EepromCommand::Nop as u16; /* clear error bits */
            wkc = ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPCTL as u16,
                ::core::mem::size_of::<u16>(),
                &mut estat as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRET3,
            )
        }
        loop {
            cnt = 0i32;
            loop {
                wkc = ecx_FPWR(
                    (*context).port.as_mut().unwrap(),
                    configadr,
                    EthercatRegister::ECT_REG_EEPDAT as u16,
                    ::core::mem::size_of::<u16>(),
                    &mut data as *mut u16 as *mut libc::c_void,
                    EC_TIMEOUTRET,
                );
                if !(wkc <= 0i32 && {
                    let fresh55 = cnt;
                    cnt = cnt + 1;
                    (fresh55) < EC_DEFAULTRETRIES
                }) {
                    break;
                }
            }
            ed.comm = EepromCommand::Write as u16;
            ed.addr = eeproma;
            ed.d2 = 0u16;
            cnt = 0i32;
            loop {
                wkc = ecx_FPWR(
                    (*context).port.as_mut().unwrap(),
                    configadr,
                    EthercatRegister::ECT_REG_EEPCTL as u16,
                    ::core::mem::size_of::<ec_eepromt>(),
                    &mut ed as *mut ec_eepromt as *mut libc::c_void,
                    EC_TIMEOUTRET,
                );
                if !(wkc <= 0i32 && {
                    let fresh56 = cnt;
                    cnt = cnt + 1;
                    (fresh56) < EC_DEFAULTRETRIES
                }) {
                    break;
                }
            }
            if wkc != 0 {
                osal_usleep((200i32 * 2i32) as u32);
                estat = 0u16;
                if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, timeout) != 0 {
                    if estat as libc::c_int & EC_ESTAT_NACK != 0 {
                        nackcnt += 1;
                        osal_usleep(EC_LOCALDELAY * 5);
                    } else {
                        nackcnt = 0i32;
                        rval = 1i32
                    }
                }
            }
            if !(nackcnt > 0i32 && nackcnt < 3i32) {
                break;
            }
        }
    }
    return rval;
}
/* * Read EEPROM from slave bypassing cache.
 * Parallel read step 1, make request to slave.
 * @param[in] context     = context struct
 * @param[in] slave       = Slave number
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 */
#[no_mangle]
pub unsafe fn ecx_readeeprom1(context: *mut ecx_contextt, slave: u16, eeproma: u16) {
    let mut configadr: u16 = 0; /* set eeprom control to master */
    let mut estat: u16 = 0;
    let mut ed: ec_eepromt = ec_eepromt {
        comm: 0,
        addr: 0,
        d2: 0,
    };
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0i32;
    ecx_eeprom2master(context, slave);
    configadr = (*context).slavelist[slave as usize].configadr;
    if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, EC_TIMEOUTEEP) != 0 {
        if estat as libc::c_int & EC_ESTAT_EMASK != 0 {
            /* error bits are set */
            estat = EepromCommand::Nop as u16; /* clear error bits */
            wkc = ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPCTL as u16,
                ::core::mem::size_of::<u16>(),
                &mut estat as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRET3,
            )
        }
        ed.comm = EepromCommand::Read as u16;
        ed.addr = eeproma;
        ed.d2 = 0u16;
        loop {
            wkc = ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPCTL as u16,
                ::core::mem::size_of::<ec_eepromt>(),
                &mut ed as *mut ec_eepromt as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            if !(wkc <= 0i32 && {
                let fresh57 = cnt;
                cnt = cnt + 1;
                (fresh57) < EC_DEFAULTRETRIES
            }) {
                break;
            }
        }
    };
}
/* * Read EEPROM from slave bypassing cache.
 * Parallel read step 2, actual read from slave.
 * @param[in]  context        = context struct
 * @param[in] slave       = Slave number
 * @param[in] timeout     = Timeout in us.
 * @return EEPROM data 32bit
 */
#[no_mangle]
pub unsafe fn ecx_readeeprom2(context: *mut ecx_contextt, slave: u16, timeout: u32) -> u32 {
    let mut estat: u16 = 0;
    let mut configadr: u16 = 0;
    let mut edat: u32 = 0;
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0i32;
    configadr = (*context).slavelist[slave as usize].configadr;
    edat = 0u32;
    estat = 0u16;
    if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, timeout) != 0 {
        loop {
            wkc = ecx_FPRD(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPDAT as u16,
                ::core::mem::size_of::<u32>(),
                &mut edat as *mut u32 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            if !(wkc <= 0i32 && {
                let fresh58 = cnt;
                cnt = cnt + 1;
                (fresh58) < EC_DEFAULTRETRIES
            }) {
                break;
            }
        }
    }
    return edat;
}
/* * Push index of segmented LRD/LWR/LRW combination.
 * @param[in]  context        = context struct
 * @param[in] idx         = Used datagram index.
 * @param[in] data        = Pointer to process data segment.
 * @param[in] length      = Length of data segment in bytes.
 * @param[in] DCO         = Offset position of DC frame.
 */
unsafe fn ecx_pushindex(
    mut context: *mut ecx_contextt,
    idx: u8,
    data: *mut libc::c_void,
    length: usize,
    DCO: u16,
) {
    if ((*(*context).idxstack).pushed as libc::c_int) < 16i32 {
        (*(*context).idxstack).idx[(*(*context).idxstack).pushed as usize] = idx;
        (*(*context).idxstack).data[(*(*context).idxstack).pushed as usize] = data;
        (*(*context).idxstack).length[(*(*context).idxstack).pushed as usize] = length as u16;
        (*(*context).idxstack).dcoffset[(*(*context).idxstack).pushed as usize] = DCO;
        (*(*context).idxstack).pushed = (*(*context).idxstack).pushed.wrapping_add(1)
    };
}
/* * Pull index of segmented LRD/LWR/LRW combination.
 * @param[in]  context        = context struct
 * @return Stack location, -1 if stack is empty.
 */
unsafe fn ecx_pullindex(mut context: *mut ecx_contextt) -> libc::c_int {
    let mut rval: libc::c_int = -1;
    if ((*(*context).idxstack).pulled as libc::c_int) < (*(*context).idxstack).pushed as libc::c_int
    {
        rval = (*(*context).idxstack).pulled as libc::c_int;
        (*(*context).idxstack).pulled = (*(*context).idxstack).pulled.wrapping_add(1)
    }
    return rval;
}
/* *
 * Clear the idx stack.
 *
 * @param context           = context struct
 */
unsafe fn ecx_clearindex(mut context: *mut ecx_contextt) {
    (*(*context).idxstack).pushed = 0u8;
    (*(*context).idxstack).pulled = 0u8;
}
/* * Transmit processdata to slaves.
 * Uses LRW, or LRD/LWR if LRW is not allowed (blockLRW).
 * Both the input and output processdata are transmitted.
 * The outputs with the actual data, the inputs have a placeholder.
 * The inputs are gathered with the receive processdata function.
 * In contrast to the base LRW function this function is non-blocking.
 * If the processdata does not fit in one datagram, multiple are used.
 * In order to recombine the slave response, a stack is used.
 * @param[in]  context        = context struct
 * @param[in]  group          = group number
 * @param[in]  use_overlap_io = flag if overlapped iomap is used
 * @return >0 if processdata is transmitted.
 */
unsafe fn ecx_main_send_processdata(
    context: *mut ecx_contextt,
    group: u8,
    use_overlap_io: bool,
) -> libc::c_int {
    let mut LogAdr: u32 = 0;
    let mut w1: u16 = 0;
    let mut w2: u16 = 0;
    let mut length: libc::c_int = 0;
    let mut sublength: usize = 0;
    let mut idx: u8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut data: *mut u8 = 0 as *mut u8;
    let mut first: bool = false;
    let mut currentsegment: u16 = 0u16;
    let mut iomapinputoffset: u32 = 0;
    let mut DCO: u16 = 0;
    wkc = 0i32;
    if (*(*context).grouplist.offset(group as isize)).hasdc == true {
        first = true
    }
    /* For overlapping IO map use the biggest */
    if use_overlap_io == true {
        /* For overlap IOmap make the frame EQ big to biggest part */
        length = if (*(*context).grouplist.offset(group as isize)).Obytes
            > (*(*context).grouplist.offset(group as isize)).Ibytes
        {
            (*(*context).grouplist.offset(group as isize)).Obytes
        } else {
            (*(*context).grouplist.offset(group as isize)).Ibytes
        } as libc::c_int;
        /* Save the offset used to compensate where to save inputs when frame returns */
        iomapinputoffset = (*(*context).grouplist.offset(group as isize)).Obytes
    } else {
        length = (*(*context).grouplist.offset(group as isize))
            .Obytes
            .wrapping_add((*(*context).grouplist.offset(group as isize)).Ibytes)
            as libc::c_int;
        iomapinputoffset = 0u32
    }
    LogAdr = (*(*context).grouplist.offset(group as isize)).logstartaddr;
    if length != 0 {
        wkc = 1i32;
        /* LRW blocked by one or more slaves ? */
        if (*(*context).grouplist.offset(group as isize)).blockLRW != 0 {
            /* if inputs available generate LRD */
            if (*(*context).grouplist.offset(group as isize)).Ibytes != 0 {
                currentsegment = (*(*context).grouplist.offset(group as isize)).Isegment;
                data = (*(*context).grouplist.offset(group as isize)).inputs;
                length = (*(*context).grouplist.offset(group as isize)).Ibytes as libc::c_int;
                LogAdr =
                    (LogAdr).wrapping_add((*(*context).grouplist.offset(group as isize)).Obytes);
                loop
                /* segment transfer if needed */
                {
                    if currentsegment as libc::c_int
                        == (*(*context).grouplist.offset(group as isize)).Isegment as libc::c_int
                    {
                        let fresh59 = currentsegment;
                        currentsegment = currentsegment.wrapping_add(1);
                        sublength = (*(*context).grouplist.offset(group as isize)).IOsegment
                            [fresh59 as usize]
                            .wrapping_sub(
                                (*(*context).grouplist.offset(group as isize)).Ioffset
                                    as libc::c_uint,
                            ) as usize
                    } else {
                        let fresh60 = currentsegment;
                        currentsegment = currentsegment.wrapping_add(1);
                        sublength = (*(*context).grouplist.offset(group as isize)).IOsegment
                            [fresh60 as usize] as usize
                    }
                    /* get new index */
                    idx = ecx_getindex((*context).port.as_mut().unwrap());
                    w1 = (LogAdr & 0xffffu32) as u16;
                    w2 = (LogAdr >> 16i32) as u16;
                    DCO = 0u16;
                    ecx_setupdatagram(
                        (*context).port.as_mut().unwrap(),
                        &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                            as *mut ec_bufT as *mut libc::c_void,
                        Command::Lrd,
                        idx,
                        w1,
                        w2,
                        sublength,
                        data as *mut libc::c_void,
                    );
                    if first == true {
                        /* FPRMW in second datagram */
                        DCO = ecx_adddatagram(
                            (*context).port.as_mut().unwrap(),
                            &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                                as *mut ec_bufT as *mut libc::c_void,
                            Command::Frmw,
                            idx,
                            false,
                            (*context).slavelist
                                [(*(*context).grouplist.offset(group as isize)).DCnext as usize]
                                .configadr,
                            EthercatRegister::ECT_REG_DCSYSTIME as u16,
                            ::core::mem::size_of::<i64>(),
                            (*context).DCtime as *mut libc::c_void,
                        );
                        first = false
                    }
                    /* send frame */
                    ecx_outframe_red((*context).port.as_mut().unwrap(), idx);
                    /* push index and data pointer on stack */
                    ecx_pushindex(context, idx, data as *mut libc::c_void, sublength, DCO);
                    length -= sublength as libc::c_int;
                    LogAdr = (LogAdr).wrapping_add(sublength as libc::c_uint);
                    data = data.offset(sublength as libc::c_int as isize);
                    if !(length != 0
                        && (currentsegment as libc::c_int)
                            < (*(*context).grouplist.offset(group as isize)).nsegments
                                as libc::c_int)
                    {
                        break;
                    }
                }
            }
            /* if outputs available generate LWR */
            if (*(*context).grouplist.offset(group as isize)).Obytes != 0 {
                data = (*(*context).grouplist.offset(group as isize)).outputs;
                length = (*(*context).grouplist.offset(group as isize)).Obytes as libc::c_int;
                LogAdr = (*(*context).grouplist.offset(group as isize)).logstartaddr;
                currentsegment = 0u16;
                loop
                /* segment transfer if needed */
                {
                    let fresh61 = currentsegment;
                    currentsegment = currentsegment.wrapping_add(1);
                    sublength = (*(*context).grouplist.offset(group as isize)).IOsegment
                        [fresh61 as usize] as usize;
                    if (length - sublength as libc::c_int) < 0i32 {
                        sublength = length as usize
                    }
                    /* get new index */
                    idx = ecx_getindex((*context).port.as_mut().unwrap());
                    w1 = (LogAdr & 0xffffu32) as u16;
                    w2 = (LogAdr >> 16i32) as u16;
                    DCO = 0u16;
                    ecx_setupdatagram(
                        (*context).port.as_mut().unwrap(),
                        &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                            as *mut ec_bufT as *mut libc::c_void,
                        Command::Lwr,
                        idx,
                        w1,
                        w2,
                        sublength,
                        data as *mut libc::c_void,
                    );
                    if first == true {
                        /* FPRMW in second datagram */
                        DCO = ecx_adddatagram(
                            (*context).port.as_mut().unwrap(),
                            &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                                as *mut ec_bufT as *mut libc::c_void,
                            Command::Frmw,
                            idx,
                            false,
                            (*context).slavelist
                                [(*(*context).grouplist.offset(group as isize)).DCnext as usize]
                                .configadr,
                            EthercatRegister::ECT_REG_DCSYSTIME as u16,
                            ::core::mem::size_of::<i64>(),
                            (*context).DCtime as *mut libc::c_void,
                        );
                        first = false
                    }
                    /* send frame */
                    ecx_outframe_red((*context).port.as_mut().unwrap(), idx);
                    /* push index and data pointer on stack */
                    ecx_pushindex(context, idx, data as *mut libc::c_void, sublength, DCO);
                    length -= sublength as libc::c_int;
                    LogAdr = (LogAdr).wrapping_add(sublength as libc::c_uint);
                    data = data.offset(sublength as libc::c_int as isize);
                    if !(length != 0
                        && (currentsegment as libc::c_int)
                            < (*(*context).grouplist.offset(group as isize)).nsegments
                                as libc::c_int)
                    {
                        break;
                    }
                }
            }
        } else {
            /* LRW can be used */
            if (*(*context).grouplist.offset(group as isize)).Obytes != 0 {
                data = (*(*context).grouplist.offset(group as isize)).outputs
            } else {
                data = (*(*context).grouplist.offset(group as isize)).inputs;
                /* Clear offset, don't compensate for overlapping IOmap if we only got inputs */
                iomapinputoffset = 0u32
            }
            loop
            /* segment transfer if needed */
            {
                let fresh62 = currentsegment;
                currentsegment = currentsegment.wrapping_add(1);
                sublength = (*(*context).grouplist.offset(group as isize)).IOsegment
                    [fresh62 as usize] as usize;
                /* get new index */
                idx = ecx_getindex((*context).port.as_mut().unwrap());
                w1 = (LogAdr & 0xffffu32) as u16;
                w2 = (LogAdr >> 16i32) as u16;
                DCO = 0u16;
                ecx_setupdatagram(
                    (*context).port.as_mut().unwrap(),
                    &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                        as *mut libc::c_void,
                    Command::Lrw,
                    idx,
                    w1,
                    w2,
                    sublength,
                    data as *mut libc::c_void,
                );
                if first == true {
                    /* FPRMW in second datagram */
                    DCO = ecx_adddatagram(
                        (*context).port.as_mut().unwrap(),
                        &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                            as *mut ec_bufT as *mut libc::c_void,
                        Command::Frmw,
                        idx,
                        false,
                        (*context).slavelist
                            [(*(*context).grouplist.offset(group as isize)).DCnext as usize]
                            .configadr,
                        EthercatRegister::ECT_REG_DCSYSTIME as u16,
                        ::core::mem::size_of::<i64>(),
                        (*context).DCtime as *mut libc::c_void,
                    );
                    first = false
                }
                /* send frame */
                ecx_outframe_red((*context).port.as_mut().unwrap(), idx);
                /* push index and data pointer on stack.
                 * the iomapinputoffset compensate for where the inputs are stored
                 * in the IOmap if we use an overlapping IOmap. If a regular IOmap
                 * is used it should always be 0.
                 */
                ecx_pushindex(
                    context,
                    idx,
                    data.offset(iomapinputoffset as isize) as *mut libc::c_void,
                    sublength,
                    DCO,
                );
                length -= sublength as libc::c_int;
                LogAdr = (LogAdr).wrapping_add(sublength as libc::c_uint);
                data = data.offset(sublength as libc::c_int as isize);
                if !(length != 0
                    && (currentsegment as libc::c_int)
                        < (*(*context).grouplist.offset(group as isize)).nsegments as libc::c_int)
                {
                    break;
                }
            }
        }
    }
    return wkc;
}
/* * Transmit processdata to slaves.
* Uses LRW, or LRD/LWR if LRW is not allowed (blockLRW).
* Both the input and output processdata are transmitted in the overlapped IOmap.
* The outputs with the actual data, the inputs replace the output data in the
* returning frame. The inputs are gathered with the receive processdata function.
* In contrast to the base LRW function this function is non-blocking.
* If the processdata does not fit in one datagram, multiple are used.
* In order to recombine the slave response, a stack is used.
* @param[in]  context        = context struct
* @param[in]  group          = group number
* @return >0 if processdata is transmitted.
*/
#[no_mangle]
pub unsafe fn ecx_send_overlap_processdata_group(
    context: *mut ecx_contextt,
    group: u8,
) -> libc::c_int {
    return ecx_main_send_processdata(context, group, true);
}
/* * Transmit processdata to slaves.
* Uses LRW, or LRD/LWR if LRW is not allowed (blockLRW).
* Both the input and output processdata are transmitted.
* The outputs with the actual data, the inputs have a placeholder.
* The inputs are gathered with the receive processdata function.
* In contrast to the base LRW function this function is non-blocking.
* If the processdata does not fit in one datagram, multiple are used.
* In order to recombine the slave response, a stack is used.
* @param[in]  context        = context struct
* @param[in]  group          = group number
* @return >0 if processdata is transmitted.
*/
#[no_mangle]
pub unsafe fn ecx_send_processdata_group(context: *mut ecx_contextt, group: u8) -> libc::c_int {
    return ecx_main_send_processdata(context, group, false);
}
/* * Receive processdata from slaves.
 * Second part from ec_send_processdata().
 * Received datagrams are recombined with the processdata with help from the stack.
 * If a datagram contains input processdata it copies it to the processdata structure.
 * @param[in]  context        = context struct
 * @param[in]  group          = group number
 * @param[in]  timeout        = Timeout in us.
 * @return Work counter.
 */
#[no_mangle]
pub unsafe fn ecx_receive_processdata_group(
    context: *mut ecx_contextt,
    group: u8,
    timeout: u32,
) -> libc::c_int {
    let mut idx: u8 = 0;
    let mut pos: libc::c_int = 0;
    let mut wkc: libc::c_int = 0i32;
    let mut wkc2: libc::c_int = 0;
    let mut le_wkc: u16 = 0u16;
    let mut valid_wkc: libc::c_int = 0i32;
    let mut le_DCtime: i64 = 0;
    let mut idxstack: *mut ec_idxstackT = 0 as *mut ec_idxstackT;
    let mut rxbuf: *mut ec_bufT = 0 as *mut ec_bufT;
    /* just to prevent compiler warning for unused group */
    wkc2 = group as libc::c_int;
    idxstack = (*context).idxstack;
    rxbuf = (*(*context).port).rxbuf.as_mut_ptr();
    /* get first index */
    pos = ecx_pullindex(context);
    /* read the same number of frames as send */
    while pos >= 0i32 {
        idx = (*idxstack).idx[pos as usize];
        wkc2 = ecx_waitinframe((*context).port.as_mut().unwrap(), idx, timeout);
        /* check if there is input data in frame */
        if wkc2 > -1 {
            if (*rxbuf.offset(idx as isize))[::core::mem::size_of::<u16>()] as libc::c_int
                == Command::Lrd as libc::c_int
                || (*rxbuf.offset(idx as isize))[::core::mem::size_of::<u16>()] as libc::c_int
                    == Command::Lrw as libc::c_int
            {
                if (*idxstack).dcoffset[pos as usize] as libc::c_int > 0i32 {
                    memcpy(
                        (*idxstack).data[pos as usize],
                        &mut *(*rxbuf.offset(idx as isize))
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<EthercatHeader>() as isize)
                            as *mut u8 as *const libc::c_void,
                        (*idxstack).length[pos as usize] as usize,
                    );
                    memcpy(
                        &mut le_wkc as *mut u16 as *mut libc::c_void,
                        &mut *(*rxbuf.offset(idx as isize)).as_mut_ptr().offset(
                            core::mem::size_of::<EthercatHeader>().wrapping_add(
                                *(*idxstack).length.as_mut_ptr().offset(pos as isize) as usize,
                            ) as isize,
                        ) as *mut u8 as *const libc::c_void,
                        core::mem::size_of::<u16>(),
                    );
                    wkc = le_wkc as libc::c_int;
                    memcpy(&mut le_DCtime as *mut i64 as *mut libc::c_void,
                           &mut *(*rxbuf.offset(idx as
                                                    isize)).as_mut_ptr().offset(*(*idxstack).dcoffset.as_mut_ptr().offset(pos
                                                                                                                              as
                                                                                                                              isize)
                                                                                    as
                                                                                    isize)
                               as *mut u8 as *const libc::c_void,
                           core::mem::size_of::<i64>(),);
                    *(*context).DCtime = le_DCtime
                } else {
                    /* copy input data back to process data buffer */
                    memcpy(
                        (*idxstack).data[pos as usize],
                        &mut *(*rxbuf.offset(idx as isize))
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<EthercatHeader>() as isize)
                            as *mut u8 as *const libc::c_void,
                        (*idxstack).length[pos as usize] as usize,
                    );
                    wkc += wkc2
                }
                valid_wkc = 1i32
            } else if (*rxbuf.offset(idx as isize))[::core::mem::size_of::<u16>()] as libc::c_int
                == Command::Lwr as libc::c_int
            {
                if (*idxstack).dcoffset[pos as usize] as libc::c_int > 0i32 {
                    memcpy(
                        &mut le_wkc as *mut u16 as *mut libc::c_void,
                        &mut *(*rxbuf.offset(idx as isize)).as_mut_ptr().offset(
                            core::mem::size_of::<EthercatHeader>().wrapping_add(
                                *(*idxstack).length.as_mut_ptr().offset(pos as isize) as usize,
                            ) as isize,
                        ) as *mut u8 as *const libc::c_void,
                        core::mem::size_of::<u16>(),
                    );
                    /* output WKC counts 2 times when using LRW, emulate the same for LWR */
                    wkc = le_wkc as libc::c_int * 2i32;
                    memcpy(&mut le_DCtime as *mut i64 as *mut libc::c_void,
                           &mut *(*rxbuf.offset(idx as
                                                    isize)).as_mut_ptr().offset(*(*idxstack).dcoffset.as_mut_ptr().offset(pos
                                                                                                                              as
                                                                                                                              isize)
                                                                                    as
                                                                                    isize)
                               as *mut u8 as *const libc::c_void,
                           core::mem::size_of::<i64>(),);
                    *(*context).DCtime = le_DCtime
                } else {
                    /* output WKC counts 2 times when using LRW, emulate the same for LWR */
                    wkc += wkc2 * 2i32
                }
                valid_wkc = 1i32
            }
        }
        /* release buffer */
        ecx_setbufstat((*context).port.as_mut().unwrap(), idx, BufferState::Empty);
        /* get next index */
        pos = ecx_pullindex(context)
    }
    ecx_clearindex(context);
    /* if no frames has arrived */
    if valid_wkc == 0i32 {
        return -1;
    }
    return wkc;
}
#[no_mangle]
pub unsafe fn ecx_send_processdata(context: *mut ecx_contextt) -> libc::c_int {
    return ecx_send_processdata_group(context, 0u8);
}
#[no_mangle]
pub unsafe fn ecx_send_overlap_processdata(context: *mut ecx_contextt) -> libc::c_int {
    return ecx_send_overlap_processdata_group(context, 0u8);
}
#[no_mangle]
pub unsafe fn ecx_receive_processdata(context: *mut ecx_contextt, timeout: u32) -> libc::c_int {
    return ecx_receive_processdata_group(context, 0u8, timeout);
}
#[no_mangle]
pub unsafe fn ec_pusherror(Ec: ec_errort) {
    ecx_pusherror(&mut ecx_context, Ec);
}
#[no_mangle]
pub unsafe fn ec_poperror(Ec: &mut ec_errort) -> bool {
    return ecx_poperror(&mut ecx_context, Ec);
}
#[no_mangle]
pub unsafe fn ec_iserror() -> bool {
    return ecx_iserror(&mut ecx_context);
}
#[no_mangle]
pub unsafe fn ec_packeterror(Slave: u16, Index: u16, SubIdx: u8, ErrorCode: u16) {
    ecx_packeterror(&mut ecx_context, Slave, Index, SubIdx, ErrorCode);
}
/* * Initialise lib in single NIC mode
 * @param[in] ifname   = Dev name, f.e. "eth0"
 * @return >0 if OK
 * @see ecx_init
 */
#[no_mangle]
pub unsafe fn ec_init(ifname: *const libc::c_char) -> libc::c_int {
    return ecx_init(&mut ecx_context, ifname);
}
// FIXME: Remove all this global stuff
// /* * Initialise lib in redundant NIC mode
//  * @param[in]  ifname   = Primary Dev name, f.e. "eth0"
//  * @param[in]  if2name  = Secondary Dev name, f.e. "eth1"
//  * @return >0 if OK
//  * @see ecx_init_redundant
//  */
// #[no_mangle]
// pub unsafe fn ec_init_redundant(
//     ifname: *const libc::c_char,
//     if2name: *mut libc::c_char,
// ) -> libc::c_int {
//     return ecx_init_redundant(&mut ecx_context, ecx_redport, ifname, if2name);
// }
/* * Close lib.
 * @see ecx_close
 */
#[no_mangle]
pub unsafe fn ec_close() {
    ecx_close(&mut ecx_context);
}
/* * Read one byte from slave EEPROM via cache.
 *  If the cache location is empty then a read request is made to the slave.
 *  Depending on the slave capabillities the request is 4 or 8 bytes.
 *  @param[in] slave   = slave number
 *  @param[in] address = eeprom address in bytes (slave uses words)
 *  @return requested byte, if not available then 0xff
 * @see ecx_siigetbyte
 */
#[no_mangle]
pub unsafe fn ec_siigetbyte(slave: u16, address: u16) -> u8 {
    return ecx_siigetbyte(&mut ecx_context, slave, address);
}
/* * Find SII section header in slave EEPROM.
 *  @param[in] slave   = slave number
 *  @param[in] cat     = section category
 *  @return byte address of section at section length entry, if not available then 0
 *  @see ecx_siifind
 */
#[no_mangle]
pub unsafe fn ec_siifind(slave: u16, cat: u16) -> i16 {
    return ecx_siifind(&mut ecx_context, slave, cat);
}
/* * Get string from SII string section in slave EEPROM.
 *  @param[out] str    = requested string, 0x00 if not found
 *  @param[in]  slave  = slave number
 *  @param[in]  Sn     = string number
 *  @see ecx_siistring
 */
#[no_mangle]
pub unsafe fn ec_siistring(str: *mut libc::c_char, slave: u16, Sn: u16) {
    ecx_siistring(&mut ecx_context, str, slave, Sn);
}
/* * Get FMMU data from SII FMMU section in slave EEPROM.
 *  @param[in]  slave  = slave number
 *  @param[out] FMMU   = FMMU struct from SII, max. 4 FMMU's
 *  @return number of FMMU's defined in section
 *  @see ecx_siiFMMU
 */
#[no_mangle]
pub unsafe fn ec_siiFMMU(slave: u16, FMMU: *mut ec_eepromFMMUt) -> u16 {
    return ecx_siiFMMU(&mut ecx_context, slave, FMMU);
}
/* * Get SM data from SII SM section in slave EEPROM.
 *  @param[in]  slave   = slave number
 *  @param[out] SM      = first SM struct from SII
 *  @return number of SM's defined in section
 *  @see ecx_siiSM
 */
#[no_mangle]
pub unsafe fn ec_siiSM(slave: u16, SM: *mut ec_eepromSMt) -> u16 {
    return ecx_siiSM(&mut ecx_context, slave, SM);
}
/* * Get next SM data from SII SM section in slave EEPROM.
 *  @param[in]  slave  = slave number
 *  @param[out] SM     = first SM struct from SII
 *  @param[in]  n      = SM number
 *  @return >0 if OK
 *  @see ecx_siiSMnext
 */
#[no_mangle]
pub unsafe fn ec_siiSMnext(slave: u16, SM: *mut ec_eepromSMt, n: u16) -> u16 {
    return ecx_siiSMnext(&mut ecx_context, slave, SM, n);
}
/* * Get PDO data from SII PDO section in slave EEPROM.
 *  @param[in]  slave  = slave number
 *  @param[out] PDO    = PDO struct from SII
 *  @param[in]  t      = 0=RXPDO 1=TXPDO
 *  @return mapping size in bits of PDO
 *  @see ecx_siiPDO
 */
#[no_mangle]
pub unsafe fn ec_siiPDO(slave: u16, PDO: *mut ec_eepromPDOt, t: u8) -> u32 {
    return ecx_siiPDO(&mut ecx_context, slave, PDO, t);
}
/* * Read all slave states in ec_slave.
 * @return lowest state found
 * @see ecx_readstate
 */
#[no_mangle]
pub unsafe fn ec_readstate() -> libc::c_int {
    return ecx_readstate(&mut ecx_context);
}
/* * Write slave state, if slave = 0 then write to all slaves.
 * The function does not check if the actual state is changed.
 * @param[in] slave = Slave number, 0 = master
 * @return 0
 * @see ecx_writestate
 */
#[no_mangle]
pub unsafe fn ec_writestate(slave: u16) -> libc::c_int {
    return ecx_writestate(&mut ecx_context, slave);
}
/* * Check actual slave state.
 * This is a blocking function.
 * @param[in] slave       = Slave number, 0 = all slaves
 * @param[in] reqstate    = Requested state
 * @param[in] timeout     = Timeout value in us
 * @return Requested state, or found state after timeout.
 * @see ecx_statecheck
 */
#[no_mangle]
pub unsafe fn ec_statecheck(slave: u16, reqstate: u16, timeout: u32) -> u16 {
    return ecx_statecheck(&mut ecx_context, slave, reqstate, timeout);
}
/* * Check if IN mailbox of slave is empty.
 * @param[in] slave    = Slave number
 * @param[in] timeout  = Timeout in us
 * @return >0 is success
 * @see ecx_mbxempty
 */
#[no_mangle]
pub unsafe fn ec_mbxempty(slave: u16, timeout: u32) -> libc::c_int {
    return ecx_mbxempty(&mut ecx_context, slave, timeout);
}
/* * Write IN mailbox to slave.
 * @param[in]  slave      = Slave number
 * @param[out] mbx        = Mailbox data
 * @param[in]  timeout    = Timeout in us
 * @return Work counter (>0 is success)
 * @see ecx_mbxsend
 */
#[no_mangle]
pub unsafe fn ec_mbxsend(slave: u16, mbx: *mut ec_mbxbuft, timeout: u32) -> libc::c_int {
    return ecx_mbxsend(&mut ecx_context, slave, mbx, timeout);
}
/* * Read OUT mailbox from slave.
 * Supports Mailbox Link Layer with repeat requests.
 * @param[in]  slave      = Slave number
 * @param[out] mbx        = Mailbox data
 * @param[in]  timeout    = Timeout in us
 * @return Work counter (>0 is success)
 * @see ecx_mbxreceive
 */
#[no_mangle]
pub unsafe fn ec_mbxreceive(slave: u16, mbx: *mut ec_mbxbuft, timeout: u32) -> libc::c_int {
    return ecx_mbxreceive(&mut ecx_context, slave, mbx, timeout);
}
/* * Dump complete EEPROM data from slave in buffer.
 * @param[in]  slave    = Slave number
 * @param[out] esibuf   = EEPROM data buffer, make sure it is big enough.
 * @see ecx_esidump
 */
#[no_mangle]
pub unsafe fn ec_esidump(slave: u16, esibuf: *mut u8) {
    ecx_esidump(&mut ecx_context, slave, esibuf);
}
/* * Read EEPROM from slave bypassing cache.
 * @param[in] slave     = Slave number
 * @param[in] eeproma   = (WORD) Address in the EEPROM
 * @param[in] timeout   = Timeout in us.
 * @return EEPROM data 32bit
 * @see ecx_readeeprom
 */
#[no_mangle]
pub unsafe fn ec_readeeprom(slave: u16, eeproma: u16, timeout: u32) -> u32 {
    return ecx_readeeprom(&mut ecx_context, slave, eeproma, timeout);
}
/* * Write EEPROM to slave bypassing cache.
 * @param[in] slave     = Slave number
 * @param[in] eeproma   = (WORD) Address in the EEPROM
 * @param[in] data      = 16bit data
 * @param[in] timeout   = Timeout in us.
 * @return >0 if OK
 * @see ecx_writeeeprom
 */
#[no_mangle]
pub unsafe fn ec_writeeeprom(slave: u16, eeproma: u16, data: u16, timeout: u32) -> libc::c_int {
    return ecx_writeeeprom(&mut ecx_context, slave, eeproma, data, timeout);
}
/* * Set eeprom control to master. Only if set to PDI.
 * @param[in] slave = Slave number
 * @return >0 if OK
 * @see ecx_eeprom2master
 */
#[no_mangle]
pub unsafe fn ec_eeprom2master(slave: u16) -> libc::c_int {
    return ecx_eeprom2master(&mut ecx_context, slave);
}
#[no_mangle]
pub unsafe fn ec_eeprom2pdi(slave: u16) -> libc::c_int {
    return ecx_eeprom2pdi(&mut ecx_context, slave);
}
#[no_mangle]
pub unsafe fn ec_eeprom_waitnotbusyAP(aiadr: u16, estat: *mut u16, timeout: u32) -> u16 {
    return ecx_eeprom_waitnotbusyAP(&mut ecx_context, aiadr, estat, timeout);
}
/* * Read EEPROM from slave bypassing cache. APRD method.
 * @param[in] aiadr       = auto increment address of slave
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 * @param[in] timeout     = Timeout in us.
 * @return EEPROM data 64bit or 32bit
 */
#[no_mangle]
pub unsafe fn ec_readeepromAP(aiadr: u16, eeproma: u16, timeout: u32) -> u64 {
    return ecx_readeepromAP(&mut ecx_context, aiadr, eeproma, timeout);
}
/* * Write EEPROM to slave bypassing cache. APWR method.
 * @param[in] aiadr     = configured address of slave
 * @param[in] eeproma   = (WORD) Address in the EEPROM
 * @param[in] data      = 16bit data
 * @param[in] timeout   = Timeout in us.
 * @return >0 if OK
 * @see ecx_writeeepromAP
 */
#[no_mangle]
pub unsafe fn ec_writeeepromAP(aiadr: u16, eeproma: u16, data: u16, timeout: u32) -> libc::c_int {
    return ecx_writeeepromAP(&mut ecx_context, aiadr, eeproma, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_eeprom_waitnotbusyFP(configadr: u16, estat: *mut u16, timeout: u32) -> u16 {
    return ecx_eeprom_waitnotbusyFP(&mut ecx_context, configadr, estat, timeout);
}
/* * Read EEPROM from slave bypassing cache. FPRD method.
 * @param[in] configadr   = configured address of slave
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 * @param[in] timeout     = Timeout in us.
 * @return EEPROM data 64bit or 32bit
 * @see ecx_readeepromFP
 */
#[no_mangle]
pub unsafe fn ec_readeepromFP(configadr: u16, eeproma: u16, timeout: u32) -> u64 {
    return ecx_readeepromFP(&mut ecx_context, configadr, eeproma, timeout);
}
/* * Write EEPROM to slave bypassing cache. FPWR method.
 * @param[in] configadr   = configured address of slave
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 * @param[in] data        = 16bit data
 * @param[in] timeout     = Timeout in us.
 * @return >0 if OK
 * @see ecx_writeeepromFP
 */
#[no_mangle]
pub unsafe fn ec_writeeepromFP(
    configadr: u16,
    eeproma: u16,
    data: u16,
    timeout: u32,
) -> libc::c_int {
    return ecx_writeeepromFP(&mut ecx_context, configadr, eeproma, data, timeout);
}
/* * Read EEPROM from slave bypassing cache.
 * Parallel read step 1, make request to slave.
 * @param[in] slave       = Slave number
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 * @see ecx_readeeprom1
 */
#[no_mangle]
pub unsafe fn ec_readeeprom1(slave: u16, eeproma: u16) {
    ecx_readeeprom1(&mut ecx_context, slave, eeproma);
}
/* * Read EEPROM from slave bypassing cache.
 * Parallel read step 2, actual read from slave.
 * @param[in] slave       = Slave number
 * @param[in] timeout     = Timeout in us.
 * @return EEPROM data 32bit
 * @see ecx_readeeprom2
 */
#[no_mangle]
pub unsafe fn ec_readeeprom2(slave: u16, timeout: u32) -> u32 {
    return ecx_readeeprom2(&mut ecx_context, slave, timeout);
}
/* * Transmit processdata to slaves.
 * Uses LRW, or LRD/LWR if LRW is not allowed (blockLRW).
 * Both the input and output processdata are transmitted.
 * The outputs with the actual data, the inputs have a placeholder.
 * The inputs are gathered with the receive processdata function.
 * In contrast to the base LRW function this function is non-blocking.
 * If the processdata does not fit in one datagram, multiple are used.
 * In order to recombine the slave response, a stack is used.
 * @param[in]  group          = group number
 * @return >0 if processdata is transmitted.
 * @see ecx_send_processdata_group
 */
#[no_mangle]
pub unsafe fn ec_send_processdata_group(group: u8) -> libc::c_int {
    return ecx_send_processdata_group(&mut ecx_context, group);
}
/* * Transmit processdata to slaves.
* Uses LRW, or LRD/LWR if LRW is not allowed (blockLRW).
* Both the input and output processdata are transmitted in the overlapped IOmap.
* The outputs with the actual data, the inputs replace the output data in the
* returning frame. The inputs are gathered with the receive processdata function.
* In contrast to the base LRW function this function is non-blocking.
* If the processdata does not fit in one datagram, multiple are used.
* In order to recombine the slave response, a stack is used.
* @param[in]  group          = group number
* @return >0 if processdata is transmitted.
* @see ecx_send_overlap_processdata_group
*/
#[no_mangle]
pub unsafe fn ec_send_overlap_processdata_group(group: u8) -> libc::c_int {
    return ecx_send_overlap_processdata_group(&mut ecx_context, group);
}
/* * Receive processdata from slaves.
 * Second part from ec_send_processdata().
 * Received datagrams are recombined with the processdata with help from the stack.
 * If a datagram contains input processdata it copies it to the processdata structure.
 * @param[in]  group          = group number
 * @param[in]  timeout        = Timeout in us.
 * @return Work counter.
 * @see ecx_receive_processdata_group
 */
#[no_mangle]
pub unsafe fn ec_receive_processdata_group(group: u8, timeout: u32) -> libc::c_int {
    return ecx_receive_processdata_group(&mut ecx_context, group, timeout);
}
#[no_mangle]
pub unsafe fn ec_send_processdata() -> libc::c_int {
    return ec_send_processdata_group(0u8);
}
#[no_mangle]
pub unsafe fn ec_send_overlap_processdata() -> libc::c_int {
    return ec_send_overlap_processdata_group(0u8);
}
#[no_mangle]
pub unsafe fn ec_receive_processdata(timeout: u32) -> libc::c_int {
    return ec_receive_processdata_group(0u8, timeout);
}
unsafe fn run_static_initializers() {
    ecx_context = {
        let init = ecx_context {
            port: &mut ecx_port,
            slavelist: heapless::Vec::new(),
            slavecount: 0,
            maxslave: EC_MAXSLAVE as i32,
            grouplist: &mut *ec_group.as_mut_ptr().offset(0isize) as *mut ec_groupt,
            maxgroup: 2i32,
            esibuf: &mut *EC_ESI_BUF.as_mut_ptr().offset(0isize) as *mut u8,
            esimap: &mut *EC_ESI_MAP.as_mut_ptr().offset(0isize) as *mut u32,
            esislave: 0u16,
            elist: heapless::Vec::new(),
            idxstack: &mut EC_IDX_STACK,
            ecaterror: EcatError,
            DCtime: &mut ec_DCtime,
            SMcommtype: &mut *EC_SM_COMMTYPE.as_mut_ptr().offset(0isize) as *mut ec_SMcommtypet,
            PDOassign: &mut *EC_PDO_ASSIGN.as_mut_ptr().offset(0isize) as *mut ec_PDOassignt,
            PDOdesc: &mut *EC_PDO_DESC.as_mut_ptr().offset(0isize) as *mut ec_PDOdesct,
            eepSM: &mut EC_SM,
            eepFMMU: &mut EC_FMMU,
            FOEhook: None,
            EOEhook: None,
            manualstatechange: 0i32,
            userdata: 0 as *mut libc::c_void,
        };
        init
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
