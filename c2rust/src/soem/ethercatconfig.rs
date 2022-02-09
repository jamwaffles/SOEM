use ::libc;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn osal_usleep(usec: uint32) -> libc::c_int;
    #[no_mangle]
    static mut ecx_context: ecx_contextt;
    #[no_mangle]
    fn ecx_siigetbyte(context: *mut ecx_contextt, slave: uint16,
                      address: uint16) -> uint8;
    #[no_mangle]
    fn ecx_siifind(context: *mut ecx_contextt, slave: uint16, cat: uint16)
     -> int16;
    #[no_mangle]
    fn ecx_siistring(context: *mut ecx_contextt, str: *mut libc::c_char,
                     slave: uint16, Sn: uint16);
    #[no_mangle]
    fn ecx_siiFMMU(context: *mut ecx_contextt, slave: uint16,
                   FMMU: *mut ec_eepromFMMUt) -> uint16;
    #[no_mangle]
    fn ecx_siiSM(context: *mut ecx_contextt, slave: uint16,
                 SM: *mut ec_eepromSMt) -> uint16;
    #[no_mangle]
    fn ecx_siiSMnext(context: *mut ecx_contextt, slave: uint16,
                     SM: *mut ec_eepromSMt, n: uint16) -> uint16;
    #[no_mangle]
    fn ecx_siiPDO(context: *mut ecx_contextt, slave: uint16,
                  PDO: *mut ec_eepromPDOt, t: uint8) -> uint32;
    #[no_mangle]
    fn ecx_statecheck(context: *mut ecx_contextt, slave: uint16,
                      reqstate: uint16, timeout: libc::c_int) -> uint16;
    #[no_mangle]
    fn ecx_readeeprom(context: *mut ecx_contextt, slave: uint16,
                      eeproma: uint16, timeout: libc::c_int) -> uint32;
    #[no_mangle]
    fn ecx_eeprom2master(context: *mut ecx_contextt, slave: uint16)
     -> libc::c_int;
    #[no_mangle]
    fn ecx_eeprom2pdi(context: *mut ecx_contextt, slave: uint16)
     -> libc::c_int;
    #[no_mangle]
    fn ecx_readeeprom1(context: *mut ecx_contextt, slave: uint16,
                       eeproma: uint16);
    #[no_mangle]
    fn ecx_readeeprom2(context: *mut ecx_contextt, slave: uint16,
                       timeout: libc::c_int) -> uint32;
    #[no_mangle]
    fn ecx_BWR(port: *mut ecx_portt, ADP: uint16, ADO: uint16, length: uint16,
               data: *mut libc::c_void, timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ecx_BRD(port: *mut ecx_portt, ADP: uint16, ADO: uint16, length: uint16,
               data: *mut libc::c_void, timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ecx_APRD(port: *mut ecx_portt, ADP: uint16, ADO: uint16,
                length: uint16, data: *mut libc::c_void, timeout: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ecx_APRDw(port: *mut ecx_portt, ADP: uint16, ADO: uint16,
                 timeout: libc::c_int) -> uint16;
    #[no_mangle]
    fn ecx_FPRD(port: *mut ecx_portt, ADP: uint16, ADO: uint16,
                length: uint16, data: *mut libc::c_void, timeout: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ecx_FPRDw(port: *mut ecx_portt, ADP: uint16, ADO: uint16,
                 timeout: libc::c_int) -> uint16;
    #[no_mangle]
    fn ecx_APWRw(port: *mut ecx_portt, ADP: uint16, ADO: uint16, data: uint16,
                 timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ecx_FPWRw(port: *mut ecx_portt, ADP: uint16, ADO: uint16, data: uint16,
                 timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ecx_FPWR(port: *mut ecx_portt, ADP: uint16, ADO: uint16,
                length: uint16, data: *mut libc::c_void, timeout: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ecx_readPDOmap(context: *mut ecx_contextt, Slave: uint16,
                      Osize: *mut uint32, Isize: *mut uint32) -> libc::c_int;
    #[no_mangle]
    fn ecx_readPDOmapCA(context: *mut ecx_contextt, Slave: uint16,
                        Thread_n: libc::c_int, Osize: *mut uint32,
                        Isize: *mut uint32) -> libc::c_int;
    #[no_mangle]
    fn ecx_readIDNmap(context: *mut ecx_contextt, slave: uint16,
                      Osize: *mut uint32, Isize: *mut uint32) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type boolean = uint8_t;
pub type int16 = int16_t;
pub type int32 = int32_t;
pub type uint8 = uint8_t;
pub type uint16 = uint16_t;
pub type uint32 = uint32_t;
pub type int64 = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_timet {
    pub sec: uint32,
    pub usec: uint32,
}
pub type ec_bufT = [uint8; 1518];
pub type C2RustUnnamed = libc::c_uint;
pub const EC_STATE_ERROR: C2RustUnnamed = 16;
pub const EC_STATE_ACK: C2RustUnnamed = 16;
pub const EC_STATE_OPERATIONAL: C2RustUnnamed = 8;
pub const EC_STATE_SAFE_OP: C2RustUnnamed = 4;
pub const EC_STATE_BOOT: C2RustUnnamed = 3;
pub const EC_STATE_PRE_OP: C2RustUnnamed = 2;
pub const EC_STATE_INIT: C2RustUnnamed = 1;
pub const EC_STATE_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ECT_SII_PDO: C2RustUnnamed_0 = 50;
pub const ECT_SII_SM: C2RustUnnamed_0 = 41;
pub const ECT_SII_FMMU: C2RustUnnamed_0 = 40;
pub const ECT_SII_GENERAL: C2RustUnnamed_0 = 30;
pub const ECT_SII_STRING: C2RustUnnamed_0 = 10;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const ECT_SII_MBXPROTO: C2RustUnnamed_1 = 28;
pub const ECT_SII_RXMBXADR: C2RustUnnamed_1 = 24;
pub const ECT_SII_TXMBXADR: C2RustUnnamed_1 = 26;
pub const ECT_SII_MBXSIZE: C2RustUnnamed_1 = 25;
pub const ECT_SII_BOOTTXMBX: C2RustUnnamed_1 = 22;
pub const ECT_SII_BOOTRXMBX: C2RustUnnamed_1 = 20;
pub const ECT_SII_REV: C2RustUnnamed_1 = 12;
pub const ECT_SII_ID: C2RustUnnamed_1 = 10;
pub const ECT_SII_MANUF: C2RustUnnamed_1 = 8;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ECT_REG_DCCYCLE1: C2RustUnnamed_2 = 2468;
pub const ECT_REG_DCCYCLE0: C2RustUnnamed_2 = 2464;
pub const ECT_REG_DCSTART0: C2RustUnnamed_2 = 2448;
pub const ECT_REG_DCSYNCACT: C2RustUnnamed_2 = 2433;
pub const ECT_REG_DCCUC: C2RustUnnamed_2 = 2432;
pub const ECT_REG_DCTIMEFILT: C2RustUnnamed_2 = 2356;
pub const ECT_REG_DCSPEEDCNT: C2RustUnnamed_2 = 2352;
pub const ECT_REG_DCSYSDIFF: C2RustUnnamed_2 = 2348;
pub const ECT_REG_DCSYSDELAY: C2RustUnnamed_2 = 2344;
pub const ECT_REG_DCSYSOFFSET: C2RustUnnamed_2 = 2336;
pub const ECT_REG_DCSOF: C2RustUnnamed_2 = 2328;
pub const ECT_REG_DCSYSTIME: C2RustUnnamed_2 = 2320;
pub const ECT_REG_DCTIME3: C2RustUnnamed_2 = 2316;
pub const ECT_REG_DCTIME2: C2RustUnnamed_2 = 2312;
pub const ECT_REG_DCTIME1: C2RustUnnamed_2 = 2308;
pub const ECT_REG_DCTIME0: C2RustUnnamed_2 = 2304;
pub const ECT_REG_SM1CONTR: C2RustUnnamed_2 = 2063;
pub const ECT_REG_SM1ACT: C2RustUnnamed_2 = 2062;
pub const ECT_REG_SM1STAT: C2RustUnnamed_2 = 2061;
pub const ECT_REG_SM0STAT: C2RustUnnamed_2 = 2053;
pub const ECT_REG_SM3: C2RustUnnamed_2 = 2072;
pub const ECT_REG_SM2: C2RustUnnamed_2 = 2064;
pub const ECT_REG_SM1: C2RustUnnamed_2 = 2056;
pub const ECT_REG_SM0: C2RustUnnamed_2 = 2048;
pub const ECT_REG_FMMU3: C2RustUnnamed_2 = 1584;
pub const ECT_REG_FMMU2: C2RustUnnamed_2 = 1568;
pub const ECT_REG_FMMU1: C2RustUnnamed_2 = 1552;
pub const ECT_REG_FMMU0: C2RustUnnamed_2 = 1536;
pub const ECT_REG_EEPDAT: C2RustUnnamed_2 = 1288;
pub const ECT_REG_EEPADR: C2RustUnnamed_2 = 1284;
pub const ECT_REG_EEPSTAT: C2RustUnnamed_2 = 1282;
pub const ECT_REG_EEPCTL: C2RustUnnamed_2 = 1282;
pub const ECT_REG_EEPCFG: C2RustUnnamed_2 = 1280;
pub const ECT_REG_WDCNT: C2RustUnnamed_2 = 1090;
pub const ECT_REG_LLCNT: C2RustUnnamed_2 = 784;
pub const ECT_REG_PECODE: C2RustUnnamed_2 = 782;
pub const ECT_REG_PECNT: C2RustUnnamed_2 = 781;
pub const ECT_REG_EPUECNT: C2RustUnnamed_2 = 780;
pub const ECT_REG_FRXERR: C2RustUnnamed_2 = 776;
pub const ECT_REG_RXERR: C2RustUnnamed_2 = 768;
pub const ECT_REG_IRQMASK: C2RustUnnamed_2 = 512;
pub const ECT_REG_PDICTL: C2RustUnnamed_2 = 320;
pub const ECT_REG_ALSTATCODE: C2RustUnnamed_2 = 308;
pub const ECT_REG_ALSTAT: C2RustUnnamed_2 = 304;
pub const ECT_REG_ALCTL: C2RustUnnamed_2 = 288;
pub const ECT_REG_DLSTAT: C2RustUnnamed_2 = 272;
pub const ECT_REG_DLALIAS: C2RustUnnamed_2 = 259;
pub const ECT_REG_DLPORT: C2RustUnnamed_2 = 257;
pub const ECT_REG_DLCTL: C2RustUnnamed_2 = 256;
pub const ECT_REG_ALIAS: C2RustUnnamed_2 = 18;
pub const ECT_REG_STADR: C2RustUnnamed_2 = 16;
pub const ECT_REG_ESCSUP: C2RustUnnamed_2 = 8;
pub const ECT_REG_PORTDES: C2RustUnnamed_2 = 7;
pub const ECT_REG_TYPE: C2RustUnnamed_2 = 0;
pub type ec_err_type = libc::c_uint;
pub const EC_ERR_TYPE_EOE_INVALID_RX_DATA: ec_err_type = 11;
pub const EC_ERR_TYPE_FOE_FILE_NOTFOUND: ec_err_type = 10;
pub const EC_ERR_TYPE_MBX_ERROR: ec_err_type = 9;
pub const EC_ERR_TYPE_SOE_ERROR: ec_err_type = 8;
pub const EC_ERR_TYPE_FOE_PACKETNUMBER: ec_err_type = 7;
pub const EC_ERR_TYPE_FOE_BUF2SMALL: ec_err_type = 6;
pub const EC_ERR_TYPE_FOE_ERROR: ec_err_type = 5;
pub const EC_ERR_TYPE_SDOINFO_ERROR: ec_err_type = 4;
pub const EC_ERR_TYPE_PACKET_ERROR: ec_err_type = 3;
pub const EC_ERR_TYPE_EMERGENCY: ec_err_type = 1;
pub const EC_ERR_TYPE_SDO_ERROR: ec_err_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_errort {
    pub Time: ec_timet,
    pub Signal: boolean,
    pub Slave: uint16,
    pub Index: uint16,
    pub SubIdx: uint8,
    pub Etype: ec_err_type,
    pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub AbortCode: int32,
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub ErrorCode: uint16,
    pub ErrorReg: uint8,
    pub b1: uint8,
    pub w1: uint16,
    pub w2: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_stackT {
    pub sock: *mut libc::c_int,
    pub txbuf: *mut [ec_bufT; 16],
    pub txbuflength: *mut [libc::c_int; 16],
    pub tempbuf: *mut ec_bufT,
    pub rxbuf: *mut [ec_bufT; 16],
    pub rxbufstat: *mut [libc::c_int; 16],
    pub rxsa: *mut [libc::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecx_redportt {
    pub stack: ec_stackT,
    pub sockhandle: libc::c_int,
    pub rxbuf: [ec_bufT; 16],
    pub rxbufstat: [libc::c_int; 16],
    pub rxsa: [libc::c_int; 16],
    pub tempinbuf: ec_bufT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecx_portt {
    pub stack: ec_stackT,
    pub sockhandle: libc::c_int,
    pub rxbuf: [ec_bufT; 16],
    pub rxbufstat: [libc::c_int; 16],
    pub rxsa: [libc::c_int; 16],
    pub tempinbuf: ec_bufT,
    pub tempinbufs: libc::c_int,
    pub txbuf: [ec_bufT; 16],
    pub txbuflength: [libc::c_int; 16],
    pub txbuf2: ec_bufT,
    pub txbuflength2: libc::c_int,
    pub lastidx: uint8,
    pub redstate: libc::c_int,
    pub redport: *mut ecx_redportt,
    pub getindex_mutex: pthread_mutex_t,
    pub tx_mutex: pthread_mutex_t,
    pub rx_mutex: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_sm {
    pub StartAddr: uint16,
    pub SMlength: uint16,
    pub SMflags: uint32,
}
pub type ec_smt = ec_sm;
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub FOEhook: Option<unsafe extern "C" fn(_: uint16, _: libc::c_int,
                                             _: libc::c_int) -> libc::c_int>,
    pub EOEhook: Option<unsafe extern "C" fn(_: *mut ecx_contextt, _: uint16,
                                             _: *mut libc::c_void)
                            -> libc::c_int>,
    pub manualstatechange: libc::c_int,
    pub userdata: *mut libc::c_void,
}
pub type ecx_contextt = ecx_context;
pub type ec_eepromFMMUt = ec_eepromFMMU;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_eepromFMMU {
    pub Startpos: uint16,
    pub nFMMU: uint8,
    pub FMMU0: uint8,
    pub FMMU1: uint8,
    pub FMMU2: uint8,
    pub FMMU3: uint8,
}
pub type ec_eepromSMt = ec_eepromSM;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_PDOdesc {
    pub n: uint8,
    pub nu1: uint8,
    pub PDO: [uint32; 256],
}
pub type ec_PDOassignt = ec_PDOassign;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_PDOassign {
    pub n: uint8,
    pub nu1: uint8,
    pub index: [uint16; 256],
}
pub type ec_SMcommtypet = ec_SMcommtype;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_SMcommtype {
    pub n: uint8,
    pub nu1: uint8,
    pub SMtype: [uint8; 8],
}
pub type ec_idxstackT = ec_idxstack;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_idxstack {
    pub pushed: uint8,
    pub pulled: uint8,
    pub idx: [uint8; 16],
    pub data: [*mut libc::c_void; 16],
    pub length: [uint16; 16],
    pub dcoffset: [uint16; 16],
}
pub type ec_eringt = ec_ering;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_ering {
    pub head: int16,
    pub tail: int16,
    pub Error: [ec_errort; 65],
}
pub type ec_groupt = ec_group;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub PO2SOconfigx: Option<unsafe extern "C" fn(_: *mut ecx_contextt,
                                                  _: uint16) -> libc::c_int>,
    pub name: [libc::c_char; 41],
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_configlist_t {
    pub man: uint32,
    pub id: uint32,
    pub name: [libc::c_char; 41],
    pub Dtype: uint8,
    pub Ibits: uint16,
    pub Obits: uint16,
    pub SM2a: uint16,
    pub SM2f: uint32,
    pub SM3a: uint16,
    pub SM3f: uint32,
    pub FM0ac: uint8,
    pub FM1ac: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecx_mapt_t {
    pub thread_n: libc::c_int,
    pub running: libc::c_int,
    pub context: *mut ecx_contextt,
    pub slave: uint16,
}
#[no_mangle]
pub static mut ecx_mapt: [ecx_mapt_t; 1] =
    [ecx_mapt_t{thread_n: 0,
                running: 0,
                context: 0 as *const ecx_contextt as *mut ecx_contextt,
                slave: 0,}; 1];
#[no_mangle]
pub static mut ec_configlist: [ec_configlist_t; 24] =
    unsafe {
        [{
             let mut init =
                 ec_configlist_t{man: 0 as libc::c_int as uint32,
                                 id: 0 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 0 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x44c2c52 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EK1100\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 1 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x3ea3052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL1002\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 2 as libc::c_int as uint8,
                                 Ibits: 2 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x3ec3052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL1004\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 2 as libc::c_int as uint8,
                                 Ibits: 4 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x3f43052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL1012\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 2 as libc::c_int as uint8,
                                 Ibits: 2 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x3f63052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL1014\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 2 as libc::c_int as uint8,
                                 Ibits: 4 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x3fa3052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 2 as libc::c_int as uint8,
                                 Ibits: 8 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x7d23052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL2002\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 3 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 2 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x7d43052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL2004\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 3 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 4 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x7d83052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL2008\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 3 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 8 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x7f03052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL2032\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 6 as libc::c_int as uint8,
                                 Ibits: 2 as libc::c_int as uint16,
                                 Obits: 2 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0xc1e3052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL3102\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 4 as libc::c_int as uint8,
                                 Ibits: 48 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x24 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x10020 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 1 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0xc283052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL3112\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 4 as libc::c_int as uint8,
                                 Ibits: 48 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x24 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x10020 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 1 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0xc323052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL3122\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 4 as libc::c_int as uint8,
                                 Ibits: 48 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x24 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x10020 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 1 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0xc463052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL3142\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 4 as libc::c_int as uint8,
                                 Ibits: 48 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x24 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x10020 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 1 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0xc503052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL3152\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 4 as libc::c_int as uint8,
                                 Ibits: 48 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x24 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x10020 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 1 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0xc5a3052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL3162\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 4 as libc::c_int as uint8,
                                 Ibits: 48 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x24 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x10020 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 1 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0xfc03052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL4032\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 5 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 32 as libc::c_int as uint16,
                                 SM2a: 0x1100 as libc::c_int as uint16,
                                 SM2f: 0x10024 as libc::c_int as uint32,
                                 SM3a: 0x1180 as libc::c_int as uint16,
                                 SM3f: 0x22 as libc::c_int as uint32,
                                 FM0ac: 1 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x10063052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL4102\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 5 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 32 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x10024 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x22 as libc::c_int as uint32,
                                 FM0ac: 1 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x10103052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL4112\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 5 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 32 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x10024 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x22 as libc::c_int as uint32,
                                 FM0ac: 1 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x101a3052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL4122\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 5 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 32 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x10024 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x22 as libc::c_int as uint32,
                                 FM0ac: 1 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x10243052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL4132\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 5 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 32 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x10024 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x22 as libc::c_int as uint32,
                                 FM0ac: 1 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0x2 as libc::c_int as uint32,
                                 id: 0x13ed3052 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"EL5101\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 7 as libc::c_int as uint8,
                                 Ibits: 40 as libc::c_int as uint16,
                                 Obits: 24 as libc::c_int as uint16,
                                 SM2a: 0x1000 as libc::c_int as uint16,
                                 SM2f: 0x10024 as libc::c_int as uint32,
                                 SM3a: 0x1100 as libc::c_int as uint16,
                                 SM3f: 0x10020 as libc::c_int as uint32,
                                 FM0ac: 1 as libc::c_int as uint8,
                                 FM1ac: 1 as libc::c_int as uint8,};
             init
         },
         {
             let mut init =
                 ec_configlist_t{man: 0xffffffff as libc::c_uint,
                                 id: 0 as libc::c_int as uint32,
                                 name:
                                     *::core::mem::transmute::<&[u8; 41],
                                                               &mut [libc::c_char; 41]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 Dtype: 0 as libc::c_int as uint8,
                                 Ibits: 0 as libc::c_int as uint16,
                                 Obits: 0 as libc::c_int as uint16,
                                 SM2a: 0 as libc::c_int as uint16,
                                 SM2f: 0 as libc::c_int as uint32,
                                 SM3a: 0 as libc::c_int as uint16,
                                 SM3f: 0 as libc::c_int as uint32,
                                 FM0ac: 0 as libc::c_int as uint8,
                                 FM1ac: 0 as libc::c_int as uint8,};
             init
         }]
    };
/* * standard SM0 flags configuration for mailbox slaves */
/* * standard SM1 flags configuration for mailbox slaves */
/* * standard SM0 flags configuration for digital output slaves */
/* * Find slave in standard configuration list ec_configlist[]
 *
 * @param[in] man      = manufacturer
 * @param[in] id       = ID
 * @return index in ec_configlist[] when found, otherwise 0
 */
#[no_mangle]
pub unsafe extern "C" fn ec_findconfig(mut man: uint32, mut id: uint32)
 -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    loop  {
        i += 1;
        if !(ec_configlist[i as usize].man != 0xffffffff as libc::c_uint &&
                 (ec_configlist[i as usize].man != man ||
                      ec_configlist[i as usize].id != id)) {
            break ;
        }
    }
    if ec_configlist[i as usize].man == 0xffffffff as libc::c_uint {
        i = 0 as libc::c_int
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn ecx_init_context(mut context: *mut ecx_contextt) {
    let mut lp: libc::c_int = 0;
    *(*context).slavecount = 0 as libc::c_int;
    /* clean ec_slave array */
    memset((*context).slavelist as *mut libc::c_void, 0 as libc::c_int,
           (::core::mem::size_of::<ec_slavet>() as
                libc::c_ulong).wrapping_mul((*context).maxslave as
                                                libc::c_ulong));
    memset((*context).grouplist as *mut libc::c_void, 0 as libc::c_int,
           (::core::mem::size_of::<ec_groupt>() as
                libc::c_ulong).wrapping_mul((*context).maxgroup as
                                                libc::c_ulong));
    /* clear slave eeprom cache, does not actually read any eeprom */
    ecx_siigetbyte(context, 0 as libc::c_int as uint16,
                   ((128 as libc::c_int) << 5 as libc::c_int) as uint16);
    lp = 0 as libc::c_int;
    while lp < (*context).maxgroup {
        /* default start address per group entry */
        (*(*context).grouplist.offset(lp as isize)).logstartaddr =
            (lp << 16 as libc::c_int) as uint32;
        lp += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn ecx_detect_slaves(mut context: *mut ecx_contextt)
 -> libc::c_int {
    let mut b: uint8 = 0;
    let mut w: uint16 = 0;
    let mut wkc: libc::c_int = 0;
    /* make special pre-init register writes to enable MAC[1] local administered bit *
    * setting for old netX100 slaves */
    b = 0 as libc::c_int as uint8; /* Ignore Alias register */
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_DLALIAS as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int *
                3 as libc::c_int); /* Reset all slaves to Init */
    b = (EC_STATE_INIT as libc::c_int | EC_STATE_ACK as libc::c_int) as uint8;
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_ALCTL as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    /* netX100 should now be happy */
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_ALCTL as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int *
                3 as libc::c_int); /* Reset all slaves to Init */
    wkc =
        ecx_BRD((*context).port, 0 as libc::c_int as uint16,
                ECT_REG_TYPE as libc::c_int as uint16,
                ::core::mem::size_of::<uint16>() as libc::c_ulong as uint16,
                &mut w as *mut uint16 as *mut libc::c_void,
                20000 as libc::c_int); /* detect number of slaves */
    if wkc > 0 as libc::c_int {
        /* this is strictly "less than" since the master is "slave 0" */
        if wkc < 200 as libc::c_int {
            *(*context).slavecount = wkc
        } else { return -(4 as libc::c_int) }
    } /* deact loop manual */
    return wkc; /* set IRQ mask */
}
unsafe extern "C" fn ecx_set_slaves_to_default(mut context:
                                                   *mut ecx_contextt) {
    let mut b: uint8 = 0; /* reset CRC counters */
    let mut w: uint16 = 0; /* reset FMMU's */
    let mut zbuf: [uint8; 64] = [0; 64]; /* reset SyncM */
    memset(&mut zbuf as *mut [uint8; 64] as *mut libc::c_void,
           0 as libc::c_int,
           ::core::mem::size_of::<[uint8; 64]>() as
               libc::c_ulong); /* reset activation register */
    b = 0 as libc::c_int as uint8; /* reset system time+ofs */
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_DLPORT as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int); /* DC speedstart */
    w = 0x4 as libc::c_int as uint16; /* DC filt expr */
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_IRQMASK as libc::c_int as uint16,
            ::core::mem::size_of::<uint16>() as libc::c_ulong as uint16,
            &mut w as *mut uint16 as *mut libc::c_void,
            2000 as libc::c_int *
                3 as libc::c_int); /* Ignore Alias register */
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_RXERR as libc::c_int as uint16,
            8 as libc::c_int as uint16,
            &mut zbuf as *mut [uint8; 64] as *mut libc::c_void,
            2000 as libc::c_int *
                3 as libc::c_int); /* Reset all slaves to Init */
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_FMMU0 as libc::c_int as uint16,
            (16 as libc::c_int * 3 as libc::c_int) as uint16,
            &mut zbuf as *mut [uint8; 64] as *mut libc::c_void,
            2000 as libc::c_int *
                3 as libc::c_int); /* force Eeprom from PDI */
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_SM0 as libc::c_int as uint16,
            (8 as libc::c_int * 4 as libc::c_int) as uint16,
            &mut zbuf as *mut [uint8; 64] as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    b = 0 as libc::c_int as uint8;
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_DCSYNCACT as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_DCSYSTIME as libc::c_int as uint16,
            4 as libc::c_int as uint16,
            &mut zbuf as *mut [uint8; 64] as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    w = 0x1000 as libc::c_int as uint16;
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_DCSPEEDCNT as libc::c_int as uint16,
            ::core::mem::size_of::<uint16>() as libc::c_ulong as uint16,
            &mut w as *mut uint16 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    w = 0xc00 as libc::c_int as uint16;
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_DCTIMEFILT as libc::c_int as uint16,
            ::core::mem::size_of::<uint16>() as libc::c_ulong as uint16,
            &mut w as *mut uint16 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    b = 0 as libc::c_int as uint8;
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_DLALIAS as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    b = (EC_STATE_INIT as libc::c_int | EC_STATE_ACK as libc::c_int) as uint8;
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_ALCTL as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    b = 2 as libc::c_int as uint8;
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_EEPCFG as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    b = 0 as libc::c_int as uint8;
    ecx_BWR((*context).port, 0 as libc::c_int as uint16,
            ECT_REG_EEPCFG as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut b as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int * 3 as libc::c_int);
    /* set Eeprom to master */
}
unsafe extern "C" fn ecx_config_from_table(mut context: *mut ecx_contextt,
                                           mut slave: uint16) -> libc::c_int {
    let mut cindex: libc::c_int = 0;
    let mut csl: *mut ec_slavet = 0 as *mut ec_slavet;
    csl = &mut *(*context).slavelist.offset(slave as isize) as *mut ec_slavet;
    cindex = ec_findconfig((*csl).eep_man, (*csl).eep_id);
    (*csl).configindex = cindex as uint16;
    /* slave found in configuration table ? */
    if cindex != 0 {
        (*csl).Dtype = ec_configlist[cindex as usize].Dtype as uint16;
        strcpy((*csl).name.as_mut_ptr(),
               ec_configlist[cindex as usize].name.as_ptr());
        (*csl).Ibits = ec_configlist[cindex as usize].Ibits;
        (*csl).Obits = ec_configlist[cindex as usize].Obits;
        if (*csl).Obits != 0 { (*csl).FMMU0func = 1 as libc::c_int as uint8 }
        if (*csl).Ibits != 0 { (*csl).FMMU1func = 2 as libc::c_int as uint8 }
        (*csl).FMMU[0 as libc::c_int as usize].FMMUactive =
            ec_configlist[cindex as usize].FM0ac;
        (*csl).FMMU[1 as libc::c_int as usize].FMMUactive =
            ec_configlist[cindex as usize].FM1ac;
        (*csl).SM[2 as libc::c_int as usize].StartAddr =
            ec_configlist[cindex as usize].SM2a;
        (*csl).SM[2 as libc::c_int as usize].SMflags =
            ec_configlist[cindex as usize].SM2f;
        /* simple (no mailbox) output slave found ? */
        if (*csl).Obits as libc::c_int != 0 &&
               (*csl).SM[2 as libc::c_int as usize].StartAddr == 0 {
            (*csl).SM[0 as libc::c_int as usize].StartAddr =
                0xf00 as libc::c_int as uint16;
            (*csl).SM[0 as libc::c_int as usize].SMlength =
                (((*csl).Obits as libc::c_int + 7 as libc::c_int) /
                     8 as libc::c_int) as uint16;
            (*csl).SM[0 as libc::c_int as usize].SMflags =
                0x10044 as libc::c_int as uint32;
            (*csl).FMMU[0 as libc::c_int as usize].FMMUactive =
                1 as libc::c_int as uint8;
            (*csl).FMMU[0 as libc::c_int as usize].FMMUtype =
                2 as libc::c_int as uint8;
            (*csl).SMtype[0 as libc::c_int as usize] =
                3 as libc::c_int as uint8
        } else {
            /* complex output slave */
            (*csl).SM[2 as libc::c_int as usize].SMlength =
                (((*csl).Obits as libc::c_int + 7 as libc::c_int) /
                     8 as libc::c_int) as uint16;
            (*csl).SMtype[2 as libc::c_int as usize] =
                3 as libc::c_int as uint8
        }
        (*csl).SM[3 as libc::c_int as usize].StartAddr =
            ec_configlist[cindex as usize].SM3a;
        (*csl).SM[3 as libc::c_int as usize].SMflags =
            ec_configlist[cindex as usize].SM3f;
        /* simple (no mailbox) input slave found ? */
        if (*csl).Ibits as libc::c_int != 0 &&
               (*csl).SM[3 as libc::c_int as usize].StartAddr == 0 {
            (*csl).SM[1 as libc::c_int as usize].StartAddr =
                0x1000 as libc::c_int as uint16;
            (*csl).SM[1 as libc::c_int as usize].SMlength =
                (((*csl).Ibits as libc::c_int + 7 as libc::c_int) /
                     8 as libc::c_int) as uint16;
            (*csl).SM[1 as libc::c_int as usize].SMflags =
                0 as libc::c_int as uint32;
            (*csl).FMMU[1 as libc::c_int as usize].FMMUactive =
                1 as libc::c_int as uint8;
            (*csl).FMMU[1 as libc::c_int as usize].FMMUtype =
                1 as libc::c_int as uint8;
            (*csl).SMtype[1 as libc::c_int as usize] =
                4 as libc::c_int as uint8
        } else {
            /* complex input slave */
            (*csl).SM[3 as libc::c_int as usize].SMlength =
                (((*csl).Ibits as libc::c_int + 7 as libc::c_int) /
                     8 as libc::c_int) as uint16;
            (*csl).SMtype[3 as libc::c_int as usize] =
                4 as libc::c_int as uint8
        }
    }
    return cindex;
}
/* If slave has SII and same slave ID done before, use previous data.
 * This is safe because SII is constant for same slave ID.
 */
unsafe extern "C" fn ecx_lookup_prev_sii(mut context: *mut ecx_contextt,
                                         mut slave: uint16) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nSM: libc::c_int = 0;
    if slave as libc::c_int > 1 as libc::c_int &&
           *(*context).slavecount > 0 as libc::c_int {
        i = 1 as libc::c_int;
        while ((*(*context).slavelist.offset(i as isize)).eep_man !=
                   (*(*context).slavelist.offset(slave as isize)).eep_man ||
                   (*(*context).slavelist.offset(i as isize)).eep_id !=
                       (*(*context).slavelist.offset(slave as isize)).eep_id
                   ||
                   (*(*context).slavelist.offset(i as isize)).eep_rev !=
                       (*(*context).slavelist.offset(slave as isize)).eep_rev)
                  && i < slave as libc::c_int {
            i += 1
        }
        if i < slave as libc::c_int {
            (*(*context).slavelist.offset(slave as isize)).CoEdetails =
                (*(*context).slavelist.offset(i as isize)).CoEdetails;
            (*(*context).slavelist.offset(slave as isize)).FoEdetails =
                (*(*context).slavelist.offset(i as isize)).FoEdetails;
            (*(*context).slavelist.offset(slave as isize)).EoEdetails =
                (*(*context).slavelist.offset(i as isize)).EoEdetails;
            (*(*context).slavelist.offset(slave as isize)).SoEdetails =
                (*(*context).slavelist.offset(i as isize)).SoEdetails;
            if (*(*context).slavelist.offset(i as isize)).blockLRW as
                   libc::c_int > 0 as libc::c_int {
                (*(*context).slavelist.offset(slave as isize)).blockLRW =
                    1 as libc::c_int as uint8;
                let ref mut fresh0 =
                    (*(*context).slavelist.offset(0 as libc::c_int as
                                                      isize)).blockLRW;
                *fresh0 = (*fresh0).wrapping_add(1)
            }
            (*(*context).slavelist.offset(slave as isize)).Ebuscurrent =
                (*(*context).slavelist.offset(i as isize)).Ebuscurrent;
            let ref mut fresh1 =
                (*(*context).slavelist.offset(0 as libc::c_int as
                                                  isize)).Ebuscurrent;
            *fresh1 =
                (*fresh1 as libc::c_int +
                     (*(*context).slavelist.offset(slave as
                                                       isize)).Ebuscurrent as
                         libc::c_int) as int16;
            memcpy((*(*context).slavelist.offset(slave as
                                                     isize)).name.as_mut_ptr()
                       as *mut libc::c_void,
                   (*(*context).slavelist.offset(i as
                                                     isize)).name.as_mut_ptr()
                       as *const libc::c_void,
                   (40 as libc::c_int + 1 as libc::c_int) as libc::c_ulong);
            nSM = 0 as libc::c_int;
            while nSM < 8 as libc::c_int {
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[nSM as
                                                                 usize].StartAddr
                    =
                    (*(*context).slavelist.offset(i as
                                                      isize)).SM[nSM as
                                                                     usize].StartAddr;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[nSM as
                                                                 usize].SMlength
                    =
                    (*(*context).slavelist.offset(i as
                                                      isize)).SM[nSM as
                                                                     usize].SMlength;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[nSM as
                                                                 usize].SMflags
                    =
                    (*(*context).slavelist.offset(i as
                                                      isize)).SM[nSM as
                                                                     usize].SMflags;
                nSM += 1
            }
            (*(*context).slavelist.offset(slave as isize)).FMMU0func =
                (*(*context).slavelist.offset(i as isize)).FMMU0func;
            (*(*context).slavelist.offset(slave as isize)).FMMU1func =
                (*(*context).slavelist.offset(i as isize)).FMMU1func;
            (*(*context).slavelist.offset(slave as isize)).FMMU2func =
                (*(*context).slavelist.offset(i as isize)).FMMU2func;
            (*(*context).slavelist.offset(slave as isize)).FMMU3func =
                (*(*context).slavelist.offset(i as isize)).FMMU3func;
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* * Enumerate and init all slaves.
 *
 * @param[in] context      = context struct
 * @param[in] usetable     = TRUE when using configtable to init slaves, FALSE otherwise
 * @return Workcounter of slave discover datagram = number of slaves found
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_config_init(mut context: *mut ecx_contextt,
                                         mut usetable: uint8) -> libc::c_int {
    let mut slave: uint16 = 0;
    let mut ADPh: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut ssigen: uint16 = 0;
    let mut topology: uint16 = 0;
    let mut estat: uint16 = 0;
    let mut topoc: int16 = 0;
    let mut slavec: int16 = 0;
    let mut aliasadr: int16 = 0;
    let mut b: uint8 = 0;
    let mut h: uint8 = 0;
    let mut SMc: uint8 = 0;
    let mut eedat: uint32 = 0;
    let mut wkc: libc::c_int = 0;
    let mut cindex: libc::c_int = 0;
    let mut nSM: libc::c_int = 0;
    let mut val16: uint16 = 0;
    ecx_init_context(context);
    wkc = ecx_detect_slaves(context);
    if wkc > 0 as libc::c_int {
        ecx_set_slaves_to_default(context);
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            ADPh = (1 as libc::c_int - slave as libc::c_int) as uint16;
            /* Manuf */
            val16 =
                ecx_APRDw((*context).port, ADPh,
                          ECT_REG_PDICTL as libc::c_int as uint16,
                          2000 as libc::c_int *
                              3 as
                                  libc::c_int); /* read interface type of slave */
            (*(*context).slavelist.offset(slave as isize)).Itype = val16;
            ecx_APWRw((*context).port, ADPh,
                      ECT_REG_STADR as libc::c_int as uint16,
                      (slave as libc::c_int + 0x1000 as libc::c_int) as
                          uint16, 2000 as libc::c_int * 3 as libc::c_int);
            if slave as libc::c_int == 1 as libc::c_int {
                b = 1 as libc::c_int as uint8
                /* a node offset is used to improve readability of network frames */
         /* this has no impact on the number of addressable slaves (auto wrap around) */
                /* set node address of slave */
                /* kill non ecat frames for first slave */
            } else {
                b = 0 as libc::c_int as uint8
                /* pass all frames for following slaves */
            } /* set non ecat frame behaviour */
            ecx_APWRw((*context).port, ADPh,
                      ECT_REG_DLCTL as libc::c_int as uint16, b as uint16,
                      2000 as libc::c_int * 3 as libc::c_int);
            configadr =
                ecx_APRDw((*context).port, ADPh,
                          ECT_REG_STADR as libc::c_int as uint16,
                          2000 as libc::c_int * 3 as libc::c_int);
            configadr = configadr;
            (*(*context).slavelist.offset(slave as isize)).configadr =
                configadr;
            ecx_FPRD((*context).port, configadr,
                     ECT_REG_ALIAS as libc::c_int as uint16,
                     ::core::mem::size_of::<int16>() as libc::c_ulong as
                         uint16,
                     &mut aliasadr as *mut int16 as *mut libc::c_void,
                     2000 as libc::c_int * 3 as libc::c_int);
            (*(*context).slavelist.offset(slave as isize)).aliasadr =
                aliasadr as uint16;
            ecx_FPRD((*context).port, configadr,
                     ECT_REG_EEPSTAT as libc::c_int as uint16,
                     ::core::mem::size_of::<uint16>() as libc::c_ulong as
                         uint16,
                     &mut estat as *mut uint16 as *mut libc::c_void,
                     2000 as libc::c_int * 3 as libc::c_int);
            estat = estat;
            if estat as libc::c_int & 0x40 as libc::c_int != 0 {
                /* check if slave can read 8 byte chunks */
                (*(*context).slavelist.offset(slave as isize)).eep_8byte =
                    1 as libc::c_int as uint8
            } /* Manuf */
            ecx_readeeprom1(context, slave,
                            ECT_SII_MANUF as libc::c_int as uint16);
            slave = slave.wrapping_add(1)
        }
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            eedat = ecx_readeeprom2(context, slave, 20000 as libc::c_int);
            (*(*context).slavelist.offset(slave as isize)).eep_man = eedat;
            ecx_readeeprom1(context, slave,
                            ECT_SII_ID as libc::c_int as uint16);
            slave = slave.wrapping_add(1)
            /* ID */
        } /* ID */
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            eedat = ecx_readeeprom2(context, slave, 20000 as libc::c_int);
            (*(*context).slavelist.offset(slave as isize)).eep_id = eedat;
            ecx_readeeprom1(context, slave,
                            ECT_SII_REV as libc::c_int as uint16);
            slave = slave.wrapping_add(1)
            /* revision */
        } /* revision */
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            eedat = ecx_readeeprom2(context, slave, 20000 as libc::c_int);
            (*(*context).slavelist.offset(slave as isize)).eep_rev = eedat;
            ecx_readeeprom1(context, slave,
                            ECT_SII_RXMBXADR as libc::c_int as uint16);
            slave = slave.wrapping_add(1)
            /* write mailbox address + mailboxsize */
        } /* write mailbox address and mailboxsize */
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            eedat = ecx_readeeprom2(context, slave, 20000 as libc::c_int);
            (*(*context).slavelist.offset(slave as isize)).mbx_wo =
                (eedat & 0xffff as libc::c_int as libc::c_uint) as uint16;
            (*(*context).slavelist.offset(slave as isize)).mbx_l =
                (eedat >> 16 as libc::c_int) as uint16;
            if (*(*context).slavelist.offset(slave as isize)).mbx_l as
                   libc::c_int > 0 as libc::c_int {
                ecx_readeeprom1(context, slave,
                                ECT_SII_TXMBXADR as libc::c_int as uint16);
                /* read mailbox offset */
            } /* read mailbox offset */
            slave = slave.wrapping_add(1)
        } /* read mailbox offset */
        slave = 1 as libc::c_int as uint16; /*read mailbox length */
        while slave as libc::c_int <= *(*context).slavecount {
            if (*(*context).slavelist.offset(slave as isize)).mbx_l as
                   libc::c_int > 0 as libc::c_int {
                eedat = ecx_readeeprom2(context, slave, 20000 as libc::c_int);
                (*(*context).slavelist.offset(slave as isize)).mbx_ro =
                    (eedat & 0xffff as libc::c_int as libc::c_uint) as uint16;
                (*(*context).slavelist.offset(slave as isize)).mbx_rl =
                    (eedat >> 16 as libc::c_int) as uint16;
                if (*(*context).slavelist.offset(slave as isize)).mbx_rl as
                       libc::c_int == 0 as libc::c_int {
                    (*(*context).slavelist.offset(slave as isize)).mbx_rl =
                        (*(*context).slavelist.offset(slave as isize)).mbx_l
                }
                ecx_readeeprom1(context, slave,
                                ECT_SII_MBXPROTO as libc::c_int as uint16);
            }
            configadr =
                (*(*context).slavelist.offset(slave as isize)).configadr;
            val16 =
                ecx_FPRDw((*context).port, configadr,
                          ECT_REG_ESCSUP as libc::c_int as uint16,
                          2000 as libc::c_int * 3 as libc::c_int);
            if val16 as libc::c_int & 0x4 as libc::c_int > 0 as libc::c_int {
                /* Support DC? */
                (*(*context).slavelist.offset(slave as isize)).hasdc =
                    1 as libc::c_int as boolean
            } else {
                (*(*context).slavelist.offset(slave as isize)).hasdc =
                    0 as libc::c_int as boolean
            } /* extract topology from DL status */
            topology =
                ecx_FPRDw((*context).port, configadr,
                          ECT_REG_DLSTAT as libc::c_int as uint16,
                          2000 as libc::c_int * 3 as libc::c_int);
            topology = topology;
            h = 0 as libc::c_int as uint8;
            b = 0 as libc::c_int as uint8;
            if topology as libc::c_int & 0x300 as libc::c_int ==
                   0x200 as libc::c_int {
                /* port0 open and communication established */
                h = h.wrapping_add(1);
                b = (b as libc::c_int | 0x1 as libc::c_int) as uint8
            }
            if topology as libc::c_int & 0xc00 as libc::c_int ==
                   0x800 as libc::c_int {
                /* port1 open and communication established */
                h = h.wrapping_add(1);
                b = (b as libc::c_int | 0x2 as libc::c_int) as uint8
            }
            if topology as libc::c_int & 0x3000 as libc::c_int ==
                   0x2000 as libc::c_int {
                /* port2 open and communication established */
                h = h.wrapping_add(1);
                b = (b as libc::c_int | 0x4 as libc::c_int) as uint8
            }
            if topology as libc::c_int & 0xc000 as libc::c_int ==
                   0x8000 as libc::c_int {
                /* port3 open and communication established */
                h = h.wrapping_add(1);
                b = (b as libc::c_int | 0x8 as libc::c_int) as uint8
            }
            /* ptype = Physical type*/
            val16 =
                ecx_FPRDw((*context).port, configadr,
                          ECT_REG_PORTDES as libc::c_int as uint16,
                          2000 as libc::c_int * 3 as libc::c_int);
            (*(*context).slavelist.offset(slave as isize)).ptype =
                (val16 as libc::c_int & 0xff as libc::c_int) as uint8;
            (*(*context).slavelist.offset(slave as isize)).topology = h;
            (*(*context).slavelist.offset(slave as isize)).activeports = b;
            /* 0=no links, not possible             */
         /* 1=1 link  , end of line              */
         /* 2=2 links , one before and one after */
         /* 3=3 links , split point              */
         /* 4=4 links , cross point              */
         /* search for parent */
            (*(*context).slavelist.offset(slave as isize)).parent =
                0 as libc::c_int as uint16; /* parent is master */
            if slave as libc::c_int > 1 as libc::c_int {
                topoc = 0 as libc::c_int as int16;
                slavec = (slave as libc::c_int - 1 as libc::c_int) as int16;
                loop  {
                    topology =
                        (*(*context).slavelist.offset(slavec as
                                                          isize)).topology as
                            uint16;
                    if topology as libc::c_int == 1 as libc::c_int {
                        topoc -= 1
                        /* endpoint found */
                    }
                    if topology as libc::c_int == 3 as libc::c_int {
                        topoc += 1
                        /* split found */
                    }
                    if topology as libc::c_int == 4 as libc::c_int {
                        topoc =
                            (topoc as libc::c_int + 2 as libc::c_int) as int16
                        /* cross found */
                    }
                    if topoc as libc::c_int >= 0 as libc::c_int &&
                           topology as libc::c_int > 1 as libc::c_int ||
                           slavec as libc::c_int == 1 as libc::c_int {
                        /* parent found */
                        (*(*context).slavelist.offset(slave as isize)).parent
                            = slavec as uint16; //* check state change Init */
                        slavec = 1 as libc::c_int as int16
                    }
                    slavec -= 1;
                    if !(slavec as libc::c_int > 0 as libc::c_int) { break ; }
                }
            }
            ecx_statecheck(context, slave,
                           EC_STATE_INIT as libc::c_int as uint16,
                           2000000 as libc::c_int);
            /* set default mailbox configuration if slave has mailbox */
            if (*(*context).slavelist.offset(slave as isize)).mbx_l as
                   libc::c_int > 0 as libc::c_int {
                (*(*context).slavelist.offset(slave as
                                                  isize)).SMtype[0 as
                                                                     libc::c_int
                                                                     as usize]
                    = 1 as libc::c_int as uint8;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SMtype[1 as
                                                                     libc::c_int
                                                                     as usize]
                    = 2 as libc::c_int as uint8;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SMtype[2 as
                                                                     libc::c_int
                                                                     as usize]
                    = 3 as libc::c_int as uint8;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SMtype[3 as
                                                                     libc::c_int
                                                                     as usize]
                    = 4 as libc::c_int as uint8;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[0 as libc::c_int
                                                                 as
                                                                 usize].StartAddr
                    = (*(*context).slavelist.offset(slave as isize)).mbx_wo;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[0 as libc::c_int
                                                                 as
                                                                 usize].SMlength
                    = (*(*context).slavelist.offset(slave as isize)).mbx_l;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[0 as libc::c_int
                                                                 as
                                                                 usize].SMflags
                    = 0x10026 as libc::c_int as uint32;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[1 as libc::c_int
                                                                 as
                                                                 usize].StartAddr
                    = (*(*context).slavelist.offset(slave as isize)).mbx_ro;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[1 as libc::c_int
                                                                 as
                                                                 usize].SMlength
                    = (*(*context).slavelist.offset(slave as isize)).mbx_rl;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[1 as libc::c_int
                                                                 as
                                                                 usize].SMflags
                    = 0x10022 as libc::c_int as uint32;
                eedat = ecx_readeeprom2(context, slave, 20000 as libc::c_int);
                (*(*context).slavelist.offset(slave as isize)).mbx_proto =
                    eedat as uint16
            }
            cindex = 0 as libc::c_int;
            /* use configuration table ? */
            if usetable as libc::c_int == 1 as libc::c_int {
                cindex = ecx_config_from_table(context, slave)
            }
            /* slave not in configuration table, find out via SII */
            if cindex == 0 && ecx_lookup_prev_sii(context, slave) == 0 {
                ssigen =
                    ecx_siifind(context, slave,
                                ECT_SII_GENERAL as libc::c_int as uint16) as
                        uint16;
                /* SII general section */
                if ssigen != 0 {
                    (*(*context).slavelist.offset(slave as isize)).CoEdetails
                        =
                        ecx_siigetbyte(context, slave,
                                       (ssigen as libc::c_int +
                                            0x7 as libc::c_int) as uint16);
                    (*(*context).slavelist.offset(slave as isize)).FoEdetails
                        =
                        ecx_siigetbyte(context, slave,
                                       (ssigen as libc::c_int +
                                            0x8 as libc::c_int) as uint16);
                    (*(*context).slavelist.offset(slave as isize)).EoEdetails
                        =
                        ecx_siigetbyte(context, slave,
                                       (ssigen as libc::c_int +
                                            0x9 as libc::c_int) as uint16);
                    (*(*context).slavelist.offset(slave as isize)).SoEdetails
                        =
                        ecx_siigetbyte(context, slave,
                                       (ssigen as libc::c_int +
                                            0xa as libc::c_int) as uint16);
                    if ecx_siigetbyte(context, slave,
                                      (ssigen as libc::c_int +
                                           0xd as libc::c_int) as uint16) as
                           libc::c_int & 0x2 as libc::c_int > 0 as libc::c_int
                       {
                        (*(*context).slavelist.offset(slave as
                                                          isize)).blockLRW =
                            1 as libc::c_int as uint8;
                        let ref mut fresh2 =
                            (*(*context).slavelist.offset(0 as libc::c_int as
                                                              isize)).blockLRW;
                        *fresh2 = (*fresh2).wrapping_add(1)
                    }
                    (*(*context).slavelist.offset(slave as isize)).Ebuscurrent
                        =
                        ecx_siigetbyte(context, slave,
                                       (ssigen as libc::c_int +
                                            0xe as libc::c_int) as uint16) as
                            int16;
                    let ref mut fresh3 =
                        (*(*context).slavelist.offset(slave as
                                                          isize)).Ebuscurrent;
                    *fresh3 =
                        (*fresh3 as libc::c_int +
                             ((ecx_siigetbyte(context, slave,
                                              (ssigen as libc::c_int +
                                                   0xf as libc::c_int) as
                                                  uint16) as libc::c_int) <<
                                  8 as libc::c_int)) as int16;
                    let ref mut fresh4 =
                        (*(*context).slavelist.offset(0 as libc::c_int as
                                                          isize)).Ebuscurrent;
                    *fresh4 =
                        (*fresh4 as libc::c_int +
                             (*(*context).slavelist.offset(slave as
                                                               isize)).Ebuscurrent
                                 as libc::c_int) as int16
                }
                /* SII strings section */
                if ecx_siifind(context, slave,
                               ECT_SII_STRING as libc::c_int as uint16) as
                       libc::c_int > 0 as libc::c_int {
                    ecx_siistring(context,
                                  (*(*context).slavelist.offset(slave as
                                                                    isize)).name.as_mut_ptr(),
                                  slave, 1 as libc::c_int as uint16);
                } else {
                    /* no name for slave found, use constructed name */
                    sprintf((*(*context).slavelist.offset(slave as
                                                              isize)).name.as_mut_ptr(),
                            b"? M:%8.8x I:%8.8x\x00" as *const u8 as
                                *const libc::c_char,
                            (*(*context).slavelist.offset(slave as
                                                              isize)).eep_man,
                            (*(*context).slavelist.offset(slave as
                                                              isize)).eep_id);
                }
                /* SII SM section */
                nSM =
                    ecx_siiSM(context, slave, (*context).eepSM) as
                        libc::c_int;
                if nSM > 0 as libc::c_int {
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].StartAddr
                        = (*(*context).eepSM).PhStart;
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].SMlength
                        = (*(*context).eepSM).Plength;
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].SMflags
                        =
                        ((*(*context).eepSM).Creg as libc::c_int +
                             (((*(*context).eepSM).Activate as libc::c_int) <<
                                  16 as libc::c_int)) as uint32;
                    SMc = 1 as libc::c_int as uint8;
                    while (SMc as libc::c_int) < 8 as libc::c_int &&
                              ecx_siiSMnext(context, slave, (*context).eepSM,
                                            SMc as uint16) as libc::c_int != 0
                          {
                        (*(*context).slavelist.offset(slave as
                                                          isize)).SM[SMc as
                                                                         usize].StartAddr
                            = (*(*context).eepSM).PhStart;
                        (*(*context).slavelist.offset(slave as
                                                          isize)).SM[SMc as
                                                                         usize].SMlength
                            = (*(*context).eepSM).Plength;
                        (*(*context).slavelist.offset(slave as
                                                          isize)).SM[SMc as
                                                                         usize].SMflags
                            =
                            ((*(*context).eepSM).Creg as libc::c_int +
                                 (((*(*context).eepSM).Activate as
                                       libc::c_int) << 16 as libc::c_int)) as
                                uint32;
                        SMc = SMc.wrapping_add(1)
                    }
                }
                /* SII FMMU section */
                if ecx_siiFMMU(context, slave, (*context).eepFMMU) != 0 {
                    if (*(*context).eepFMMU).FMMU0 as libc::c_int !=
                           0xff as libc::c_int {
                        (*(*context).slavelist.offset(slave as
                                                          isize)).FMMU0func =
                            (*(*context).eepFMMU).FMMU0
                    }
                    if (*(*context).eepFMMU).FMMU1 as libc::c_int !=
                           0xff as libc::c_int {
                        (*(*context).slavelist.offset(slave as
                                                          isize)).FMMU1func =
                            (*(*context).eepFMMU).FMMU1
                    }
                    if (*(*context).eepFMMU).FMMU2 as libc::c_int !=
                           0xff as libc::c_int {
                        (*(*context).slavelist.offset(slave as
                                                          isize)).FMMU2func =
                            (*(*context).eepFMMU).FMMU2
                    }
                    if (*(*context).eepFMMU).FMMU3 as libc::c_int !=
                           0xff as libc::c_int {
                        (*(*context).slavelist.offset(slave as
                                                          isize)).FMMU3func =
                            (*(*context).eepFMMU).FMMU3
                    }
                }
            }
            if (*(*context).slavelist.offset(slave as isize)).mbx_l as
                   libc::c_int > 0 as libc::c_int {
                if (*(*context).slavelist.offset(slave as
                                                     isize)).SM[0 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].StartAddr
                       as libc::c_int == 0 as libc::c_int {
                    /* should never happen */
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].StartAddr
                        = 0x1000 as libc::c_int as uint16;
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].SMlength
                        = 0x80 as libc::c_int as uint16;
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].SMflags
                        = 0x10026 as libc::c_int as uint32;
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SMtype[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                        = 1 as libc::c_int as uint8
                }
                if (*(*context).slavelist.offset(slave as
                                                     isize)).SM[1 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].StartAddr
                       as libc::c_int == 0 as libc::c_int {
                    /* should never happen */
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[1 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].StartAddr
                        = 0x1080 as libc::c_int as uint16;
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[1 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].SMlength
                        = 0x80 as libc::c_int as uint16;
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[1 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].SMflags
                        = 0x10022 as libc::c_int as uint32;
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SMtype[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                        = 2 as libc::c_int as uint8
                }
                /* program SM0 mailbox in and SM1 mailbox out for slave */
            /* writing both SM in one datagram will solve timing issue in old NETX */
                ecx_FPWR((*context).port, configadr,
                         ECT_REG_SM0 as libc::c_int as uint16,
                         (::core::mem::size_of::<ec_smt>() as
                              libc::c_ulong).wrapping_mul(2 as libc::c_int as
                                                              libc::c_ulong)
                             as uint16,
                         &mut *(*(*context).slavelist.offset(slave as
                                                                 isize)).SM.as_mut_ptr().offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                             as *mut ec_smt as *mut libc::c_void,
                         2000 as libc::c_int * 3 as libc::c_int);
            }
            /* some slaves need eeprom available to PDI in init->preop transition */
            ecx_eeprom2pdi(context, slave);
            /* User may override automatic state change */
            if (*context).manualstatechange == 0 as libc::c_int {
                /* request pre_op for slave */
                ecx_FPWRw((*context).port, configadr,
                          ECT_REG_ALCTL as libc::c_int as uint16,
                          (EC_STATE_PRE_OP as libc::c_int |
                               EC_STATE_ACK as libc::c_int) as uint16,
                          2000 as libc::c_int * 3 as libc::c_int);
                /* set preop status */
            }
            slave = slave.wrapping_add(1)
        }
    }
    return wkc;
}
/* If slave has SII mapping and same slave ID done before, use previous mapping.
 * This is safe because SII mapping is constant for same slave ID.
 */
unsafe extern "C" fn ecx_lookup_mapping(mut context: *mut ecx_contextt,
                                        mut slave: uint16,
                                        mut Osize: *mut uint32,
                                        mut Isize: *mut uint32)
 -> libc::c_int {
    let mut i: libc::c_int = 0; /* check state change pre-op */
    let mut nSM: libc::c_int = 0;
    if slave as libc::c_int > 1 as libc::c_int &&
           *(*context).slavecount > 0 as libc::c_int {
        i = 1 as libc::c_int;
        while ((*(*context).slavelist.offset(i as isize)).eep_man !=
                   (*(*context).slavelist.offset(slave as isize)).eep_man ||
                   (*(*context).slavelist.offset(i as isize)).eep_id !=
                       (*(*context).slavelist.offset(slave as isize)).eep_id
                   ||
                   (*(*context).slavelist.offset(i as isize)).eep_rev !=
                       (*(*context).slavelist.offset(slave as isize)).eep_rev)
                  && i < slave as libc::c_int {
            i += 1
        }
        if i < slave as libc::c_int {
            nSM = 0 as libc::c_int;
            while nSM < 8 as libc::c_int {
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[nSM as
                                                                 usize].SMlength
                    =
                    (*(*context).slavelist.offset(i as
                                                      isize)).SM[nSM as
                                                                     usize].SMlength;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SMtype[nSM as usize]
                    =
                    (*(*context).slavelist.offset(i as
                                                      isize)).SMtype[nSM as
                                                                         usize];
                nSM += 1
            }
            *Osize =
                (*(*context).slavelist.offset(i as isize)).Obits as uint32;
            *Isize =
                (*(*context).slavelist.offset(i as isize)).Ibits as uint32;
            (*(*context).slavelist.offset(slave as isize)).Obits =
                *Osize as uint16;
            (*(*context).slavelist.offset(slave as isize)).Ibits =
                *Isize as uint16;
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ecx_map_coe_soe(mut context: *mut ecx_contextt,
                                     mut slave: uint16,
                                     mut thread_n: libc::c_int)
 -> libc::c_int {
    let mut Isize: uint32 = 0;
    let mut Osize: uint32 = 0;
    let mut rval: libc::c_int = 0;
    ecx_statecheck(context, slave, EC_STATE_PRE_OP as libc::c_int as uint16,
                   2000000 as libc::c_int);
    /* execute special slave configuration hook Pre-Op to Safe-OP */
    if (*(*context).slavelist.offset(slave as isize)).PO2SOconfig.is_some() {
        /* only if registered */
        (*(*context).slavelist.offset(slave as
                                          isize)).PO2SOconfig.expect("non-null function pointer")(slave);
    }
    if (*(*context).slavelist.offset(slave as isize)).PO2SOconfigx.is_some() {
        /* only if registered */
        (*(*context).slavelist.offset(slave as
                                          isize)).PO2SOconfigx.expect("non-null function pointer")(context,
                                                                                                   slave);
    }
    /* if slave not found in configlist find IO mapping in slave self */
    if (*(*context).slavelist.offset(slave as isize)).configindex == 0 {
        Isize = 0 as libc::c_int as uint32;
        Osize = 0 as libc::c_int as uint32;
        if (*(*context).slavelist.offset(slave as isize)).mbx_proto as
               libc::c_int & 0x4 as libc::c_int != 0 {
            /* has CoE */
            rval = 0 as libc::c_int;
            if (*(*context).slavelist.offset(slave as isize)).CoEdetails as
                   libc::c_int & 0x20 as libc::c_int != 0 {
                /* has Complete Access */
                /* read PDO mapping via CoE and use Complete Access */
                rval =
                    ecx_readPDOmapCA(context, slave, thread_n, &mut Osize,
                                     &mut Isize)
            }
            if rval == 0 {
                /* CA not available or not succeeded */
                /* read PDO mapping via CoE */
                rval = ecx_readPDOmap(context, slave, &mut Osize, &mut Isize)
            }
        }
        if Isize == 0 && Osize == 0 &&
               (*(*context).slavelist.offset(slave as isize)).mbx_proto as
                   libc::c_int & 0x10 as libc::c_int != 0 {
            /* has SoE */
            /* read AT / MDT mapping via SoE */
            rval = ecx_readIDNmap(context, slave, &mut Osize, &mut Isize);
            (*(*context).slavelist.offset(slave as
                                              isize)).SM[2 as libc::c_int as
                                                             usize].SMlength =
                Osize.wrapping_add(7 as libc::c_int as
                                       libc::c_uint).wrapping_div(8 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                    as uint16;
            (*(*context).slavelist.offset(slave as
                                              isize)).SM[3 as libc::c_int as
                                                             usize].SMlength =
                Isize.wrapping_add(7 as libc::c_int as
                                       libc::c_uint).wrapping_div(8 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                    as uint16
        }
        (*(*context).slavelist.offset(slave as isize)).Obits =
            Osize as uint16;
        (*(*context).slavelist.offset(slave as isize)).Ibits = Isize as uint16
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ecx_map_sii(mut context: *mut ecx_contextt,
                                 mut slave: uint16) -> libc::c_int {
    let mut Isize: uint32 = 0;
    let mut Osize: uint32 = 0;
    let mut nSM: libc::c_int = 0;
    let mut eepPDO: ec_eepromPDOt =
        ec_eepromPDOt{Startpos: 0,
                      Length: 0,
                      nPDO: 0,
                      Index: [0; 512],
                      SyncM: [0; 512],
                      BitSize: [0; 512],
                      SMbitsize: [0; 8],};
    Osize = (*(*context).slavelist.offset(slave as isize)).Obits as uint32;
    Isize = (*(*context).slavelist.offset(slave as isize)).Ibits as uint32;
    if Isize == 0 && Osize == 0 {
        /* find PDO in previous slave with same ID */
        ecx_lookup_mapping(context, slave, &mut Osize, &mut Isize);
    }
    if Isize == 0 && Osize == 0 {
        /* find PDO mapping by SII */
        memset(&mut eepPDO as *mut ec_eepromPDOt as *mut libc::c_void,
               0 as libc::c_int,
               ::core::mem::size_of::<ec_eepromPDOt>() as libc::c_ulong);
        Isize =
            ecx_siiPDO(context, slave, &mut eepPDO,
                       0 as libc::c_int as uint8);
        nSM = 0 as libc::c_int;
        while nSM < 8 as libc::c_int {
            if eepPDO.SMbitsize[nSM as usize] as libc::c_int >
                   0 as libc::c_int {
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[nSM as
                                                                 usize].SMlength
                    =
                    ((eepPDO.SMbitsize[nSM as usize] as libc::c_int +
                          7 as libc::c_int) / 8 as libc::c_int) as uint16;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SMtype[nSM as usize]
                    = 4 as libc::c_int as uint8
            }
            nSM += 1
        }
        Osize =
            ecx_siiPDO(context, slave, &mut eepPDO,
                       1 as libc::c_int as uint8);
        nSM = 0 as libc::c_int;
        while nSM < 8 as libc::c_int {
            if eepPDO.SMbitsize[nSM as usize] as libc::c_int >
                   0 as libc::c_int {
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[nSM as
                                                                 usize].SMlength
                    =
                    ((eepPDO.SMbitsize[nSM as usize] as libc::c_int +
                          7 as libc::c_int) / 8 as libc::c_int) as uint16;
                (*(*context).slavelist.offset(slave as
                                                  isize)).SMtype[nSM as usize]
                    = 3 as libc::c_int as uint8
            }
            nSM += 1
        }
    }
    (*(*context).slavelist.offset(slave as isize)).Obits = Osize as uint16;
    (*(*context).slavelist.offset(slave as isize)).Ibits = Isize as uint16;
    return 1 as libc::c_int;
}
unsafe extern "C" fn ecx_map_sm(mut context: *mut ecx_contextt,
                                mut slave: uint16) -> libc::c_int {
    let mut configadr: uint16 = 0;
    let mut nSM: libc::c_int = 0;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    if (*(*context).slavelist.offset(slave as isize)).mbx_l == 0 &&
           (*(*context).slavelist.offset(slave as
                                             isize)).SM[0 as libc::c_int as
                                                            usize].StartAddr
               as libc::c_int != 0 {
        ecx_FPWR((*context).port, configadr,
                 ECT_REG_SM0 as libc::c_int as uint16,
                 ::core::mem::size_of::<ec_smt>() as libc::c_ulong as uint16,
                 &mut *(*(*context).slavelist.offset(slave as
                                                         isize)).SM.as_mut_ptr().offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)
                     as *mut ec_smt as *mut libc::c_void,
                 2000 as libc::c_int * 3 as libc::c_int);
    }
    if (*(*context).slavelist.offset(slave as isize)).mbx_l == 0 &&
           (*(*context).slavelist.offset(slave as
                                             isize)).SM[1 as libc::c_int as
                                                            usize].StartAddr
               as libc::c_int != 0 {
        ecx_FPWR((*context).port, configadr,
                 ECT_REG_SM1 as libc::c_int as uint16,
                 ::core::mem::size_of::<ec_smt>() as libc::c_ulong as uint16,
                 &mut *(*(*context).slavelist.offset(slave as
                                                         isize)).SM.as_mut_ptr().offset(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)
                     as *mut ec_smt as *mut libc::c_void,
                 2000 as libc::c_int * 3 as libc::c_int);
    }
    /* program SM2 to SMx */
    nSM = 2 as libc::c_int;
    while nSM < 8 as libc::c_int {
        if (*(*context).slavelist.offset(slave as
                                             isize)).SM[nSM as
                                                            usize].StartAddr
               != 0 {
            /* check if SM length is zero -> clear enable flag */
            if (*(*context).slavelist.offset(slave as
                                                 isize)).SM[nSM as
                                                                usize].SMlength
                   as libc::c_int == 0 as libc::c_int {
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[nSM as
                                                                 usize].SMflags
                    =
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[nSM as
                                                                     usize].SMflags
                        & 0xfffeffff as libc::c_uint
            } else {
                /* if SM length is non zero always set enable flag */
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[nSM as
                                                                 usize].SMflags
                    =
                    (*(*context).slavelist.offset(slave as
                                                      isize)).SM[nSM as
                                                                     usize].SMflags
                        | !(0xfffeffff as libc::c_uint)
            }
            ecx_FPWR((*context).port, configadr,
                     (ECT_REG_SM0 as libc::c_int as
                          libc::c_ulong).wrapping_add((nSM as
                                                           libc::c_ulong).wrapping_mul(::core::mem::size_of::<ec_smt>()
                                                                                           as
                                                                                           libc::c_ulong))
                         as uint16,
                     ::core::mem::size_of::<ec_smt>() as libc::c_ulong as
                         uint16,
                     &mut *(*(*context).slavelist.offset(slave as
                                                             isize)).SM.as_mut_ptr().offset(nSM
                                                                                                as
                                                                                                isize)
                         as *mut ec_smt as *mut libc::c_void,
                     2000 as libc::c_int * 3 as libc::c_int);
        }
        nSM += 1
    }
    if (*(*context).slavelist.offset(slave as isize)).Ibits as libc::c_int >
           7 as libc::c_int {
        (*(*context).slavelist.offset(slave as isize)).Ibytes =
            (((*(*context).slavelist.offset(slave as isize)).Ibits as
                  libc::c_int + 7 as libc::c_int) / 8 as libc::c_int) as
                uint32
    }
    if (*(*context).slavelist.offset(slave as isize)).Obits as libc::c_int >
           7 as libc::c_int {
        (*(*context).slavelist.offset(slave as isize)).Obytes =
            (((*(*context).slavelist.offset(slave as isize)).Obits as
                  libc::c_int + 7 as libc::c_int) / 8 as libc::c_int) as
                uint32
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ecx_get_threadcount() -> libc::c_int {
    let mut thrc: libc::c_int = 0;
    let mut thrn: libc::c_int = 0;
    thrc = 0 as libc::c_int;
    thrn = 0 as libc::c_int;
    while thrn < 1 as libc::c_int {
        thrc += ecx_mapt[thrn as usize].running;
        thrn += 1
    }
    return thrc;
}
unsafe extern "C" fn ecx_config_find_mappings(mut context: *mut ecx_contextt,
                                              mut group: uint8) {
    let mut thrn: libc::c_int = 0;
    let mut thrc: libc::c_int = 0;
    let mut slave: uint16 = 0;
    thrn = 0 as libc::c_int;
    while thrn < 1 as libc::c_int {
        ecx_mapt[thrn as usize].running = 0 as libc::c_int;
        thrn += 1
    }
    /* find CoE and SoE mapping of slaves in multiple threads */
    slave = 1 as libc::c_int as uint16;
    while slave as libc::c_int <= *(*context).slavecount {
        if group == 0 ||
               group as libc::c_int ==
                   (*(*context).slavelist.offset(slave as isize)).group as
                       libc::c_int {
            /* serialised version */
            ecx_map_coe_soe(context, slave, 0 as libc::c_int);
        }
        slave = slave.wrapping_add(1)
    }
    loop 
         /* wait for all threads to finish */
         {
        thrc = ecx_get_threadcount();
        if thrc != 0 { osal_usleep(1000 as libc::c_int as uint32); }
        if !(thrc != 0) { break ; }
    }
    /* find SII mapping of slave and program SM */
    slave = 1 as libc::c_int as uint16;
    while slave as libc::c_int <= *(*context).slavecount {
        if group == 0 ||
               group as libc::c_int ==
                   (*(*context).slavelist.offset(slave as isize)).group as
                       libc::c_int {
            ecx_map_sii(context, slave);
            ecx_map_sm(context, slave);
        }
        slave = slave.wrapping_add(1)
    };
}
unsafe extern "C" fn ecx_config_create_input_mappings(mut context:
                                                          *mut ecx_contextt,
                                                      mut pIOmap:
                                                          *mut libc::c_void,
                                                      mut group: uint8,
                                                      mut slave: int16,
                                                      mut LogAddr:
                                                          *mut uint32,
                                                      mut BitPos:
                                                          *mut uint8) {
    let mut BitCount: libc::c_int = 0 as libc::c_int;
    let mut FMMUdone: libc::c_int = 0 as libc::c_int;
    let mut AddToInputsWKC: libc::c_int = 0 as libc::c_int;
    let mut ByteCount: uint16 = 0 as libc::c_int as uint16;
    let mut FMMUsize: uint16 = 0 as libc::c_int as uint16;
    let mut SMc: uint8 = 0 as libc::c_int as uint8;
    let mut EndAddr: uint16 = 0;
    let mut SMlength: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut FMMUc: uint8 = 0;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    FMMUc = (*(*context).slavelist.offset(slave as isize)).FMMUunused;
    if (*(*context).slavelist.offset(slave as isize)).Obits != 0 {
        /* find free FMMU */
        while (*(*context).slavelist.offset(slave as
                                                isize)).FMMU[FMMUc as
                                                                 usize].LogStart
                  != 0 {
            FMMUc = FMMUc.wrapping_add(1)
        }
    }
    /* search for SM that contribute to the input mapping */
    while (SMc as libc::c_int) < 8 as libc::c_int - 1 as libc::c_int &&
              FMMUdone <
                  ((*(*context).slavelist.offset(slave as isize)).Ibits as
                       libc::c_int + 7 as libc::c_int) / 8 as libc::c_int {
        while (SMc as libc::c_int) < 8 as libc::c_int - 1 as libc::c_int &&
                  (*(*context).slavelist.offset(slave as
                                                    isize)).SMtype[SMc as
                                                                       usize]
                      as libc::c_int != 4 as libc::c_int {
            SMc = SMc.wrapping_add(1)
        }
        (*(*context).slavelist.offset(slave as
                                          isize)).FMMU[FMMUc as
                                                           usize].PhysStart =
            (*(*context).slavelist.offset(slave as
                                              isize)).SM[SMc as
                                                             usize].StartAddr;
        SMlength =
            (*(*context).slavelist.offset(slave as
                                              isize)).SM[SMc as
                                                             usize].SMlength;
        ByteCount =
            (ByteCount as libc::c_int + SMlength as libc::c_int) as uint16;
        BitCount += SMlength as libc::c_int * 8 as libc::c_int;
        EndAddr =
            ((*(*context).slavelist.offset(slave as
                                               isize)).SM[SMc as
                                                              usize].StartAddr
                 as libc::c_int + SMlength as libc::c_int) as uint16;
        while BitCount <
                  (*(*context).slavelist.offset(slave as isize)).Ibits as
                      libc::c_int &&
                  (SMc as libc::c_int) < 8 as libc::c_int - 1 as libc::c_int {
            /* more SM for input */
            SMc = SMc.wrapping_add(1);
            while (SMc as libc::c_int) < 8 as libc::c_int - 1 as libc::c_int
                      &&
                      (*(*context).slavelist.offset(slave as
                                                        isize)).SMtype[SMc as
                                                                           usize]
                          as libc::c_int != 4 as libc::c_int {
                SMc = SMc.wrapping_add(1)
            }
            /* if addresses from more SM connect use one FMMU otherwise break up in multiple FMMU */
            if (*(*context).slavelist.offset(slave as
                                                 isize)).SM[SMc as
                                                                usize].StartAddr
                   as libc::c_int > EndAddr as libc::c_int {
                break ;
            }
            SMlength =
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[SMc as
                                                                 usize].SMlength;
            ByteCount =
                (ByteCount as libc::c_int + SMlength as libc::c_int) as
                    uint16;
            BitCount += SMlength as libc::c_int * 8 as libc::c_int;
            EndAddr =
                ((*(*context).slavelist.offset(slave as
                                                   isize)).SM[SMc as
                                                                  usize].StartAddr
                     as libc::c_int + SMlength as libc::c_int) as uint16
        }
        /* bit oriented slave */
        if (*(*context).slavelist.offset(slave as isize)).Ibytes == 0 {
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogStart
                = *LogAddr;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogStartbit
                = *BitPos;
            *BitPos =
                (*BitPos as libc::c_int +
                     ((*(*context).slavelist.offset(slave as isize)).Ibits as
                          libc::c_int - 1 as libc::c_int)) as uint8;
            if *BitPos as libc::c_int > 7 as libc::c_int {
                *LogAddr =
                    (*LogAddr as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32 as uint32;
                *BitPos = (*BitPos as libc::c_int - 8 as libc::c_int) as uint8
            }
            FMMUsize =
                (*LogAddr).wrapping_sub((*(*context).slavelist.offset(slave as
                                                                          isize)).FMMU[FMMUc
                                                                                           as
                                                                                           usize].LogStart).wrapping_add(1
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             libc::c_uint)
                    as uint16;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogLength
                = FMMUsize;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogEndbit
                = *BitPos;
            *BitPos = (*BitPos as libc::c_int + 1 as libc::c_int) as uint8;
            if *BitPos as libc::c_int > 7 as libc::c_int {
                *LogAddr =
                    (*LogAddr as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32 as uint32;
                *BitPos = (*BitPos as libc::c_int - 8 as libc::c_int) as uint8
            }
        } else {
            /* byte oriented slave */
            if *BitPos != 0 {
                *LogAddr =
                    (*LogAddr as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32 as uint32;
                *BitPos = 0 as libc::c_int as uint8
            }
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogStart
                = *LogAddr;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogStartbit
                = *BitPos;
            *BitPos = 7 as libc::c_int as uint8;
            FMMUsize = ByteCount;
            if FMMUsize as libc::c_int + FMMUdone >
                   (*(*context).slavelist.offset(slave as isize)).Ibytes as
                       libc::c_int {
                FMMUsize =
                    (*(*context).slavelist.offset(slave as
                                                      isize)).Ibytes.wrapping_sub(FMMUdone
                                                                                      as
                                                                                      libc::c_uint)
                        as uint16
            }
            *LogAddr =
                (*LogAddr as
                     libc::c_uint).wrapping_add(FMMUsize as libc::c_uint) as
                    uint32 as uint32;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogLength
                = FMMUsize;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogEndbit
                = *BitPos;
            *BitPos = 0 as libc::c_int as uint8
        }
        FMMUdone += FMMUsize as libc::c_int;
        if (*(*context).slavelist.offset(slave as
                                             isize)).FMMU[FMMUc as
                                                              usize].LogLength
               != 0 {
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].PhysStartBit
                = 0 as libc::c_int as uint8;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].FMMUtype
                = 1 as libc::c_int as uint8;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].FMMUactive
                = 1 as libc::c_int as uint8;
            /* program FMMU for input */
            ecx_FPWR((*context).port, configadr,
                     (ECT_REG_FMMU0 as libc::c_int as
                          libc::c_ulong).wrapping_add((::core::mem::size_of::<ec_fmmut>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(FMMUc
                                                                                           as
                                                                                           libc::c_ulong))
                         as uint16,
                     ::core::mem::size_of::<ec_fmmut>() as libc::c_ulong as
                         uint16,
                     &mut *(*(*context).slavelist.offset(slave as
                                                             isize)).FMMU.as_mut_ptr().offset(FMMUc
                                                                                                  as
                                                                                                  isize)
                         as *mut ec_fmmut as *mut libc::c_void,
                     2000 as libc::c_int * 3 as libc::c_int);
            /* Set flag to add one for an input FMMU,
            a single ESC can only contribute once */
            AddToInputsWKC = 1 as libc::c_int
        }
        if (*(*context).slavelist.offset(slave as isize)).inputs.is_null() {
            if group != 0 {
                let ref mut fresh5 =
                    (*(*context).slavelist.offset(slave as isize)).inputs;
                *fresh5 =
                    (pIOmap as
                         *mut uint8).offset((*(*context).slavelist.offset(slave
                                                                              as
                                                                              isize)).FMMU[FMMUc
                                                                                               as
                                                                                               usize].LogStart
                                                as
                                                isize).offset(-((*(*context).grouplist.offset(group
                                                                                                  as
                                                                                                  isize)).logstartaddr
                                                                    as isize))
            } else {
                let ref mut fresh6 =
                    (*(*context).slavelist.offset(slave as isize)).inputs;
                *fresh6 =
                    (pIOmap as
                         *mut uint8).offset((*(*context).slavelist.offset(slave
                                                                              as
                                                                              isize)).FMMU[FMMUc
                                                                                               as
                                                                                               usize].LogStart
                                                as isize)
            }
            (*(*context).slavelist.offset(slave as isize)).Istartbit =
                (*(*context).slavelist.offset(slave as
                                                  isize)).FMMU[FMMUc as
                                                                   usize].LogStartbit
        }
        FMMUc = FMMUc.wrapping_add(1)
    }
    (*(*context).slavelist.offset(slave as isize)).FMMUunused = FMMUc;
    /* Add one WKC for an input if flag is true */
    if AddToInputsWKC != 0 {
        let ref mut fresh7 =
            (*(*context).grouplist.offset(group as isize)).inputsWKC;
        *fresh7 = (*fresh7).wrapping_add(1)
    };
}
unsafe extern "C" fn ecx_config_create_output_mappings(mut context:
                                                           *mut ecx_contextt,
                                                       mut pIOmap:
                                                           *mut libc::c_void,
                                                       mut group: uint8,
                                                       mut slave: int16,
                                                       mut LogAddr:
                                                           *mut uint32,
                                                       mut BitPos:
                                                           *mut uint8) {
    let mut BitCount: libc::c_int = 0 as libc::c_int;
    let mut FMMUdone: libc::c_int = 0 as libc::c_int;
    let mut AddToOutputsWKC: libc::c_int = 0 as libc::c_int;
    let mut ByteCount: uint16 = 0 as libc::c_int as uint16;
    let mut FMMUsize: uint16 = 0 as libc::c_int as uint16;
    let mut SMc: uint8 = 0 as libc::c_int as uint8;
    let mut EndAddr: uint16 = 0;
    let mut SMlength: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut FMMUc: uint8 = 0;
    FMMUc = (*(*context).slavelist.offset(slave as isize)).FMMUunused;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    /* search for SM that contribute to the output mapping */
    while (SMc as libc::c_int) < 8 as libc::c_int - 1 as libc::c_int &&
              FMMUdone <
                  ((*(*context).slavelist.offset(slave as isize)).Obits as
                       libc::c_int + 7 as libc::c_int) / 8 as libc::c_int {
        while (SMc as libc::c_int) < 8 as libc::c_int - 1 as libc::c_int &&
                  (*(*context).slavelist.offset(slave as
                                                    isize)).SMtype[SMc as
                                                                       usize]
                      as libc::c_int != 3 as libc::c_int {
            SMc = SMc.wrapping_add(1)
        }
        (*(*context).slavelist.offset(slave as
                                          isize)).FMMU[FMMUc as
                                                           usize].PhysStart =
            (*(*context).slavelist.offset(slave as
                                              isize)).SM[SMc as
                                                             usize].StartAddr;
        SMlength =
            (*(*context).slavelist.offset(slave as
                                              isize)).SM[SMc as
                                                             usize].SMlength;
        ByteCount =
            (ByteCount as libc::c_int + SMlength as libc::c_int) as uint16;
        BitCount += SMlength as libc::c_int * 8 as libc::c_int;
        EndAddr =
            ((*(*context).slavelist.offset(slave as
                                               isize)).SM[SMc as
                                                              usize].StartAddr
                 as libc::c_int + SMlength as libc::c_int) as uint16;
        while BitCount <
                  (*(*context).slavelist.offset(slave as isize)).Obits as
                      libc::c_int &&
                  (SMc as libc::c_int) < 8 as libc::c_int - 1 as libc::c_int {
            /* more SM for output */
            SMc = SMc.wrapping_add(1);
            while (SMc as libc::c_int) < 8 as libc::c_int - 1 as libc::c_int
                      &&
                      (*(*context).slavelist.offset(slave as
                                                        isize)).SMtype[SMc as
                                                                           usize]
                          as libc::c_int != 3 as libc::c_int {
                SMc = SMc.wrapping_add(1)
            }
            /* if addresses from more SM connect use one FMMU otherwise break up in multiple FMMU */
            if (*(*context).slavelist.offset(slave as
                                                 isize)).SM[SMc as
                                                                usize].StartAddr
                   as libc::c_int > EndAddr as libc::c_int {
                break ;
            }
            SMlength =
                (*(*context).slavelist.offset(slave as
                                                  isize)).SM[SMc as
                                                                 usize].SMlength;
            ByteCount =
                (ByteCount as libc::c_int + SMlength as libc::c_int) as
                    uint16;
            BitCount += SMlength as libc::c_int * 8 as libc::c_int;
            EndAddr =
                ((*(*context).slavelist.offset(slave as
                                                   isize)).SM[SMc as
                                                                  usize].StartAddr
                     as libc::c_int + SMlength as libc::c_int) as uint16
        }
        /* bit oriented slave */
        if (*(*context).slavelist.offset(slave as isize)).Obytes == 0 {
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogStart
                = *LogAddr;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogStartbit
                = *BitPos;
            *BitPos =
                (*BitPos as libc::c_int +
                     ((*(*context).slavelist.offset(slave as isize)).Obits as
                          libc::c_int - 1 as libc::c_int)) as uint8;
            if *BitPos as libc::c_int > 7 as libc::c_int {
                *LogAddr =
                    (*LogAddr as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32 as uint32;
                *BitPos = (*BitPos as libc::c_int - 8 as libc::c_int) as uint8
            }
            FMMUsize =
                (*LogAddr).wrapping_sub((*(*context).slavelist.offset(slave as
                                                                          isize)).FMMU[FMMUc
                                                                                           as
                                                                                           usize].LogStart).wrapping_add(1
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             libc::c_uint)
                    as uint16;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogLength
                = FMMUsize;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogEndbit
                = *BitPos;
            *BitPos = (*BitPos as libc::c_int + 1 as libc::c_int) as uint8;
            if *BitPos as libc::c_int > 7 as libc::c_int {
                *LogAddr =
                    (*LogAddr as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32 as uint32;
                *BitPos = (*BitPos as libc::c_int - 8 as libc::c_int) as uint8
            }
        } else {
            /* byte oriented slave */
            if *BitPos != 0 {
                *LogAddr =
                    (*LogAddr as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32 as uint32;
                *BitPos = 0 as libc::c_int as uint8
            }
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogStart
                = *LogAddr;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogStartbit
                = *BitPos;
            *BitPos = 7 as libc::c_int as uint8;
            FMMUsize = ByteCount;
            if FMMUsize as libc::c_int + FMMUdone >
                   (*(*context).slavelist.offset(slave as isize)).Obytes as
                       libc::c_int {
                FMMUsize =
                    (*(*context).slavelist.offset(slave as
                                                      isize)).Obytes.wrapping_sub(FMMUdone
                                                                                      as
                                                                                      libc::c_uint)
                        as uint16
            }
            *LogAddr =
                (*LogAddr as
                     libc::c_uint).wrapping_add(FMMUsize as libc::c_uint) as
                    uint32 as uint32;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogLength
                = FMMUsize;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].LogEndbit
                = *BitPos;
            *BitPos = 0 as libc::c_int as uint8
        }
        FMMUdone += FMMUsize as libc::c_int;
        if (*(*context).slavelist.offset(slave as
                                             isize)).FMMU[FMMUc as
                                                              usize].LogLength
               != 0 {
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].PhysStartBit
                = 0 as libc::c_int as uint8;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].FMMUtype
                = 2 as libc::c_int as uint8;
            (*(*context).slavelist.offset(slave as
                                              isize)).FMMU[FMMUc as
                                                               usize].FMMUactive
                = 1 as libc::c_int as uint8;
            /* program FMMU for output */
            ecx_FPWR((*context).port, configadr,
                     (ECT_REG_FMMU0 as libc::c_int as
                          libc::c_ulong).wrapping_add((::core::mem::size_of::<ec_fmmut>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(FMMUc
                                                                                           as
                                                                                           libc::c_ulong))
                         as uint16,
                     ::core::mem::size_of::<ec_fmmut>() as libc::c_ulong as
                         uint16,
                     &mut *(*(*context).slavelist.offset(slave as
                                                             isize)).FMMU.as_mut_ptr().offset(FMMUc
                                                                                                  as
                                                                                                  isize)
                         as *mut ec_fmmut as *mut libc::c_void,
                     2000 as libc::c_int * 3 as libc::c_int);
            /* Set flag to add one for an output FMMU,
            a single ESC can only contribute once */
            AddToOutputsWKC = 1 as libc::c_int
        }
        if (*(*context).slavelist.offset(slave as isize)).outputs.is_null() {
            if group != 0 {
                let ref mut fresh8 =
                    (*(*context).slavelist.offset(slave as isize)).outputs;
                *fresh8 =
                    (pIOmap as
                         *mut uint8).offset((*(*context).slavelist.offset(slave
                                                                              as
                                                                              isize)).FMMU[FMMUc
                                                                                               as
                                                                                               usize].LogStart
                                                as
                                                isize).offset(-((*(*context).grouplist.offset(group
                                                                                                  as
                                                                                                  isize)).logstartaddr
                                                                    as isize))
            } else {
                let ref mut fresh9 =
                    (*(*context).slavelist.offset(slave as isize)).outputs;
                *fresh9 =
                    (pIOmap as
                         *mut uint8).offset((*(*context).slavelist.offset(slave
                                                                              as
                                                                              isize)).FMMU[FMMUc
                                                                                               as
                                                                                               usize].LogStart
                                                as isize)
            }
            (*(*context).slavelist.offset(slave as isize)).Ostartbit =
                (*(*context).slavelist.offset(slave as
                                                  isize)).FMMU[FMMUc as
                                                                   usize].LogStartbit
        }
        FMMUc = FMMUc.wrapping_add(1)
    }
    (*(*context).slavelist.offset(slave as isize)).FMMUunused = FMMUc;
    /* Add one WKC for an output if flag is true */
    if AddToOutputsWKC != 0 {
        let ref mut fresh10 =
            (*(*context).grouplist.offset(group as isize)).outputsWKC;
        *fresh10 = (*fresh10).wrapping_add(1)
    };
}
/* * Map all PDOs in one group of slaves to IOmap with Outputs/Inputs
* in sequential order (legacy SOEM way).
*
 *
 * @param[in]  context    = context struct
 * @param[out] pIOmap     = pointer to IOmap
 * @param[in]  group      = group to map, 0 = all groups
 * @return IOmap size
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_config_map_group(mut context: *mut ecx_contextt,
                                              mut pIOmap: *mut libc::c_void,
                                              mut group: uint8)
 -> libc::c_int {
    let mut slave: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut BitPos: uint8 = 0;
    let mut LogAddr: uint32 = 0 as libc::c_int as uint32;
    let mut oLogAddr: uint32 = 0 as libc::c_int as uint32;
    let mut diff: uint32 = 0;
    let mut currentsegment: uint16 = 0 as libc::c_int as uint16;
    let mut segmentsize: uint32 = 0 as libc::c_int as uint32;
    if *(*context).slavecount > 0 as libc::c_int &&
           (group as libc::c_int) < (*context).maxgroup {
        LogAddr = (*(*context).grouplist.offset(group as isize)).logstartaddr;
        oLogAddr = LogAddr;
        BitPos = 0 as libc::c_int as uint8;
        (*(*context).grouplist.offset(group as isize)).nsegments =
            0 as libc::c_int as uint16;
        (*(*context).grouplist.offset(group as isize)).outputsWKC =
            0 as libc::c_int as uint16;
        (*(*context).grouplist.offset(group as isize)).inputsWKC =
            0 as libc::c_int as uint16;
        /* Find mappings and program syncmanagers */
        ecx_config_find_mappings(context, group);
        /* do output mapping of slave and program FMMUs */
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            configadr =
                (*(*context).slavelist.offset(slave as isize)).configadr;
            if group == 0 ||
                   group as libc::c_int ==
                       (*(*context).slavelist.offset(slave as isize)).group as
                           libc::c_int {
                /* create output mapping */
                if (*(*context).slavelist.offset(slave as isize)).Obits != 0 {
                    ecx_config_create_output_mappings(context, pIOmap, group,
                                                      slave as int16,
                                                      &mut LogAddr,
                                                      &mut BitPos);
                    diff = LogAddr.wrapping_sub(oLogAddr);
                    oLogAddr = LogAddr;
                    if segmentsize.wrapping_add(diff) >
                           (1518 as libc::c_int - 14 as libc::c_int -
                                2 as libc::c_int - 10 as libc::c_int -
                                2 as libc::c_int - 4 as libc::c_int -
                                20 as libc::c_int) as libc::c_uint {
                        (*(*context).grouplist.offset(group as
                                                          isize)).IOsegment[currentsegment
                                                                                as
                                                                                usize]
                            = segmentsize;
                        if (currentsegment as libc::c_int) <
                               64 as libc::c_int - 1 as libc::c_int {
                            currentsegment = currentsegment.wrapping_add(1);
                            segmentsize = diff
                        }
                    } else {
                        segmentsize =
                            (segmentsize as libc::c_uint).wrapping_add(diff)
                                as uint32 as uint32
                    }
                }
            }
            slave = slave.wrapping_add(1)
        }
        if BitPos != 0 {
            LogAddr = LogAddr.wrapping_add(1);
            oLogAddr = LogAddr;
            BitPos = 0 as libc::c_int as uint8;
            if segmentsize.wrapping_add(1 as libc::c_int as libc::c_uint) >
                   (1518 as libc::c_int - 14 as libc::c_int - 2 as libc::c_int
                        - 10 as libc::c_int - 2 as libc::c_int -
                        4 as libc::c_int - 20 as libc::c_int) as libc::c_uint
               {
                (*(*context).grouplist.offset(group as
                                                  isize)).IOsegment[currentsegment
                                                                        as
                                                                        usize]
                    = segmentsize;
                if (currentsegment as libc::c_int) <
                       64 as libc::c_int - 1 as libc::c_int {
                    currentsegment = currentsegment.wrapping_add(1);
                    segmentsize = 1 as libc::c_int as uint32
                }
            } else {
                segmentsize =
                    (segmentsize as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32 as uint32
            }
        }
        let ref mut fresh11 =
            (*(*context).grouplist.offset(group as isize)).outputs;
        *fresh11 = pIOmap as *mut uint8;
        (*(*context).grouplist.offset(group as isize)).Obytes =
            LogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                   isize)).logstartaddr);
        (*(*context).grouplist.offset(group as isize)).nsegments =
            (currentsegment as libc::c_int + 1 as libc::c_int) as uint16;
        (*(*context).grouplist.offset(group as isize)).Isegment =
            currentsegment;
        (*(*context).grouplist.offset(group as isize)).Ioffset =
            segmentsize as uint16;
        if group == 0 {
            let ref mut fresh12 =
                (*(*context).slavelist.offset(0 as libc::c_int as
                                                  isize)).outputs;
            *fresh12 = pIOmap as *mut uint8;
            (*(*context).slavelist.offset(0 as libc::c_int as isize)).Obytes =
                LogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                       isize)).logstartaddr)
            /* store output bytes in master record */
        }
        /* do input mapping of slave and program FMMUs */
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            configadr =
                (*(*context).slavelist.offset(slave as isize)).configadr;
            if group == 0 ||
                   group as libc::c_int ==
                       (*(*context).slavelist.offset(slave as isize)).group as
                           libc::c_int {
                /* create input mapping */
                if (*(*context).slavelist.offset(slave as isize)).Ibits != 0 {
                    ecx_config_create_input_mappings(context, pIOmap, group,
                                                     slave as int16,
                                                     &mut LogAddr,
                                                     &mut BitPos); /* set Eeprom control to PDI */
                    diff = LogAddr.wrapping_sub(oLogAddr);
                    oLogAddr = LogAddr;
                    if segmentsize.wrapping_add(diff) >
                           (1518 as libc::c_int - 14 as libc::c_int -
                                2 as libc::c_int - 10 as libc::c_int -
                                2 as libc::c_int - 4 as libc::c_int -
                                20 as libc::c_int) as libc::c_uint {
                        (*(*context).grouplist.offset(group as
                                                          isize)).IOsegment[currentsegment
                                                                                as
                                                                                usize]
                            = segmentsize;
                        if (currentsegment as libc::c_int) <
                               64 as libc::c_int - 1 as libc::c_int {
                            currentsegment = currentsegment.wrapping_add(1);
                            segmentsize = diff
                        }
                    } else {
                        segmentsize =
                            (segmentsize as libc::c_uint).wrapping_add(diff)
                                as uint32 as uint32
                    }
                }
                ecx_eeprom2pdi(context, slave);
                /* User may override automatic state change */
                if (*context).manualstatechange == 0 as libc::c_int {
                    /* request safe_op for slave */
                    ecx_FPWRw((*context).port, configadr,
                              ECT_REG_ALCTL as libc::c_int as uint16,
                              EC_STATE_SAFE_OP as libc::c_int as uint16,
                              2000 as libc::c_int * 3 as libc::c_int);
                    /* set safeop status */
                }
                if (*(*context).slavelist.offset(slave as isize)).blockLRW !=
                       0 {
                    let ref mut fresh13 =
                        (*(*context).grouplist.offset(group as
                                                          isize)).blockLRW;
                    *fresh13 = (*fresh13).wrapping_add(1)
                }
                let ref mut fresh14 =
                    (*(*context).grouplist.offset(group as
                                                      isize)).Ebuscurrent;
                *fresh14 =
                    (*fresh14 as libc::c_int +
                         (*(*context).slavelist.offset(slave as
                                                           isize)).Ebuscurrent
                             as libc::c_int) as int16
            }
            slave = slave.wrapping_add(1)
        }
        if BitPos != 0 {
            LogAddr = LogAddr.wrapping_add(1);
            oLogAddr = LogAddr;
            BitPos = 0 as libc::c_int as uint8;
            if segmentsize.wrapping_add(1 as libc::c_int as libc::c_uint) >
                   (1518 as libc::c_int - 14 as libc::c_int - 2 as libc::c_int
                        - 10 as libc::c_int - 2 as libc::c_int -
                        4 as libc::c_int - 20 as libc::c_int) as libc::c_uint
               {
                (*(*context).grouplist.offset(group as
                                                  isize)).IOsegment[currentsegment
                                                                        as
                                                                        usize]
                    = segmentsize;
                if (currentsegment as libc::c_int) <
                       64 as libc::c_int - 1 as libc::c_int {
                    currentsegment = currentsegment.wrapping_add(1);
                    segmentsize = 1 as libc::c_int as uint32
                }
            } else {
                segmentsize =
                    (segmentsize as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32 as uint32
            }
        }
        (*(*context).grouplist.offset(group as
                                          isize)).IOsegment[currentsegment as
                                                                usize] =
            segmentsize;
        (*(*context).grouplist.offset(group as isize)).nsegments =
            (currentsegment as libc::c_int + 1 as libc::c_int) as uint16;
        let ref mut fresh15 =
            (*(*context).grouplist.offset(group as isize)).inputs;
        *fresh15 =
            (pIOmap as
                 *mut uint8).offset((*(*context).grouplist.offset(group as
                                                                      isize)).Obytes
                                        as isize);
        (*(*context).grouplist.offset(group as isize)).Ibytes =
            LogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                   isize)).logstartaddr).wrapping_sub((*(*context).grouplist.offset(group
                                                                                                                                        as
                                                                                                                                        isize)).Obytes);
        if group == 0 {
            let ref mut fresh16 =
                (*(*context).slavelist.offset(0 as libc::c_int as
                                                  isize)).inputs;
            *fresh16 =
                (pIOmap as
                     *mut uint8).offset((*(*context).slavelist.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).Obytes
                                            as isize);
            (*(*context).slavelist.offset(0 as libc::c_int as isize)).Ibytes =
                LogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                       isize)).logstartaddr).wrapping_sub((*(*context).slavelist.offset(0
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            isize)).Obytes)
            /* store input bytes in master record */
        }
        return LogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                      isize)).logstartaddr)
                   as libc::c_int
    }
    return 0 as libc::c_int;
}
/* * Map all PDOs in one group of slaves to IOmap with Outputs/Inputs
 * overlapping. NOTE: Must use this for TI ESC when using LRW.
 *
 * @param[in]  context    = context struct
 * @param[out] pIOmap     = pointer to IOmap
 * @param[in]  group      = group to map, 0 = all groups
 * @return IOmap size
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_config_overlap_map_group(mut context:
                                                          *mut ecx_contextt,
                                                      mut pIOmap:
                                                          *mut libc::c_void,
                                                      mut group: uint8)
 -> libc::c_int {
    let mut slave: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut BitPos: uint8 = 0;
    let mut mLogAddr: uint32 = 0 as libc::c_int as uint32;
    let mut siLogAddr: uint32 = 0 as libc::c_int as uint32;
    let mut soLogAddr: uint32 = 0 as libc::c_int as uint32;
    let mut tempLogAddr: uint32 = 0;
    let mut diff: uint32 = 0;
    let mut currentsegment: uint16 = 0 as libc::c_int as uint16;
    let mut segmentsize: uint32 = 0 as libc::c_int as uint32;
    if *(*context).slavecount > 0 as libc::c_int &&
           (group as libc::c_int) < (*context).maxgroup {
        mLogAddr =
            (*(*context).grouplist.offset(group as isize)).logstartaddr;
        siLogAddr = mLogAddr;
        soLogAddr = mLogAddr;
        BitPos = 0 as libc::c_int as uint8;
        (*(*context).grouplist.offset(group as isize)).nsegments =
            0 as libc::c_int as uint16;
        (*(*context).grouplist.offset(group as isize)).outputsWKC =
            0 as libc::c_int as uint16;
        (*(*context).grouplist.offset(group as isize)).inputsWKC =
            0 as libc::c_int as uint16;
        /* Find mappings and program syncmanagers */
        ecx_config_find_mappings(context, group);
        /* do IO mapping of slave and program FMMUs */
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            configadr =
                (*(*context).slavelist.offset(slave as isize)).configadr;
            soLogAddr = mLogAddr;
            siLogAddr = soLogAddr;
            if group == 0 ||
                   group as libc::c_int ==
                       (*(*context).slavelist.offset(slave as isize)).group as
                           libc::c_int {
                /* create output mapping */
                if (*(*context).slavelist.offset(slave as isize)).Obits != 0 {
                    ecx_config_create_output_mappings(context, pIOmap, group,
                                                      slave as int16,
                                                      &mut soLogAddr,
                                                      &mut BitPos);
                    if BitPos != 0 {
                        soLogAddr = soLogAddr.wrapping_add(1);
                        BitPos = 0 as libc::c_int as uint8
                    }
                }
                /* create input mapping */
                if (*(*context).slavelist.offset(slave as isize)).Ibits != 0 {
                    ecx_config_create_input_mappings(context, pIOmap, group,
                                                     slave as int16,
                                                     &mut siLogAddr,
                                                     &mut BitPos); /* set Eeprom control to PDI */
                    if BitPos != 0 {
                        siLogAddr = siLogAddr.wrapping_add(1);
                        BitPos = 0 as libc::c_int as uint8
                    }
                }
                tempLogAddr =
                    if siLogAddr > soLogAddr { siLogAddr } else { soLogAddr };
                diff = tempLogAddr.wrapping_sub(mLogAddr);
                mLogAddr = tempLogAddr;
                if segmentsize.wrapping_add(diff) >
                       (1518 as libc::c_int - 14 as libc::c_int -
                            2 as libc::c_int - 10 as libc::c_int -
                            2 as libc::c_int - 4 as libc::c_int -
                            20 as libc::c_int) as libc::c_uint {
                    (*(*context).grouplist.offset(group as
                                                      isize)).IOsegment[currentsegment
                                                                            as
                                                                            usize]
                        = segmentsize;
                    if (currentsegment as libc::c_int) <
                           64 as libc::c_int - 1 as libc::c_int {
                        currentsegment = currentsegment.wrapping_add(1);
                        segmentsize = diff
                    }
                } else {
                    segmentsize =
                        (segmentsize as libc::c_uint).wrapping_add(diff) as
                            uint32 as uint32
                }
                ecx_eeprom2pdi(context, slave);
                /* User may override automatic state change */
                if (*context).manualstatechange == 0 as libc::c_int {
                    /* request safe_op for slave */
                    ecx_FPWRw((*context).port, configadr,
                              ECT_REG_ALCTL as libc::c_int as uint16,
                              EC_STATE_SAFE_OP as libc::c_int as uint16,
                              2000 as libc::c_int * 3 as libc::c_int);
                }
                if (*(*context).slavelist.offset(slave as isize)).blockLRW !=
                       0 {
                    let ref mut fresh17 =
                        (*(*context).grouplist.offset(group as
                                                          isize)).blockLRW;
                    *fresh17 = (*fresh17).wrapping_add(1)
                }
                let ref mut fresh18 =
                    (*(*context).grouplist.offset(group as
                                                      isize)).Ebuscurrent;
                *fresh18 =
                    (*fresh18 as libc::c_int +
                         (*(*context).slavelist.offset(slave as
                                                           isize)).Ebuscurrent
                             as libc::c_int) as int16
            }
            slave = slave.wrapping_add(1)
        }
        (*(*context).grouplist.offset(group as
                                          isize)).IOsegment[currentsegment as
                                                                usize] =
            segmentsize;
        (*(*context).grouplist.offset(group as isize)).nsegments =
            (currentsegment as libc::c_int + 1 as libc::c_int) as uint16;
        (*(*context).grouplist.offset(group as isize)).Isegment =
            0 as libc::c_int as uint16;
        (*(*context).grouplist.offset(group as isize)).Ioffset =
            0 as libc::c_int as uint16;
        (*(*context).grouplist.offset(group as isize)).Obytes =
            soLogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                     isize)).logstartaddr);
        (*(*context).grouplist.offset(group as isize)).Ibytes =
            siLogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                     isize)).logstartaddr);
        let ref mut fresh19 =
            (*(*context).grouplist.offset(group as isize)).outputs;
        *fresh19 = pIOmap as *mut uint8;
        let ref mut fresh20 =
            (*(*context).grouplist.offset(group as isize)).inputs;
        *fresh20 =
            (pIOmap as
                 *mut uint8).offset((*(*context).grouplist.offset(group as
                                                                      isize)).Obytes
                                        as isize);
        /* Move calculated inputs with OBytes offset*/
        slave = 1 as libc::c_int as uint16;
        while slave as libc::c_int <= *(*context).slavecount {
            if group == 0 ||
                   group as libc::c_int ==
                       (*(*context).slavelist.offset(slave as isize)).group as
                           libc::c_int {
                if (*(*context).slavelist.offset(slave as isize)).Ibits as
                       libc::c_int > 0 as libc::c_int {
                    let ref mut fresh21 =
                        (*(*context).slavelist.offset(slave as isize)).inputs;
                    *fresh21 =
                        (*fresh21).offset((*(*context).grouplist.offset(group
                                                                            as
                                                                            isize)).Obytes
                                              as isize)
                }
            }
            slave = slave.wrapping_add(1)
        }
        if group == 0 {
            /* store output bytes in master record */
            let ref mut fresh22 =
                (*(*context).slavelist.offset(0 as libc::c_int as
                                                  isize)).outputs;
            *fresh22 = pIOmap as *mut uint8;
            (*(*context).slavelist.offset(0 as libc::c_int as isize)).Obytes =
                soLogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                         isize)).logstartaddr);
            let ref mut fresh23 =
                (*(*context).slavelist.offset(0 as libc::c_int as
                                                  isize)).inputs;
            *fresh23 =
                (pIOmap as
                     *mut uint8).offset((*(*context).slavelist.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).Obytes
                                            as isize);
            (*(*context).slavelist.offset(0 as libc::c_int as isize)).Ibytes =
                siLogAddr.wrapping_sub((*(*context).grouplist.offset(group as
                                                                         isize)).logstartaddr)
        }
        return (*(*context).grouplist.offset(group as
                                                 isize)).Obytes.wrapping_add((*(*context).grouplist.offset(group
                                                                                                               as
                                                                                                               isize)).Ibytes)
                   as libc::c_int
    }
    return 0 as libc::c_int;
}
/* * Recover slave.
 *
 * @param[in] context = context struct
 * @param[in] slave   = slave to recover
 * @param[in] timeout = local timeout f.e. EC_TIMEOUTRET3
 * @return >0 if successful
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_recover_slave(mut context: *mut ecx_contextt,
                                           mut slave: uint16,
                                           mut timeout: libc::c_int)
 -> libc::c_int {
    let mut rval: libc::c_int = 0;
    let mut wkc: libc::c_int = 0;
    let mut ADPh: uint16 = 0;
    let mut configadr: uint16 = 0;
    let mut readadr: uint16 = 0;
    rval = 0 as libc::c_int;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    ADPh = (1 as libc::c_int - slave as libc::c_int) as uint16;
    /* check if we found another slave than the requested */
    readadr = 0xfffe as libc::c_int as uint16;
    wkc =
        ecx_APRD((*context).port, ADPh,
                 ECT_REG_STADR as libc::c_int as uint16,
                 ::core::mem::size_of::<uint16>() as libc::c_ulong as uint16,
                 &mut readadr as *mut uint16 as *mut libc::c_void, timeout);
    /* correct slave found, finished */
    if readadr as libc::c_int == configadr as libc::c_int {
        return 1 as libc::c_int
    }
    /* only try if no config address*/
    if wkc > 0 as libc::c_int && readadr as libc::c_int == 0 as libc::c_int {
        /* clear possible slaves at EC_TEMPNODE */
        ecx_FPWRw((*context).port, 0xffff as libc::c_int as uint16,
                  ECT_REG_STADR as libc::c_int as uint16,
                  0 as libc::c_int as uint16, 0 as libc::c_int);
        /* set temporary node address of slave */
        if ecx_APWRw((*context).port, ADPh,
                     ECT_REG_STADR as libc::c_int as uint16,
                     0xffff as libc::c_int as uint16, timeout) <=
               0 as libc::c_int {
            ecx_FPWRw((*context).port, 0xffff as libc::c_int as uint16,
                      ECT_REG_STADR as libc::c_int as uint16,
                      0 as libc::c_int as uint16, 0 as libc::c_int);
            return 0 as libc::c_int
            /* slave fails to respond */
        } /* temporary config address */
        (*(*context).slavelist.offset(slave as isize)).configadr =
            0xffff as libc::c_int as
                uint16; /* set Eeprom control to master */
        ecx_eeprom2master(context, slave);
        /* check if slave is the same as configured before */
        if ecx_FPRDw((*context).port, 0xffff as libc::c_int as uint16,
                     ECT_REG_ALIAS as libc::c_int as uint16, timeout) as
               libc::c_int ==
               (*(*context).slavelist.offset(slave as isize)).aliasadr as
                   libc::c_int &&
               ecx_readeeprom(context, slave,
                              ECT_SII_ID as libc::c_int as uint16,
                              20000 as libc::c_int) ==
                   (*(*context).slavelist.offset(slave as isize)).eep_id &&
               ecx_readeeprom(context, slave,
                              ECT_SII_MANUF as libc::c_int as uint16,
                              20000 as libc::c_int) ==
                   (*(*context).slavelist.offset(slave as isize)).eep_man &&
               ecx_readeeprom(context, slave,
                              ECT_SII_REV as libc::c_int as uint16,
                              20000 as libc::c_int) ==
                   (*(*context).slavelist.offset(slave as isize)).eep_rev {
            rval =
                ecx_FPWRw((*context).port, 0xffff as libc::c_int as uint16,
                          ECT_REG_STADR as libc::c_int as uint16, configadr,
                          timeout);
            (*(*context).slavelist.offset(slave as isize)).configadr =
                configadr
        } else {
            /* slave is not the expected one, remove config address*/
            ecx_FPWRw((*context).port, 0xffff as libc::c_int as uint16,
                      ECT_REG_STADR as libc::c_int as uint16,
                      0 as libc::c_int as uint16, timeout);
            (*(*context).slavelist.offset(slave as isize)).configadr =
                configadr
        }
    }
    return rval;
}
/* * Reconfigure slave.
 *
 * @param[in] context = context struct
 * @param[in] slave   = slave to reconfigure
 * @param[in] timeout = local timeout f.e. EC_TIMEOUTRET3
 * @return Slave state
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_reconfig_slave(mut context: *mut ecx_contextt,
                                            mut slave: uint16,
                                            mut timeout: libc::c_int)
 -> libc::c_int {
    let mut state: libc::c_int = 0; /* set Eeprom control to PDI */
    let mut nSM: libc::c_int = 0;
    let mut FMMUc: libc::c_int = 0;
    let mut configadr: uint16 = 0;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    if ecx_FPWRw((*context).port, configadr,
                 ECT_REG_ALCTL as libc::c_int as uint16,
                 EC_STATE_INIT as libc::c_int as uint16, timeout) <=
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    state = 0 as libc::c_int;
    ecx_eeprom2pdi(context, slave);
    /* check state change init */
    state =
        ecx_statecheck(context, slave, EC_STATE_INIT as libc::c_int as uint16,
                       2000000 as libc::c_int) as libc::c_int;
    if state == EC_STATE_INIT as libc::c_int {
        /* program all enabled SM */
        nSM = 0 as libc::c_int; /* check state change pre-op */
        while nSM < 8 as libc::c_int {
            if (*(*context).slavelist.offset(slave as
                                                 isize)).SM[nSM as
                                                                usize].StartAddr
                   != 0 {
                ecx_FPWR((*context).port, configadr,
                         (ECT_REG_SM0 as libc::c_int as
                              libc::c_ulong).wrapping_add((nSM as
                                                               libc::c_ulong).wrapping_mul(::core::mem::size_of::<ec_smt>()
                                                                                               as
                                                                                               libc::c_ulong))
                             as uint16,
                         ::core::mem::size_of::<ec_smt>() as libc::c_ulong as
                             uint16,
                         &mut *(*(*context).slavelist.offset(slave as
                                                                 isize)).SM.as_mut_ptr().offset(nSM
                                                                                                    as
                                                                                                    isize)
                             as *mut ec_smt as *mut libc::c_void, timeout);
            }
            nSM += 1
        }
        ecx_FPWRw((*context).port, configadr,
                  ECT_REG_ALCTL as libc::c_int as uint16,
                  EC_STATE_PRE_OP as libc::c_int as uint16, timeout);
        state =
            ecx_statecheck(context, slave,
                           EC_STATE_PRE_OP as libc::c_int as uint16,
                           2000000 as libc::c_int) as libc::c_int;
        if state == EC_STATE_PRE_OP as libc::c_int {
            /* execute special slave configuration hook Pre-Op to Safe-OP */
            if (*(*context).slavelist.offset(slave as
                                                 isize)).PO2SOconfig.is_some()
               {
                /* only if registered */
                (*(*context).slavelist.offset(slave as
                                                  isize)).PO2SOconfig.expect("non-null function pointer")(slave);
            }
            if (*(*context).slavelist.offset(slave as
                                                 isize)).PO2SOconfigx.is_some()
               {
                /* only if registered */
                (*(*context).slavelist.offset(slave as
                                                  isize)).PO2SOconfigx.expect("non-null function pointer")(context,
                                                                                                           slave); /* set safeop status */
            } /* check state change safe-op */
            ecx_FPWRw((*context).port, configadr,
                      ECT_REG_ALCTL as libc::c_int as uint16,
                      EC_STATE_SAFE_OP as libc::c_int as uint16, timeout);
            state =
                ecx_statecheck(context, slave,
                               EC_STATE_SAFE_OP as libc::c_int as uint16,
                               2000000 as libc::c_int) as libc::c_int;
            /* program configured FMMU */
            FMMUc = 0 as libc::c_int;
            while FMMUc <
                      (*(*context).slavelist.offset(slave as
                                                        isize)).FMMUunused as
                          libc::c_int {
                ecx_FPWR((*context).port, configadr,
                         (ECT_REG_FMMU0 as libc::c_int as
                              libc::c_ulong).wrapping_add((::core::mem::size_of::<ec_fmmut>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(FMMUc
                                                                                               as
                                                                                               libc::c_ulong))
                             as uint16,
                         ::core::mem::size_of::<ec_fmmut>() as libc::c_ulong
                             as uint16,
                         &mut *(*(*context).slavelist.offset(slave as
                                                                 isize)).FMMU.as_mut_ptr().offset(FMMUc
                                                                                                      as
                                                                                                      isize)
                             as *mut ec_fmmut as *mut libc::c_void, timeout);
                FMMUc += 1
            }
        }
    }
    return state;
}
/* * Enumerate and init all slaves.
 *
 * @param[in] usetable     = TRUE when using configtable to init slaves, FALSE otherwise
 * @return Workcounter of slave discover datagram = number of slaves found
 * @see ecx_config_init
 */
#[no_mangle]
pub unsafe extern "C" fn ec_config_init(mut usetable: uint8) -> libc::c_int {
    return ecx_config_init(&mut ecx_context, usetable);
}
/* * Map all PDOs in one group of slaves to IOmap with Outputs/Inputs
 * in sequential order (legacy SOEM way).
 *
 * @param[out] pIOmap     = pointer to IOmap
 * @param[in]  group      = group to map, 0 = all groups
 * @return IOmap size
 * @see ecx_config_map_group
 */
#[no_mangle]
pub unsafe extern "C" fn ec_config_map_group(mut pIOmap: *mut libc::c_void,
                                             mut group: uint8)
 -> libc::c_int {
    return ecx_config_map_group(&mut ecx_context, pIOmap, group);
}
/* * Map all PDOs in one group of slaves to IOmap with Outputs/Inputs
* overlapping. NOTE: Must use this for TI ESC when using LRW.
*
* @param[out] pIOmap     = pointer to IOmap
* @param[in]  group      = group to map, 0 = all groups
* @return IOmap size
* @see ecx_config_overlap_map_group
*/
#[no_mangle]
pub unsafe extern "C" fn ec_config_overlap_map_group(mut pIOmap:
                                                         *mut libc::c_void,
                                                     mut group: uint8)
 -> libc::c_int {
    return ecx_config_overlap_map_group(&mut ecx_context, pIOmap, group);
}
/* * Map all PDOs from slaves to IOmap with Outputs/Inputs
 * in sequential order (legacy SOEM way).
 *
 * @param[out] pIOmap     = pointer to IOmap
 * @return IOmap size
 */
#[no_mangle]
pub unsafe extern "C" fn ec_config_map(mut pIOmap: *mut libc::c_void)
 -> libc::c_int {
    return ec_config_map_group(pIOmap, 0 as libc::c_int as uint8);
}
/* * Map all PDOs from slaves to IOmap with Outputs/Inputs
* overlapping. NOTE: Must use this for TI ESC when using LRW.
*
* @param[out] pIOmap     = pointer to IOmap
* @return IOmap size
*/
#[no_mangle]
pub unsafe extern "C" fn ec_config_overlap_map(mut pIOmap: *mut libc::c_void)
 -> libc::c_int {
    return ec_config_overlap_map_group(pIOmap, 0 as libc::c_int as uint8);
}
/* * Enumerate / map and init all slaves.
 *
 * @param[in] usetable    = TRUE when using configtable to init slaves, FALSE otherwise
 * @param[out] pIOmap     = pointer to IOmap
 * @return Workcounter of slave discover datagram = number of slaves found
 */
#[no_mangle]
pub unsafe extern "C" fn ec_config(mut usetable: uint8,
                                   mut pIOmap: *mut libc::c_void)
 -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    wkc = ec_config_init(usetable);
    if wkc != 0 { ec_config_map(pIOmap); }
    return wkc;
}
/* * Enumerate / map and init all slaves.
*
* @param[in] usetable    = TRUE when using configtable to init slaves, FALSE otherwise
* @param[out] pIOmap     = pointer to IOmap
* @return Workcounter of slave discover datagram = number of slaves found
*/
#[no_mangle]
pub unsafe extern "C" fn ec_config_overlap(mut usetable: uint8,
                                           mut pIOmap: *mut libc::c_void)
 -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    wkc = ec_config_init(usetable);
    if wkc != 0 { ec_config_overlap_map(pIOmap); }
    return wkc;
}
/* * Recover slave.
 *
 * @param[in] slave   = slave to recover
 * @param[in] timeout = local timeout f.e. EC_TIMEOUTRET3
 * @return >0 if successful
 * @see ecx_recover_slave
 */
#[no_mangle]
pub unsafe extern "C" fn ec_recover_slave(mut slave: uint16,
                                          mut timeout: libc::c_int)
 -> libc::c_int {
    return ecx_recover_slave(&mut ecx_context, slave, timeout);
}
/* * Reconfigure slave.
 *
 * @param[in] slave   = slave to reconfigure
 * @param[in] timeout = local timeout f.e. EC_TIMEOUTRET3
 * @return Slave state
 * @see ecx_reconfig_slave
 */
#[no_mangle]
pub unsafe extern "C" fn ec_reconfig_slave(mut slave: uint16,
                                           mut timeout: libc::c_int)
 -> libc::c_int {
    return ecx_reconfig_slave(&mut ecx_context, slave, timeout);
}
