use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn osal_current_time() -> ec_timet;
    #[no_mangle]
    static mut ecx_context: ecx_contextt;
    #[no_mangle]
    fn ec_nextmbxcnt(cnt: uint8) -> uint8;
    #[no_mangle]
    fn ec_clearmbx(Mbx: *mut ec_mbxbuft);
    #[no_mangle]
    fn ecx_pusherror(context: *mut ecx_contextt, Ec: *const ec_errort);
    #[no_mangle]
    fn ecx_packeterror(context: *mut ecx_contextt, Slave: uint16,
                       Index: uint16, SubIdx: uint8, ErrorCode: uint16);
    #[no_mangle]
    fn ecx_mbxreceive(context: *mut ecx_contextt, slave: uint16,
                      mbx: *mut ec_mbxbuft, timeout: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ecx_mbxsend(context: *mut ecx_contextt, slave: uint16,
                   mbx: *mut ec_mbxbuft, timeout: libc::c_int) -> libc::c_int;
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
pub const ECT_MBXT_VOE: C2RustUnnamed = 15;
pub const ECT_MBXT_SOE: C2RustUnnamed = 5;
pub const ECT_MBXT_FOE: C2RustUnnamed = 4;
pub const ECT_MBXT_COE: C2RustUnnamed = 3;
pub const ECT_MBXT_EOE: C2RustUnnamed = 2;
pub const ECT_MBXT_AOE: C2RustUnnamed = 1;
pub const ECT_MBXT_ERR: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ECT_COES_SDOINFO: C2RustUnnamed_0 = 8;
pub const ECT_COES_RXPDO_RR: C2RustUnnamed_0 = 7;
pub const ECT_COES_TXPDO_RR: C2RustUnnamed_0 = 6;
pub const ECT_COES_RXPDO: C2RustUnnamed_0 = 5;
pub const ECT_COES_TXPDO: C2RustUnnamed_0 = 4;
pub const ECT_COES_SDORES: C2RustUnnamed_0 = 3;
pub const ECT_COES_SDOREQ: C2RustUnnamed_0 = 2;
pub const ECT_COES_EMERGENCY: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const ECT_SDO_ABORT: C2RustUnnamed_1 = 128;
pub const ECT_SDO_SEG_UP_REQ: C2RustUnnamed_1 = 96;
pub const ECT_SDO_UP_REQ_CA: C2RustUnnamed_1 = 80;
pub const ECT_SDO_UP_REQ: C2RustUnnamed_1 = 64;
pub const ECT_SDO_DOWN_INIT_CA: C2RustUnnamed_1 = 49;
pub const ECT_SDO_DOWN_EXP: C2RustUnnamed_1 = 35;
pub const ECT_SDO_DOWN_INIT: C2RustUnnamed_1 = 33;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ECT_SDOINFO_ERROR: C2RustUnnamed_2 = 7;
pub const ECT_GET_OE_RES: C2RustUnnamed_2 = 6;
pub const ECT_GET_OE_REQ: C2RustUnnamed_2 = 5;
pub const ECT_GET_OD_RES: C2RustUnnamed_2 = 4;
pub const ECT_GET_OD_REQ: C2RustUnnamed_2 = 3;
pub const ECT_GET_ODLIST_RES: C2RustUnnamed_2 = 2;
pub const ECT_GET_ODLIST_REQ: C2RustUnnamed_2 = 1;
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
pub type ec_mbxbuft = [uint8; 1487];
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_mbxheader {
    pub length: uint16,
    pub address: uint16,
    pub priority: uint8,
    pub mbxtype: uint8,
}
pub type ec_mbxheadert = ec_mbxheader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_ODlistt {
    pub Slave: uint16,
    pub Entries: uint16,
    pub Index: [uint16; 1024],
    pub DataType: [uint16; 1024],
    pub ObjectCode: [uint8; 1024],
    pub MaxSub: [uint8; 1024],
    pub Name: [[libc::c_char; 41]; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_OElistt {
    pub Entries: uint16,
    pub ValueInfo: [uint8; 256],
    pub DataType: [uint16; 256],
    pub BitLength: [uint16; 256],
    pub ObjAccess: [uint16; 256],
    pub Name: [[libc::c_char; 41]; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub bdata: [uint8; 512],
    pub wdata: [uint16; 256],
    pub ldata: [uint32; 128],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_SDOt {
    pub MbxHeader: ec_mbxheadert,
    pub CANOpen: uint16,
    pub Command: uint8,
    pub Index: uint16,
    pub SubIndex: uint8,
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub bdata: [uint8; 512],
    pub wdata: [uint16; 256],
    pub ldata: [uint32; 128],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ec_SDOservicet {
    pub MbxHeader: ec_mbxheadert,
    pub CANOpen: uint16,
    pub Opcode: uint8,
    pub Reserved: uint8,
    pub Fragments: uint16,
    pub c2rust_unnamed: C2RustUnnamed_6,
}
/* * Report SDO error.
 *
 * @param[in]  context    = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index that generated error
 * @param[in]  SubIdx     = Subindex that generated error
 * @param[in]  AbortCode  = Abortcode, see EtherCAT documentation for list
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_SDOerror(mut context: *mut ecx_contextt,
                                      mut Slave: uint16, mut Index: uint16,
                                      mut SubIdx: uint8,
                                      mut AbortCode: int32) {
    let mut Ec: ec_errort =
        ec_errort{Time: ec_timet{sec: 0, usec: 0,},
                  Signal: 0,
                  Slave: 0,
                  Index: 0,
                  SubIdx: 0,
                  Etype: EC_ERR_TYPE_SDO_ERROR,
                  c2rust_unnamed: C2RustUnnamed_3{AbortCode: 0,},};
    memset(&mut Ec as *mut ec_errort as *mut libc::c_void, 0 as libc::c_int,
           ::core::mem::size_of::<ec_errort>() as libc::c_ulong);
    Ec.Time = osal_current_time();
    Ec.Slave = Slave;
    Ec.Index = Index;
    Ec.SubIdx = SubIdx;
    *(*context).ecaterror = 1 as libc::c_int as boolean;
    Ec.Etype = EC_ERR_TYPE_SDO_ERROR;
    Ec.c2rust_unnamed.AbortCode = AbortCode;
    ecx_pusherror(context, &mut Ec);
}
/* * Report SDO info error
 *
 * @param[in]  context    = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index that generated error
 * @param[in]  SubIdx     = Subindex that generated error
 * @param[in]  AbortCode  = Abortcode, see EtherCAT documentation for list
 */
unsafe extern "C" fn ecx_SDOinfoerror(mut context: *mut ecx_contextt,
                                      mut Slave: uint16, mut Index: uint16,
                                      mut SubIdx: uint8,
                                      mut AbortCode: int32) {
    let mut Ec: ec_errort =
        ec_errort{Time: ec_timet{sec: 0, usec: 0,},
                  Signal: 0,
                  Slave: 0,
                  Index: 0,
                  SubIdx: 0,
                  Etype: EC_ERR_TYPE_SDO_ERROR,
                  c2rust_unnamed: C2RustUnnamed_3{AbortCode: 0,},};
    memset(&mut Ec as *mut ec_errort as *mut libc::c_void, 0 as libc::c_int,
           ::core::mem::size_of::<ec_errort>() as libc::c_ulong);
    Ec.Slave = Slave;
    Ec.Index = Index;
    Ec.SubIdx = SubIdx;
    *(*context).ecaterror = 1 as libc::c_int as boolean;
    Ec.Etype = EC_ERR_TYPE_SDOINFO_ERROR;
    Ec.c2rust_unnamed.AbortCode = AbortCode;
    ecx_pusherror(context, &mut Ec);
}
/* * CoE SDO read, blocking. Single subindex or Complete Access.
 *
 * Only a "normal" upload request is issued. If the requested parameter is <= 4bytes
 * then a "expedited" response is returned, otherwise a "normal" response. If a "normal"
 * response is larger than the mailbox size then the response is segmented. The function
 * will combine all segments and copy them to the parameter buffer.
 *
 * @param[in]  context    = context struct
 * @param[in]  slave      = Slave number
 * @param[in]  index      = Index to read
 * @param[in]  subindex   = Subindex to read, must be 0 or 1 if CA is used.
 * @param[in]  CA         = FALSE = single subindex. TRUE = Complete Access, all subindexes read.
 * @param[in,out] psize   = Size in bytes of parameter buffer, returns bytes read from SDO.
 * @param[out] p          = Pointer to parameter buffer
 * @param[in]  timeout    = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_SDOread(mut context: *mut ecx_contextt,
                                     mut slave: uint16, mut index: uint16,
                                     mut subindex: uint8, mut CA: boolean,
                                     mut psize: *mut libc::c_int,
                                     mut p: *mut libc::c_void,
                                     mut timeout: libc::c_int)
 -> libc::c_int {
    let mut SDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut aSDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut bytesize: uint16 = 0;
    let mut Framedatasize: uint16 = 0;
    let mut wkc: libc::c_int = 0;
    let mut SDOlen: int32 = 0;
    let mut bp: *mut uint8 = 0 as *mut uint8;
    let mut hp: *mut uint8 = 0 as *mut uint8;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut toggle: uint8 = 0;
    let mut NotLast: boolean = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc =
        ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft,
                       0 as libc::c_int);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOt;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
    (*SDOp).MbxHeader.length = 0xa as libc::c_int as uint16;
    (*SDOp).MbxHeader.address = 0 as libc::c_int as uint16;
    (*SDOp).MbxHeader.priority = 0 as libc::c_int as uint8;
    /* get new mailbox count value, used as session handle */
    cnt =
        ec_nextmbxcnt((*(*context).slavelist.offset(slave as
                                                        isize)).mbx_cnt); /* CoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt =
        cnt; /* number 9bits service upper 4 bits (SDO request) */
    (*SDOp).MbxHeader.mbxtype =
        (ECT_MBXT_COE as libc::c_int +
             ((cnt as libc::c_int) << 4 as libc::c_int) as uint8 as
                 libc::c_int) as uint8;
    (*SDOp).CANOpen =
        (0 as libc::c_int +
             ((ECT_COES_SDOREQ as libc::c_int) << 12 as libc::c_int)) as
            uint16;
    if CA != 0 {
        (*SDOp).Command = ECT_SDO_UP_REQ_CA as libc::c_int as uint8
        /* upload request complete access */
    } else {
        (*SDOp).Command = ECT_SDO_UP_REQ as libc::c_int as uint8
        /* upload request normal */
    }
    (*SDOp).Index = index;
    if CA as libc::c_int != 0 && subindex as libc::c_int > 1 as libc::c_int {
        subindex = 1 as libc::c_int as uint8
    }
    (*SDOp).SubIndex = subindex;
    (*SDOp).c2rust_unnamed.ldata[0 as libc::c_int as usize] =
        0 as libc::c_int as uint32;
    /* send CoE request to slave */
    wkc =
        ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft,
                    20000 as libc::c_int);
    if wkc > 0 as libc::c_int {
        /* succeeded to place mailbox in slave ? */
        /* clean mailboxbuffer */
        ec_clearmbx(&mut MbxIn);
        wkc =
            ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft,
                           timeout);
        if wkc > 0 as libc::c_int {
            /* read slave response */
            /* succeeded to read slave response ? */
            /* slave response should be CoE, SDO response and the correct index */
            if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xf as libc::c_int
                   == ECT_MBXT_COE as libc::c_int &&
                   (*aSDOp).CANOpen as libc::c_int >> 12 as libc::c_int ==
                       ECT_COES_SDORES as libc::c_int &&
                   (*aSDOp).Index as libc::c_int ==
                       (*SDOp).Index as libc::c_int {
                if (*aSDOp).Command as libc::c_int & 0x2 as libc::c_int >
                       0 as libc::c_int {
                    /* expedited frame response */
                    bytesize =
                        (4 as libc::c_int -
                             ((*aSDOp).Command as libc::c_int >>
                                  2 as libc::c_int & 0x3 as libc::c_int)) as
                            uint16;
                    if *psize >= bytesize as libc::c_int {
                        /* parameter buffer big enough ? */
                        /* copy parameter in parameter buffer */
                        memcpy(p,
                               &mut *(*aSDOp).c2rust_unnamed.ldata.as_mut_ptr().offset(0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                   as *mut uint32 as *const libc::c_void,
                               bytesize as libc::c_ulong);
                        *psize = bytesize as libc::c_int
                    } else {
                        wkc = 0 as libc::c_int;
                        ecx_packeterror(context, slave, index, subindex,
                                        3 as libc::c_int as uint16);
                        /* return the real parameter size */
                        /*  data container too small for type */
                    }
                } else {
                    /* normal frame response */
                    SDOlen =
                        (*aSDOp).c2rust_unnamed.ldata[0 as libc::c_int as
                                                          usize] as int32;
                    /* Does parameter fit in parameter buffer ? */
                    if SDOlen <= *psize {
                        bp = p as *mut uint8;
                        hp = p as *mut uint8;
                        /* calculate mailbox transfer size */
                        Framedatasize =
                            ((*aSDOp).MbxHeader.length as libc::c_int -
                                 10 as libc::c_int) as uint16;
                        if (Framedatasize as libc::c_int) < SDOlen {
                            /* transfer in segments? */
                            /* copy parameter data in parameter buffer */
                            memcpy(hp as *mut libc::c_void,
                                   &mut *(*aSDOp).c2rust_unnamed.ldata.as_mut_ptr().offset(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)
                                       as *mut uint32 as *const libc::c_void,
                                   Framedatasize as libc::c_ulong);
                            hp =
                                hp.offset(Framedatasize as libc::c_int as
                                              isize);
                            *psize = Framedatasize as libc::c_int;
                            NotLast = 1 as libc::c_int as boolean;
                            toggle = 0 as libc::c_int as uint8;
                            while NotLast != 0 {
                                /* increment buffer pointer */
                                /* segmented transfer */
                                SDOp =
                                    &mut MbxOut as *mut ec_mbxbuft as
                                        *mut ec_SDOt;
                                (*SDOp).MbxHeader.length =
                                    0xa as libc::c_int as uint16;
                                (*SDOp).MbxHeader.address =
                                    0 as libc::c_int as uint16;
                                (*SDOp).MbxHeader.priority =
                                    0 as libc::c_int as uint8;
                                cnt =
                                    ec_nextmbxcnt((*(*context).slavelist.offset(slave
                                                                                    as
                                                                                    isize)).mbx_cnt);
                                (*(*context).slavelist.offset(slave as
                                                                  isize)).mbx_cnt
                                    = cnt;
                                /* toggle bit for segment request */
                                (*SDOp).MbxHeader.mbxtype =
                                    (ECT_MBXT_COE as libc::c_int +
                                         ((cnt as libc::c_int) <<
                                              4 as libc::c_int) as uint8 as
                                             libc::c_int) as uint8; /* CoE */
                                (*SDOp).CANOpen =
                                    (0 as libc::c_int +
                                         ((ECT_COES_SDOREQ as libc::c_int) <<
                                              12 as libc::c_int)) as
                                        uint16; /* number 9bits service upper 4 bits (SDO request) */
                                (*SDOp).Command =
                                    (ECT_SDO_SEG_UP_REQ as libc::c_int +
                                         toggle as libc::c_int) as
                                        uint8; /* segment upload request */
                                (*SDOp).Index = index;
                                (*SDOp).SubIndex = subindex;
                                (*SDOp).c2rust_unnamed.ldata[0 as libc::c_int
                                                                 as usize] =
                                    0 as libc::c_int as uint32;
                                wkc =
                                    ecx_mbxsend(context, slave,
                                                &mut MbxOut as
                                                    *mut ec_mbxbuft,
                                                20000 as libc::c_int);
                                if wkc > 0 as libc::c_int {
                                    ec_clearmbx(&mut MbxIn);
                                    /* send segmented upload request to slave */
                                    /* is mailbox transferred to slave ? */
                                    /* read slave response */
                                    wkc =
                                        ecx_mbxreceive(context, slave,
                                                       &mut MbxIn as
                                                           *mut ec_mbxbuft,
                                                       timeout);
                                    /* has slave responded ? */
                                    if wkc > 0 as libc::c_int {
                                        /* slave response should be CoE, SDO response */
                                        if (*aSDOp).MbxHeader.mbxtype as
                                               libc::c_int &
                                               0xf as libc::c_int ==
                                               ECT_MBXT_COE as libc::c_int &&
                                               (*aSDOp).CANOpen as libc::c_int
                                                   >> 12 as libc::c_int ==
                                                   ECT_COES_SDORES as
                                                       libc::c_int &&
                                               (*aSDOp).Command as libc::c_int
                                                   & 0xe0 as libc::c_int ==
                                                   0 as libc::c_int {
                                            /* calculate mailbox transfer size */
                                            Framedatasize =
                                                ((*aSDOp).MbxHeader.length as
                                                     libc::c_int -
                                                     3 as libc::c_int) as
                                                    uint16;
                                            if (*aSDOp).Command as libc::c_int
                                                   & 0x1 as libc::c_int >
                                                   0 as libc::c_int {
                                                /* last segment */
                                                NotLast =
                                                    0 as libc::c_int as
                                                        boolean;
                                                if Framedatasize as
                                                       libc::c_int ==
                                                       7 as libc::c_int {
                                                    /* subtract unused bytes from frame */
                                                    Framedatasize =
                                                        (Framedatasize as
                                                             libc::c_int -
                                                             (((*aSDOp).Command
                                                                   as
                                                                   libc::c_int
                                                                   &
                                                                   0xe as
                                                                       libc::c_int)
                                                                  >>
                                                                  1 as
                                                                      libc::c_int))
                                                            as uint16
                                                }
                                                /* copy to parameter buffer */
                                                memcpy(hp as
                                                           *mut libc::c_void,
                                                       &mut (*aSDOp).Index as
                                                           *mut uint16 as
                                                           *const libc::c_void,
                                                       Framedatasize as
                                                           libc::c_ulong);
                                            } else {
                                                /* segments follow */
                                                /* copy to parameter buffer */
                                                memcpy(hp as
                                                           *mut libc::c_void,
                                                       &mut (*aSDOp).Index as
                                                           *mut uint16 as
                                                           *const libc::c_void,
                                                       Framedatasize as
                                                           libc::c_ulong);
                                                hp =
                                                    hp.offset(Framedatasize as
                                                                  libc::c_int
                                                                  as isize)
                                            }
                                            /* increment buffer pointer */
                                            /* update parameter size */
                                            *psize +=
                                                Framedatasize as libc::c_int
                                        } else {
                                            /* unexpected frame returned from slave */
                                            NotLast =
                                                0 as libc::c_int as
                                                    boolean; /* Unexpected frame returned */
                                            if (*aSDOp).Command as libc::c_int
                                                   ==
                                                   ECT_SDO_ABORT as
                                                       libc::c_int {
                                                /* SDO abort frame received */
                                                ecx_SDOerror(context, slave,
                                                             index, subindex,
                                                             (*aSDOp).c2rust_unnamed.ldata[0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               usize]
                                                                 as int32);
                                            } else {
                                                ecx_packeterror(context,
                                                                slave, index,
                                                                subindex,
                                                                1 as
                                                                    libc::c_int
                                                                    as
                                                                    uint16);
                                            }
                                            wkc = 0 as libc::c_int
                                        }
                                    }
                                }
                                toggle =
                                    (toggle as libc::c_int ^
                                         0x10 as libc::c_int) as uint8
                            }
                        } else {
                            /* non segmented transfer */
                            /* copy to parameter buffer */
                            memcpy(bp as *mut libc::c_void,
                                   &mut *(*aSDOp).c2rust_unnamed.ldata.as_mut_ptr().offset(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)
                                       as *mut uint32 as *const libc::c_void,
                                   SDOlen as libc::c_ulong);
                            *psize = SDOlen
                        }
                    } else {
                        /* parameter buffer too small */
                        wkc = 0 as libc::c_int;
                        ecx_packeterror(context, slave, index, subindex,
                                        3 as libc::c_int as uint16);
                        /*  data container too small for type */
                    }
                }
            } else {
                /* other slave response */
                if (*aSDOp).Command as libc::c_int ==
                       ECT_SDO_ABORT as libc::c_int {
                    /* SDO abort frame received */
                    ecx_SDOerror(context, slave, index, subindex,
                                 (*aSDOp).c2rust_unnamed.ldata[0 as
                                                                   libc::c_int
                                                                   as usize]
                                     as int32);
                } else {
                    ecx_packeterror(context, slave, index, subindex,
                                    1 as libc::c_int as uint16);
                    /* Unexpected frame returned */
                }
                wkc = 0 as libc::c_int
            }
        }
    }
    return wkc;
}
/* * CoE SDO write, blocking. Single subindex or Complete Access.
 *
 * A "normal" download request is issued, unless we have
 * small data, then a "expedited" transfer is used. If the parameter is larger than
 * the mailbox size then the download is segmented. The function will split the
 * parameter data in segments and send them to the slave one by one.
 *
 * @param[in]  context    = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index to write
 * @param[in]  SubIndex   = Subindex to write, must be 0 or 1 if CA is used.
 * @param[in]  CA         = FALSE = single subindex. TRUE = Complete Access, all subindexes written.
 * @param[in]  psize      = Size in bytes of parameter buffer.
 * @param[out] p          = Pointer to parameter buffer
 * @param[in]  Timeout    = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_SDOwrite(mut context: *mut ecx_contextt,
                                      mut Slave: uint16, mut Index: uint16,
                                      mut SubIndex: uint8, mut CA: boolean,
                                      mut psize: libc::c_int,
                                      mut p: *mut libc::c_void,
                                      mut Timeout: libc::c_int)
 -> libc::c_int {
    let mut SDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut aSDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut wkc: libc::c_int = 0;
    let mut maxdata: libc::c_int = 0;
    let mut framedatasize: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut toggle: uint8 = 0;
    let mut NotLast: boolean = 0;
    let mut hp: *mut uint8 = 0 as *mut uint8;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc =
        ecx_mbxreceive(context, Slave, &mut MbxIn as *mut ec_mbxbuft,
                       0 as
                           libc::c_int); /* data section=mailbox size - 6 mbx - 2 CoE - 8 sdo req */
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOt;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
    maxdata =
        (*(*context).slavelist.offset(Slave as isize)).mbx_l as libc::c_int -
            0x10 as libc::c_int;
    /* if small data use expedited transfer */
    if psize <= 4 as libc::c_int && CA == 0 {
        (*SDOp).MbxHeader.length = 0xa as libc::c_int as uint16;
        (*SDOp).MbxHeader.address = 0 as libc::c_int as uint16;
        (*SDOp).MbxHeader.priority = 0 as libc::c_int as uint8;
        /* get new mailbox counter, used for session handle */
        cnt =
            ec_nextmbxcnt((*(*context).slavelist.offset(Slave as
                                                            isize)).mbx_cnt); /* CoE */
        (*(*context).slavelist.offset(Slave as isize)).mbx_cnt =
            cnt; /* number 9bits service upper 4 bits */
        (*SDOp).MbxHeader.mbxtype =
            (ECT_MBXT_COE as libc::c_int +
                 ((cnt as libc::c_int) << 4 as libc::c_int) as uint8 as
                     libc::c_int) as
                uint8; /* expedited SDO download transfer */
        (*SDOp).CANOpen =
            (0 as libc::c_int +
                 ((ECT_COES_SDOREQ as libc::c_int) << 12 as libc::c_int)) as
                uint16;
        (*SDOp).Command =
            (ECT_SDO_DOWN_EXP as libc::c_int |
                 4 as libc::c_int - psize << 2 as libc::c_int &
                     0xc as libc::c_int) as uint8;
        (*SDOp).Index = Index;
        (*SDOp).SubIndex = SubIndex;
        hp = p as *mut uint8;
        /* copy parameter data to mailbox */
        memcpy(&mut *(*SDOp).c2rust_unnamed.ldata.as_mut_ptr().offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                   as *mut uint32 as *mut libc::c_void,
               hp as *const libc::c_void, psize as libc::c_ulong);
        /* send mailbox SDO download request to slave */
        wkc =
            ecx_mbxsend(context, Slave, &mut MbxOut as *mut ec_mbxbuft,
                        20000 as libc::c_int);
        if wkc > 0 as libc::c_int {
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc =
                ecx_mbxreceive(context, Slave, &mut MbxIn as *mut ec_mbxbuft,
                               Timeout);
            if wkc > 0 as libc::c_int {
                /* response should be CoE, SDO response, correct index and subindex */
                if !((*aSDOp).MbxHeader.mbxtype as libc::c_int &
                         0xf as libc::c_int == ECT_MBXT_COE as libc::c_int &&
                         (*aSDOp).CANOpen as libc::c_int >> 12 as libc::c_int
                             == ECT_COES_SDORES as libc::c_int &&
                         (*aSDOp).Index as libc::c_int ==
                             (*SDOp).Index as libc::c_int &&
                         (*aSDOp).SubIndex as libc::c_int ==
                             (*SDOp).SubIndex as libc::c_int) {
                    /* unexpected response from slave */
                    if (*aSDOp).Command as libc::c_int ==
                           ECT_SDO_ABORT as libc::c_int {
                        /* SDO abort frame received */
                        ecx_SDOerror(context, Slave, Index, SubIndex,
                                     (*aSDOp).c2rust_unnamed.ldata[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                         as int32);
                    } else {
                        ecx_packeterror(context, Slave, Index, SubIndex,
                                        1 as libc::c_int as uint16);
                        /* Unexpected frame returned */
                    } /*  segmented transfer needed  */
                    wkc = 0 as libc::c_int
                }
            }
        }
    } else {
        framedatasize = psize;
        NotLast = 0 as libc::c_int as boolean;
        if framedatasize > maxdata {
            framedatasize = maxdata;
            NotLast = 1 as libc::c_int as boolean
        }
        (*SDOp).MbxHeader.length =
            (0xa as libc::c_int + framedatasize) as uint16;
        (*SDOp).MbxHeader.address = 0 as libc::c_int as uint16;
        (*SDOp).MbxHeader.priority = 0 as libc::c_int as uint8;
        /* get new mailbox counter, used for session handle */
        cnt =
            ec_nextmbxcnt((*(*context).slavelist.offset(Slave as
                                                            isize)).mbx_cnt); /* CoE */
        (*(*context).slavelist.offset(Slave as isize)).mbx_cnt =
            cnt; /* number 9bits service upper 4 bits */
        (*SDOp).MbxHeader.mbxtype =
            (ECT_MBXT_COE as libc::c_int +
                 ((cnt as libc::c_int) << 4 as libc::c_int) as uint8 as
                     libc::c_int) as uint8;
        (*SDOp).CANOpen =
            (0 as libc::c_int +
                 ((ECT_COES_SDOREQ as libc::c_int) << 12 as libc::c_int)) as
                uint16;
        if CA != 0 {
            (*SDOp).Command = ECT_SDO_DOWN_INIT_CA as libc::c_int as uint8
            /* Complete Access, normal SDO init download transfer */
        } else {
            (*SDOp).Command = ECT_SDO_DOWN_INIT as libc::c_int as uint8
            /* normal SDO init download transfer */
        }
        (*SDOp).Index = Index;
        (*SDOp).SubIndex = SubIndex;
        if CA as libc::c_int != 0 &&
               SubIndex as libc::c_int > 1 as libc::c_int {
            (*SDOp).SubIndex = 1 as libc::c_int as uint8
        }
        (*SDOp).c2rust_unnamed.ldata[0 as libc::c_int as usize] =
            psize as uint32;
        hp = p as *mut uint8;
        /* copy parameter data to mailbox */
        memcpy(&mut *(*SDOp).c2rust_unnamed.ldata.as_mut_ptr().offset(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                   as *mut uint32 as *mut libc::c_void,
               hp as *const libc::c_void, framedatasize as libc::c_ulong);
        hp = hp.offset(framedatasize as isize);
        psize -= framedatasize;
        /* send mailbox SDO download request to slave */
        wkc =
            ecx_mbxsend(context, Slave, &mut MbxOut as *mut ec_mbxbuft,
                        20000 as libc::c_int);
        if wkc > 0 as libc::c_int {
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc =
                ecx_mbxreceive(context, Slave, &mut MbxIn as *mut ec_mbxbuft,
                               Timeout);
            if wkc > 0 as libc::c_int {
                /* response should be CoE, SDO response, correct index and subindex */
                if (*aSDOp).MbxHeader.mbxtype as libc::c_int &
                       0xf as libc::c_int == ECT_MBXT_COE as libc::c_int &&
                       (*aSDOp).CANOpen as libc::c_int >> 12 as libc::c_int ==
                           ECT_COES_SDORES as libc::c_int &&
                       (*aSDOp).Index as libc::c_int ==
                           (*SDOp).Index as libc::c_int &&
                       (*aSDOp).SubIndex as libc::c_int ==
                           (*SDOp).SubIndex as libc::c_int {
                    /* all ok */
                    maxdata += 7 as libc::c_int;
                    toggle = 0 as libc::c_int as uint8;
                    /* repeat while segments left */
                    while NotLast != 0 {
                        SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
                        framedatasize = psize;
                        NotLast = 0 as libc::c_int as boolean;
                        /* toggle bit for segment request */
                        (*SDOp).Command =
                            0x1 as libc::c_int as uint8; /* last segment */
                        if framedatasize > maxdata {
                            framedatasize = maxdata;
                            NotLast = 1 as libc::c_int as boolean;
                            (*SDOp).Command =
                                0 as libc::c_int as
                                    uint8 /*  more segments needed  */
                            /* segments follow */
                        } /* minimum size */
                        if NotLast == 0 && framedatasize < 7 as libc::c_int {
                            (*SDOp).MbxHeader.length =
                                0xa as libc::c_int as uint16;
                            (*SDOp).Command =
                                (0x1 as libc::c_int +
                                     (7 as libc::c_int - framedatasize <<
                                          1 as libc::c_int)) as uint8
                            /* last segment reduced octets */
                        } else {
                            (*SDOp).MbxHeader.length =
                                (framedatasize + 3 as libc::c_int) as uint16
                            /* data + 2 CoE + 1 SDO */
                        }
                        (*SDOp).MbxHeader.address =
                            0 as libc::c_int as uint16;
                        (*SDOp).MbxHeader.priority =
                            0 as libc::c_int as uint8;
                        cnt =
                            ec_nextmbxcnt((*(*context).slavelist.offset(Slave
                                                                            as
                                                                            isize)).mbx_cnt);
                        (*(*context).slavelist.offset(Slave as isize)).mbx_cnt
                            = cnt;
                        (*SDOp).MbxHeader.mbxtype =
                            (ECT_MBXT_COE as libc::c_int +
                                 ((cnt as libc::c_int) << 4 as libc::c_int) as
                                     uint8 as libc::c_int) as uint8;
                        (*SDOp).CANOpen =
                            (0 as libc::c_int +
                                 ((ECT_COES_SDOREQ as libc::c_int) <<
                                      12 as libc::c_int)) as uint16;
                        (*SDOp).Command =
                            ((*SDOp).Command as libc::c_int +
                                 toggle as libc::c_int) as uint8;
                        memcpy(&mut (*SDOp).Index as *mut uint16 as
                                   *mut libc::c_void,
                               hp as *const libc::c_void,
                               framedatasize as libc::c_ulong);
                        hp = hp.offset(framedatasize as isize);
                        psize -= framedatasize;
                        wkc =
                            ecx_mbxsend(context, Slave,
                                        &mut MbxOut as *mut ec_mbxbuft,
                                        20000 as libc::c_int);
                        if wkc > 0 as libc::c_int {
                            ec_clearmbx(&mut MbxIn);
                            /* get new mailbox counter value */
                            /* CoE */
                            /* number 9bits service upper 4 bits (SDO request) */
                            /* add toggle bit to command byte */
                            /* copy parameter data to mailbox */
                            /* update parameter buffer pointer */
                            /* send SDO download request */
                            /* read slave response */
                            wkc =
                                ecx_mbxreceive(context, Slave,
                                               &mut MbxIn as *mut ec_mbxbuft,
                                               Timeout);
                            if wkc > 0 as libc::c_int {
                                if !((*aSDOp).MbxHeader.mbxtype as libc::c_int
                                         & 0xf as libc::c_int ==
                                         ECT_MBXT_COE as libc::c_int &&
                                         (*aSDOp).CANOpen as libc::c_int >>
                                             12 as libc::c_int ==
                                             ECT_COES_SDORES as libc::c_int &&
                                         (*aSDOp).Command as libc::c_int &
                                             0xe0 as libc::c_int ==
                                             0x20 as libc::c_int) {
                                    if (*aSDOp).Command as libc::c_int ==
                                           ECT_SDO_ABORT as libc::c_int {
                                        /* SDO abort frame received */
                                        ecx_SDOerror(context, Slave, Index,
                                                     SubIndex,
                                                     (*aSDOp).c2rust_unnamed.ldata[0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                                                         as int32);
                                    } else {
                                        ecx_packeterror(context, Slave, Index,
                                                        SubIndex,
                                                        1 as libc::c_int as
                                                            uint16);
                                        /* Unexpected frame returned */
                                    }
                                    wkc = 0 as libc::c_int;
                                    NotLast = 0 as libc::c_int as boolean
                                }
                            }
                        }
                        toggle =
                            (toggle as libc::c_int ^ 0x10 as libc::c_int) as
                                uint8
                    }
                } else {
                    /* unexpected response from slave */
                    if (*aSDOp).Command as libc::c_int ==
                           ECT_SDO_ABORT as libc::c_int {
                        /* SDO abort frame received */
                        ecx_SDOerror(context, Slave, Index, SubIndex,
                                     (*aSDOp).c2rust_unnamed.ldata[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                         as int32);
                    } else {
                        ecx_packeterror(context, Slave, Index, SubIndex,
                                        1 as libc::c_int as uint16);
                        /* Unexpected frame returned */
                    }
                    wkc = 0 as libc::c_int
                }
            }
        }
    }
    return wkc;
}
/* * CoE RxPDO write, blocking.
 *
 * A RxPDO download request is issued.
 *
 * @param[in]  context       = context struct
 * @param[in]  Slave         = Slave number
 * @param[in]  RxPDOnumber   = Related RxPDO number
 * @param[in]  psize         = Size in bytes of PDO buffer.
 * @param[out] p             = Pointer to PDO buffer
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_RxPDO(mut context: *mut ecx_contextt,
                                   mut Slave: uint16, mut RxPDOnumber: uint16,
                                   mut psize: libc::c_int,
                                   mut p: *mut libc::c_void) -> libc::c_int {
    let mut SDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut wkc: libc::c_int = 0;
    let mut maxdata: libc::c_int = 0;
    let mut framedatasize: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc =
        ecx_mbxreceive(context, Slave, &mut MbxIn as *mut ec_mbxbuft,
                       0 as
                           libc::c_int); /* data section=mailbox size - 6 mbx - 2 CoE */
    ec_clearmbx(&mut MbxOut);
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
    maxdata =
        (*(*context).slavelist.offset(Slave as isize)).mbx_l as libc::c_int -
            0x8 as libc::c_int;
    framedatasize = psize;
    if framedatasize > maxdata {
        framedatasize = maxdata
        /*  limit transfer */
    }
    (*SDOp).MbxHeader.length = (0x2 as libc::c_int + framedatasize) as uint16;
    (*SDOp).MbxHeader.address = 0 as libc::c_int as uint16;
    (*SDOp).MbxHeader.priority = 0 as libc::c_int as uint8;
    /* get new mailbox counter, used for session handle */
    cnt =
        ec_nextmbxcnt((*(*context).slavelist.offset(Slave as
                                                        isize)).mbx_cnt); /* CoE */
    (*(*context).slavelist.offset(Slave as isize)).mbx_cnt =
        cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype =
        (ECT_MBXT_COE as libc::c_int +
             ((cnt as libc::c_int) << 4 as libc::c_int) as uint8 as
                 libc::c_int) as uint8;
    (*SDOp).CANOpen =
        ((RxPDOnumber as libc::c_int & 0x1ff as libc::c_int) +
             ((ECT_COES_RXPDO as libc::c_int) << 12 as libc::c_int)) as
            uint16;
    /* copy PDO data to mailbox */
    memcpy(&mut (*SDOp).Command as *mut uint8 as *mut libc::c_void, p,
           framedatasize as libc::c_ulong);
    /* send mailbox RxPDO request to slave */
    wkc =
        ecx_mbxsend(context, Slave, &mut MbxOut as *mut ec_mbxbuft,
                    20000 as libc::c_int);
    return wkc;
}
/* * CoE TxPDO read remote request, blocking.
 *
 * A RxPDO download request is issued.
 *
 * @param[in]  context       = context struct
 * @param[in]  slave         = Slave number
 * @param[in]  TxPDOnumber   = Related TxPDO number
 * @param[in,out] psize      = Size in bytes of PDO buffer, returns bytes read from PDO.
 * @param[out] p             = Pointer to PDO buffer
 * @param[in]  timeout       = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_TxPDO(mut context: *mut ecx_contextt,
                                   mut slave: uint16, mut TxPDOnumber: uint16,
                                   mut psize: *mut libc::c_int,
                                   mut p: *mut libc::c_void,
                                   mut timeout: libc::c_int) -> libc::c_int {
    let mut SDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut aSDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut wkc: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut framedatasize: uint16 = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc =
        ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft,
                       0 as libc::c_int);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOt;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
    (*SDOp).MbxHeader.length = 0x2 as libc::c_int as uint16;
    (*SDOp).MbxHeader.address = 0 as libc::c_int as uint16;
    (*SDOp).MbxHeader.priority = 0 as libc::c_int as uint8;
    /* get new mailbox counter, used for session handle */
    cnt =
        ec_nextmbxcnt((*(*context).slavelist.offset(slave as
                                                        isize)).mbx_cnt); /* CoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt =
        cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype =
        (ECT_MBXT_COE as libc::c_int +
             ((cnt as libc::c_int) << 4 as libc::c_int) as uint8 as
                 libc::c_int) as uint8;
    (*SDOp).CANOpen =
        ((TxPDOnumber as libc::c_int & 0x1ff as libc::c_int) +
             ((ECT_COES_TXPDO_RR as libc::c_int) << 12 as libc::c_int)) as
            uint16;
    wkc =
        ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft,
                    20000 as libc::c_int);
    if wkc > 0 as libc::c_int {
        /* clean mailboxbuffer */
        ec_clearmbx(&mut MbxIn);
        /* read slave response */
        wkc =
            ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft,
                           timeout);
        if wkc > 0 as libc::c_int {
            /* succeeded to read slave response ? */
            /* slave response should be CoE, TxPDO */
            if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xf as libc::c_int
                   == ECT_MBXT_COE as libc::c_int &&
                   (*aSDOp).CANOpen as libc::c_int >> 12 as libc::c_int ==
                       ECT_COES_TXPDO as libc::c_int {
                /* TxPDO response */
                framedatasize =
                    ((*aSDOp).MbxHeader.length as libc::c_int -
                         2 as libc::c_int) as uint16;
                if *psize >= framedatasize as libc::c_int {
                    /* parameter buffer big enough ? */
                    /* copy parameter in parameter buffer */
                    memcpy(p,
                           &mut (*aSDOp).Command as *mut uint8 as
                               *const libc::c_void,
                           framedatasize as libc::c_ulong);
                    *psize = framedatasize as libc::c_int
                } else {
                    /* return the real parameter size */
                    /* parameter buffer too small */
                    wkc = 0 as libc::c_int;
                    ecx_packeterror(context, slave,
                                    0 as libc::c_int as uint16,
                                    0 as libc::c_int as uint8,
                                    3 as libc::c_int as uint16);
                    /*  data container too small for type */
                }
            } else {
                /* other slave response */
                if (*aSDOp).Command as libc::c_int ==
                       ECT_SDO_ABORT as libc::c_int {
                    /* SDO abort frame received */
                    ecx_SDOerror(context, slave, 0 as libc::c_int as uint16,
                                 0 as libc::c_int as uint8,
                                 (*aSDOp).c2rust_unnamed.ldata[0 as
                                                                   libc::c_int
                                                                   as usize]
                                     as int32);
                } else {
                    ecx_packeterror(context, slave,
                                    0 as libc::c_int as uint16,
                                    0 as libc::c_int as uint8,
                                    1 as libc::c_int as uint16);
                    /* Unexpected frame returned */
                }
                wkc = 0 as libc::c_int
            }
        }
    }
    return wkc;
}
/* * Read PDO assign structure
 * @param[in]  context       = context struct
 * @param[in]  Slave         = Slave number
 * @param[in]  PDOassign     = PDO assign object
 * @return total bitlength of PDO assign
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readPDOassign(mut context: *mut ecx_contextt,
                                           mut Slave: uint16,
                                           mut PDOassign: uint16) -> uint32 {
    let mut idxloop: uint16 = 0;
    let mut nidx: uint16 = 0;
    let mut subidxloop: uint16 = 0;
    let mut rdat: uint16 = 0;
    let mut idx: uint16 = 0;
    let mut subidx: uint16 = 0;
    let mut subcnt: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut rdat2: int32 = 0;
    let mut bsize: uint32 = 0 as libc::c_int as uint32;
    rdl = ::core::mem::size_of::<uint16>() as libc::c_ulong as libc::c_int;
    rdat = 0 as libc::c_int as uint16;
    /* read PDO assign subindex 0 ( = number of PDO's) */
    wkc =
        ecx_SDOread(context, Slave, PDOassign, 0 as libc::c_int as uint8,
                    0 as libc::c_int as boolean, &mut rdl,
                    &mut rdat as *mut uint16 as *mut libc::c_void,
                    700000 as libc::c_int);
    rdat = rdat;
    /* positive result from slave ? */
    if wkc > 0 as libc::c_int && rdat as libc::c_int > 0 as libc::c_int {
        /* number of available sub indexes */
        nidx = rdat;
        bsize = 0 as libc::c_int as uint32;
        /* read all PDO's */
        idxloop = 1 as libc::c_int as uint16;
        while idxloop as libc::c_int <= nidx as libc::c_int {
            rdl =
                ::core::mem::size_of::<uint16>() as libc::c_ulong as
                    libc::c_int;
            rdat = 0 as libc::c_int as uint16;
            /* read PDO assign */
            wkc =
                ecx_SDOread(context, Slave, PDOassign, idxloop as uint8,
                            0 as libc::c_int as boolean, &mut rdl,
                            &mut rdat as *mut uint16 as *mut libc::c_void,
                            700000 as libc::c_int);
            /* result is index of PDO */
            idx = rdat;
            if idx as libc::c_int > 0 as libc::c_int {
                rdl =
                    ::core::mem::size_of::<uint8>() as libc::c_ulong as
                        libc::c_int;
                subcnt = 0 as libc::c_int as uint8;
                /* read number of subindexes of PDO */
                wkc =
                    ecx_SDOread(context, Slave, idx,
                                0 as libc::c_int as uint8,
                                0 as libc::c_int as boolean, &mut rdl,
                                &mut subcnt as *mut uint8 as
                                    *mut libc::c_void, 700000 as libc::c_int);
                subidx = subcnt as uint16;
                /* for each subindex */
                subidxloop = 1 as libc::c_int as uint16;
                while subidxloop as libc::c_int <= subidx as libc::c_int {
                    rdl =
                        ::core::mem::size_of::<int32>() as libc::c_ulong as
                            libc::c_int;
                    rdat2 = 0 as libc::c_int;
                    /* read SDO that is mapped in PDO */
                    wkc =
                        ecx_SDOread(context, Slave, idx, subidxloop as uint8,
                                    0 as libc::c_int as boolean, &mut rdl,
                                    &mut rdat2 as *mut int32 as
                                        *mut libc::c_void,
                                    700000 as libc::c_int);
                    rdat2 = rdat2;
                    /* extract bitlength of SDO */
                    if (rdat2 & 0xff as libc::c_int) < 0xff as libc::c_int {
                        bsize =
                            (bsize as
                                 libc::c_uint).wrapping_add((rdat2 &
                                                                 0xff as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                as uint32 as uint32
                    } else {
                        rdl =
                            ::core::mem::size_of::<uint16>() as libc::c_ulong
                                as libc::c_int;
                        rdat = 0xff as libc::c_int as uint16;
                        /* read Object Entry in Object database */
//                  wkc = ec_readOEsingle(idx, (uint8)SubCount, pODlist, pOElist);
                        bsize =
                            (bsize as
                                 libc::c_uint).wrapping_add(rdat as
                                                                libc::c_uint)
                                as uint32 as uint32
                    }
                    subidxloop = subidxloop.wrapping_add(1)
                }
            }
            idxloop = idxloop.wrapping_add(1)
        }
    }
    /* return total found bitlength (PDO) */
    return bsize;
}
/* * Read PDO assign structure in Complete Access mode
 * @param[in]  context       = context struct
 * @param[in]  Slave         = Slave number
 * @param[in]  Thread_n      = Calling thread index
 * @param[in]  PDOassign     = PDO assign object
 * @return total bitlength of PDO assign
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readPDOassignCA(mut context: *mut ecx_contextt,
                                             mut Slave: uint16,
                                             mut Thread_n: libc::c_int,
                                             mut PDOassign: uint16)
 -> uint32 {
    let mut idxloop: uint16 = 0;
    let mut nidx: uint16 = 0;
    let mut subidxloop: uint16 = 0;
    let mut idx: uint16 = 0;
    let mut subidx: uint16 = 0;
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut bsize: uint32 = 0 as libc::c_int as uint32;
    /* find maximum size of PDOassign buffer */
    rdl =
        ::core::mem::size_of::<ec_PDOassignt>() as libc::c_ulong as
            libc::c_int;
    (*(*context).PDOassign.offset(Thread_n as isize)).n =
        0 as libc::c_int as uint8;
    /* read rxPDOassign in CA mode, all subindexes are read in one struct */
    wkc =
        ecx_SDOread(context, Slave, PDOassign, 0 as libc::c_int as uint8,
                    1 as libc::c_int as boolean, &mut rdl,
                    &mut *(*context).PDOassign.offset(Thread_n as isize) as
                        *mut ec_PDOassignt as *mut libc::c_void,
                    700000 as libc::c_int);
    /* positive result from slave ? */
    if wkc > 0 as libc::c_int &&
           (*(*context).PDOassign.offset(Thread_n as isize)).n as libc::c_int
               > 0 as libc::c_int {
        nidx = (*(*context).PDOassign.offset(Thread_n as isize)).n as uint16;
        bsize = 0 as libc::c_int as uint32;
        /* for each PDO do */
        idxloop = 1 as libc::c_int as uint16;
        while idxloop as libc::c_int <= nidx as libc::c_int {
            /* get index from PDOassign struct */
            idx =
                (*(*context).PDOassign.offset(Thread_n as
                                                  isize)).index[(idxloop as
                                                                     libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as usize];
            if idx as libc::c_int > 0 as libc::c_int {
                rdl =
                    ::core::mem::size_of::<ec_PDOdesct>() as libc::c_ulong as
                        libc::c_int;
                (*(*context).PDOdesc.offset(Thread_n as isize)).n =
                    0 as libc::c_int as uint8;
                /* read SDO's that are mapped in PDO, CA mode */
                wkc =
                    ecx_SDOread(context, Slave, idx,
                                0 as libc::c_int as uint8,
                                1 as libc::c_int as boolean, &mut rdl,
                                &mut *(*context).PDOdesc.offset(Thread_n as
                                                                    isize) as
                                    *mut ec_PDOdesct as *mut libc::c_void,
                                700000 as libc::c_int);
                subidx =
                    (*(*context).PDOdesc.offset(Thread_n as isize)).n as
                        uint16;
                /* extract all bitlengths of SDO's */
                subidxloop = 1 as libc::c_int as uint16;
                while subidxloop as libc::c_int <= subidx as libc::c_int {
                    bsize =
                        (bsize as
                             libc::c_uint).wrapping_add((*(*context).PDOdesc.offset(Thread_n
                                                                                        as
                                                                                        isize)).PDO[(subidxloop
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         -
                                                                                                         1
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                        as
                                                                                                        usize]
                                                            &
                                                            0xff as
                                                                libc::c_int as
                                                                libc::c_uint)
                            as uint32 as uint32;
                    subidxloop = subidxloop.wrapping_add(1)
                }
            }
            idxloop = idxloop.wrapping_add(1)
        }
    }
    /* return total found bitlength (PDO) */
    return bsize;
}
/* * CoE read PDO mapping.
 *
 * CANopen has standard indexes defined for PDO mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave.
 *
 * Principal structure in slave:\n
 * 1C00:00 is number of SM defined\n
 * 1C00:01 SM0 type -> 1C10\n
 * 1C00:02 SM1 type -> 1C11\n
 * 1C00:03 SM2 type -> 1C12\n
 * 1C00:04 SM3 type -> 1C13\n
 * Type 0 = unused, 1 = mailbox in, 2 = mailbox out,
 * 3 = outputs (RxPDO), 4 = inputs (TxPDO).
 *
 * 1C12:00 is number of PDO's defined for SM2\n
 * 1C12:01 PDO assign SDO #1 -> f.e. 1A00\n
 * 1C12:02 PDO assign SDO #2 -> f.e. 1A04\
 *
 * 1A00:00 is number of object defined for this PDO\n
 * 1A00:01 object mapping #1, f.e. 60100710 (SDO 6010 SI 07 bitlength 0x10)
 *
 * @param[in]  context = context struct
 * @param[in]  Slave   = Slave number
 * @param[out] Osize   = Size in bits of output mapping (rxPDO) found
 * @param[out] Isize   = Size in bits of input mapping (txPDO) found
 * @return >0 if mapping successful.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readPDOmap(mut context: *mut ecx_contextt,
                                        mut Slave: uint16,
                                        mut Osize: *mut uint32,
                                        mut Isize: *mut uint32)
 -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut retVal: libc::c_int = 0 as libc::c_int;
    let mut nSM: uint8 = 0;
    let mut iSM: uint8 = 0;
    let mut tSM: uint8 = 0;
    let mut Tsize: uint32 = 0;
    let mut SMt_bug_add: uint8 = 0;
    *Isize = 0 as libc::c_int as uint32;
    *Osize = 0 as libc::c_int as uint32;
    SMt_bug_add = 0 as libc::c_int as uint8;
    rdl = ::core::mem::size_of::<uint8>() as libc::c_ulong as libc::c_int;
    nSM = 0 as libc::c_int as uint8;
    /* read SyncManager Communication Type object count */
    wkc =
        ecx_SDOread(context, Slave, 0x1c00 as libc::c_int as uint16,
                    0 as libc::c_int as uint8, 0 as libc::c_int as boolean,
                    &mut rdl, &mut nSM as *mut uint8 as *mut libc::c_void,
                    700000 as libc::c_int);
    /* positive result from slave ? */
    if wkc > 0 as libc::c_int && nSM as libc::c_int > 2 as libc::c_int {
        /* limit to maximum number of SM defined, if true the slave can't be configured */
        if nSM as libc::c_int > 8 as libc::c_int {
            nSM = 8 as libc::c_int as uint8
        }
        /* iterate for every SM type defined */
        iSM = 2 as libc::c_int as uint8;
        while (iSM as libc::c_int) < nSM as libc::c_int {
            rdl =
                ::core::mem::size_of::<uint8>() as libc::c_ulong as
                    libc::c_int;
            tSM = 0 as libc::c_int as uint8;
            /* read SyncManager Communication Type */
            wkc =
                ecx_SDOread(context, Slave, 0x1c00 as libc::c_int as uint16,
                            (iSM as libc::c_int + 1 as libc::c_int) as uint8,
                            0 as libc::c_int as boolean, &mut rdl,
                            &mut tSM as *mut uint8 as *mut libc::c_void,
                            700000 as libc::c_int);
            if wkc > 0 as libc::c_int {
                // start slave bug prevention code, remove if possible
                if iSM as libc::c_int == 2 as libc::c_int &&
                       tSM as libc::c_int == 2 as libc::c_int {
                    // SM2 has type 2 == mailbox out, this is a bug in the slave!
                    SMt_bug_add = 1 as libc::c_int as uint8
                    // try to correct, this works if the types are 0 1 2 3 and should be 1 2 3 4
                }
                if tSM != 0 {
                    tSM =
                        (tSM as libc::c_int + SMt_bug_add as libc::c_int) as
                            uint8
                    // only add if SMt > 0
                }
                if iSM as libc::c_int == 2 as libc::c_int &&
                       tSM as libc::c_int == 0 as libc::c_int {
                    // SM2 has type 0, this is a bug in the slave!
                    tSM = 3 as libc::c_int as uint8
                }
                if iSM as libc::c_int == 3 as libc::c_int &&
                       tSM as libc::c_int == 0 as libc::c_int {
                    // SM3 has type 0, this is a bug in the slave!
                    tSM = 4 as libc::c_int as uint8
                }
                // end slave bug prevention code
                (*(*context).slavelist.offset(Slave as
                                                  isize)).SMtype[iSM as usize]
                    = tSM;
                /* check if SM is unused -> clear enable flag */
                if tSM as libc::c_int == 0 as libc::c_int {
                    (*(*context).slavelist.offset(Slave as
                                                      isize)).SM[iSM as
                                                                     usize].SMflags
                        =
                        (*(*context).slavelist.offset(Slave as
                                                          isize)).SM[iSM as
                                                                         usize].SMflags
                            & 0xfffeffff as libc::c_uint
                }
                if tSM as libc::c_int == 3 as libc::c_int ||
                       tSM as libc::c_int == 4 as libc::c_int {
                    /* read the assign PDO */
                    Tsize =
                        ecx_readPDOassign(context, Slave,
                                          (0x1c10 as libc::c_int +
                                               iSM as libc::c_int) as uint16);
                    /* if a mapping is found */
                    if Tsize != 0 {
                        (*(*context).slavelist.offset(Slave as
                                                          isize)).SM[iSM as
                                                                         usize].SMlength
                            =
                            Tsize.wrapping_add(7 as libc::c_int as
                                                   libc::c_uint).wrapping_div(8
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                as uint16;
                        if tSM as libc::c_int == 3 as libc::c_int {
                            /* we are doing outputs */
                            *Osize =
                                (*Osize as libc::c_uint).wrapping_add(Tsize)
                                    as uint32 as uint32
                        } else {
                            /* we are doing inputs */
                            *Isize =
                                (*Isize as libc::c_uint).wrapping_add(Tsize)
                                    as uint32 as uint32
                        }
                    }
                }
            }
            iSM = iSM.wrapping_add(1)
        }
    }
    /* found some I/O bits ? */
    if *Isize > 0 as libc::c_int as libc::c_uint ||
           *Osize > 0 as libc::c_int as libc::c_uint {
        retVal = 1 as libc::c_int
    }
    return retVal;
}
/* * CoE read PDO mapping in Complete Access mode (CA).
 *
 * CANopen has standard indexes defined for PDO mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave. Slave has to support CA, otherwise use ec_readPDOmap().
 *
 * @param[in]  context  = context struct
 * @param[in]  Slave    = Slave number
 * @param[in]  Thread_n = Calling thread index
 * @param[out] Osize    = Size in bits of output mapping (rxPDO) found
 * @param[out] Isize    = Size in bits of input mapping (txPDO) found
 * @return >0 if mapping successful.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readPDOmapCA(mut context: *mut ecx_contextt,
                                          mut Slave: uint16,
                                          mut Thread_n: libc::c_int,
                                          mut Osize: *mut uint32,
                                          mut Isize: *mut uint32)
 -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut retVal: libc::c_int = 0 as libc::c_int;
    let mut nSM: uint8 = 0;
    let mut iSM: uint8 = 0;
    let mut tSM: uint8 = 0;
    let mut Tsize: uint32 = 0;
    let mut SMt_bug_add: uint8 = 0;
    *Isize = 0 as libc::c_int as uint32;
    *Osize = 0 as libc::c_int as uint32;
    SMt_bug_add = 0 as libc::c_int as uint8;
    rdl =
        ::core::mem::size_of::<ec_SMcommtypet>() as libc::c_ulong as
            libc::c_int;
    (*(*context).SMcommtype.offset(Thread_n as isize)).n =
        0 as libc::c_int as uint8;
    /* read SyncManager Communication Type object count Complete Access*/
    wkc =
        ecx_SDOread(context, Slave, 0x1c00 as libc::c_int as uint16,
                    0 as libc::c_int as uint8, 1 as libc::c_int as boolean,
                    &mut rdl,
                    &mut *(*context).SMcommtype.offset(Thread_n as isize) as
                        *mut ec_SMcommtypet as *mut libc::c_void,
                    700000 as libc::c_int);
    /* positive result from slave ? */
    if wkc > 0 as libc::c_int &&
           (*(*context).SMcommtype.offset(Thread_n as isize)).n as libc::c_int
               > 2 as libc::c_int {
        nSM = (*(*context).SMcommtype.offset(Thread_n as isize)).n;
        /* limit to maximum number of SM defined, if true the slave can't be configured */
        if nSM as libc::c_int > 8 as libc::c_int {
            nSM = 8 as libc::c_int as uint8;
            ecx_packeterror(context, Slave, 0 as libc::c_int as uint16,
                            0 as libc::c_int as uint8,
                            10 as libc::c_int as uint16);
            /* #SM larger than EC_MAXSM */
        }
        /* iterate for every SM type defined */
        iSM = 2 as libc::c_int as uint8;
        while (iSM as libc::c_int) < nSM as libc::c_int {
            tSM =
                (*(*context).SMcommtype.offset(Thread_n as
                                                   isize)).SMtype[iSM as
                                                                      usize];
            // start slave bug prevention code, remove if possible
            if iSM as libc::c_int == 2 as libc::c_int &&
                   tSM as libc::c_int == 2 as libc::c_int {
                // SM2 has type 2 == mailbox out, this is a bug in the slave!
                SMt_bug_add = 1 as libc::c_int as uint8
                // try to correct, this works if the types are 0 1 2 3 and should be 1 2 3 4
            }
            if tSM != 0 {
                tSM =
                    (tSM as libc::c_int + SMt_bug_add as libc::c_int) as uint8
                // only add if SMt > 0
            }
            // end slave bug prevention code
            (*(*context).slavelist.offset(Slave as
                                              isize)).SMtype[iSM as usize] =
                tSM;
            /* check if SM is unused -> clear enable flag */
            if tSM as libc::c_int == 0 as libc::c_int {
                (*(*context).slavelist.offset(Slave as
                                                  isize)).SM[iSM as
                                                                 usize].SMflags
                    =
                    (*(*context).slavelist.offset(Slave as
                                                      isize)).SM[iSM as
                                                                     usize].SMflags
                        & 0xfffeffff as libc::c_uint
            }
            if tSM as libc::c_int == 3 as libc::c_int ||
                   tSM as libc::c_int == 4 as libc::c_int {
                /* read the assign PDO */
                Tsize =
                    ecx_readPDOassignCA(context, Slave, Thread_n,
                                        (0x1c10 as libc::c_int +
                                             iSM as libc::c_int) as uint16);
                /* if a mapping is found */
                if Tsize != 0 {
                    (*(*context).slavelist.offset(Slave as
                                                      isize)).SM[iSM as
                                                                     usize].SMlength
                        =
                        Tsize.wrapping_add(7 as libc::c_int as
                                               libc::c_uint).wrapping_div(8 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                            as uint16;
                    if tSM as libc::c_int == 3 as libc::c_int {
                        /* we are doing outputs */
                        *Osize =
                            (*Osize as libc::c_uint).wrapping_add(Tsize) as
                                uint32 as uint32
                    } else {
                        /* we are doing inputs */
                        *Isize =
                            (*Isize as libc::c_uint).wrapping_add(Tsize) as
                                uint32 as uint32
                    }
                }
            }
            iSM = iSM.wrapping_add(1)
        }
    }
    /* found some I/O bits ? */
    if *Isize > 0 as libc::c_int as libc::c_uint ||
           *Osize > 0 as libc::c_int as libc::c_uint {
        retVal = 1 as libc::c_int
    }
    return retVal;
}
/* * CoE read Object Description List.
 *
 * @param[in]  context  = context struct
 * @param[in]  Slave    = Slave number.
 * @param[out] pODlist  = resulting Object Description list.
 * @return Workcounter of slave response.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readODlist(mut context: *mut ecx_contextt,
                                        mut Slave: uint16,
                                        mut pODlist: *mut ec_ODlistt)
 -> libc::c_int {
    let mut SDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut aSDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut wkc: libc::c_int = 0;
    let mut x: uint16 = 0;
    let mut n: uint16 = 0;
    let mut i: uint16 = 0;
    let mut sp: uint16 = 0;
    let mut offset: uint16 = 0;
    let mut stop: boolean = 0;
    let mut cnt: uint8 = 0;
    let mut First: boolean = 0;
    (*pODlist).Slave = Slave;
    (*pODlist).Entries = 0 as libc::c_int as uint16;
    ec_clearmbx(&mut MbxIn);
    /* clear pending out mailbox in slave if available. Timeout is set to 0 */
    wkc = ecx_mbxreceive(context, Slave, &mut MbxIn, 0 as libc::c_int);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOservicet;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOservicet;
    (*SDOp).MbxHeader.length = 0x8 as libc::c_int as uint16;
    (*SDOp).MbxHeader.address = 0 as libc::c_int as uint16;
    (*SDOp).MbxHeader.priority = 0 as libc::c_int as uint8;
    /* Get new mailbox counter value */
    cnt =
        ec_nextmbxcnt((*(*context).slavelist.offset(Slave as
                                                        isize)).mbx_cnt); /* CoE */
    (*(*context).slavelist.offset(Slave as isize)).mbx_cnt =
        cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype =
        (ECT_MBXT_COE as libc::c_int +
             ((cnt as libc::c_int) << 4 as libc::c_int) as uint8 as
                 libc::c_int) as
            uint8; /* get object description list request */
    (*SDOp).CANOpen =
        (0 as libc::c_int +
             ((ECT_COES_SDOINFO as libc::c_int) << 12 as libc::c_int)) as
            uint16; /* fragments left */
    (*SDOp).Opcode =
        ECT_GET_ODLIST_REQ as libc::c_int as uint8; /* all objects */
    (*SDOp).Reserved = 0 as libc::c_int as uint8;
    (*SDOp).Fragments = 0 as libc::c_int as uint16;
    (*SDOp).c2rust_unnamed.wdata[0 as libc::c_int as usize] =
        0x1 as libc::c_int as uint16;
    /* send get object description list request to slave */
    wkc = ecx_mbxsend(context, Slave, &mut MbxOut, 20000 as libc::c_int);
    /* mailbox placed in slave ? */
    if wkc > 0 as libc::c_int {
        x =
            0 as libc::c_int as
                uint16; /* offset to skip info header in first frame, otherwise set to 0 */
        sp = 0 as libc::c_int as uint16; /* assume this is last iteration */
        First = 1 as libc::c_int as boolean;
        offset = 1 as libc::c_int as uint16;
        loop  {
            stop = 1 as libc::c_int as boolean;
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc =
                ecx_mbxreceive(context, Slave, &mut MbxIn,
                               700000 as libc::c_int);
            /* got response ? */
            if wkc > 0 as libc::c_int {
                /* response should be CoE and "get object description list response" */
                if (*aSDOp).MbxHeader.mbxtype as libc::c_int &
                       0xf as libc::c_int == ECT_MBXT_COE as libc::c_int &&
                       (*aSDOp).Opcode as libc::c_int & 0x7f as libc::c_int ==
                           ECT_GET_ODLIST_RES as libc::c_int {
                    if First != 0 {
                        /* extract number of indexes from mailbox data size */
                        n =
                            (((*aSDOp).MbxHeader.length as libc::c_int -
                                  (6 as libc::c_int + 2 as libc::c_int)) /
                                 2 as libc::c_int) as uint16
                    } else {
                        /* extract number of indexes from mailbox data size */
                        n =
                            (((*aSDOp).MbxHeader.length as libc::c_int -
                                  6 as libc::c_int) / 2 as libc::c_int) as
                                uint16
                    }
                    /* check if indexes fit in buffer structure */
                    if sp as libc::c_int + n as libc::c_int >
                           1024 as libc::c_int {
                        n =
                            (1024 as libc::c_int + 1 as libc::c_int -
                                 sp as libc::c_int) as
                                uint16; /* Too many entries for master buffer */
                        ecx_SDOinfoerror(context, Slave,
                                         0 as libc::c_int as uint16,
                                         0 as libc::c_int as uint8,
                                         0xf000000 as libc::c_int);
                        stop = 1 as libc::c_int as boolean
                    }
                    /* trim to maximum number of ODlist entries defined */
                    if (*pODlist).Entries as libc::c_int + n as libc::c_int >
                           1024 as libc::c_int {
                        n =
                            (1024 as libc::c_int -
                                 (*pODlist).Entries as libc::c_int) as uint16
                    }
                    (*pODlist).Entries =
                        ((*pODlist).Entries as libc::c_int + n as libc::c_int)
                            as uint16;
                    /* extract indexes one by one */
                    i = 0 as libc::c_int as uint16;
                    while (i as libc::c_int) < n as libc::c_int {
                        (*pODlist).Index[(sp as libc::c_int +
                                              i as libc::c_int) as usize] =
                            (*aSDOp).c2rust_unnamed.wdata[(i as libc::c_int +
                                                               offset as
                                                                   libc::c_int)
                                                              as usize];
                        i = i.wrapping_add(1)
                    }
                    sp = (sp as libc::c_int + n as libc::c_int) as uint16;
                    /* check if more fragments will follow */
                    if (*aSDOp).Fragments as libc::c_int > 0 as libc::c_int {
                        stop = 0 as libc::c_int as boolean
                    }
                    First = 0 as libc::c_int as boolean;
                    offset = 0 as libc::c_int as uint16
                } else {
                    /* got unexpected response from slave */
                    if (*aSDOp).Opcode as libc::c_int & 0x7f as libc::c_int ==
                           ECT_SDOINFO_ERROR as libc::c_int {
                        /* SDO info error received */
                        ecx_SDOinfoerror(context, Slave,
                                         0 as libc::c_int as uint16,
                                         0 as libc::c_int as uint8,
                                         (*aSDOp).c2rust_unnamed.ldata[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                             as int32);
                        stop = 1 as libc::c_int as boolean
                    } else {
                        ecx_packeterror(context, Slave,
                                        0 as libc::c_int as uint16,
                                        0 as libc::c_int as uint8,
                                        1 as libc::c_int as uint16);
                        /* Unexpected frame returned */
                    }
                    wkc = 0 as libc::c_int;
                    x = (x as libc::c_int + 20 as libc::c_int) as uint16
                }
            }
            x = x.wrapping_add(1);
            if !(x as libc::c_int <= 128 as libc::c_int && stop == 0) {
                break ;
            }
        }
    }
    return wkc;
}
/* * CoE read Object Description. Adds textual description to object indexes.
 *
 * @param[in]  context       = context struct
 * @param[in] Item           = Item number in ODlist.
 * @param[in,out] pODlist    = referencing Object Description list.
 * @return Workcounter of slave response.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readODdescription(mut context: *mut ecx_contextt,
                                               mut Item: uint16,
                                               mut pODlist: *mut ec_ODlistt)
 -> libc::c_int {
    let mut SDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut aSDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut wkc: libc::c_int = 0;
    let mut n: uint16 = 0;
    let mut Slave: uint16 = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    Slave = (*pODlist).Slave;
    (*pODlist).DataType[Item as usize] = 0 as libc::c_int as uint16;
    (*pODlist).ObjectCode[Item as usize] = 0 as libc::c_int as uint8;
    (*pODlist).MaxSub[Item as usize] = 0 as libc::c_int as uint8;
    (*pODlist).Name[Item as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    ec_clearmbx(&mut MbxIn);
    /* clear pending out mailbox in slave if available. Timeout is set to 0 */
    wkc = ecx_mbxreceive(context, Slave, &mut MbxIn, 0 as libc::c_int);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOservicet;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOservicet;
    (*SDOp).MbxHeader.length = 0x8 as libc::c_int as uint16;
    (*SDOp).MbxHeader.address = 0 as libc::c_int as uint16;
    (*SDOp).MbxHeader.priority = 0 as libc::c_int as uint8;
    /* Get new mailbox counter value */
    cnt =
        ec_nextmbxcnt((*(*context).slavelist.offset(Slave as
                                                        isize)).mbx_cnt); /* CoE */
    (*(*context).slavelist.offset(Slave as isize)).mbx_cnt =
        cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype =
        (ECT_MBXT_COE as libc::c_int +
             ((cnt as libc::c_int) << 4 as libc::c_int) as uint8 as
                 libc::c_int) as uint8; /* get object description request */
    (*SDOp).CANOpen =
        (0 as libc::c_int +
             ((ECT_COES_SDOINFO as libc::c_int) << 12 as libc::c_int)) as
            uint16; /* fragments left */
    (*SDOp).Opcode =
        ECT_GET_OD_REQ as libc::c_int as uint8; /* Data of Index */
    (*SDOp).Reserved = 0 as libc::c_int as uint8;
    (*SDOp).Fragments = 0 as libc::c_int as uint16;
    (*SDOp).c2rust_unnamed.wdata[0 as libc::c_int as usize] =
        (*pODlist).Index[Item as usize];
    /* send get object description request to slave */
    wkc = ecx_mbxsend(context, Slave, &mut MbxOut, 20000 as libc::c_int);
    /* mailbox placed in slave ? */
    if wkc > 0 as libc::c_int {
        ec_clearmbx(&mut MbxIn);
        /* read slave response */
        wkc =
            ecx_mbxreceive(context, Slave, &mut MbxIn, 700000 as libc::c_int);
        /* got response ? */
        if wkc > 0 as libc::c_int {
            if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xf as libc::c_int
                   == ECT_MBXT_COE as libc::c_int &&
                   (*aSDOp).Opcode as libc::c_int & 0x7f as libc::c_int ==
                       ECT_GET_OD_RES as libc::c_int {
                n =
                    ((*aSDOp).MbxHeader.length as libc::c_int -
                         12 as libc::c_int) as uint16;
                if n as libc::c_int > 40 as libc::c_int {
                    n =
                        40 as libc::c_int as
                            uint16 /* length of string(name of object) */
                    /* String terminator */
                    /* max chars */
                }
                (*pODlist).DataType[Item as usize] =
                    (*aSDOp).c2rust_unnamed.wdata[1 as libc::c_int as usize];
                (*pODlist).ObjectCode[Item as usize] =
                    (*aSDOp).c2rust_unnamed.bdata[5 as libc::c_int as usize];
                (*pODlist).MaxSub[Item as usize] =
                    (*aSDOp).c2rust_unnamed.bdata[4 as libc::c_int as usize];
                strncpy((*pODlist).Name[Item as usize].as_mut_ptr(),
                        &mut *(*aSDOp).c2rust_unnamed.bdata.as_mut_ptr().offset(6
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                            as *mut uint8 as *mut libc::c_char,
                        n as libc::c_ulong);
                (*pODlist).Name[Item as usize][n as usize] =
                    0 as libc::c_int as libc::c_char
            } else {
                /* got unexpected response from slave */
                if (*aSDOp).Opcode as libc::c_int & 0x7f as libc::c_int ==
                       ECT_SDOINFO_ERROR as libc::c_int {
                    /* SDO info error received */
                    ecx_SDOinfoerror(context, Slave,
                                     (*pODlist).Index[Item as usize],
                                     0 as libc::c_int as uint8,
                                     (*aSDOp).c2rust_unnamed.ldata[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                         as int32);
                } else {
                    ecx_packeterror(context, Slave,
                                    (*pODlist).Index[Item as usize],
                                    0 as libc::c_int as uint8,
                                    1 as libc::c_int as uint16);
                    /* Unexpected frame returned */
                }
                wkc = 0 as libc::c_int
            }
        }
    }
    return wkc;
}
/* * CoE read SDO service object entry, single subindex.
 * Used in ec_readOE().
 *
 * @param[in]  context       = context struct
 * @param[in] Item           = Item in ODlist.
 * @param[in] SubI           = Subindex of item in ODlist.
 * @param[in] pODlist        = Object description list for reference.
 * @param[out] pOElist       = resulting object entry structure.
 * @return Workcounter of slave response.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readOEsingle(mut context: *mut ecx_contextt,
                                          mut Item: uint16, mut SubI: uint8,
                                          mut pODlist: *mut ec_ODlistt,
                                          mut pOElist: *mut ec_OElistt)
 -> libc::c_int {
    let mut SDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut aSDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut wkc: libc::c_int = 0;
    let mut Index: uint16 = 0;
    let mut Slave: uint16 = 0;
    let mut n: int16 = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    wkc = 0 as libc::c_int;
    Slave = (*pODlist).Slave;
    Index = (*pODlist).Index[Item as usize];
    ec_clearmbx(&mut MbxIn);
    /* clear pending out mailbox in slave if available. Timeout is set to 0 */
    wkc = ecx_mbxreceive(context, Slave, &mut MbxIn, 0 as libc::c_int);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOservicet;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOservicet;
    (*SDOp).MbxHeader.length = 0xa as libc::c_int as uint16;
    (*SDOp).MbxHeader.address = 0 as libc::c_int as uint16;
    (*SDOp).MbxHeader.priority = 0 as libc::c_int as uint8;
    /* Get new mailbox counter value */
    cnt =
        ec_nextmbxcnt((*(*context).slavelist.offset(Slave as
                                                        isize)).mbx_cnt); /* CoE */
    (*(*context).slavelist.offset(Slave as isize)).mbx_cnt =
        cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype =
        (ECT_MBXT_COE as libc::c_int +
             ((cnt as libc::c_int) << 4 as libc::c_int) as uint8 as
                 libc::c_int) as
            uint8; /* get object entry description request */
    (*SDOp).CANOpen =
        (0 as libc::c_int +
             ((ECT_COES_SDOINFO as libc::c_int) << 12 as libc::c_int)) as
            uint16; /* fragments left */
    (*SDOp).Opcode = ECT_GET_OE_REQ as libc::c_int as uint8; /* Index */
    (*SDOp).Reserved = 0 as libc::c_int as uint8; /* SubIndex */
    (*SDOp).Fragments =
        0 as libc::c_int as
            uint16; /* get access rights, object category, PDO */
    (*SDOp).c2rust_unnamed.wdata[0 as libc::c_int as usize] = Index;
    (*SDOp).c2rust_unnamed.bdata[2 as libc::c_int as usize] = SubI;
    (*SDOp).c2rust_unnamed.bdata[3 as libc::c_int as usize] =
        (1 as libc::c_int + 2 as libc::c_int + 4 as libc::c_int) as uint8;
    /* send get object entry description request to slave */
    wkc = ecx_mbxsend(context, Slave, &mut MbxOut, 20000 as libc::c_int);
    /* mailbox placed in slave ? */
    if wkc > 0 as libc::c_int {
        ec_clearmbx(&mut MbxIn);
        /* read slave response */
        wkc =
            ecx_mbxreceive(context, Slave, &mut MbxIn, 700000 as libc::c_int);
        /* got response ? */
        if wkc > 0 as libc::c_int {
            if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xf as libc::c_int
                   == ECT_MBXT_COE as libc::c_int &&
                   (*aSDOp).Opcode as libc::c_int & 0x7f as libc::c_int ==
                       ECT_GET_OE_RES as libc::c_int {
                (*pOElist).Entries = (*pOElist).Entries.wrapping_add(1);
                /* string terminator */
                n =
                    ((*aSDOp).MbxHeader.length as libc::c_int -
                         16 as libc::c_int) as
                        int16; /* length of string(name of object) */
                if n as libc::c_int > 40 as libc::c_int {
                    n = 40 as libc::c_int as int16
                    /* max string length */
                }
                if (n as libc::c_int) < 0 as libc::c_int {
                    n = 0 as libc::c_int as int16
                }
                (*pOElist).ValueInfo[SubI as usize] =
                    (*aSDOp).c2rust_unnamed.bdata[3 as libc::c_int as usize];
                (*pOElist).DataType[SubI as usize] =
                    (*aSDOp).c2rust_unnamed.wdata[2 as libc::c_int as usize];
                (*pOElist).BitLength[SubI as usize] =
                    (*aSDOp).c2rust_unnamed.wdata[3 as libc::c_int as usize];
                (*pOElist).ObjAccess[SubI as usize] =
                    (*aSDOp).c2rust_unnamed.wdata[4 as libc::c_int as usize];
                strncpy((*pOElist).Name[SubI as usize].as_mut_ptr(),
                        &mut *(*aSDOp).c2rust_unnamed.wdata.as_mut_ptr().offset(5
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                            as *mut uint16 as *mut libc::c_char,
                        n as libc::c_ulong);
                (*pOElist).Name[SubI as usize][n as usize] =
                    0 as libc::c_int as libc::c_char
            } else {
                /* got unexpected response from slave */
                if (*aSDOp).Opcode as libc::c_int & 0x7f as libc::c_int ==
                       ECT_SDOINFO_ERROR as libc::c_int {
                    /* SDO info error received */
                    ecx_SDOinfoerror(context, Slave, Index, SubI,
                                     (*aSDOp).c2rust_unnamed.ldata[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                         as int32);
                } else {
                    ecx_packeterror(context, Slave, Index, SubI,
                                    1 as libc::c_int as uint16);
                    /* Unexpected frame returned */
                }
                wkc = 0 as libc::c_int
            }
        }
    }
    return wkc;
}
/* * CoE read SDO service object entry.
 *
 * @param[in] context        = context struct
 * @param[in] Item           = Item in ODlist.
 * @param[in] pODlist        = Object description list for reference.
 * @param[out] pOElist       = resulting object entry structure.
 * @return Workcounter of slave response.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readOE(mut context: *mut ecx_contextt,
                                    mut Item: uint16,
                                    mut pODlist: *mut ec_ODlistt,
                                    mut pOElist: *mut ec_OElistt)
 -> libc::c_int {
    let mut SubCount: uint16 = 0;
    let mut wkc: libc::c_int = 0;
    let mut SubI: uint8 = 0;
    wkc = 0 as libc::c_int;
    (*pOElist).Entries = 0 as libc::c_int as uint16;
    SubI = (*pODlist).MaxSub[Item as usize];
    /* for each entry found in ODlist */
    SubCount = 0 as libc::c_int as uint16;
    while SubCount as libc::c_int <= SubI as libc::c_int {
        /* read subindex of entry */
        wkc =
            ecx_readOEsingle(context, Item, SubCount as uint8, pODlist,
                             pOElist);
        SubCount = SubCount.wrapping_add(1)
    }
    return wkc;
}
/* * Report SDO error.
 *
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index that generated error
 * @param[in]  SubIdx     = Subindex that generated error
 * @param[in]  AbortCode  = Abortcode, see EtherCAT documentation for list
 * @see ecx_SDOerror
 */
#[no_mangle]
pub unsafe extern "C" fn ec_SDOerror(mut Slave: uint16, mut Index: uint16,
                                     mut SubIdx: uint8,
                                     mut AbortCode: int32) {
    ecx_SDOerror(&mut ecx_context, Slave, Index, SubIdx, AbortCode);
}
/* * CoE SDO read, blocking. Single subindex or Complete Access.
 *
 * Only a "normal" upload request is issued. If the requested parameter is <= 4bytes
 * then a "expedited" response is returned, otherwise a "normal" response. If a "normal"
 * response is larger than the mailbox size then the response is segmented. The function
 * will combine all segments and copy them to the parameter buffer.
 *
 * @param[in]  slave      = Slave number
 * @param[in]  index      = Index to read
 * @param[in]  subindex   = Subindex to read, must be 0 or 1 if CA is used.
 * @param[in]  CA         = FALSE = single subindex. TRUE = Complete Access, all subindexes read.
 * @param[in,out] psize   = Size in bytes of parameter buffer, returns bytes read from SDO.
 * @param[out] p          = Pointer to parameter buffer
 * @param[in]  timeout    = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 * @see ecx_SDOread
 */
#[no_mangle]
pub unsafe extern "C" fn ec_SDOread(mut slave: uint16, mut index: uint16,
                                    mut subindex: uint8, mut CA: boolean,
                                    mut psize: *mut libc::c_int,
                                    mut p: *mut libc::c_void,
                                    mut timeout: libc::c_int) -> libc::c_int {
    return ecx_SDOread(&mut ecx_context, slave, index, subindex, CA, psize, p,
                       timeout);
}
/* * CoE SDO write, blocking. Single subindex or Complete Access.
 *
 * A "normal" download request is issued, unless we have
 * small data, then a "expedited" transfer is used. If the parameter is larger than
 * the mailbox size then the download is segmented. The function will split the
 * parameter data in segments and send them to the slave one by one.
 *
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index to write
 * @param[in]  SubIndex   = Subindex to write, must be 0 or 1 if CA is used.
 * @param[in]  CA         = FALSE = single subindex. TRUE = Complete Access, all subindexes written.
 * @param[in]  psize      = Size in bytes of parameter buffer.
 * @param[out] p          = Pointer to parameter buffer
 * @param[in]  Timeout    = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 * @see ecx_SDOwrite
 */
#[no_mangle]
pub unsafe extern "C" fn ec_SDOwrite(mut Slave: uint16, mut Index: uint16,
                                     mut SubIndex: uint8, mut CA: boolean,
                                     mut psize: libc::c_int,
                                     mut p: *mut libc::c_void,
                                     mut Timeout: libc::c_int)
 -> libc::c_int {
    return ecx_SDOwrite(&mut ecx_context, Slave, Index, SubIndex, CA, psize,
                        p, Timeout);
}
/* * CoE RxPDO write, blocking.
 *
 * A RxPDO download request is issued.
 *
 * @param[in]  Slave         = Slave number
 * @param[in]  RxPDOnumber   = Related RxPDO number
 * @param[in]  psize         = Size in bytes of PDO buffer.
 * @param[out] p             = Pointer to PDO buffer
 * @return Workcounter from last slave response
 * @see ecx_RxPDO
 */
#[no_mangle]
pub unsafe extern "C" fn ec_RxPDO(mut Slave: uint16, mut RxPDOnumber: uint16,
                                  mut psize: libc::c_int,
                                  mut p: *mut libc::c_void) -> libc::c_int {
    return ecx_RxPDO(&mut ecx_context, Slave, RxPDOnumber, psize, p);
}
/* * CoE TxPDO read remote request, blocking.
 *
 * A RxPDO download request is issued.
 *
 * @param[in]  slave         = Slave number
 * @param[in]  TxPDOnumber   = Related TxPDO number
 * @param[in,out] psize      = Size in bytes of PDO buffer, returns bytes read from PDO.
 * @param[out] p             = Pointer to PDO buffer
 * @param[in]  timeout       = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 * @see ecx_TxPDO
 */
#[no_mangle]
pub unsafe extern "C" fn ec_TxPDO(mut slave: uint16, mut TxPDOnumber: uint16,
                                  mut psize: *mut libc::c_int,
                                  mut p: *mut libc::c_void,
                                  mut timeout: libc::c_int) -> libc::c_int {
    return ecx_TxPDO(&mut ecx_context, slave, TxPDOnumber, psize, p, timeout);
}
/* * Read PDO assign structure
 * @param[in]  Slave         = Slave number
 * @param[in]  PDOassign     = PDO assign object
 * @return total bitlength of PDO assign
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readPDOassign(mut Slave: uint16,
                                          mut PDOassign: uint16) -> uint32 {
    return ecx_readPDOassign(&mut ecx_context, Slave, PDOassign);
}
/* * Read PDO assign structure in Complete Access mode
 * @param[in]  Slave         = Slave number
 * @param[in]  PDOassign     = PDO assign object
 * @param[in]  Thread_n      = Calling thread index
 * @return total bitlength of PDO assign
 * @see ecx_readPDOmap
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readPDOassignCA(mut Slave: uint16,
                                            mut PDOassign: uint16,
                                            mut Thread_n: libc::c_int)
 -> uint32 {
    return ecx_readPDOassignCA(&mut ecx_context, Slave, Thread_n, PDOassign);
}
/* * CoE read PDO mapping.
 *
 * CANopen has standard indexes defined for PDO mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave.
 *
 * For details, see #ecx_readPDOmap
 *
 * @param[in] Slave    = Slave number
 * @param[out] Osize   = Size in bits of output mapping (rxPDO) found
 * @param[out] Isize   = Size in bits of input mapping (txPDO) found
 * @return >0 if mapping succesful.
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readPDOmap(mut Slave: uint16,
                                       mut Osize: *mut uint32,
                                       mut Isize: *mut uint32)
 -> libc::c_int {
    return ecx_readPDOmap(&mut ecx_context, Slave, Osize, Isize);
}
/* * CoE read PDO mapping in Complete Access mode (CA).
 *
 * CANopen has standard indexes defined for PDO mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave. Slave has to support CA, otherwise use ec_readPDOmap().
 *
 * @param[in] Slave    = Slave number
 * @param[in] Thread_n = Calling thread index
 * @param[out] Osize   = Size in bits of output mapping (rxPDO) found
 * @param[out] Isize   = Size in bits of input mapping (txPDO) found
 * @return >0 if mapping succesful.
 * @see ecx_readPDOmap ec_readPDOmapCA
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readPDOmapCA(mut Slave: uint16,
                                         mut Thread_n: libc::c_int,
                                         mut Osize: *mut uint32,
                                         mut Isize: *mut uint32)
 -> libc::c_int {
    return ecx_readPDOmapCA(&mut ecx_context, Slave, Thread_n, Osize, Isize);
}
/* * CoE read Object Description List.
 *
 * @param[in] Slave      = Slave number.
 * @param[out] pODlist  = resulting Object Description list.
 * @return Workcounter of slave response.
 * @see ecx_readODlist
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readODlist(mut Slave: uint16,
                                       mut pODlist: *mut ec_ODlistt)
 -> libc::c_int {
    return ecx_readODlist(&mut ecx_context, Slave, pODlist);
}
/* * CoE read Object Description. Adds textual description to object indexes.
 *
 * @param[in] Item           = Item number in ODlist.
 * @param[in,out] pODlist    = referencing Object Description list.
 * @return Workcounter of slave response.
 * @see ecx_readODdescription
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readODdescription(mut Item: uint16,
                                              mut pODlist: *mut ec_ODlistt)
 -> libc::c_int {
    return ecx_readODdescription(&mut ecx_context, Item, pODlist);
}
#[no_mangle]
pub unsafe extern "C" fn ec_readOEsingle(mut Item: uint16, mut SubI: uint8,
                                         mut pODlist: *mut ec_ODlistt,
                                         mut pOElist: *mut ec_OElistt)
 -> libc::c_int {
    return ecx_readOEsingle(&mut ecx_context, Item, SubI, pODlist, pOElist);
}
/* * CoE read SDO service object entry.
 *
 * @param[in] Item           = Item in ODlist.
 * @param[in] pODlist        = Object description list for reference.
 * @param[out] pOElist       = resulting object entry structure.
 * @return Workcounter of slave response.
 * @see ecx_readOE
 */
#[no_mangle]
pub unsafe extern "C" fn ec_readOE(mut Item: uint16,
                                   mut pODlist: *mut ec_ODlistt,
                                   mut pOElist: *mut ec_OElistt)
 -> libc::c_int {
    return ecx_readOE(&mut ecx_context, Item, pODlist, pOElist);
}
