pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type boolean = uint8_t;
pub type int8 = int8_t;
pub type int16 = int16_t;
pub type int32 = int32_t;
pub type uint8 = uint8_t;
pub type uint16 = uint16_t;
pub type uint32 = uint32_t;
pub type int64 = int64_t;
pub type uint64 = uint64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_timet {
    pub sec: uint32,
    pub usec: uint32,
}
pub type ec_bufT = [uint8; 1518];
pub type C2RustUnnamed = libc::c_uint;
pub const ECT_REG_DCCYCLE1: C2RustUnnamed = 2468;
pub const ECT_REG_DCCYCLE0: C2RustUnnamed = 2464;
pub const ECT_REG_DCSTART0: C2RustUnnamed = 2448;
pub const ECT_REG_DCSYNCACT: C2RustUnnamed = 2433;
pub const ECT_REG_DCCUC: C2RustUnnamed = 2432;
pub const ECT_REG_DCTIMEFILT: C2RustUnnamed = 2356;
pub const ECT_REG_DCSPEEDCNT: C2RustUnnamed = 2352;
pub const ECT_REG_DCSYSDIFF: C2RustUnnamed = 2348;
pub const ECT_REG_DCSYSDELAY: C2RustUnnamed = 2344;
pub const ECT_REG_DCSYSOFFSET: C2RustUnnamed = 2336;
pub const ECT_REG_DCSOF: C2RustUnnamed = 2328;
pub const ECT_REG_DCSYSTIME: C2RustUnnamed = 2320;
pub const ECT_REG_DCTIME3: C2RustUnnamed = 2316;
pub const ECT_REG_DCTIME2: C2RustUnnamed = 2312;
pub const ECT_REG_DCTIME1: C2RustUnnamed = 2308;
pub const ECT_REG_DCTIME0: C2RustUnnamed = 2304;
pub const ECT_REG_SM1CONTR: C2RustUnnamed = 2063;
pub const ECT_REG_SM1ACT: C2RustUnnamed = 2062;
pub const ECT_REG_SM1STAT: C2RustUnnamed = 2061;
pub const ECT_REG_SM0STAT: C2RustUnnamed = 2053;
pub const ECT_REG_SM3: C2RustUnnamed = 2072;
pub const ECT_REG_SM2: C2RustUnnamed = 2064;
pub const ECT_REG_SM1: C2RustUnnamed = 2056;
pub const ECT_REG_SM0: C2RustUnnamed = 2048;
pub const ECT_REG_FMMU3: C2RustUnnamed = 1584;
pub const ECT_REG_FMMU2: C2RustUnnamed = 1568;
pub const ECT_REG_FMMU1: C2RustUnnamed = 1552;
pub const ECT_REG_FMMU0: C2RustUnnamed = 1536;
pub const ECT_REG_EEPDAT: C2RustUnnamed = 1288;
pub const ECT_REG_EEPADR: C2RustUnnamed = 1284;
pub const ECT_REG_EEPSTAT: C2RustUnnamed = 1282;
pub const ECT_REG_EEPCTL: C2RustUnnamed = 1282;
pub const ECT_REG_EEPCFG: C2RustUnnamed = 1280;
pub const ECT_REG_WDCNT: C2RustUnnamed = 1090;
pub const ECT_REG_LLCNT: C2RustUnnamed = 784;
pub const ECT_REG_PECODE: C2RustUnnamed = 782;
pub const ECT_REG_PECNT: C2RustUnnamed = 781;
pub const ECT_REG_EPUECNT: C2RustUnnamed = 780;
pub const ECT_REG_FRXERR: C2RustUnnamed = 776;
pub const ECT_REG_RXERR: C2RustUnnamed = 768;
pub const ECT_REG_IRQMASK: C2RustUnnamed = 512;
pub const ECT_REG_PDICTL: C2RustUnnamed = 320;
pub const ECT_REG_ALSTATCODE: C2RustUnnamed = 308;
pub const ECT_REG_ALSTAT: C2RustUnnamed = 304;
pub const ECT_REG_ALCTL: C2RustUnnamed = 288;
pub const ECT_REG_DLSTAT: C2RustUnnamed = 272;
pub const ECT_REG_DLALIAS: C2RustUnnamed = 259;
pub const ECT_REG_DLPORT: C2RustUnnamed = 257;
pub const ECT_REG_DLCTL: C2RustUnnamed = 256;
pub const ECT_REG_ALIAS: C2RustUnnamed = 18;
pub const ECT_REG_STADR: C2RustUnnamed = 16;
pub const ECT_REG_ESCSUP: C2RustUnnamed = 8;
pub const ECT_REG_PORTDES: C2RustUnnamed = 7;
pub const ECT_REG_TYPE: C2RustUnnamed = 0;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_errort {
    pub Time: ec_timet,
    pub Signal: boolean,
    pub Slave: uint16,
    pub Index: uint16,
    pub SubIdx: uint8,
    pub Etype: ec_err_type,
    pub c2rust_unnamed: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub AbortCode: int32,
    pub c2rust_unnamed: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub ErrorCode: uint16,
    pub ErrorReg: uint8,
    pub b1: uint8,
    pub w1: uint16,
    pub w2: uint16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_stackT {
    pub sock: *mut libc::c_int,
    pub txbuf: *mut [ec_bufT; 16],
    pub txbuflength: *mut [libc::c_int; 16],
    pub tempbuf: *mut ec_bufT,
    pub rxbuf: *mut [ec_bufT; 16],
    pub rxbufstat: *mut [libc::c_int; 16],
    pub rxsa: *mut [libc::c_int; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecx_redportt {
    pub stack: ec_stackT,
    pub sockhandle: libc::c_int,
    pub rxbuf: [ec_bufT; 16],
    pub rxbufstat: [libc::c_int; 16],
    pub rxsa: [libc::c_int; 16],
    pub tempinbuf: ec_bufT,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
/* *
 * Set DC of slave to fire sync0 at CyclTime interval with CyclShift offset.
 *
 * @param[in]  context        = context struct
 * @param [in] slave            Slave number.
 * @param [in] act              TRUE = active, FALSE = deactivated
 * @param [in] CyclTime         Cycltime in ns.
 * @param [in] CyclShift        CyclShift in ns.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_dcsync0(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut act: boolean,
    mut CyclTime: uint32,
    mut CyclShift: int32,
) {
    let mut h: uint8 = 0;
    let mut RA: uint8 = 0;
    let mut slaveh: uint16 = 0;
    let mut t: int64 = 0;
    let mut t1: int64 = 0;
    let mut tc: int32 = 0;
    slaveh = (*(*context).slavelist.offset(slave as isize)).configadr;
    RA = 0u8;
    /* stop cyclic operation, ready for next trigger */
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCSYNCACT as uint16,
        ::core::mem::size_of::<uint8>() as uint16,
        &mut RA as *mut uint8 as *mut libc::c_void,
        2000i32,
    );
    if act != 0 {
        RA = (1i32 + 2i32) as uint8
        /* act cyclic operation and sync0, sync1 deactivated */
    } /* write access to ethercat */
    h = 0u8; /* read local time of slave */
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCCUC as uint16,
        ::core::mem::size_of::<uint8>() as uint16,
        &mut h as *mut uint8 as *mut libc::c_void,
        2000i32,
    );
    t1 = 0i64;
    ecx_FPRD(
        (*context).port,
        slaveh,
        ECT_REG_DCSYSTIME as uint16,
        ::core::mem::size_of::<int64>() as uint16,
        &mut t1 as *mut int64 as *mut libc::c_void,
        2000i32,
    );
    t1 = t1;
    /* Calculate first trigger time, always a whole multiple of CyclTime rounded up
    plus the shifttime (can be negative)
    This insures best synchronization between slaves, slaves with the same CyclTime
    will sync at the same moment (you can use CyclShift to shift the sync) */
    if CyclTime > 0u32 {
        t = (t1 + 100000000i64) / CyclTime as libc::c_long * CyclTime as libc::c_long
            + CyclTime as libc::c_long
            + CyclShift as libc::c_long
    } else {
        t = t1 + 100000000i64 + CyclShift as libc::c_long
        /* first trigger at T1 + CyclTime + SyncDelay + CyclShift in ns */
    } /* SYNC0 start time */
    t = t; /* SYNC0 cycle time */
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCSTART0 as uint16,
        ::core::mem::size_of::<int64>() as uint16,
        &mut t as *mut int64 as *mut libc::c_void,
        2000i32,
    ); /* activate cyclic operation */
    tc = CyclTime as int32;
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCCYCLE0 as uint16,
        ::core::mem::size_of::<int32>() as uint16,
        &mut tc as *mut int32 as *mut libc::c_void,
        2000i32,
    );
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCSYNCACT as uint16,
        ::core::mem::size_of::<uint8>() as uint16,
        &mut RA as *mut uint8 as *mut libc::c_void,
        2000i32,
    );
    // update ec_slave state
    (*(*context).slavelist.offset(slave as isize)).DCactive = act;
    (*(*context).slavelist.offset(slave as isize)).DCshift = CyclShift;
    (*(*context).slavelist.offset(slave as isize)).DCcycle = CyclTime as int32;
}
/* *
* Set DC of slave to fire sync0 and sync1 at CyclTime interval with CyclShift offset.
*
* @param[in]  context        = context struct
* @param [in] slave            Slave number.
* @param [in] act              TRUE = active, FALSE = deactivated
* @param [in] CyclTime0        Cycltime SYNC0 in ns.
* @param [in] CyclTime1        Cycltime SYNC1 in ns. This time is a delta time in relation to
                               the SYNC0 fire. If CylcTime1 = 0 then SYNC1 fires a the same time
                               as SYNC0.
* @param [in] CyclShift        CyclShift in ns.
*/
#[no_mangle]
pub unsafe extern "C" fn ecx_dcsync01(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut act: boolean,
    mut CyclTime0: uint32,
    mut CyclTime1: uint32,
    mut CyclShift: int32,
) {
    let mut h: uint8 = 0;
    let mut RA: uint8 = 0;
    let mut slaveh: uint16 = 0;
    let mut t: int64 = 0;
    let mut t1: int64 = 0;
    let mut tc: int32 = 0;
    let mut TrueCyclTime: uint32 = 0;
    /* Sync1 can be used as a multiple of Sync0, use true cycle time */
    TrueCyclTime = CyclTime1
        .wrapping_div(CyclTime0)
        .wrapping_add(1u32)
        .wrapping_mul(CyclTime0);
    slaveh = (*(*context).slavelist.offset(slave as isize)).configadr;
    RA = 0u8;
    /* stop cyclic operation, ready for next trigger */
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCSYNCACT as uint16,
        ::core::mem::size_of::<uint8>() as uint16,
        &mut RA as *mut uint8 as *mut libc::c_void,
        2000i32,
    );
    if act != 0 {
        RA = (1i32 + 2i32 + 4i32) as uint8
        /* act cyclic operation and sync0 + sync1 */
    } /* write access to ethercat */
    h = 0u8; /* read local time of slave */
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCCUC as uint16,
        ::core::mem::size_of::<uint8>() as uint16,
        &mut h as *mut uint8 as *mut libc::c_void,
        2000i32,
    );
    t1 = 0i64;
    ecx_FPRD(
        (*context).port,
        slaveh,
        ECT_REG_DCSYSTIME as uint16,
        ::core::mem::size_of::<int64>() as uint16,
        &mut t1 as *mut int64 as *mut libc::c_void,
        2000i32,
    );
    t1 = t1;
    /* Calculate first trigger time, always a whole multiple of TrueCyclTime rounded up
    plus the shifttime (can be negative)
    This insures best synchronization between slaves, slaves with the same CyclTime
    will sync at the same moment (you can use CyclShift to shift the sync) */
    if CyclTime0 > 0u32 {
        t = (t1 + 100000000i64) / TrueCyclTime as libc::c_long * TrueCyclTime as libc::c_long
            + TrueCyclTime as libc::c_long
            + CyclShift as libc::c_long
    } else {
        t = t1 + 100000000i64 + CyclShift as libc::c_long
        /* first trigger at T1 + CyclTime + SyncDelay + CyclShift in ns */
    } /* SYNC0 start time */
    t = t; /* SYNC0 cycle time */
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCSTART0 as uint16,
        ::core::mem::size_of::<int64>() as uint16,
        &mut t as *mut int64 as *mut libc::c_void,
        2000i32,
    ); /* SYNC1 cycle time */
    tc = CyclTime0 as int32; /* activate cyclic operation */
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCCYCLE0 as uint16,
        ::core::mem::size_of::<int32>() as uint16,
        &mut tc as *mut int32 as *mut libc::c_void,
        2000i32,
    );
    tc = CyclTime1 as int32;
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCCYCLE1 as uint16,
        ::core::mem::size_of::<int32>() as uint16,
        &mut tc as *mut int32 as *mut libc::c_void,
        2000i32,
    );
    ecx_FPWR(
        (*context).port,
        slaveh,
        ECT_REG_DCSYNCACT as uint16,
        ::core::mem::size_of::<uint8>() as uint16,
        &mut RA as *mut uint8 as *mut libc::c_void,
        2000i32,
    );
    // update ec_slave state
    (*(*context).slavelist.offset(slave as isize)).DCactive = act;
    (*(*context).slavelist.offset(slave as isize)).DCshift = CyclShift;
    (*(*context).slavelist.offset(slave as isize)).DCcycle = CyclTime0 as int32;
}
/* latched port time of slave */
unsafe extern "C" fn ecx_porttime(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut port: uint8,
) -> int32 {
    let mut ts: int32 = 0;
    match port as libc::c_int {
        0 => ts = (*(*context).slavelist.offset(slave as isize)).DCrtA,
        1 => ts = (*(*context).slavelist.offset(slave as isize)).DCrtB,
        2 => ts = (*(*context).slavelist.offset(slave as isize)).DCrtC,
        3 => ts = (*(*context).slavelist.offset(slave as isize)).DCrtD,
        _ => ts = 0i32,
    }
    return ts;
}
/* calculate previous active port of a slave */
unsafe extern "C" fn ecx_prevport(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut port: uint8,
) -> uint8 {
    let mut pport: uint8 = port;
    let mut aport: uint8 = (*(*context).slavelist.offset(slave as isize)).activeports;
    match port as libc::c_int {
        0 => {
            if aport as libc::c_int & 0x4i32 != 0 {
                pport = 2u8
            } else if aport as libc::c_int & 0x2i32 != 0 {
                pport = 1u8
            } else if aport as libc::c_int & 0x8i32 != 0 {
                pport = 3u8
            }
        }
        1 => {
            if aport as libc::c_int & 0x8i32 != 0 {
                pport = 3u8
            } else if aport as libc::c_int & 0x1i32 != 0 {
                pport = 0u8
            } else if aport as libc::c_int & 0x4i32 != 0 {
                pport = 2u8
            }
        }
        2 => {
            if aport as libc::c_int & 0x2i32 != 0 {
                pport = 1u8
            } else if aport as libc::c_int & 0x8i32 != 0 {
                pport = 3u8
            } else if aport as libc::c_int & 0x1i32 != 0 {
                pport = 0u8
            }
        }
        3 => {
            if aport as libc::c_int & 0x1i32 != 0 {
                pport = 0u8
            } else if aport as libc::c_int & 0x4i32 != 0 {
                pport = 2u8
            } else if aport as libc::c_int & 0x2i32 != 0 {
                pport = 1u8
            }
        }
        _ => {}
    }
    return pport;
}
/* search unconsumed ports in parent, consume and return first open port */
unsafe extern "C" fn ecx_parentport(mut context: *mut ecx_contextt, mut parent: uint16) -> uint8 {
    let mut parentport: uint8 = 0u8;
    let mut b: uint8 = 0;
    /* search order is important, here 3 - 1 - 2 - 0 */
    b = (*(*context).slavelist.offset(parent as isize)).consumedports;
    if b as libc::c_int & 0x8i32 != 0 {
        parentport = 3u8;
        b = (b as libc::c_int & !(0x8i32) as uint8 as libc::c_int) as uint8
    } else if b as libc::c_int & 0x2i32 != 0 {
        parentport = 1u8;
        b = (b as libc::c_int & !(0x2i32) as uint8 as libc::c_int) as uint8
    } else if b as libc::c_int & 0x4i32 != 0 {
        parentport = 2u8;
        b = (b as libc::c_int & !(0x4i32) as uint8 as libc::c_int) as uint8
    } else if b as libc::c_int & 0x1i32 != 0 {
        parentport = 0u8;
        b = (b as libc::c_int & !(0x1i32) as uint8 as libc::c_int) as uint8
    }
    (*(*context).slavelist.offset(parent as isize)).consumedports = b;
    return parentport;
}
/* *
 * Locate DC slaves, measure propagation delays.
 *
 * @param[in]  context        = context struct
 * @return boolean if slaves are found with DC
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_configdc(mut context: *mut ecx_contextt) -> boolean {
    let mut i: uint16 = 0; /* latch DCrecvTimeA of all slaves */
    let mut slaveh: uint16 = 0; /* EtherCAT uses 2000-01-01 as epoch start instead of 1970-01-01 */
    let mut parent: uint16 = 0;
    let mut child: uint16 = 0;
    let mut parenthold: uint16 = 0u16;
    let mut prevDCslave: uint16 = 0u16;
    let mut ht: int32 = 0;
    let mut dt1: int32 = 0;
    let mut dt2: int32 = 0;
    let mut dt3: int32 = 0;
    let mut hrt: int64 = 0;
    let mut entryport: uint8 = 0;
    let mut nlist: int8 = 0;
    let mut plist: [int8; 4] = [0; 4];
    let mut tlist: [int32; 4] = [0; 4];
    let mut mastertime: ec_timet = ec_timet { sec: 0, usec: 0 };
    let mut mastertime64: uint64 = 0;
    (*(*context).slavelist.offset(0isize)).hasdc = 0u8;
    (*(*context).grouplist.offset(0isize)).hasdc = 0u8;
    ht = 0i32;
    ecx_BWR(
        (*context).port,
        0u16,
        ECT_REG_DCTIME0 as uint16,
        ::core::mem::size_of::<int32>() as uint16,
        &mut ht as *mut int32 as *mut libc::c_void,
        2000i32,
    );
    mastertime = osal_current_time();
    mastertime.sec = (mastertime.sec as libc::c_ulong).wrapping_sub(946684800u64) as uint32;
    mastertime64 = (mastertime.sec as uint64)
        .wrapping_mul(1000000u64)
        .wrapping_add(mastertime.usec as uint64)
        .wrapping_mul(1000u64);
    i = 1u16;
    while i as libc::c_int <= *(*context).slavecount {
        (*(*context).slavelist.offset(i as isize)).consumedports =
            (*(*context).slavelist.offset(i as isize)).activeports;
        if (*(*context).slavelist.offset(i as isize)).hasdc != 0 {
            if (*(*context).slavelist.offset(0isize)).hasdc == 0 {
                (*(*context).slavelist.offset(0isize)).hasdc = 1u8;
                (*(*context).slavelist.offset(0isize)).DCnext = i;
                (*(*context).slavelist.offset(i as isize)).DCprevious = 0u16;
                (*(*context)
                    .grouplist
                    .offset((*(*context).slavelist.offset(i as isize)).group as isize))
                .hasdc = 1u8;
                (*(*context)
                    .grouplist
                    .offset((*(*context).slavelist.offset(i as isize)).group as isize))
                .DCnext = i
            } else {
                (*(*context).slavelist.offset(prevDCslave as isize)).DCnext = i;
                (*(*context).slavelist.offset(i as isize)).DCprevious = prevDCslave
            }
            /* this branch has DC slave so remove parenthold */
            parenthold = 0u16;
            prevDCslave = i;
            slaveh = (*(*context).slavelist.offset(i as isize)).configadr;
            ecx_FPRD(
                (*context).port,
                slaveh,
                ECT_REG_DCTIME0 as uint16,
                ::core::mem::size_of::<int32>() as uint16,
                &mut ht as *mut int32 as *mut libc::c_void,
                2000i32,
            );
            (*(*context).slavelist.offset(i as isize)).DCrtA = ht;
            /* 64bit latched DCrecvTimeA of each specific slave */
            ecx_FPRD(
                (*context).port,
                slaveh,
                ECT_REG_DCSOF as uint16,
                ::core::mem::size_of::<int64>() as uint16,
                &mut hrt as *mut int64 as *mut libc::c_void,
                2000i32,
            );
            /* use it as offset in order to set local time around 0 + mastertime */
            hrt = (-hrt as libc::c_ulong).wrapping_add(mastertime64) as int64;
            /* save it in the offset register */
            ecx_FPWR(
                (*context).port,
                slaveh,
                ECT_REG_DCSYSOFFSET as uint16,
                ::core::mem::size_of::<int64>() as uint16,
                &mut hrt as *mut int64 as *mut libc::c_void,
                2000i32,
            );
            ecx_FPRD(
                (*context).port,
                slaveh,
                ECT_REG_DCTIME1 as uint16,
                ::core::mem::size_of::<int32>() as uint16,
                &mut ht as *mut int32 as *mut libc::c_void,
                2000i32,
            );
            (*(*context).slavelist.offset(i as isize)).DCrtB = ht;
            ecx_FPRD(
                (*context).port,
                slaveh,
                ECT_REG_DCTIME2 as uint16,
                ::core::mem::size_of::<int32>() as uint16,
                &mut ht as *mut int32 as *mut libc::c_void,
                2000i32,
            );
            (*(*context).slavelist.offset(i as isize)).DCrtC = ht;
            ecx_FPRD(
                (*context).port,
                slaveh,
                ECT_REG_DCTIME3 as uint16,
                ::core::mem::size_of::<int32>() as uint16,
                &mut ht as *mut int32 as *mut libc::c_void,
                2000i32,
            );
            (*(*context).slavelist.offset(i as isize)).DCrtD = ht;
            /* make list of active ports and their time stamps */
            nlist = 0i8;
            if (*(*context).slavelist.offset(i as isize)).activeports as libc::c_int & 0x1i32 != 0 {
                plist[nlist as usize] = 0i8;
                tlist[nlist as usize] = (*(*context).slavelist.offset(i as isize)).DCrtA;
                nlist += 1
            }
            if (*(*context).slavelist.offset(i as isize)).activeports as libc::c_int & 0x8i32 != 0 {
                plist[nlist as usize] = 3i8;
                tlist[nlist as usize] = (*(*context).slavelist.offset(i as isize)).DCrtD;
                nlist += 1
            }
            if (*(*context).slavelist.offset(i as isize)).activeports as libc::c_int & 0x2i32 != 0 {
                plist[nlist as usize] = 1i8;
                tlist[nlist as usize] = (*(*context).slavelist.offset(i as isize)).DCrtB;
                nlist += 1
            }
            if (*(*context).slavelist.offset(i as isize)).activeports as libc::c_int & 0x4i32 != 0 {
                plist[nlist as usize] = 2i8;
                tlist[nlist as usize] = (*(*context).slavelist.offset(i as isize)).DCrtC;
                nlist += 1
            }
            /* entryport is port with the lowest timestamp */
            entryport = 0u8;
            if nlist as libc::c_int > 1i32 && tlist[1usize] < tlist[entryport as usize] {
                entryport = 1u8
            }
            if nlist as libc::c_int > 2i32 && tlist[2usize] < tlist[entryport as usize] {
                entryport = 2u8
            }
            if nlist as libc::c_int > 3i32 && tlist[3usize] < tlist[entryport as usize] {
                entryport = 3u8
            }
            entryport = plist[entryport as usize] as uint8;
            (*(*context).slavelist.offset(i as isize)).entryport = entryport;
            /* consume entryport from activeports */
            let ref mut fresh0 = (*(*context).slavelist.offset(i as isize)).consumedports;
            *fresh0 = (*fresh0 as libc::c_int
                & !((1i32) << entryport as libc::c_int) as uint8 as libc::c_int)
                as uint8;
            /* finding DC parent of current */
            parent = i;
            loop {
                child = parent;
                parent = (*(*context).slavelist.offset(parent as isize)).parent;
                if parent as libc::c_int == 0i32
                    || (*(*context).slavelist.offset(parent as isize)).hasdc as libc::c_int != 0
                {
                    break;
                }
            }
            /* only calculate propagation delay if slave is not the first */
            if parent as libc::c_int > 0i32 {
                /* find port on parent this slave is connected to */
                (*(*context).slavelist.offset(i as isize)).parentport =
                    ecx_parentport(context, parent);
                if (*(*context).slavelist.offset(parent as isize)).topology as libc::c_int == 1i32 {
                    (*(*context).slavelist.offset(i as isize)).parentport =
                        (*(*context).slavelist.offset(parent as isize)).entryport
                }
                dt1 = 0i32;
                dt2 = 0i32;
                /* delta time of (parentport - 1) - parentport */
                /* note: order of ports is 0 - 3 - 1 -2 */
                /* non active ports are skipped */
                dt3 = ecx_porttime(
                    context,
                    parent,
                    (*(*context).slavelist.offset(i as isize)).parentport,
                ) - ecx_porttime(
                    context,
                    parent,
                    ecx_prevport(
                        context,
                        parent,
                        (*(*context).slavelist.offset(i as isize)).parentport,
                    ),
                );
                /* current slave has children */
                /* those children's delays need to be subtracted */
                if (*(*context).slavelist.offset(i as isize)).topology as libc::c_int > 1i32 {
                    dt1 = ecx_porttime(
                        context,
                        i,
                        ecx_prevport(
                            context,
                            i,
                            (*(*context).slavelist.offset(i as isize)).entryport,
                        ),
                    ) - ecx_porttime(
                        context,
                        i,
                        (*(*context).slavelist.offset(i as isize)).entryport,
                    )
                }
                /* we are only interested in positive difference */
                if dt1 > dt3 {
                    dt1 = -dt1
                }
                /* current slave is not the first child of parent */
                /* previous child's delays need to be added */
                if child as libc::c_int - parent as libc::c_int > 1i32 {
                    dt2 = ecx_porttime(
                        context,
                        parent,
                        ecx_prevport(
                            context,
                            parent,
                            (*(*context).slavelist.offset(i as isize)).parentport,
                        ),
                    ) - ecx_porttime(
                        context,
                        parent,
                        (*(*context).slavelist.offset(parent as isize)).entryport,
                    )
                }
                if dt2 < 0i32 {
                    dt2 = -dt2
                }
                /* calculate current slave delay from delta times */
                /* assumption : forward delay equals return delay */
                (*(*context).slavelist.offset(i as isize)).pdelay = (dt3 - dt1) / 2i32
                    + dt2
                    + (*(*context).slavelist.offset(parent as isize)).pdelay;
                ht = (*(*context).slavelist.offset(i as isize)).pdelay;
                /* write propagation delay*/
                ecx_FPWR(
                    (*context).port,
                    slaveh,
                    ECT_REG_DCSYSDELAY as uint16,
                    ::core::mem::size_of::<int32>() as uint16,
                    &mut ht as *mut int32 as *mut libc::c_void,
                    2000i32,
                );
            }
        } else {
            (*(*context).slavelist.offset(i as isize)).DCrtA = 0i32;
            (*(*context).slavelist.offset(i as isize)).DCrtB = 0i32;
            (*(*context).slavelist.offset(i as isize)).DCrtC = 0i32;
            (*(*context).slavelist.offset(i as isize)).DCrtD = 0i32;
            parent = (*(*context).slavelist.offset(i as isize)).parent;
            /* if non DC slave found on first position on branch hold root parent */
            if parent as libc::c_int > 0i32
                && (*(*context).slavelist.offset(parent as isize)).topology as libc::c_int > 2i32
            {
                parenthold = parent
            }
            /* if branch has no DC slaves consume port on root parent */
            if parenthold as libc::c_int != 0
                && (*(*context).slavelist.offset(i as isize)).topology as libc::c_int == 1i32
            {
                ecx_parentport(context, parenthold);
                parenthold = 0u16
            }
        }
        i = i.wrapping_add(1)
    }
    return (*(*context).slavelist.offset(0isize)).hasdc;
}
#[no_mangle]
pub unsafe extern "C" fn ec_dcsync0(
    mut slave: uint16,
    mut act: boolean,
    mut CyclTime: uint32,
    mut CyclShift: int32,
) {
    ecx_dcsync0(&mut ecx_context, slave, act, CyclTime, CyclShift);
}
#[no_mangle]
pub unsafe extern "C" fn ec_dcsync01(
    mut slave: uint16,
    mut act: boolean,
    mut CyclTime0: uint32,
    mut CyclTime1: uint32,
    mut CyclShift: int32,
) {
    ecx_dcsync01(
        &mut ecx_context,
        slave,
        act,
        CyclTime0,
        CyclTime1,
        CyclShift,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ec_configdc() -> boolean {
    return ecx_configdc(&mut ecx_context);
}
