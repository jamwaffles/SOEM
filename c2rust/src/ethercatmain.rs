use crate::{
    ethercatbase::{
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
};
use libc::{memcpy, memset, pthread_mutex_t};

pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;

pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type boolean = uint8_t;
pub type int16 = int16_t;
pub type int32 = int32_t;
pub type uint8 = uint8_t;
pub type uint16 = uint16_t;
pub type uint32 = uint32_t;
pub type int64 = int64_t;
pub type uint64 = uint64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_adapter {
    pub name: [libc::c_char; 128],
    pub desc: [libc::c_char; 128],
    pub next: *mut ec_adaptert,
}
pub type ec_adaptert = ec_adapter;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_fmmu {
    pub LogStart: uint32,
    pub LogLength: uint16,
    pub LogStartbit: uint8,
    pub LogEndbit: uint8,
    pub PhysStart: uint16,
    pub PhysStartBit: uint8,
    pub FMMUtype: uint8,
    pub FMMUactive: uint8,
    pub unused1: uint8,
    pub unused2: uint16,
}
pub type ec_fmmut = ec_fmmu;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_sm {
    pub StartAddr: uint16,
    pub SMlength: uint16,
    pub SMflags: uint32,
}
pub type ec_smt = ec_sm;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecx_context {
    pub port: *mut ecx_portt,
    pub slavelist: *mut ec_slavet,
    pub slavecount: *mut libc::c_int,
    pub maxslave: libc::c_int,
    pub grouplist: *mut ec_groupt,
    pub maxgroup: libc::c_int,
    pub esibuf: *mut uint8,
    pub esimap: *mut uint32,
    pub esislave: uint16,
    pub elist: *mut ec_eringt,
    pub idxstack: *mut ec_idxstackT,
    pub ecaterror: *mut boolean,
    pub DCtime: *mut int64,
    pub SMcommtype: *mut ec_SMcommtypet,
    pub PDOassign: *mut ec_PDOassignt,
    pub PDOdesc: *mut ec_PDOdesct,
    pub eepSM: *mut ec_eepromSMt,
    pub eepFMMU: *mut ec_eepromFMMUt,
    pub FOEhook:
        Option<unsafe extern "C" fn(_: uint16, _: libc::c_int, _: libc::c_int) -> libc::c_int>,
    pub EOEhook: Option<
        unsafe extern "C" fn(_: *mut ecx_contextt, _: uint16, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub manualstatechange: libc::c_int,
    pub userdata: *mut libc::c_void,
}
pub type ecx_contextt = ecx_context;
pub type ec_eepromFMMUt = ec_eepromFMMU;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_eepromFMMU {
    pub Startpos: uint16,
    pub nFMMU: uint8,
    pub FMMU0: uint8,
    pub FMMU1: uint8,
    pub FMMU2: uint8,
    pub FMMU3: uint8,
}
pub type ec_eepromSMt = ec_eepromSM;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_eepromSM {
    pub Startpos: uint16,
    pub nSM: uint8,
    pub PhStart: uint16,
    pub Plength: uint16,
    pub Creg: uint8,
    pub Sreg: uint8,
    pub Activate: uint8,
    pub PDIctrl: uint8,
}

pub type ec_PDOdesct = ec_PDOdesc;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_PDOdesc {
    pub n: uint8,
    pub nu1: uint8,
    pub PDO: [uint32; 256],
}

pub type ec_PDOassignt = ec_PDOassign;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_PDOassign {
    pub n: uint8,
    pub nu1: uint8,
    pub index: [uint16; 256],
}

pub type ec_SMcommtypet = ec_SMcommtype;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_SMcommtype {
    pub n: uint8,
    pub nu1: uint8,
    pub SMtype: [uint8; 8],
}

pub type ec_idxstackT = ec_idxstack;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_idxstack {
    pub pushed: uint8,
    pub pulled: uint8,
    pub idx: [uint8; 16],
    pub data: [*mut libc::c_void; 16],
    pub length: [uint16; 16],
    pub dcoffset: [uint16; 16],
}

pub type ec_eringt = ec_ering;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_ering {
    pub head: int16,
    pub tail: int16,
    pub Error: [ec_errort; 65],
}

pub type ec_groupt = ec_group;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_group {
    pub logstartaddr: uint32,
    pub Obytes: uint32,
    pub outputs: *mut uint8,
    pub Ibytes: uint32,
    pub inputs: *mut uint8,
    pub hasdc: boolean,
    pub DCnext: uint16,
    pub Ebuscurrent: int16,
    pub blockLRW: uint8,
    pub nsegments: uint16,
    pub Isegment: uint16,
    pub Ioffset: uint16,
    pub outputsWKC: uint16,
    pub inputsWKC: uint16,
    pub docheckstate: boolean,
    pub IOsegment: [uint32; 64],
}

pub type ec_slavet = ec_slave;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_slave {
    pub state: uint16,
    pub ALstatuscode: uint16,
    pub configadr: uint16,
    pub aliasadr: uint16,
    pub eep_man: uint32,
    pub eep_id: uint32,
    pub eep_rev: uint32,
    pub Itype: uint16,
    pub Dtype: uint16,
    pub Obits: uint16,
    pub Obytes: uint32,
    pub outputs: *mut uint8,
    pub Ostartbit: uint8,
    pub Ibits: uint16,
    pub Ibytes: uint32,
    pub inputs: *mut uint8,
    pub Istartbit: uint8,
    pub SM: [ec_smt; 8],
    pub SMtype: [uint8; 8],
    pub FMMU: [ec_fmmut; 4],
    pub FMMU0func: uint8,
    pub FMMU1func: uint8,
    pub FMMU2func: uint8,
    pub FMMU3func: uint8,
    pub mbx_l: uint16,
    pub mbx_wo: uint16,
    pub mbx_rl: uint16,
    pub mbx_ro: uint16,
    pub mbx_proto: uint16,
    pub mbx_cnt: uint8,
    pub hasdc: boolean,
    pub ptype: uint8,
    pub topology: uint8,
    pub activeports: uint8,
    pub consumedports: uint8,
    pub parent: uint16,
    pub parentport: uint8,
    pub entryport: uint8,
    pub DCrtA: int32,
    pub DCrtB: int32,
    pub DCrtC: int32,
    pub DCrtD: int32,
    pub pdelay: int32,
    pub DCnext: uint16,
    pub DCprevious: uint16,
    pub DCcycle: int32,
    pub DCshift: int32,
    pub DCactive: uint8,
    pub configindex: uint16,
    pub SIIindex: uint16,
    pub eep_8byte: uint8,
    pub eep_pdi: uint8,
    pub CoEdetails: uint8,
    pub FoEdetails: uint8,
    pub EoEdetails: uint8,
    pub SoEdetails: uint8,
    pub Ebuscurrent: int16,
    pub blockLRW: uint8,
    pub group: uint8,
    pub FMMUunused: uint8,
    pub islost: boolean,
    pub PO2SOconfig: Option<unsafe extern "C" fn(_: uint16) -> libc::c_int>,
    pub PO2SOconfigx: Option<unsafe extern "C" fn(_: *mut ecx_contextt, _: uint16) -> libc::c_int>,
    pub name: [libc::c_char; 41],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_eepromPDO {
    pub Startpos: uint16,
    pub Length: uint16,
    pub nPDO: uint16,
    pub Index: [uint16; 512],
    pub SyncM: [uint16; 512],
    pub BitSize: [uint16; 512],
    pub SMbitsize: [uint16; 8],
}
pub type ec_eepromPDOt = ec_eepromPDO;
pub type ec_mbxbuft = [uint8; 1487];

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_mbxheader {
    pub length: uint16,
    pub address: uint16,
    pub priority: uint8,
    pub mbxtype: uint8,
}
pub type ec_mbxheadert = ec_mbxheader;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_alstatus {
    pub alstatus: uint16,
    pub unused: uint16,
    pub alstatuscode: uint16,
}
pub type ec_alstatust = ec_alstatus;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_eepromt {
    pub comm: uint16,
    pub addr: uint16,
    pub d2: uint16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_EOEt {
    pub mbxheader: ec_mbxheadert,
    pub frameinfo1: uint16_t,
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub data: [uint8; 1476],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_8 {
    pub frameinfo2: uint16_t,
    pub result: uint16_t,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_emcyt {
    pub MbxHeader: ec_mbxheadert,
    pub CANOpen: uint16,
    pub ErrorCode: uint16,
    pub ErrorReg: uint8,
    pub bData: uint8,
    pub w1: uint16,
    pub w2: uint16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_mbxerrort {
    pub MbxHeader: ec_mbxheadert,
    pub Type: uint16,
    pub Detail: uint16,
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
    outputs: 0 as *mut uint8,
    Ostartbit: 0,
    Ibits: 0,
    Ibytes: 0,
    inputs: 0 as *mut uint8,
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
    hasdc: 0,
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
    DCactive: 0,
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
    islost: 0,
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
    outputs: 0 as *mut uint8,
    Ibytes: 0,
    inputs: 0 as *mut uint8,
    hasdc: 0,
    DCnext: 0,
    Ebuscurrent: 0,
    blockLRW: 0,
    nsegments: 0,
    Isegment: 0,
    Ioffset: 0,
    outputsWKC: 0,
    inputsWKC: 0,
    docheckstate: 0,
    IOsegment: [0; 64],
}; 2];
/* * cache for EEPROM read functions */
static mut ec_esibuf: [uint8; 4096] = [0; 4096];
/* * bitmap for filled cache buffer bytes */
static mut ec_esimap: [uint32; 128] = [0; 128];
/* * current slave for EEPROM cache buffer */
static mut ec_elist: ec_eringt = ec_eringt {
    head: 0,
    tail: 0,
    Error: [ec_errort {
        Time: ec_timet { sec: 0, usec: 0 },
        Signal: 0,
        Slave: 0,
        Index: 0,
        SubIdx: 0,
        Etype: EC_ERR_TYPE_SDO_ERROR,
        c2rust_unnamed: C2RustUnnamed_6 { AbortCode: 0 },
    }; 65],
};
static mut ec_idxstack: ec_idxstackT = ec_idxstackT {
    pushed: 0,
    pulled: 0,
    idx: [0; 16],
    data: [0 as *mut libc::c_void; 16],
    length: [0; 16],
    dcoffset: [0; 16],
};
/* * SyncManager Communication Type struct to store data of one slave */
static mut ec_SMcommtype: [ec_SMcommtypet; 1] = [ec_SMcommtypet {
    n: 0,
    nu1: 0,
    SMtype: [0; 8],
}; 1];
/* * PDO assign struct to store data of one slave */
static mut ec_PDOassign: [ec_PDOassignt; 1] = [ec_PDOassignt {
    n: 0,
    nu1: 0,
    index: [0; 256],
}; 1];
/* * PDO description struct to store data of one slave */
static mut ec_PDOdesc: [ec_PDOdesct; 1] = [ec_PDOdesct {
    n: 0,
    nu1: 0,
    PDO: [0; 256],
}; 1];
/* * buffer for EEPROM SM data */
static mut ec_SM: ec_eepromSMt = ec_eepromSMt {
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
static mut ec_FMMU: ec_eepromFMMUt = ec_eepromFMMUt {
    Startpos: 0,
    nFMMU: 0,
    FMMU0: 0,
    FMMU1: 0,
    FMMU2: 0,
    FMMU3: 0,
};
/* * Global variable TRUE if error available in error stack */
#[no_mangle]
pub static mut EcatError: boolean = 0u8;
#[no_mangle]
pub static mut ec_DCtime: int64 = 0;
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
    redstate: 0,
    redport: 0 as *mut ecx_redportt,
    getindex_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: 0 as *mut __pthread_internal_list,
                __next: 0 as *mut __pthread_internal_list,
            },
        },
    },
    tx_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: 0 as *mut __pthread_internal_list,
                __next: 0 as *mut __pthread_internal_list,
            },
        },
    },
    rx_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: 0 as *mut __pthread_internal_list,
                __next: 0 as *mut __pthread_internal_list,
            },
        },
    },
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
    slavelist: 0 as *mut ec_slavet,
    slavecount: 0 as *mut libc::c_int,
    maxslave: 0,
    grouplist: 0 as *mut ec_groupt,
    maxgroup: 0,
    esibuf: 0 as *mut uint8,
    esimap: 0 as *mut uint32,
    esislave: 0,
    elist: 0 as *mut ec_eringt,
    idxstack: 0 as *mut ec_idxstackT,
    ecaterror: 0 as *mut boolean,
    DCtime: 0 as *mut int64,
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
pub unsafe extern "C" fn ec_find_adapters() -> *mut ec_adaptert {
    let mut ret_adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
    ret_adapter = oshw_find_adapters();
    return ret_adapter;
}
/* * Free dynamically allocated list over available network adapters.
 *
 * @param[in] adapter = Struct holding adapter name, description and pointer to next.
 */
#[no_mangle]
pub unsafe extern "C" fn ec_free_adapters(mut adapter: *mut ec_adaptert) {
    oshw_free_adapters(adapter);
}
/* * Pushes an error on the error list.
 *
 * @param[in] context        = context struct
 * @param[in] Ec pointer describing the error.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_pusherror(mut context: *mut ecx_contextt, mut Ec: *const ec_errort) {
    (*(*context).elist).Error[(*(*context).elist).head as usize] = *Ec;
    (*(*context).elist).Error[(*(*context).elist).head as usize].Signal = 1u8;
    (*(*context).elist).head += 1;
    if (*(*context).elist).head as libc::c_int > 64i32 {
        (*(*context).elist).head = 0i16
    }
    if (*(*context).elist).head as libc::c_int == (*(*context).elist).tail as libc::c_int {
        (*(*context).elist).tail += 1
    }
    if (*(*context).elist).tail as libc::c_int > 64i32 {
        (*(*context).elist).tail = 0i16
    }
    *(*context).ecaterror = 1u8;
}
/* * Pops an error from the list.
 *
 * @param[in] context        = context struct
 * @param[out] Ec = Struct describing the error.
 * @return TRUE if an error was popped.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_poperror(
    mut context: *mut ecx_contextt,
    mut Ec: *mut ec_errort,
) -> boolean {
    let mut notEmpty: boolean = ((*(*context).elist).head as libc::c_int
        != (*(*context).elist).tail as libc::c_int) as boolean;
    *Ec = (*(*context).elist).Error[(*(*context).elist).tail as usize];
    (*(*context).elist).Error[(*(*context).elist).tail as usize].Signal = 0u8;
    if notEmpty != 0 {
        (*(*context).elist).tail += 1;
        if (*(*context).elist).tail as libc::c_int > 64i32 {
            (*(*context).elist).tail = 0i16
        }
    } else {
        *(*context).ecaterror = 0u8
    }
    return notEmpty;
}
/* * Check if error list has entries.
 *
 * @param[in] context        = context struct
 * @return TRUE if error list contains entries.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_iserror(mut context: *mut ecx_contextt) -> boolean {
    return ((*(*context).elist).head as libc::c_int != (*(*context).elist).tail as libc::c_int)
        as boolean;
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
pub unsafe extern "C" fn ecx_packeterror(
    mut context: *mut ecx_contextt,
    mut Slave: uint16,
    mut Index: uint16,
    mut SubIdx: uint8,
    mut ErrorCode: uint16,
) {
    let mut Ec: ec_errort = ec_errort {
        Time: ec_timet { sec: 0, usec: 0 },
        Signal: 0,
        Slave: 0,
        Index: 0,
        SubIdx: 0,
        Etype: EC_ERR_TYPE_SDO_ERROR,
        c2rust_unnamed: C2RustUnnamed_6 { AbortCode: 0 },
    };
    memset(
        &mut Ec as *mut ec_errort as *mut libc::c_void,
        0i32,
        core::mem::size_of::<ec_errort>(),
    );
    Ec.Time = osal_current_time();
    Ec.Slave = Slave;
    Ec.Index = Index;
    Ec.SubIdx = SubIdx;
    *(*context).ecaterror = 1u8;
    Ec.Etype = EC_ERR_TYPE_PACKET_ERROR;
    Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode = ErrorCode;
    ecx_pusherror(context, &mut Ec);
}
/* * Report Mailbox Error
 *
 * @param[in]  context        = context struct
 * @param[in]  Slave        = Slave number
 * @param[in]  Detail       = Following EtherCAT specification
 */
unsafe extern "C" fn ecx_mbxerror(
    mut context: *mut ecx_contextt,
    mut Slave: uint16,
    mut Detail: uint16,
) {
    let mut Ec: ec_errort = ec_errort {
        Time: ec_timet { sec: 0, usec: 0 },
        Signal: 0,
        Slave: 0,
        Index: 0,
        SubIdx: 0,
        Etype: EC_ERR_TYPE_SDO_ERROR,
        c2rust_unnamed: C2RustUnnamed_6 { AbortCode: 0 },
    };
    memset(
        &mut Ec as *mut ec_errort as *mut libc::c_void,
        0i32,
        ::core::mem::size_of::<ec_errort>(),
    );
    Ec.Time = osal_current_time();
    Ec.Slave = Slave;
    Ec.Index = 0u16;
    Ec.SubIdx = 0u8;
    Ec.Etype = EC_ERR_TYPE_MBX_ERROR;
    Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode = Detail;
    ecx_pusherror(context, &mut Ec);
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
unsafe extern "C" fn ecx_mbxemergencyerror(
    mut context: *mut ecx_contextt,
    mut Slave: uint16,
    mut ErrorCode: uint16,
    mut ErrorReg: uint16,
    mut b1: uint8,
    mut w1: uint16,
    mut w2: uint16,
) {
    let mut Ec: ec_errort = ec_errort {
        Time: ec_timet { sec: 0, usec: 0 },
        Signal: 0,
        Slave: 0,
        Index: 0,
        SubIdx: 0,
        Etype: EC_ERR_TYPE_SDO_ERROR,
        c2rust_unnamed: C2RustUnnamed_6 { AbortCode: 0 },
    };
    memset(
        &mut Ec as *mut ec_errort as *mut libc::c_void,
        0i32,
        core::mem::size_of::<ec_errort>(),
    );
    Ec.Time = osal_current_time();
    Ec.Slave = Slave;
    Ec.Index = 0u16;
    Ec.SubIdx = 0u8;
    Ec.Etype = EC_ERR_TYPE_EMERGENCY;
    Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode = ErrorCode;
    Ec.c2rust_unnamed.c2rust_unnamed.ErrorReg = ErrorReg as uint8;
    Ec.c2rust_unnamed.c2rust_unnamed.b1 = b1;
    Ec.c2rust_unnamed.c2rust_unnamed.w1 = w1;
    Ec.c2rust_unnamed.c2rust_unnamed.w2 = w2;
    ecx_pusherror(context, &mut Ec);
}
/* * Initialise lib in single NIC mode
 * @param[in]  context = context struct
 * @param[in] ifname   = Dev name, f.e. "eth0"
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_init(
    mut context: *mut ecx_contextt,
    mut ifname: *const libc::c_char,
) -> libc::c_int {
    return ecx_setupnic((*context).port, ifname, 0i32);
}
/* * Initialise lib in redundant NIC mode
 * @param[in]  context  = context struct
 * @param[in]  redport  = pointer to redport, redundant port data
 * @param[in]  ifname   = Primary Dev name, f.e. "eth0"
 * @param[in]  if2name  = Secondary Dev name, f.e. "eth1"
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_init_redundant(
    mut context: *mut ecx_contextt,
    mut redport: *mut ecx_redportt,
    mut ifname: *const libc::c_char,
    mut if2name: *mut libc::c_char,
) -> libc::c_int {
    let mut rval: libc::c_int = 0;
    let mut zbuf: libc::c_int = 0;
    let mut ehp: *mut ec_etherheadert = 0 as *mut ec_etherheadert;
    (*(*context).port).redport = redport;
    ecx_setupnic((*context).port, ifname, 0i32);
    rval = ecx_setupnic((*context).port, if2name, 1i32);
    /* prepare "dummy" BRD tx frame for redundant operation */
    ehp = &mut (*(*context).port).txbuf2 as *mut ec_bufT as *mut ec_etherheadert;
    (*ehp).sa1 = oshw_htons(secMAC[0usize]);
    zbuf = 0i32;
    ecx_setupdatagram(
        (*context).port,
        &mut (*(*context).port).txbuf2 as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_BRD as uint8,
        0u8,
        0u16,
        0u16,
        2u16,
        &mut zbuf as *mut libc::c_int as *mut libc::c_void,
    );
    (*(*context).port).txbuflength2 = core::mem::size_of::<ec_etherheadert>()
        .wrapping_add(core::mem::size_of::<ec_comt>())
        .wrapping_add(core::mem::size_of::<uint16>())
        .wrapping_add(2usize) as libc::c_int;
    return rval;
}
/* * Close lib.
 * @param[in]  context        = context struct
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_close(mut context: *mut ecx_contextt) {
    ecx_closenic((*context).port);
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
pub unsafe extern "C" fn ecx_siigetbyte(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut address: uint16,
) -> uint8 {
    let mut configadr: uint16 = 0;
    let mut eadr: uint16 = 0;
    let mut edat64: uint64 = 0;
    let mut edat32: uint32 = 0;
    let mut mapw: uint16 = 0;
    let mut mapb: uint16 = 0;
    let mut lp: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut retval: uint8 = 0;
    retval = 0xffu8;
    if slave as libc::c_int != (*context).esislave as libc::c_int {
        /* not the same slave? */
        memset(
            (*context).esimap as *mut libc::c_void,
            0i32,
            (128usize).wrapping_mul(core::mem::size_of::<uint32>()),
        ); /* clear esibuf cache map */
        (*context).esislave = slave
    }
    if (address as libc::c_int) < (128i32) << 5i32 {
        mapw = (address as libc::c_int >> 5i32) as uint16;
        mapb = (address as libc::c_int - ((mapw as libc::c_int) << 5i32)) as uint16;
        if *(*context).esimap.offset(mapw as isize) & (1u32) << mapb as libc::c_int != 0 {
            /* byte is already in buffer */
            retval = *(*context).esibuf.offset(address as isize)
        } else {
            /* byte is not in buffer, put it there */
            configadr = (*(*context).slavelist.offset(slave as isize)).configadr; /* set eeprom control to master */
            ecx_eeprom2master(context, slave);
            eadr = (address as libc::c_int >> 1i32) as uint16;
            edat64 = ecx_readeepromFP(context, configadr, eadr, 20000i32);
            /* 8 byte response */
            if (*(*context).slavelist.offset(slave as isize)).eep_8byte != 0 {
                memcpy(
                    &mut *(*context)
                        .esibuf
                        .offset(((eadr as libc::c_int) << 1i32) as isize)
                        as *mut uint8 as *mut libc::c_void,
                    &mut edat64 as *mut uint64 as *const libc::c_void,
                    8usize,
                );
                cnt = 8i32
            } else {
                /* 4 byte response */
                edat32 = edat64 as uint32;
                memcpy(
                    &mut *(*context)
                        .esibuf
                        .offset(((eadr as libc::c_int) << 1i32) as isize)
                        as *mut uint8 as *mut libc::c_void,
                    &mut edat32 as *mut uint32 as *const libc::c_void,
                    4usize,
                );
                cnt = 4i32
            }
            /* find bitmap location */
            mapw = (eadr as libc::c_int >> 4i32) as uint16;
            mapb = (((eadr as libc::c_int) << 1i32) - ((mapw as libc::c_int) << 5i32)) as uint16;
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
pub unsafe extern "C" fn ecx_siifind(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut cat: uint16,
) -> int16 {
    let mut a: int16 = 0;
    let mut p: uint16 = 0;
    let mut eectl: uint8 = (*(*context).slavelist.offset(slave as isize)).eep_pdi;
    a = ((0x40i32) << 1i32) as int16;
    /* read first SII section category */
    let fresh1 = a;
    a = a + 1;
    p = ecx_siigetbyte(context, slave, fresh1 as uint16) as uint16;
    let fresh2 = a;
    a = a + 1;
    p = (p as libc::c_int
        + ((ecx_siigetbyte(context, slave, fresh2 as uint16) as libc::c_int) << 8i32))
        as uint16;
    /* traverse SII while category is not found and not EOF */
    while p as libc::c_int != cat as libc::c_int && p as libc::c_int != 0xffffi32 {
        /* read section length */
        let fresh3 = a;
        a = a + 1;
        p = ecx_siigetbyte(context, slave, fresh3 as uint16) as uint16;
        let fresh4 = a;
        a = a + 1;
        p = (p as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh4 as uint16) as libc::c_int) << 8i32))
            as uint16;
        /* locate next section category */
        a = (a as libc::c_int + ((p as libc::c_int) << 1i32)) as int16;
        /* read section category */
        let fresh5 = a;
        a = a + 1;
        p = ecx_siigetbyte(context, slave, fresh5 as uint16) as uint16;
        let fresh6 = a;
        a = a + 1;
        p = (p as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh6 as uint16) as libc::c_int) << 8i32))
            as uint16
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
pub unsafe extern "C" fn ecx_siistring(
    mut context: *mut ecx_contextt,
    mut str: *mut libc::c_char,
    mut slave: uint16,
    mut Sn: uint16,
) {
    let mut a: uint16 = 0; /* find string section */
    let mut i: uint16 = 0; /* skip SII section header */
    let mut j: uint16 = 0; /* read number of strings in section */
    let mut l: uint16 = 0;
    let mut n: uint16 = 0;
    let mut ba: uint16 = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eectl: uint8 = (*(*context).slavelist.offset(slave as isize)).eep_pdi;
    ptr = str;
    a = ecx_siifind(context, slave, ECT_SII_STRING as uint16) as uint16;
    if a as libc::c_int > 0i32 {
        ba = (a as libc::c_int + 2i32) as uint16;
        let fresh7 = ba;
        ba = ba.wrapping_add(1);
        n = ecx_siigetbyte(context, slave, fresh7) as uint16;
        if Sn as libc::c_int <= n as libc::c_int {
            /* is req string available? */
            i = 1u16;
            while i as libc::c_int <= Sn as libc::c_int {
                /* walk through strings */
                let fresh8 = ba; /* length of this string */
                ba = ba.wrapping_add(1);
                l = ecx_siigetbyte(context, slave, fresh8) as uint16;
                if (i as libc::c_int) < Sn as libc::c_int {
                    ba = (ba as libc::c_int + l as libc::c_int) as uint16
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
            *ptr = 0i8
        /* add zero terminator */
        } else {
            ptr = str;
            *ptr = 0i8
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
pub unsafe extern "C" fn ecx_siiFMMU(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut FMMU: *mut ec_eepromFMMUt,
) -> uint16 {
    let mut a: uint16 = 0;
    let mut eectl: uint8 = (*(*context).slavelist.offset(slave as isize)).eep_pdi;
    (*FMMU).nFMMU = 0u8;
    (*FMMU).FMMU0 = 0u8;
    (*FMMU).FMMU1 = 0u8;
    (*FMMU).FMMU2 = 0u8;
    (*FMMU).FMMU3 = 0u8;
    (*FMMU).Startpos = ecx_siifind(context, slave, ECT_SII_FMMU as uint16) as uint16;
    if (*FMMU).Startpos as libc::c_int > 0i32 {
        a = (*FMMU).Startpos;
        let fresh10 = a;
        a = a.wrapping_add(1);
        (*FMMU).nFMMU = ecx_siigetbyte(context, slave, fresh10);
        let fresh11 = a;
        a = a.wrapping_add(1);
        (*FMMU).nFMMU = ((*FMMU).nFMMU as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh11) as libc::c_int) << 8i32))
            as uint8;
        (*FMMU).nFMMU = ((*FMMU).nFMMU as libc::c_int * 2i32) as uint8;
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
    return (*FMMU).nFMMU as uint16;
}
/* * Get SM data from SII SM section in slave EEPROM.
 *  @param[in]  context = context struct
 *  @param[in]  slave   = slave number
 *  @param[out] SM      = first SM struct from SII
 *  @return number of SM's defined in section
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_siiSM(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut SM: *mut ec_eepromSMt,
) -> uint16 {
    let mut a: uint16 = 0;
    let mut w: uint16 = 0;
    let mut eectl: uint8 = (*(*context).slavelist.offset(slave as isize)).eep_pdi;
    (*SM).nSM = 0u8;
    (*SM).Startpos = ecx_siifind(context, slave, ECT_SII_SM as uint16) as uint16;
    if (*SM).Startpos as libc::c_int > 0i32 {
        a = (*SM).Startpos;
        let fresh16 = a;
        a = a.wrapping_add(1);
        w = ecx_siigetbyte(context, slave, fresh16) as uint16;
        let fresh17 = a;
        a = a.wrapping_add(1);
        w = (w as libc::c_int + ((ecx_siigetbyte(context, slave, fresh17) as libc::c_int) << 8i32))
            as uint16;
        (*SM).nSM = (w as libc::c_int / 4i32) as uint8;
        let fresh18 = a;
        a = a.wrapping_add(1);
        (*SM).PhStart = ecx_siigetbyte(context, slave, fresh18) as uint16;
        let fresh19 = a;
        a = a.wrapping_add(1);
        (*SM).PhStart = ((*SM).PhStart as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh19) as libc::c_int) << 8i32))
            as uint16;
        let fresh20 = a;
        a = a.wrapping_add(1);
        (*SM).Plength = ecx_siigetbyte(context, slave, fresh20) as uint16;
        let fresh21 = a;
        a = a.wrapping_add(1);
        (*SM).Plength = ((*SM).Plength as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh21) as libc::c_int) << 8i32))
            as uint16;
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
    return (*SM).nSM as uint16;
}
/* * Get next SM data from SII SM section in slave EEPROM.
 *  @param[in]  context = context struct
 *  @param[in]  slave   = slave number
 *  @param[out] SM      = first SM struct from SII
 *  @param[in]  n       = SM number
 *  @return >0 if OK
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_siiSMnext(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut SM: *mut ec_eepromSMt,
    mut n: uint16,
) -> uint16 {
    let mut a: uint16 = 0;
    let mut retVal: uint16 = 0u16;
    let mut eectl: uint8 = (*(*context).slavelist.offset(slave as isize)).eep_pdi;
    if (n as libc::c_int) < (*SM).nSM as libc::c_int {
        a = ((*SM).Startpos as libc::c_int + 2i32 + n as libc::c_int * 8i32) as uint16;
        let fresh26 = a;
        a = a.wrapping_add(1);
        (*SM).PhStart = ecx_siigetbyte(context, slave, fresh26) as uint16;
        let fresh27 = a;
        a = a.wrapping_add(1);
        (*SM).PhStart = ((*SM).PhStart as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh27) as libc::c_int) << 8i32))
            as uint16;
        let fresh28 = a;
        a = a.wrapping_add(1);
        (*SM).Plength = ecx_siigetbyte(context, slave, fresh28) as uint16;
        let fresh29 = a;
        a = a.wrapping_add(1);
        (*SM).Plength = ((*SM).Plength as libc::c_int
            + ((ecx_siigetbyte(context, slave, fresh29) as libc::c_int) << 8i32))
            as uint16;
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
pub unsafe extern "C" fn ecx_siiPDO(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut PDO: *mut ec_eepromPDOt,
    mut t: uint8,
) -> uint32 {
    let mut a: uint16 = 0;
    let mut w: uint16 = 0;
    let mut c: uint16 = 0;
    let mut e: uint16 = 0;
    let mut er: uint16 = 0;
    let mut Size: uint16 = 0;
    let mut eectl: uint8 = (*(*context).slavelist.offset(slave as isize)).eep_pdi;
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
        (ECT_SII_PDO as libc::c_int + t as libc::c_int) as uint16,
    ) as uint16;
    if (*PDO).Startpos as libc::c_int > 0i32 {
        a = (*PDO).Startpos;
        let fresh34 = a;
        a = a.wrapping_add(1);
        w = ecx_siigetbyte(context, slave, fresh34) as uint16;
        let fresh35 = a;
        a = a.wrapping_add(1);
        w = (w as libc::c_int + ((ecx_siigetbyte(context, slave, fresh35) as libc::c_int) << 8i32))
            as uint16;
        (*PDO).Length = w;
        c = 1u16;
        loop
        /* traverse through all PDOs */
        {
            (*PDO).nPDO = (*PDO).nPDO.wrapping_add(1);
            let fresh36 = a;
            a = a.wrapping_add(1);
            (*PDO).Index[(*PDO).nPDO as usize] = ecx_siigetbyte(context, slave, fresh36) as uint16;
            let fresh37 = a;
            a = a.wrapping_add(1);
            (*PDO).Index[(*PDO).nPDO as usize] = ((*PDO).Index[(*PDO).nPDO as usize] as libc::c_int
                + ((ecx_siigetbyte(context, slave, fresh37) as libc::c_int) << 8i32))
                as uint16;
            (*PDO).BitSize[(*PDO).nPDO as usize] = 0u16;
            c = c.wrapping_add(1);
            let fresh38 = a;
            a = a.wrapping_add(1);
            e = ecx_siigetbyte(context, slave, fresh38) as uint16;
            let fresh39 = a;
            a = a.wrapping_add(1);
            (*PDO).SyncM[(*PDO).nPDO as usize] = ecx_siigetbyte(context, slave, fresh39) as uint16;
            a = (a as libc::c_int + 4i32) as uint16;
            c = (c as libc::c_int + 2i32) as uint16;
            if ((*PDO).SyncM[(*PDO).nPDO as usize] as libc::c_int) < 8i32 {
                /* active and in range SM? */
                /* read all entries defined in PDO */
                er = 1u16;
                while er as libc::c_int <= e as libc::c_int {
                    c = (c as libc::c_int + 4i32) as uint16;
                    a = (a as libc::c_int + 5i32) as uint16;
                    let fresh40 = a;
                    a = a.wrapping_add(1);
                    (*PDO).BitSize[(*PDO).nPDO as usize] = ((*PDO).BitSize[(*PDO).nPDO as usize]
                        as libc::c_int
                        + ecx_siigetbyte(context, slave, fresh40) as libc::c_int)
                        as uint16;
                    a = (a as libc::c_int + 2i32) as uint16;
                    er = er.wrapping_add(1)
                }
                (*PDO).SMbitsize[(*PDO).SyncM[(*PDO).nPDO as usize] as usize] =
                    ((*PDO).SMbitsize[(*PDO).SyncM[(*PDO).nPDO as usize] as usize] as libc::c_int
                        + (*PDO).BitSize[(*PDO).nPDO as usize] as libc::c_int)
                        as uint16;
                Size = (Size as libc::c_int + (*PDO).BitSize[(*PDO).nPDO as usize] as libc::c_int)
                    as uint16;
                c = c.wrapping_add(1)
            } else {
                /* PDO deactivated because SM is 0xff or > EC_MAXSM */
                c = (c as libc::c_int + 4i32 * e as libc::c_int) as uint16;
                a = (a as libc::c_int + 8i32 * e as libc::c_int) as uint16;
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
    return Size as uint32;
}
#[no_mangle]
pub unsafe extern "C" fn ecx_FPRD_multi(
    mut context: *mut ecx_contextt,
    mut n: libc::c_int,
    mut configlst: *mut uint16,
    mut slstatlst: *mut ec_alstatust,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: uint8 = 0;
    let mut port: *mut ecx_portt = 0 as *mut ecx_portt;
    let mut sldatapos: [uint16; 64] = [0; 64];
    let mut slcnt: libc::c_int = 0;
    port = (*context).port;
    idx = ecx_clearindex(port);
    slcnt = 0i32;
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_FPRD as uint8,
        idx,
        *configlst.offset(slcnt as isize),
        ECT_REG_ALSTAT as uint16,
        ::core::mem::size_of::<ec_alstatust>() as uint16,
        slstatlst.offset(slcnt as isize) as *mut libc::c_void,
    );
    sldatapos[slcnt as usize] = ::core::mem::size_of::<ec_comt>() as uint16;
    loop {
        slcnt += 1;
        if !(slcnt < n - 1i32) {
            break;
        }
        sldatapos[slcnt as usize] = ecx_adddatagram(
            port,
            &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                as *mut libc::c_void,
            ec_cmdtype::EC_CMD_FPRD as uint8,
            idx,
            1u8,
            *configlst.offset(slcnt as isize),
            ECT_REG_ALSTAT as uint16,
            ::core::mem::size_of::<ec_alstatust>() as uint16,
            slstatlst.offset(slcnt as isize) as *mut libc::c_void,
        )
    }
    if slcnt < n {
        sldatapos[slcnt as usize] = ecx_adddatagram(
            port,
            &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                as *mut libc::c_void,
            ec_cmdtype::EC_CMD_FPRD as uint8,
            idx,
            0u8,
            *configlst.offset(slcnt as isize),
            ECT_REG_ALSTAT as uint16,
            ::core::mem::size_of::<ec_alstatust>() as uint16,
            slstatlst.offset(slcnt as isize) as *mut libc::c_void,
        )
    }
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc >= 0i32 {
        slcnt = 0i32;
        while slcnt < n {
            memcpy(
                slstatlst.offset(slcnt as isize) as *mut libc::c_void,
                &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                    .as_mut_ptr()
                    .offset(*sldatapos.as_mut_ptr().offset(slcnt as isize) as isize)
                    as *mut uint8 as *const libc::c_void,
                ::core::mem::size_of::<ec_alstatust>(),
            );
            slcnt += 1
        }
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * Read all slave states in ec_slave.
 * @param[in] context = context struct
 * @return lowest state found
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readstate(mut context: *mut ecx_contextt) -> libc::c_int {
    let mut slave: uint16 = 0;
    let mut fslave: uint16 = 0;
    let mut lslave: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut lowest: uint16 = 0;
    let mut rval: uint16 = 0;
    let mut bitwisestate: uint16 = 0;
    let mut sl: [ec_alstatust; 64] = [ec_alstatust {
        alstatus: 0,
        unused: 0,
        alstatuscode: 0,
    }; 64];
    let mut slca: [uint16; 64] = [0; 64];
    let mut noerrorflag: boolean = 0;
    let mut allslavessamestate: boolean = 0;
    let mut allslavespresent: boolean = 0u8;
    let mut wkc: libc::c_int = 0;
    /* Try to establish the state of all slaves sending only one broadcast datagram.
     * This way a number of datagrams equal to the number of slaves will be sent only if needed.*/
    rval = 0u16;
    wkc = ecx_BRD(
        (*context).port,
        0u16,
        ECT_REG_ALSTAT as uint16,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut rval as *mut uint16 as *mut libc::c_void,
        2000i32,
    );
    if wkc >= *(*context).slavecount {
        allslavespresent = 1u8
    }
    rval = rval;
    bitwisestate = (rval as libc::c_int & 0xfi32) as uint16;
    if rval as libc::c_int & EC_STATE_ERROR as libc::c_int == 0i32 {
        noerrorflag = 1u8;
        (*(*context).slavelist.offset(0isize)).ALstatuscode = 0u16
    } else {
        noerrorflag = 0u8
    }
    match bitwisestate as libc::c_int {
        1 | 2 | 3 | 4 | 8 => {
            allslavessamestate = 1u8;
            (*(*context).slavelist.offset(0isize)).state = bitwisestate
        }
        _ => allslavessamestate = 0u8,
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
        while slave as libc::c_int <= *(*context).slavecount {
            (*(*context).slavelist.offset(slave as isize)).ALstatuscode = 0u16;
            (*(*context).slavelist.offset(slave as isize)).state = bitwisestate;
            slave = slave.wrapping_add(1)
        }
        lowest = bitwisestate
    } else {
        /* Not all slaves have the same state or at least one is in error so one datagram per slave
         * is needed. */
        (*(*context).slavelist.offset(0isize)).ALstatuscode = 0u16;
        lowest = 0xffu16;
        fslave = 1u16;
        loop {
            lslave = *(*context).slavecount as uint16;
            if lslave as libc::c_int - fslave as libc::c_int >= 64i32 {
                lslave = (fslave as libc::c_int + 64i32 - 1i32) as uint16
            }
            slave = fslave;
            while slave as libc::c_int <= lslave as libc::c_int {
                let zero: ec_alstatust = {
                    let mut init = ec_alstatus {
                        alstatus: 0u16,
                        unused: 0u16,
                        alstatuscode: 0u16,
                    };
                    init
                };
                configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
                slca[(slave as libc::c_int - fslave as libc::c_int) as usize] = configadr;
                sl[(slave as libc::c_int - fslave as libc::c_int) as usize] = zero;
                slave = slave.wrapping_add(1)
            }
            ecx_FPRD_multi(
                context,
                lslave as libc::c_int - fslave as libc::c_int + 1i32,
                &mut *slca.as_mut_ptr().offset(0isize),
                &mut *sl.as_mut_ptr().offset(0isize),
                2000i32 * 3i32,
            );
            slave = fslave;
            while slave as libc::c_int <= lslave as libc::c_int {
                configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
                rval = sl[(slave as libc::c_int - fslave as libc::c_int) as usize].alstatus;
                (*(*context).slavelist.offset(slave as isize)).ALstatuscode =
                    sl[(slave as libc::c_int - fslave as libc::c_int) as usize].alstatuscode;
                if (rval as libc::c_int & 0xfi32) < lowest as libc::c_int {
                    lowest = (rval as libc::c_int & 0xfi32) as uint16
                }
                (*(*context).slavelist.offset(slave as isize)).state = rval;
                let ref mut fresh41 = (*(*context).slavelist.offset(0isize)).ALstatuscode;
                *fresh41 = (*fresh41 as libc::c_int
                    | (*(*context).slavelist.offset(slave as isize)).ALstatuscode as libc::c_int)
                    as uint16;
                slave = slave.wrapping_add(1)
            }
            fslave = (lslave as libc::c_int + 1i32) as uint16;
            if !((lslave as libc::c_int) < *(*context).slavecount) {
                break;
            }
        }
        (*(*context).slavelist.offset(0isize)).state = lowest
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
pub unsafe extern "C" fn ecx_writestate(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut configadr: uint16 = 0;
    let mut slstate: uint16 = 0;
    if slave as libc::c_int == 0i32 {
        slstate = (*(*context).slavelist.offset(slave as isize)).state;
        ret = ecx_BWR(
            (*context).port,
            0u16,
            ECT_REG_ALCTL as uint16,
            ::core::mem::size_of::<uint16>() as uint16,
            &mut slstate as *mut uint16 as *mut libc::c_void,
            2000i32 * 3i32,
        )
    } else {
        configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
        ret = ecx_FPWRw(
            (*context).port,
            configadr,
            ECT_REG_ALCTL as uint16,
            (*(*context).slavelist.offset(slave as isize)).state,
            2000i32 * 3i32,
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
pub unsafe extern "C" fn ecx_statecheck(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut reqstate: uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    let mut configadr: uint16 = 0; /* read slave status */
    let mut state: uint16 = 0;
    let mut rval: uint16 = 0;
    let mut slstat: ec_alstatust = ec_alstatust {
        alstatus: 0,
        unused: 0,
        alstatuscode: 0,
    };
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    if slave as libc::c_int > *(*context).slavecount {
        return 0u16;
    }
    osal_timer_start(&mut timer, timeout as uint32);
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    loop {
        if (slave as libc::c_int) < 1i32 {
            rval = 0u16;
            ecx_BRD(
                (*context).port,
                0u16,
                ECT_REG_ALSTAT as uint16,
                ::core::mem::size_of::<uint16>() as uint16,
                &mut rval as *mut uint16 as *mut libc::c_void,
                2000i32,
            );
            rval = rval
        } else {
            slstat.alstatus = 0u16;
            slstat.alstatuscode = 0u16;
            ecx_FPRD(
                (*context).port,
                configadr,
                ECT_REG_ALSTAT as uint16,
                ::core::mem::size_of::<ec_alstatust>() as uint16,
                &mut slstat as *mut ec_alstatust as *mut libc::c_void,
                2000i32,
            );
            rval = slstat.alstatus;
            (*(*context).slavelist.offset(slave as isize)).ALstatuscode = slstat.alstatuscode
        }
        state = (rval as libc::c_int & 0xfi32) as uint16;
        if state as libc::c_int != reqstate as libc::c_int {
            osal_usleep(1000u32);
        }
        if !(state as libc::c_int != reqstate as libc::c_int
            && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
        {
            break;
        }
    }
    (*(*context).slavelist.offset(slave as isize)).state = rval;
    return state;
}
/* * Get index of next mailbox counter value.
 * Used for Mailbox Link Layer.
 * @param[in] cnt     = Mailbox counter value [0..7]
 * @return next mailbox counter value
 */
#[no_mangle]
pub unsafe extern "C" fn ec_nextmbxcnt(mut cnt: uint8) -> uint8 {
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
pub unsafe extern "C" fn ec_clearmbx(mut Mbx: *mut ec_mbxbuft) {
    memset(Mbx as *mut libc::c_void, 0i32, 1486usize);
}
/* * Check if IN mailbox of slave is empty.
 * @param[in] context  = context struct
 * @param[in] slave    = Slave number
 * @param[in] timeout  = Timeout in us
 * @return >0 is success
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_mbxempty(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut configadr: uint16 = 0;
    let mut SMstat: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer, timeout as uint32);
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    loop {
        SMstat = 0u8;
        wkc = ecx_FPRD(
            (*context).port,
            configadr,
            ECT_REG_SM0STAT as uint16,
            ::core::mem::size_of::<uint8>() as uint16,
            &mut SMstat as *mut uint8 as *mut libc::c_void,
            2000i32,
        );
        SMstat = SMstat;
        if SMstat as libc::c_int & 0x8i32 != 0i32 && timeout > 200i32 {
            osal_usleep(200u32);
        }
        if !((wkc <= 0i32 || SMstat as libc::c_int & 0x8i32 != 0i32)
            && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
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
pub unsafe extern "C" fn ecx_mbxsend(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut mbx: *mut ec_mbxbuft,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut mbxwo: uint16 = 0;
    let mut mbxl: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut wkc: libc::c_int = 0;
    wkc = 0i32;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    mbxl = (*(*context).slavelist.offset(slave as isize)).mbx_l;
    if mbxl as libc::c_int > 0i32 && mbxl as libc::c_int <= 1486i32 {
        if ecx_mbxempty(context, slave, timeout) != 0 {
            mbxwo = (*(*context).slavelist.offset(slave as isize)).mbx_wo;
            /* write slave in mailbox */
            wkc = ecx_FPWR(
                (*context).port,
                configadr,
                mbxwo,
                mbxl,
                mbx as *mut libc::c_void,
                2000i32 * 3i32,
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
pub unsafe extern "C" fn ecx_mbxreceive(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut mbx: *mut ec_mbxbuft,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut mbxro: uint16 = 0;
    let mut mbxl: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut wkc: libc::c_int = 0i32;
    let mut wkc2: libc::c_int = 0;
    let mut SMstat: uint16 = 0;
    let mut SMcontr: uint8 = 0;
    let mut mbxh: *mut ec_mbxheadert = 0 as *mut ec_mbxheadert;
    let mut EMp: *mut ec_emcyt = 0 as *mut ec_emcyt;
    let mut MBXEp: *mut ec_mbxerrort = 0 as *mut ec_mbxerrort;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    mbxl = (*(*context).slavelist.offset(slave as isize)).mbx_rl;
    if mbxl as libc::c_int > 0i32 && mbxl as libc::c_int <= 1486i32 {
        let mut timer: osal_timert = osal_timert {
            stop_time: ec_timet { sec: 0, usec: 0 },
        };
        osal_timer_start(&mut timer, timeout as uint32);
        wkc = 0i32;
        loop {
            /* wait for read mailbox available */
            SMstat = 0u16;
            wkc = ecx_FPRD(
                (*context).port,
                configadr,
                ECT_REG_SM1STAT as uint16,
                ::core::mem::size_of::<uint16>() as uint16,
                &mut SMstat as *mut uint16 as *mut libc::c_void,
                2000i32,
            );
            SMstat = SMstat;
            if SMstat as libc::c_int & 0x8i32 == 0i32 && timeout > 200i32 {
                osal_usleep(200u32);
            }
            if !((wkc <= 0i32 || SMstat as libc::c_int & 0x8i32 == 0i32)
                && osal_timer_is_expired(&mut timer) as libc::c_int == 0i32)
            {
                break;
            }
        }
        if wkc > 0i32 && SMstat as libc::c_int & 0x8i32 > 0i32 {
            /* read mailbox available ? */
            mbxro = (*(*context).slavelist.offset(slave as isize)).mbx_ro;
            mbxh = mbx as *mut ec_mbxheadert;
            loop {
                /* if WKC<=0 repeat */
                wkc = ecx_FPRD(
                    (*context).port,
                    configadr,
                    mbxro,
                    mbxl,
                    mbx as *mut libc::c_void,
                    2000i32,
                ); /* get mailbox */
                if wkc > 0i32 && (*mbxh).mbxtype as libc::c_int & 0xfi32 == 0i32 {
                    /* Mailbox error response? */
                    MBXEp = mbx as *mut ec_mbxerrort;
                    ecx_mbxerror(context, slave, (*MBXEp).Detail);
                    wkc = 0i32
                /* prevent emergency to cascade up, it is already handled. */
                } else if wkc > 0i32
                    && (*mbxh).mbxtype as libc::c_int & 0xfi32 == ECT_MBXT_COE as libc::c_int
                {
                    /* CoE response? */
                    EMp = mbx as *mut ec_emcyt;
                    if (*EMp).CANOpen as libc::c_int >> 12i32 == 0x1i32 {
                        /* Emergency request? */
                        ecx_mbxemergencyerror(
                            context,
                            slave,
                            (*EMp).ErrorCode,
                            (*EMp).ErrorReg as uint16,
                            (*EMp).bData,
                            (*EMp).w1,
                            (*EMp).w2,
                        );
                        wkc = 0i32
                        /* prevent emergency to cascade up, it is already handled. */
                    }
                } else if wkc > 0i32
                    && (*mbxh).mbxtype as libc::c_int & 0xfi32 == ECT_MBXT_EOE as libc::c_int
                {
                    /* EoE response? */
                    let mut eoembx: *mut ec_EOEt = mbx as *mut ec_EOEt;
                    let mut frameinfo1: uint16 = (*eoembx).frameinfo1;
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
                    SMstat = (SMstat as libc::c_int ^ 0x200i32) as uint16; /* toggle repeat request */
                    SMstat = SMstat;
                    wkc2 = ecx_FPWR(
                        (*context).port,
                        configadr,
                        ECT_REG_SM1STAT as uint16,
                        ::core::mem::size_of::<uint16>() as uint16,
                        &mut SMstat as *mut uint16 as *mut libc::c_void,
                        2000i32,
                    );
                    SMstat = SMstat;
                    loop {
                        /* wait for toggle ack */
                        wkc2 = ecx_FPRD(
                            (*context).port,
                            configadr,
                            ECT_REG_SM1CONTR as uint16,
                            ::core::mem::size_of::<uint8>() as uint16,
                            &mut SMcontr as *mut uint8 as *mut libc::c_void,
                            2000i32,
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
                            (*context).port,
                            configadr,
                            ECT_REG_SM1STAT as uint16,
                            ::core::mem::size_of::<uint16>() as uint16,
                            &mut SMstat as *mut uint16 as *mut libc::c_void,
                            2000i32,
                        );
                        SMstat = SMstat;
                        if SMstat as libc::c_int & 0x8i32 == 0i32 && timeout > 200i32 {
                            osal_usleep(200u32);
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
pub unsafe extern "C" fn ecx_esidump(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut esibuf: *mut uint8,
) {
    let mut configadr: uint16 = 0; /* set eeprom control to master */
    let mut address: uint16 = 0;
    let mut incr: uint16 = 0;
    let mut p64: *mut uint64 = 0 as *mut uint64;
    let mut p16: *mut uint16 = 0 as *mut uint16;
    let mut edat: uint64 = 0;
    let mut eectl: uint8 = (*(*context).slavelist.offset(slave as isize)).eep_pdi;
    ecx_eeprom2master(context, slave);
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    address = 0x40u16;
    p16 = esibuf as *mut uint16;
    if (*(*context).slavelist.offset(slave as isize)).eep_8byte != 0 {
        incr = 4u16
    } else {
        incr = 2u16
    }
    loop {
        edat = ecx_readeepromFP(context, configadr, address, 20000i32);
        p64 = p16 as *mut uint64;
        *p64 = edat;
        p16 = p16.offset(incr as libc::c_int as isize);
        address = (address as libc::c_int + incr as libc::c_int) as uint16;
        if !(address as libc::c_int <= (128i32) << 5i32 >> 1i32 && edat as uint32 != 0xffffffffu32)
        {
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
pub unsafe extern "C" fn ecx_readeeprom(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut eeproma: uint16,
    mut timeout: libc::c_int,
) -> uint32 {
    let mut configadr: uint16 = 0; /* set eeprom control to master */
    ecx_eeprom2master(context, slave);
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    return ecx_readeepromFP(context, configadr, eeproma, timeout) as uint32;
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
pub unsafe extern "C" fn ecx_writeeeprom(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut eeproma: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut configadr: uint16 = 0; /* set eeprom control to master */
    ecx_eeprom2master(context, slave);
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    return ecx_writeeepromFP(context, configadr, eeproma, data, timeout);
}
/* * Set eeprom control to master. Only if set to PDI.
 * @param[in] context   = context struct
 * @param[in] slave     = Slave number
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_eeprom2master(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
) -> libc::c_int {
    let mut wkc: libc::c_int = 1i32;
    let mut cnt: libc::c_int = 0i32;
    let mut configadr: uint16 = 0;
    let mut eepctl: uint8 = 0;
    if (*(*context).slavelist.offset(slave as isize)).eep_pdi != 0 {
        configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
        eepctl = 2u8;
        loop {
            wkc = ecx_FPWR(
                (*context).port,
                configadr,
                ECT_REG_EEPCFG as uint16,
                ::core::mem::size_of::<uint8>() as uint16,
                &mut eepctl as *mut uint8 as *mut libc::c_void,
                2000i32,
            );
            if !(wkc <= 0i32 && {
                let fresh42 = cnt;
                cnt = cnt + 1;
                (fresh42) < 3i32
            }) {
                break;
            }
            /* force Eeprom from PDI */
        }
        eepctl = 0u8;
        cnt = 0i32;
        loop {
            wkc = ecx_FPWR(
                (*context).port,
                configadr,
                ECT_REG_EEPCFG as uint16,
                ::core::mem::size_of::<uint8>() as uint16,
                &mut eepctl as *mut uint8 as *mut libc::c_void,
                2000i32,
            );
            if !(wkc <= 0i32 && {
                let fresh43 = cnt;
                cnt = cnt + 1;
                (fresh43) < 3i32
            }) {
                break;
            }
            /* set Eeprom to master */
        }
        (*(*context).slavelist.offset(slave as isize)).eep_pdi = 0u8
    }
    return wkc;
}
/* * Set eeprom control to PDI. Only if set to master.
 * @param[in]  context        = context struct
 * @param[in] slave     = Slave number
 * @return >0 if OK
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_eeprom2pdi(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
) -> libc::c_int {
    let mut wkc: libc::c_int = 1i32;
    let mut cnt: libc::c_int = 0i32;
    let mut configadr: uint16 = 0;
    let mut eepctl: uint8 = 0;
    if (*(*context).slavelist.offset(slave as isize)).eep_pdi == 0 {
        configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
        eepctl = 1u8;
        loop {
            wkc = ecx_FPWR(
                (*context).port,
                configadr,
                ECT_REG_EEPCFG as uint16,
                ::core::mem::size_of::<uint8>() as uint16,
                &mut eepctl as *mut uint8 as *mut libc::c_void,
                2000i32,
            );
            if !(wkc <= 0i32 && {
                let fresh44 = cnt;
                cnt = cnt + 1;
                (fresh44) < 3i32
            }) {
                break;
            }
            /* set Eeprom to PDI */
        } /* wait for eeprom ready */
        (*(*context).slavelist.offset(slave as isize)).eep_pdi = 1u8
    }
    return wkc;
}
#[no_mangle]
pub unsafe extern "C" fn ecx_eeprom_waitnotbusyAP(
    mut context: *mut ecx_contextt,
    mut aiadr: uint16,
    mut estat: *mut uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0i32;
    let mut retval: uint16 = 0u16;
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer, timeout as uint32);
    loop {
        let fresh45 = cnt;
        cnt = cnt + 1;
        if fresh45 != 0 {
            osal_usleep(200u32);
        }
        *estat = 0u16;
        wkc = ecx_APRD(
            (*context).port,
            aiadr,
            ECT_REG_EEPSTAT as uint16,
            ::core::mem::size_of::<uint16>() as uint16,
            estat as *mut libc::c_void,
            2000i32,
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
pub unsafe extern "C" fn ecx_readeepromAP(
    mut context: *mut ecx_contextt,
    mut aiadr: uint16,
    mut eeproma: uint16,
    mut timeout: libc::c_int,
) -> uint64 {
    let mut estat: uint16 = 0;
    let mut edat32: uint32 = 0;
    let mut edat64: uint64 = 0;
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
        if estat as libc::c_int & 0x7800i32 != 0 {
            /* error bits are set */
            estat = EC_ECMD_NOP as uint16; /* clear error bits */
            wkc = ecx_APWR(
                (*context).port,
                aiadr,
                ECT_REG_EEPCTL as uint16,
                ::core::mem::size_of::<uint16>() as uint16,
                &mut estat as *mut uint16 as *mut libc::c_void,
                2000i32 * 3i32,
            )
        }
        loop {
            ed.comm = EC_ECMD_READ as uint16;
            ed.addr = eeproma;
            ed.d2 = 0u16;
            cnt = 0i32;
            loop {
                wkc = ecx_APWR(
                    (*context).port,
                    aiadr,
                    ECT_REG_EEPCTL as uint16,
                    ::core::mem::size_of::<ec_eepromt>() as uint16,
                    &mut ed as *mut ec_eepromt as *mut libc::c_void,
                    2000i32,
                );
                if !(wkc <= 0i32 && {
                    let fresh46 = cnt;
                    cnt = cnt + 1;
                    (fresh46) < 3i32
                }) {
                    break;
                }
            }
            if wkc != 0 {
                osal_usleep(200u32);
                estat = 0u16;
                if ecx_eeprom_waitnotbusyAP(context, aiadr, &mut estat, timeout) != 0 {
                    if estat as libc::c_int & 0x2000i32 != 0 {
                        nackcnt += 1;
                        osal_usleep((200i32 * 5i32) as uint32);
                    } else {
                        nackcnt = 0i32;
                        if estat as libc::c_int & 0x40i32 != 0 {
                            cnt = 0i32;
                            loop {
                                wkc = ecx_APRD(
                                    (*context).port,
                                    aiadr,
                                    ECT_REG_EEPDAT as uint16,
                                    ::core::mem::size_of::<uint64>() as uint16,
                                    &mut edat64 as *mut uint64 as *mut libc::c_void,
                                    2000i32,
                                );
                                if !(wkc <= 0i32 && {
                                    let fresh47 = cnt;
                                    cnt = cnt + 1;
                                    (fresh47) < 3i32
                                }) {
                                    break;
                                }
                            }
                        } else {
                            cnt = 0i32;
                            loop {
                                wkc = ecx_APRD(
                                    (*context).port,
                                    aiadr,
                                    ECT_REG_EEPDAT as uint16,
                                    ::core::mem::size_of::<uint32>() as uint16,
                                    &mut edat32 as *mut uint32 as *mut libc::c_void,
                                    2000i32,
                                );
                                if !(wkc <= 0i32 && {
                                    let fresh48 = cnt;
                                    cnt = cnt + 1;
                                    (fresh48) < 3i32
                                }) {
                                    break;
                                }
                            }
                            edat64 = edat32 as uint64
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
pub unsafe extern "C" fn ecx_writeeepromAP(
    mut context: *mut ecx_contextt,
    mut aiadr: uint16,
    mut eeproma: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut estat: uint16 = 0;
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
        if estat as libc::c_int & 0x7800i32 != 0 {
            /* error bits are set */
            estat = EC_ECMD_NOP as uint16; /* clear error bits */
            wkc = ecx_APWR(
                (*context).port,
                aiadr,
                ECT_REG_EEPCTL as uint16,
                ::core::mem::size_of::<uint16>() as uint16,
                &mut estat as *mut uint16 as *mut libc::c_void,
                2000i32 * 3i32,
            )
        } /* wait for eeprom ready */
        loop {
            cnt = 0i32;
            loop {
                wkc = ecx_APWR(
                    (*context).port,
                    aiadr,
                    ECT_REG_EEPDAT as uint16,
                    ::core::mem::size_of::<uint16>() as uint16,
                    &mut data as *mut uint16 as *mut libc::c_void,
                    2000i32,
                );
                if !(wkc <= 0i32 && {
                    let fresh49 = cnt;
                    cnt = cnt + 1;
                    (fresh49) < 3i32
                }) {
                    break;
                }
            }
            ed.comm = EC_ECMD_WRITE as uint16;
            ed.addr = eeproma;
            ed.d2 = 0u16;
            cnt = 0i32;
            loop {
                wkc = ecx_APWR(
                    (*context).port,
                    aiadr,
                    ECT_REG_EEPCTL as uint16,
                    ::core::mem::size_of::<ec_eepromt>() as uint16,
                    &mut ed as *mut ec_eepromt as *mut libc::c_void,
                    2000i32,
                );
                if !(wkc <= 0i32 && {
                    let fresh50 = cnt;
                    cnt = cnt + 1;
                    (fresh50) < 3i32
                }) {
                    break;
                }
            }
            if wkc != 0 {
                osal_usleep((200i32 * 2i32) as uint32);
                estat = 0u16;
                if ecx_eeprom_waitnotbusyAP(context, aiadr, &mut estat, timeout) != 0 {
                    if estat as libc::c_int & 0x2000i32 != 0 {
                        nackcnt += 1;
                        osal_usleep((200i32 * 5i32) as uint32);
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
pub unsafe extern "C" fn ecx_eeprom_waitnotbusyFP(
    mut context: *mut ecx_contextt,
    mut configadr: uint16,
    mut estat: *mut uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0i32;
    let mut retval: uint16 = 0u16;
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer, timeout as uint32);
    loop {
        let fresh51 = cnt;
        cnt = cnt + 1;
        if fresh51 != 0 {
            osal_usleep(200u32);
        }
        *estat = 0u16;
        wkc = ecx_FPRD(
            (*context).port,
            configadr,
            ECT_REG_EEPSTAT as uint16,
            ::core::mem::size_of::<uint16>() as uint16,
            estat as *mut libc::c_void,
            2000i32,
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
pub unsafe extern "C" fn ecx_readeepromFP(
    mut context: *mut ecx_contextt,
    mut configadr: uint16,
    mut eeproma: uint16,
    mut timeout: libc::c_int,
) -> uint64 {
    let mut estat: uint16 = 0;
    let mut edat32: uint32 = 0;
    let mut edat64: uint64 = 0;
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
        if estat as libc::c_int & 0x7800i32 != 0 {
            /* error bits are set */
            estat = EC_ECMD_NOP as uint16; /* clear error bits */
            wkc = ecx_FPWR(
                (*context).port,
                configadr,
                ECT_REG_EEPCTL as uint16,
                ::core::mem::size_of::<uint16>() as uint16,
                &mut estat as *mut uint16 as *mut libc::c_void,
                2000i32 * 3i32,
            )
        }
        loop {
            ed.comm = EC_ECMD_READ as uint16;
            ed.addr = eeproma;
            ed.d2 = 0u16;
            cnt = 0i32;
            loop {
                wkc = ecx_FPWR(
                    (*context).port,
                    configadr,
                    ECT_REG_EEPCTL as uint16,
                    ::core::mem::size_of::<ec_eepromt>() as uint16,
                    &mut ed as *mut ec_eepromt as *mut libc::c_void,
                    2000i32,
                );
                if !(wkc <= 0i32 && {
                    let fresh52 = cnt;
                    cnt = cnt + 1;
                    (fresh52) < 3i32
                }) {
                    break;
                }
            }
            if wkc != 0 {
                osal_usleep(200u32);
                estat = 0u16;
                if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, timeout) != 0 {
                    if estat as libc::c_int & 0x2000i32 != 0 {
                        nackcnt += 1;
                        osal_usleep((200i32 * 5i32) as uint32);
                    } else {
                        nackcnt = 0i32;
                        if estat as libc::c_int & 0x40i32 != 0 {
                            cnt = 0i32;
                            loop {
                                wkc = ecx_FPRD(
                                    (*context).port,
                                    configadr,
                                    ECT_REG_EEPDAT as uint16,
                                    ::core::mem::size_of::<uint64>() as uint16,
                                    &mut edat64 as *mut uint64 as *mut libc::c_void,
                                    2000i32,
                                );
                                if !(wkc <= 0i32 && {
                                    let fresh53 = cnt;
                                    cnt = cnt + 1;
                                    (fresh53) < 3i32
                                }) {
                                    break;
                                }
                            }
                        } else {
                            cnt = 0i32;
                            loop {
                                wkc = ecx_FPRD(
                                    (*context).port,
                                    configadr,
                                    ECT_REG_EEPDAT as uint16,
                                    ::core::mem::size_of::<uint32>() as uint16,
                                    &mut edat32 as *mut uint32 as *mut libc::c_void,
                                    2000i32,
                                );
                                if !(wkc <= 0i32 && {
                                    let fresh54 = cnt;
                                    cnt = cnt + 1;
                                    (fresh54) < 3i32
                                }) {
                                    break;
                                }
                            }
                            edat64 = edat32 as uint64
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
pub unsafe extern "C" fn ecx_writeeepromFP(
    mut context: *mut ecx_contextt,
    mut configadr: uint16,
    mut eeproma: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut estat: uint16 = 0;
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
        if estat as libc::c_int & 0x7800i32 != 0 {
            /* error bits are set */
            estat = EC_ECMD_NOP as uint16; /* clear error bits */
            wkc = ecx_FPWR(
                (*context).port,
                configadr,
                ECT_REG_EEPCTL as uint16,
                ::core::mem::size_of::<uint16>() as uint16,
                &mut estat as *mut uint16 as *mut libc::c_void,
                2000i32 * 3i32,
            )
        }
        loop {
            cnt = 0i32;
            loop {
                wkc = ecx_FPWR(
                    (*context).port,
                    configadr,
                    ECT_REG_EEPDAT as uint16,
                    ::core::mem::size_of::<uint16>() as uint16,
                    &mut data as *mut uint16 as *mut libc::c_void,
                    2000i32,
                );
                if !(wkc <= 0i32 && {
                    let fresh55 = cnt;
                    cnt = cnt + 1;
                    (fresh55) < 3i32
                }) {
                    break;
                }
            }
            ed.comm = EC_ECMD_WRITE as uint16;
            ed.addr = eeproma;
            ed.d2 = 0u16;
            cnt = 0i32;
            loop {
                wkc = ecx_FPWR(
                    (*context).port,
                    configadr,
                    ECT_REG_EEPCTL as uint16,
                    ::core::mem::size_of::<ec_eepromt>() as uint16,
                    &mut ed as *mut ec_eepromt as *mut libc::c_void,
                    2000i32,
                );
                if !(wkc <= 0i32 && {
                    let fresh56 = cnt;
                    cnt = cnt + 1;
                    (fresh56) < 3i32
                }) {
                    break;
                }
            }
            if wkc != 0 {
                osal_usleep((200i32 * 2i32) as uint32);
                estat = 0u16;
                if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, timeout) != 0 {
                    if estat as libc::c_int & 0x2000i32 != 0 {
                        nackcnt += 1;
                        osal_usleep((200i32 * 5i32) as uint32);
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
pub unsafe extern "C" fn ecx_readeeprom1(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut eeproma: uint16,
) {
    let mut configadr: uint16 = 0; /* set eeprom control to master */
    let mut estat: uint16 = 0;
    let mut ed: ec_eepromt = ec_eepromt {
        comm: 0,
        addr: 0,
        d2: 0,
    };
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0i32;
    ecx_eeprom2master(context, slave);
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, 20000i32) != 0 {
        if estat as libc::c_int & 0x7800i32 != 0 {
            /* error bits are set */
            estat = EC_ECMD_NOP as uint16; /* clear error bits */
            wkc = ecx_FPWR(
                (*context).port,
                configadr,
                ECT_REG_EEPCTL as uint16,
                ::core::mem::size_of::<uint16>() as uint16,
                &mut estat as *mut uint16 as *mut libc::c_void,
                2000i32 * 3i32,
            )
        }
        ed.comm = EC_ECMD_READ as uint16;
        ed.addr = eeproma;
        ed.d2 = 0u16;
        loop {
            wkc = ecx_FPWR(
                (*context).port,
                configadr,
                ECT_REG_EEPCTL as uint16,
                ::core::mem::size_of::<ec_eepromt>() as uint16,
                &mut ed as *mut ec_eepromt as *mut libc::c_void,
                2000i32,
            );
            if !(wkc <= 0i32 && {
                let fresh57 = cnt;
                cnt = cnt + 1;
                (fresh57) < 3i32
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
pub unsafe extern "C" fn ecx_readeeprom2(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut timeout: libc::c_int,
) -> uint32 {
    let mut estat: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut edat: uint32 = 0;
    let mut wkc: libc::c_int = 0;
    let mut cnt: libc::c_int = 0i32;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    edat = 0u32;
    estat = 0u16;
    if ecx_eeprom_waitnotbusyFP(context, configadr, &mut estat, timeout) != 0 {
        loop {
            wkc = ecx_FPRD(
                (*context).port,
                configadr,
                ECT_REG_EEPDAT as uint16,
                ::core::mem::size_of::<uint32>() as uint16,
                &mut edat as *mut uint32 as *mut libc::c_void,
                2000i32,
            );
            if !(wkc <= 0i32 && {
                let fresh58 = cnt;
                cnt = cnt + 1;
                (fresh58) < 3i32
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
unsafe extern "C" fn ecx_pushindex(
    mut context: *mut ecx_contextt,
    mut idx: uint8,
    mut data: *mut libc::c_void,
    mut length: uint16,
    mut DCO: uint16,
) {
    if ((*(*context).idxstack).pushed as libc::c_int) < 16i32 {
        (*(*context).idxstack).idx[(*(*context).idxstack).pushed as usize] = idx;
        (*(*context).idxstack).data[(*(*context).idxstack).pushed as usize] = data;
        (*(*context).idxstack).length[(*(*context).idxstack).pushed as usize] = length;
        (*(*context).idxstack).dcoffset[(*(*context).idxstack).pushed as usize] = DCO;
        (*(*context).idxstack).pushed = (*(*context).idxstack).pushed.wrapping_add(1)
    };
}
/* * Pull index of segmented LRD/LWR/LRW combination.
 * @param[in]  context        = context struct
 * @return Stack location, -1 if stack is empty.
 */
unsafe extern "C" fn ecx_pullindex(mut context: *mut ecx_contextt) -> libc::c_int {
    let mut rval: libc::c_int = -(1i32);
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
unsafe extern "C" fn ecx_clearindex(mut context: *mut ecx_contextt) {
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
unsafe extern "C" fn ecx_main_send_processdata(
    mut context: *mut ecx_contextt,
    mut group: uint8,
    mut use_overlap_io: boolean,
) -> libc::c_int {
    let mut LogAdr: uint32 = 0;
    let mut w1: uint16 = 0;
    let mut w2: uint16 = 0;
    let mut length: libc::c_int = 0;
    let mut sublength: uint16 = 0;
    let mut idx: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut data: *mut uint8 = 0 as *mut uint8;
    let mut first: boolean = 0u8;
    let mut currentsegment: uint16 = 0u16;
    let mut iomapinputoffset: uint32 = 0;
    let mut DCO: uint16 = 0;
    wkc = 0i32;
    if (*(*context).grouplist.offset(group as isize)).hasdc != 0 {
        first = 1u8
    }
    /* For overlapping IO map use the biggest */
    if use_overlap_io as libc::c_int == 1i32 {
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
                            ) as uint16
                    } else {
                        let fresh60 = currentsegment;
                        currentsegment = currentsegment.wrapping_add(1);
                        sublength = (*(*context).grouplist.offset(group as isize)).IOsegment
                            [fresh60 as usize] as uint16
                    }
                    /* get new index */
                    idx = ecx_getindex((*context).port);
                    w1 = (LogAdr & 0xffffu32) as uint16;
                    w2 = (LogAdr >> 16i32) as uint16;
                    DCO = 0u16;
                    ecx_setupdatagram(
                        (*context).port,
                        &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                            as *mut ec_bufT as *mut libc::c_void,
                        ec_cmdtype::EC_CMD_LRD as uint8,
                        idx,
                        w1,
                        w2,
                        sublength,
                        data as *mut libc::c_void,
                    );
                    if first != 0 {
                        /* FPRMW in second datagram */
                        DCO = ecx_adddatagram(
                            (*context).port,
                            &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                                as *mut ec_bufT as *mut libc::c_void,
                            ec_cmdtype::EC_CMD_FRMW as uint8,
                            idx,
                            0u8,
                            (*(*context).slavelist.offset(
                                (*(*context).grouplist.offset(group as isize)).DCnext as isize,
                            ))
                            .configadr,
                            ECT_REG_DCSYSTIME as uint16,
                            ::core::mem::size_of::<int64>() as uint16,
                            (*context).DCtime as *mut libc::c_void,
                        );
                        first = 0u8
                    }
                    /* send frame */
                    ecx_outframe_red((*context).port, idx);
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
                        [fresh61 as usize] as uint16;
                    if (length - sublength as libc::c_int) < 0i32 {
                        sublength = length as uint16
                    }
                    /* get new index */
                    idx = ecx_getindex((*context).port);
                    w1 = (LogAdr & 0xffffu32) as uint16;
                    w2 = (LogAdr >> 16i32) as uint16;
                    DCO = 0u16;
                    ecx_setupdatagram(
                        (*context).port,
                        &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                            as *mut ec_bufT as *mut libc::c_void,
                        ec_cmdtype::EC_CMD_LWR as uint8,
                        idx,
                        w1,
                        w2,
                        sublength,
                        data as *mut libc::c_void,
                    );
                    if first != 0 {
                        /* FPRMW in second datagram */
                        DCO = ecx_adddatagram(
                            (*context).port,
                            &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                                as *mut ec_bufT as *mut libc::c_void,
                            ec_cmdtype::EC_CMD_FRMW as uint8,
                            idx,
                            0u8,
                            (*(*context).slavelist.offset(
                                (*(*context).grouplist.offset(group as isize)).DCnext as isize,
                            ))
                            .configadr,
                            ECT_REG_DCSYSTIME as uint16,
                            ::core::mem::size_of::<int64>() as uint16,
                            (*context).DCtime as *mut libc::c_void,
                        );
                        first = 0u8
                    }
                    /* send frame */
                    ecx_outframe_red((*context).port, idx);
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
                    [fresh62 as usize] as uint16;
                /* get new index */
                idx = ecx_getindex((*context).port);
                w1 = (LogAdr & 0xffffu32) as uint16;
                w2 = (LogAdr >> 16i32) as uint16;
                DCO = 0u16;
                ecx_setupdatagram(
                    (*context).port,
                    &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                        as *mut libc::c_void,
                    ec_cmdtype::EC_CMD_LRW as uint8,
                    idx,
                    w1,
                    w2,
                    sublength,
                    data as *mut libc::c_void,
                );
                if first != 0 {
                    /* FPRMW in second datagram */
                    DCO = ecx_adddatagram(
                        (*context).port,
                        &mut *(*(*context).port).txbuf.as_mut_ptr().offset(idx as isize)
                            as *mut ec_bufT as *mut libc::c_void,
                        ec_cmdtype::EC_CMD_FRMW as uint8,
                        idx,
                        0u8,
                        (*(*context).slavelist.offset(
                            (*(*context).grouplist.offset(group as isize)).DCnext as isize,
                        ))
                        .configadr,
                        ECT_REG_DCSYSTIME as uint16,
                        ::core::mem::size_of::<int64>() as uint16,
                        (*context).DCtime as *mut libc::c_void,
                    );
                    first = 0u8
                }
                /* send frame */
                ecx_outframe_red((*context).port, idx);
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
pub unsafe extern "C" fn ecx_send_overlap_processdata_group(
    mut context: *mut ecx_contextt,
    mut group: uint8,
) -> libc::c_int {
    return ecx_main_send_processdata(context, group, 1u8);
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
pub unsafe extern "C" fn ecx_send_processdata_group(
    mut context: *mut ecx_contextt,
    mut group: uint8,
) -> libc::c_int {
    return ecx_main_send_processdata(context, group, 0u8);
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
pub unsafe extern "C" fn ecx_receive_processdata_group(
    mut context: *mut ecx_contextt,
    mut group: uint8,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut idx: uint8 = 0;
    let mut pos: libc::c_int = 0;
    let mut wkc: libc::c_int = 0i32;
    let mut wkc2: libc::c_int = 0;
    let mut le_wkc: uint16 = 0u16;
    let mut valid_wkc: libc::c_int = 0i32;
    let mut le_DCtime: int64 = 0;
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
        wkc2 = ecx_waitinframe((*context).port, idx, timeout);
        /* check if there is input data in frame */
        if wkc2 > -(1i32) {
            if (*rxbuf.offset(idx as isize))[::core::mem::size_of::<uint16>()] as libc::c_int
                == ec_cmdtype::EC_CMD_LRD as libc::c_int
                || (*rxbuf.offset(idx as isize))[::core::mem::size_of::<uint16>()] as libc::c_int
                    == ec_cmdtype::EC_CMD_LRW as libc::c_int
            {
                if (*idxstack).dcoffset[pos as usize] as libc::c_int > 0i32 {
                    memcpy(
                        (*idxstack).data[pos as usize],
                        &mut *(*rxbuf.offset(idx as isize))
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<ec_comt>() as isize)
                            as *mut uint8 as *const libc::c_void,
                        (*idxstack).length[pos as usize] as usize,
                    );
                    memcpy(
                        &mut le_wkc as *mut uint16 as *mut libc::c_void,
                        &mut *(*rxbuf.offset(idx as isize)).as_mut_ptr().offset(
                            core::mem::size_of::<ec_comt>().wrapping_add(
                                *(*idxstack).length.as_mut_ptr().offset(pos as isize) as usize,
                            ) as isize,
                        ) as *mut uint8 as *const libc::c_void,
                        core::mem::size_of::<uint16>(),
                    );
                    wkc = le_wkc as libc::c_int;
                    memcpy(&mut le_DCtime as *mut int64 as *mut libc::c_void,
                           &mut *(*rxbuf.offset(idx as
                                                    isize)).as_mut_ptr().offset(*(*idxstack).dcoffset.as_mut_ptr().offset(pos
                                                                                                                              as
                                                                                                                              isize)
                                                                                    as
                                                                                    isize)
                               as *mut uint8 as *const libc::c_void,
                           core::mem::size_of::<int64>(),);
                    *(*context).DCtime = le_DCtime
                } else {
                    /* copy input data back to process data buffer */
                    memcpy(
                        (*idxstack).data[pos as usize],
                        &mut *(*rxbuf.offset(idx as isize))
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<ec_comt>() as isize)
                            as *mut uint8 as *const libc::c_void,
                        (*idxstack).length[pos as usize] as usize,
                    );
                    wkc += wkc2
                }
                valid_wkc = 1i32
            } else if (*rxbuf.offset(idx as isize))[::core::mem::size_of::<uint16>()] as libc::c_int
                == ec_cmdtype::EC_CMD_LWR as libc::c_int
            {
                if (*idxstack).dcoffset[pos as usize] as libc::c_int > 0i32 {
                    memcpy(
                        &mut le_wkc as *mut uint16 as *mut libc::c_void,
                        &mut *(*rxbuf.offset(idx as isize)).as_mut_ptr().offset(
                            core::mem::size_of::<ec_comt>().wrapping_add(
                                *(*idxstack).length.as_mut_ptr().offset(pos as isize) as usize,
                            ) as isize,
                        ) as *mut uint8 as *const libc::c_void,
                        core::mem::size_of::<uint16>(),
                    );
                    /* output WKC counts 2 times when using LRW, emulate the same for LWR */
                    wkc = le_wkc as libc::c_int * 2i32;
                    memcpy(&mut le_DCtime as *mut int64 as *mut libc::c_void,
                           &mut *(*rxbuf.offset(idx as
                                                    isize)).as_mut_ptr().offset(*(*idxstack).dcoffset.as_mut_ptr().offset(pos
                                                                                                                              as
                                                                                                                              isize)
                                                                                    as
                                                                                    isize)
                               as *mut uint8 as *const libc::c_void,
                           core::mem::size_of::<int64>(),);
                    *(*context).DCtime = le_DCtime
                } else {
                    /* output WKC counts 2 times when using LRW, emulate the same for LWR */
                    wkc += wkc2 * 2i32
                }
                valid_wkc = 1i32
            }
        }
        /* release buffer */
        ecx_setbufstat(
            (*context).port,
            idx,
            ec_bufstate::EC_BUF_EMPTY as libc::c_int,
        );
        /* get next index */
        pos = ecx_pullindex(context)
    }
    ecx_clearindex(context);
    /* if no frames has arrived */
    if valid_wkc == 0i32 {
        return -(1i32);
    }
    return wkc;
}
#[no_mangle]
pub unsafe extern "C" fn ecx_send_processdata(mut context: *mut ecx_contextt) -> libc::c_int {
    return ecx_send_processdata_group(context, 0u8);
}
#[no_mangle]
pub unsafe extern "C" fn ecx_send_overlap_processdata(
    mut context: *mut ecx_contextt,
) -> libc::c_int {
    return ecx_send_overlap_processdata_group(context, 0u8);
}
#[no_mangle]
pub unsafe extern "C" fn ecx_receive_processdata(
    mut context: *mut ecx_contextt,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_receive_processdata_group(context, 0u8, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_pusherror(mut Ec: *const ec_errort) {
    ecx_pusherror(&mut ecx_context, Ec);
}
#[no_mangle]
pub unsafe extern "C" fn ec_poperror(mut Ec: *mut ec_errort) -> boolean {
    return ecx_poperror(&mut ecx_context, Ec);
}
#[no_mangle]
pub unsafe extern "C" fn ec_iserror() -> boolean {
    return ecx_iserror(&mut ecx_context);
}
#[no_mangle]
pub unsafe extern "C" fn ec_packeterror(
    mut Slave: uint16,
    mut Index: uint16,
    mut SubIdx: uint8,
    mut ErrorCode: uint16,
) {
    ecx_packeterror(&mut ecx_context, Slave, Index, SubIdx, ErrorCode);
}
/* * Initialise lib in single NIC mode
 * @param[in] ifname   = Dev name, f.e. "eth0"
 * @return >0 if OK
 * @see ecx_init
 */
#[no_mangle]
pub unsafe extern "C" fn ec_init(mut ifname: *const libc::c_char) -> libc::c_int {
    return ecx_init(&mut ecx_context, ifname);
}
/* * Initialise lib in redundant NIC mode
 * @param[in]  ifname   = Primary Dev name, f.e. "eth0"
 * @param[in]  if2name  = Secondary Dev name, f.e. "eth1"
 * @return >0 if OK
 * @see ecx_init_redundant
 */
#[no_mangle]
pub unsafe extern "C" fn ec_init_redundant(
    mut ifname: *const libc::c_char,
    mut if2name: *mut libc::c_char,
) -> libc::c_int {
    return ecx_init_redundant(&mut ecx_context, &mut ecx_redport, ifname, if2name);
}
/* * Close lib.
 * @see ecx_close
 */
#[no_mangle]
pub unsafe extern "C" fn ec_close() {
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
pub unsafe extern "C" fn ec_siigetbyte(mut slave: uint16, mut address: uint16) -> uint8 {
    return ecx_siigetbyte(&mut ecx_context, slave, address);
}
/* * Find SII section header in slave EEPROM.
 *  @param[in] slave   = slave number
 *  @param[in] cat     = section category
 *  @return byte address of section at section length entry, if not available then 0
 *  @see ecx_siifind
 */
#[no_mangle]
pub unsafe extern "C" fn ec_siifind(mut slave: uint16, mut cat: uint16) -> int16 {
    return ecx_siifind(&mut ecx_context, slave, cat);
}
/* * Get string from SII string section in slave EEPROM.
 *  @param[out] str    = requested string, 0x00 if not found
 *  @param[in]  slave  = slave number
 *  @param[in]  Sn     = string number
 *  @see ecx_siistring
 */
#[no_mangle]
pub unsafe extern "C" fn ec_siistring(
    mut str: *mut libc::c_char,
    mut slave: uint16,
    mut Sn: uint16,
) {
    ecx_siistring(&mut ecx_context, str, slave, Sn);
}
/* * Get FMMU data from SII FMMU section in slave EEPROM.
 *  @param[in]  slave  = slave number
 *  @param[out] FMMU   = FMMU struct from SII, max. 4 FMMU's
 *  @return number of FMMU's defined in section
 *  @see ecx_siiFMMU
 */
#[no_mangle]
pub unsafe extern "C" fn ec_siiFMMU(mut slave: uint16, mut FMMU: *mut ec_eepromFMMUt) -> uint16 {
    return ecx_siiFMMU(&mut ecx_context, slave, FMMU);
}
/* * Get SM data from SII SM section in slave EEPROM.
 *  @param[in]  slave   = slave number
 *  @param[out] SM      = first SM struct from SII
 *  @return number of SM's defined in section
 *  @see ecx_siiSM
 */
#[no_mangle]
pub unsafe extern "C" fn ec_siiSM(mut slave: uint16, mut SM: *mut ec_eepromSMt) -> uint16 {
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
pub unsafe extern "C" fn ec_siiSMnext(
    mut slave: uint16,
    mut SM: *mut ec_eepromSMt,
    mut n: uint16,
) -> uint16 {
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
pub unsafe extern "C" fn ec_siiPDO(
    mut slave: uint16,
    mut PDO: *mut ec_eepromPDOt,
    mut t: uint8,
) -> uint32 {
    return ecx_siiPDO(&mut ecx_context, slave, PDO, t);
}
/* * Read all slave states in ec_slave.
 * @return lowest state found
 * @see ecx_readstate
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readstate() -> libc::c_int {
    return ecx_readstate(&mut ecx_context);
}
/* * Write slave state, if slave = 0 then write to all slaves.
 * The function does not check if the actual state is changed.
 * @param[in] slave = Slave number, 0 = master
 * @return 0
 * @see ecx_writestate
 */
#[no_mangle]
pub unsafe extern "C" fn ec_writestate(mut slave: uint16) -> libc::c_int {
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
pub unsafe extern "C" fn ec_statecheck(
    mut slave: uint16,
    mut reqstate: uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    return ecx_statecheck(&mut ecx_context, slave, reqstate, timeout);
}
/* * Check if IN mailbox of slave is empty.
 * @param[in] slave    = Slave number
 * @param[in] timeout  = Timeout in us
 * @return >0 is success
 * @see ecx_mbxempty
 */
#[no_mangle]
pub unsafe extern "C" fn ec_mbxempty(mut slave: uint16, mut timeout: libc::c_int) -> libc::c_int {
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
pub unsafe extern "C" fn ec_mbxsend(
    mut slave: uint16,
    mut mbx: *mut ec_mbxbuft,
    mut timeout: libc::c_int,
) -> libc::c_int {
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
pub unsafe extern "C" fn ec_mbxreceive(
    mut slave: uint16,
    mut mbx: *mut ec_mbxbuft,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_mbxreceive(&mut ecx_context, slave, mbx, timeout);
}
/* * Dump complete EEPROM data from slave in buffer.
 * @param[in]  slave    = Slave number
 * @param[out] esibuf   = EEPROM data buffer, make sure it is big enough.
 * @see ecx_esidump
 */
#[no_mangle]
pub unsafe extern "C" fn ec_esidump(mut slave: uint16, mut esibuf: *mut uint8) {
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
pub unsafe extern "C" fn ec_readeeprom(
    mut slave: uint16,
    mut eeproma: uint16,
    mut timeout: libc::c_int,
) -> uint32 {
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
pub unsafe extern "C" fn ec_writeeeprom(
    mut slave: uint16,
    mut eeproma: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_writeeeprom(&mut ecx_context, slave, eeproma, data, timeout);
}
/* * Set eeprom control to master. Only if set to PDI.
 * @param[in] slave = Slave number
 * @return >0 if OK
 * @see ecx_eeprom2master
 */
#[no_mangle]
pub unsafe extern "C" fn ec_eeprom2master(mut slave: uint16) -> libc::c_int {
    return ecx_eeprom2master(&mut ecx_context, slave);
}
#[no_mangle]
pub unsafe extern "C" fn ec_eeprom2pdi(mut slave: uint16) -> libc::c_int {
    return ecx_eeprom2pdi(&mut ecx_context, slave);
}
#[no_mangle]
pub unsafe extern "C" fn ec_eeprom_waitnotbusyAP(
    mut aiadr: uint16,
    mut estat: *mut uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    return ecx_eeprom_waitnotbusyAP(&mut ecx_context, aiadr, estat, timeout);
}
/* * Read EEPROM from slave bypassing cache. APRD method.
 * @param[in] aiadr       = auto increment address of slave
 * @param[in] eeproma     = (WORD) Address in the EEPROM
 * @param[in] timeout     = Timeout in us.
 * @return EEPROM data 64bit or 32bit
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readeepromAP(
    mut aiadr: uint16,
    mut eeproma: uint16,
    mut timeout: libc::c_int,
) -> uint64 {
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
pub unsafe extern "C" fn ec_writeeepromAP(
    mut aiadr: uint16,
    mut eeproma: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_writeeepromAP(&mut ecx_context, aiadr, eeproma, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_eeprom_waitnotbusyFP(
    mut configadr: uint16,
    mut estat: *mut uint16,
    mut timeout: libc::c_int,
) -> uint16 {
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
pub unsafe extern "C" fn ec_readeepromFP(
    mut configadr: uint16,
    mut eeproma: uint16,
    mut timeout: libc::c_int,
) -> uint64 {
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
pub unsafe extern "C" fn ec_writeeepromFP(
    mut configadr: uint16,
    mut eeproma: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
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
pub unsafe extern "C" fn ec_readeeprom1(mut slave: uint16, mut eeproma: uint16) {
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
pub unsafe extern "C" fn ec_readeeprom2(mut slave: uint16, mut timeout: libc::c_int) -> uint32 {
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
pub unsafe extern "C" fn ec_send_processdata_group(mut group: uint8) -> libc::c_int {
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
pub unsafe extern "C" fn ec_send_overlap_processdata_group(mut group: uint8) -> libc::c_int {
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
pub unsafe extern "C" fn ec_receive_processdata_group(
    mut group: uint8,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_receive_processdata_group(&mut ecx_context, group, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_send_processdata() -> libc::c_int {
    return ec_send_processdata_group(0u8);
}
#[no_mangle]
pub unsafe extern "C" fn ec_send_overlap_processdata() -> libc::c_int {
    return ec_send_overlap_processdata_group(0u8);
}
#[no_mangle]
pub unsafe extern "C" fn ec_receive_processdata(mut timeout: libc::c_int) -> libc::c_int {
    return ec_receive_processdata_group(0u8, timeout);
}
unsafe extern "C" fn run_static_initializers() {
    ecx_context = {
        let mut init = ecx_context {
            port: &mut ecx_port,
            slavelist: &mut *ec_slave.as_mut_ptr().offset(0isize) as *mut ec_slavet,
            slavecount: &mut ec_slavecount,
            maxslave: 200i32,
            grouplist: &mut *ec_group.as_mut_ptr().offset(0isize) as *mut ec_groupt,
            maxgroup: 2i32,
            esibuf: &mut *ec_esibuf.as_mut_ptr().offset(0isize) as *mut uint8,
            esimap: &mut *ec_esimap.as_mut_ptr().offset(0isize) as *mut uint32,
            esislave: 0u16,
            elist: &mut ec_elist,
            idxstack: &mut ec_idxstack,
            ecaterror: &mut EcatError,
            DCtime: &mut ec_DCtime,
            SMcommtype: &mut *ec_SMcommtype.as_mut_ptr().offset(0isize) as *mut ec_SMcommtypet,
            PDOassign: &mut *ec_PDOassign.as_mut_ptr().offset(0isize) as *mut ec_PDOassignt,
            PDOdesc: &mut *ec_PDOdesc.as_mut_ptr().offset(0isize) as *mut ec_PDOdesct,
            eepSM: &mut ec_SM,
            eepFMMU: &mut ec_FMMU,
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
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
