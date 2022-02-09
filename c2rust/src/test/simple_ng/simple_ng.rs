use libc;
extern "C" {
    #[no_mangle]
    fn ec_find_adapters() -> *mut ec_adaptert;
    #[no_mangle]
    fn ec_free_adapters(adapter: *mut ec_adaptert);
    #[no_mangle]
    fn ecx_init(context: *mut ecx_contextt, ifname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn ecx_close(context: *mut ecx_contextt);
    #[no_mangle]
    fn ecx_readstate(context: *mut ecx_contextt) -> libc::c_int;
    #[no_mangle]
    fn ecx_writestate(context: *mut ecx_contextt, slave: uint16) -> libc::c_int;
    #[no_mangle]
    fn ecx_statecheck(
        context: *mut ecx_contextt,
        slave: uint16,
        reqstate: uint16,
        timeout: libc::c_int,
    ) -> uint16;
    #[no_mangle]
    fn ecx_send_processdata(context: *mut ecx_contextt) -> libc::c_int;
    #[no_mangle]
    fn ecx_receive_processdata(context: *mut ecx_contextt, timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ecx_configdc(context: *mut ecx_contextt) -> boolean;
    #[no_mangle]
    fn ecx_config_init(context: *mut ecx_contextt, usetable: uint8) -> libc::c_int;
    #[no_mangle]
    fn ecx_config_map_group(
        context: *mut ecx_contextt,
        pIOmap: *mut libc::c_void,
        group: uint8,
    ) -> libc::c_int;
    #[no_mangle]
    fn ecx_recover_slave(
        context: *mut ecx_contextt,
        slave: uint16,
        timeout: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ecx_reconfig_slave(
        context: *mut ecx_contextt,
        slave: uint16,
        timeout: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ec_ALstatuscode2string(ALstatuscode: uint16) -> *mut libc::c_char;
    #[no_mangle]
    fn osal_usleep(usec: uint32) -> libc::c_int;
    #[no_mangle]
    fn osal_current_time() -> ec_timet;
    #[no_mangle]
    fn osal_time_diff(start: *mut ec_timet, end: *mut ec_timet, diff: *mut ec_timet);
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
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
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub AbortCode: int32,
    pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
#[repr(C)]
pub struct ec_adapter {
    pub name: [libc::c_char; 128],
    pub desc: [libc::c_char; 128],
    pub next: *mut ec_adaptert,
}
pub type ec_adaptert = ec_adapter;
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
    pub PO2SOconfigx: Option<unsafe extern "C" fn(_: *mut ecx_contextt, _: uint16) -> libc::c_int>,
    pub name: [libc::c_char; 41],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fieldbus {
    pub context: ecx_contextt,
    pub iface: *mut libc::c_char,
    pub group: uint8,
    pub roundtrip_time: libc::c_int,
    pub map: [uint8; 4096],
    pub port: ecx_portt,
    pub slavelist: [ec_slavet; 200],
    pub slavecount: libc::c_int,
    pub grouplist: [ec_groupt; 2],
    pub esibuf: [uint8; 4096],
    pub esimap: [uint32; 128],
    pub elist: ec_eringt,
    pub idxstack: ec_idxstackT,
    pub ecaterror: boolean,
    pub DCtime: int64,
    pub SMcommtype: [ec_SMcommtypet; 1],
    pub PDOassign: [ec_PDOassignt; 1],
    pub PDOdesc: [ec_PDOdesct; 1],
    pub eepSM: ec_eepromSMt,
    pub eepFMMU: ec_eepromFMMUt,
}
unsafe extern "C" fn fieldbus_initialize(
    mut fieldbus: *mut Fieldbus,
    mut iface: *mut libc::c_char,
) {
    let mut context: *mut ecx_contextt = 0 as *mut ecx_contextt;
    /* Let's start by 0-filling `fieldbus` to avoid surprises */
    memset(
        fieldbus as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Fieldbus>() as libc::c_ulong,
    );
    (*fieldbus).iface = iface;
    (*fieldbus).group = 0 as libc::c_int as uint8;
    (*fieldbus).roundtrip_time = 0 as libc::c_int;
    (*fieldbus).ecaterror = 0 as libc::c_int as boolean;
    /* Initialize the ecx_contextt data structure */
    context = &mut (*fieldbus).context;
    (*context).port = &mut (*fieldbus).port;
    (*context).slavelist = (*fieldbus).slavelist.as_mut_ptr();
    (*context).slavecount = &mut (*fieldbus).slavecount;
    (*context).maxslave = 200 as libc::c_int;
    (*context).grouplist = (*fieldbus).grouplist.as_mut_ptr();
    (*context).maxgroup = 2 as libc::c_int;
    (*context).esibuf = (*fieldbus).esibuf.as_mut_ptr();
    (*context).esimap = (*fieldbus).esimap.as_mut_ptr();
    (*context).esislave = 0 as libc::c_int as uint16;
    (*context).elist = &mut (*fieldbus).elist;
    (*context).idxstack = &mut (*fieldbus).idxstack;
    (*context).ecaterror = &mut (*fieldbus).ecaterror;
    (*context).DCtime = &mut (*fieldbus).DCtime;
    (*context).SMcommtype = (*fieldbus).SMcommtype.as_mut_ptr();
    (*context).PDOassign = (*fieldbus).PDOassign.as_mut_ptr();
    (*context).PDOdesc = (*fieldbus).PDOdesc.as_mut_ptr();
    (*context).eepSM = &mut (*fieldbus).eepSM;
    (*context).eepFMMU = &mut (*fieldbus).eepFMMU;
    (*context).FOEhook = None;
    (*context).EOEhook = None;
    (*context).manualstatechange = 0 as libc::c_int;
}
unsafe extern "C" fn fieldbus_roundtrip(mut fieldbus: *mut Fieldbus) -> libc::c_int {
    let mut context: *mut ecx_contextt = 0 as *mut ecx_contextt;
    let mut start: ec_timet = ec_timet { sec: 0, usec: 0 };
    let mut end: ec_timet = ec_timet { sec: 0, usec: 0 };
    let mut diff: ec_timet = ec_timet { sec: 0, usec: 0 };
    let mut wkc: libc::c_int = 0;
    context = &mut (*fieldbus).context;
    start = osal_current_time();
    ecx_send_processdata(context);
    wkc = ecx_receive_processdata(context, 2000 as libc::c_int);
    end = osal_current_time();
    osal_time_diff(&mut start, &mut end, &mut diff);
    (*fieldbus).roundtrip_time = diff
        .sec
        .wrapping_mul(1000000 as libc::c_int as libc::c_uint)
        .wrapping_add(diff.usec) as libc::c_int;
    return wkc;
}
unsafe extern "C" fn fieldbus_start(mut fieldbus: *mut Fieldbus) -> boolean {
    let mut context: *mut ecx_contextt = 0 as *mut ecx_contextt;
    let mut grp: *mut ec_groupt = 0 as *mut ec_groupt;
    let mut slave: *mut ec_slavet = 0 as *mut ec_slavet;
    let mut i: libc::c_int = 0;
    context = &mut (*fieldbus).context;
    grp = (*fieldbus)
        .grouplist
        .as_mut_ptr()
        .offset((*fieldbus).group as libc::c_int as isize);
    printf(
        b"Initializing SOEM on \'%s\'... \x00" as *const u8 as *const libc::c_char,
        (*fieldbus).iface,
    );
    if ecx_init(context, (*fieldbus).iface) == 0 {
        printf(b"no socket connection\n\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int as boolean;
    }
    printf(b"done\n\x00" as *const u8 as *const libc::c_char);
    printf(b"Finding autoconfig slaves... \x00" as *const u8 as *const libc::c_char);
    if ecx_config_init(context, 0 as libc::c_int as uint8) <= 0 as libc::c_int {
        printf(b"no slaves found\n\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int as boolean;
    }
    printf(
        b"%d slaves found\n\x00" as *const u8 as *const libc::c_char,
        (*fieldbus).slavecount,
    );
    printf(b"Sequential mapping of I/O... \x00" as *const u8 as *const libc::c_char);
    ecx_config_map_group(
        context,
        (*fieldbus).map.as_mut_ptr() as *mut libc::c_void,
        (*fieldbus).group,
    );
    printf(
        b"mapped %dO+%dI bytes from %d segments\x00" as *const u8 as *const libc::c_char,
        (*grp).Obytes,
        (*grp).Ibytes,
        (*grp).nsegments as libc::c_int,
    );
    if (*grp).nsegments as libc::c_int > 1 as libc::c_int {
        /* Show how slaves are distrubuted */
        i = 0 as libc::c_int;
        while i < (*grp).nsegments as libc::c_int {
            printf(
                b"%s%d\x00" as *const u8 as *const libc::c_char,
                if i == 0 as libc::c_int {
                    b" (\x00" as *const u8 as *const libc::c_char
                } else {
                    b"+\x00" as *const u8 as *const libc::c_char
                },
                (*grp).IOsegment[i as usize],
            );
            i += 1
        }
        printf(b" slaves)\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    printf(b"Configuring distributed clock... \x00" as *const u8 as *const libc::c_char);
    ecx_configdc(context);
    printf(b"done\n\x00" as *const u8 as *const libc::c_char);
    printf(
        b"Waiting for all slaves in safe operational... \x00" as *const u8 as *const libc::c_char,
    );
    ecx_statecheck(
        context,
        0 as libc::c_int as uint16,
        EC_STATE_SAFE_OP as libc::c_int as uint16,
        2000000 as libc::c_int * 4 as libc::c_int,
    );
    printf(b"done\n\x00" as *const u8 as *const libc::c_char);
    printf(
        b"Send a roundtrip to make outputs in slaves happy... \x00" as *const u8
            as *const libc::c_char,
    );
    fieldbus_roundtrip(fieldbus);
    printf(b"done\n\x00" as *const u8 as *const libc::c_char);
    printf(b"Setting operational state..\x00" as *const u8 as *const libc::c_char);
    /* Act on slave 0 (a virtual slave used for broadcasting) */
    slave = (*fieldbus).slavelist.as_mut_ptr();
    (*slave).state = EC_STATE_OPERATIONAL as libc::c_int as uint16;
    ecx_writestate(context, 0 as libc::c_int as uint16);
    /* Poll the result ten times before giving up */
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        printf(b".\x00" as *const u8 as *const libc::c_char);
        fieldbus_roundtrip(fieldbus);
        ecx_statecheck(
            context,
            0 as libc::c_int as uint16,
            EC_STATE_OPERATIONAL as libc::c_int as uint16,
            2000000 as libc::c_int / 10 as libc::c_int,
        );
        if (*slave).state as libc::c_int == EC_STATE_OPERATIONAL as libc::c_int {
            printf(b" all slaves are now operational\n\x00" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int as boolean;
        }
        i += 1
    }
    printf(b" failed,\x00" as *const u8 as *const libc::c_char);
    ecx_readstate(context);
    i = 1 as libc::c_int;
    while i <= (*fieldbus).slavecount {
        slave = (*fieldbus).slavelist.as_mut_ptr().offset(i as isize);
        if (*slave).state as libc::c_int != EC_STATE_OPERATIONAL as libc::c_int {
            printf(
                b" slave %d is 0x%04X (AL-status=0x%04X %s)\x00" as *const u8
                    as *const libc::c_char,
                i,
                (*slave).state as libc::c_int,
                (*slave).ALstatuscode as libc::c_int,
                ec_ALstatuscode2string((*slave).ALstatuscode),
            );
        }
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as boolean;
}
unsafe extern "C" fn fieldbus_stop(mut fieldbus: *mut Fieldbus) {
    let mut context: *mut ecx_contextt = 0 as *mut ecx_contextt;
    let mut slave: *mut ec_slavet = 0 as *mut ec_slavet;
    context = &mut (*fieldbus).context;
    /* Act on slave 0 (a virtual slave used for broadcasting) */
    slave = (*fieldbus).slavelist.as_mut_ptr();
    printf(b"Requesting init state on all slaves... \x00" as *const u8 as *const libc::c_char);
    (*slave).state = EC_STATE_INIT as libc::c_int as uint16;
    ecx_writestate(context, 0 as libc::c_int as uint16);
    printf(b"done\n\x00" as *const u8 as *const libc::c_char);
    printf(b"Close socket... \x00" as *const u8 as *const libc::c_char);
    ecx_close(context);
    printf(b"done\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fieldbus_dump(mut fieldbus: *mut Fieldbus) -> boolean {
    let mut grp: *mut ec_groupt = 0 as *mut ec_groupt;
    let mut n: uint32 = 0;
    let mut wkc: libc::c_int = 0;
    let mut expected_wkc: libc::c_int = 0;
    grp = (*fieldbus)
        .grouplist
        .as_mut_ptr()
        .offset((*fieldbus).group as libc::c_int as isize);
    wkc = fieldbus_roundtrip(fieldbus);
    expected_wkc =
        (*grp).outputsWKC as libc::c_int * 2 as libc::c_int + (*grp).inputsWKC as libc::c_int;
    printf(
        b"%6d usec  WKC %d\x00" as *const u8 as *const libc::c_char,
        (*fieldbus).roundtrip_time,
        wkc,
    );
    if wkc < expected_wkc {
        printf(
            b" wrong (expected %d)\n\x00" as *const u8 as *const libc::c_char,
            expected_wkc,
        );
        return 0 as libc::c_int as boolean;
    }
    printf(b"  O:\x00" as *const u8 as *const libc::c_char);
    n = 0 as libc::c_int as uint32;
    while n < (*grp).Obytes {
        printf(
            b" %02X\x00" as *const u8 as *const libc::c_char,
            *(*grp).outputs.offset(n as isize) as libc::c_int,
        );
        n = n.wrapping_add(1)
    }
    printf(b"  I:\x00" as *const u8 as *const libc::c_char);
    n = 0 as libc::c_int as uint32;
    while n < (*grp).Ibytes {
        printf(
            b" %02X\x00" as *const u8 as *const libc::c_char,
            *(*grp).inputs.offset(n as isize) as libc::c_int,
        );
        n = n.wrapping_add(1)
    }
    printf(
        b"  T: %lld\r\x00" as *const u8 as *const libc::c_char,
        (*fieldbus).DCtime as libc::c_longlong,
    );
    return 1 as libc::c_int as boolean;
}
unsafe extern "C" fn fieldbus_check_state(mut fieldbus: *mut Fieldbus) {
    let mut context: *mut ecx_contextt = 0 as *mut ecx_contextt;
    let mut grp: *mut ec_groupt = 0 as *mut ec_groupt;
    let mut slave: *mut ec_slavet = 0 as *mut ec_slavet;
    let mut i: libc::c_int = 0;
    context = &mut (*fieldbus).context;
    grp = (*context)
        .grouplist
        .offset((*fieldbus).group as libc::c_int as isize);
    (*grp).docheckstate = 0 as libc::c_int as boolean;
    ecx_readstate(context);
    i = 1 as libc::c_int;
    while i <= (*fieldbus).slavecount {
        slave = (*context).slavelist.offset(i as isize);
        if !((*slave).group as libc::c_int != (*fieldbus).group as libc::c_int) {
            if (*slave).state as libc::c_int != EC_STATE_OPERATIONAL as libc::c_int {
                (*grp).docheckstate = 1 as libc::c_int as boolean;
                if (*slave).state as libc::c_int
                    == EC_STATE_SAFE_OP as libc::c_int + EC_STATE_ERROR as libc::c_int
                {
                    printf(
                        b"* Slave %d is in SAFE_OP+ERROR, attempting ACK\n\x00" as *const u8
                            as *const libc::c_char,
                        i,
                    );
                    (*slave).state =
                        (EC_STATE_SAFE_OP as libc::c_int + EC_STATE_ACK as libc::c_int) as uint16;
                    ecx_writestate(context, i as uint16);
                } else if (*slave).state as libc::c_int == EC_STATE_SAFE_OP as libc::c_int {
                    printf(
                        b"* Slave %d is in SAFE_OP, change to OPERATIONAL\n\x00" as *const u8
                            as *const libc::c_char,
                        i,
                    );
                    (*slave).state = EC_STATE_OPERATIONAL as libc::c_int as uint16;
                    ecx_writestate(context, i as uint16);
                } else if (*slave).state as libc::c_int > EC_STATE_NONE as libc::c_int {
                    if ecx_reconfig_slave(context, i as uint16, 2000 as libc::c_int) != 0 {
                        (*slave).islost = 0 as libc::c_int as boolean;
                        printf(
                            b"* Slave %d reconfigured\n\x00" as *const u8 as *const libc::c_char,
                            i,
                        );
                    }
                } else if (*slave).islost == 0 {
                    ecx_statecheck(
                        context,
                        i as uint16,
                        EC_STATE_OPERATIONAL as libc::c_int as uint16,
                        2000 as libc::c_int,
                    );
                    if (*slave).state as libc::c_int == EC_STATE_NONE as libc::c_int {
                        (*slave).islost = 1 as libc::c_int as boolean;
                        printf(
                            b"* Slave %d lost\n\x00" as *const u8 as *const libc::c_char,
                            i,
                        );
                    }
                }
            } else if (*slave).islost != 0 {
                if (*slave).state as libc::c_int != EC_STATE_NONE as libc::c_int {
                    (*slave).islost = 0 as libc::c_int as boolean;
                    printf(
                        b"* Slave %d found\n\x00" as *const u8 as *const libc::c_char,
                        i,
                    );
                } else if ecx_recover_slave(context, i as uint16, 2000 as libc::c_int) != 0 {
                    (*slave).islost = 0 as libc::c_int as boolean;
                    printf(
                        b"* Slave %d recovered\n\x00" as *const u8 as *const libc::c_char,
                        i,
                    );
                }
            }
        }
        i += 1
    }
    if (*grp).docheckstate == 0 {
        printf(b"All slaves resumed OPERATIONAL\n\x00" as *const u8 as *const libc::c_char);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut fieldbus: Fieldbus = Fieldbus {
        context: ecx_contextt {
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
        },
        iface: 0 as *mut libc::c_char,
        group: 0,
        roundtrip_time: 0,
        map: [0; 4096],
        port: ecx_portt {
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
        },
        slavelist: [ec_slavet {
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
        }; 200],
        slavecount: 0,
        grouplist: [ec_groupt {
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
        }; 2],
        esibuf: [0; 4096],
        esimap: [0; 128],
        elist: ec_eringt {
            head: 0,
            tail: 0,
            Error: [ec_errort {
                Time: ec_timet { sec: 0, usec: 0 },
                Signal: 0,
                Slave: 0,
                Index: 0,
                SubIdx: 0,
                Etype: EC_ERR_TYPE_SDO_ERROR,
                c2rust_unnamed: C2RustUnnamed_0 { AbortCode: 0 },
            }; 65],
        },
        idxstack: ec_idxstackT {
            pushed: 0,
            pulled: 0,
            idx: [0; 16],
            data: [0 as *mut libc::c_void; 16],
            length: [0; 16],
            dcoffset: [0; 16],
        },
        ecaterror: 0,
        DCtime: 0,
        SMcommtype: [ec_SMcommtypet {
            n: 0,
            nu1: 0,
            SMtype: [0; 8],
        }; 1],
        PDOassign: [ec_PDOassignt {
            n: 0,
            nu1: 0,
            index: [0; 256],
        }; 1],
        PDOdesc: [ec_PDOdesct {
            n: 0,
            nu1: 0,
            PDO: [0; 256],
        }; 1],
        eepSM: ec_eepromSMt {
            Startpos: 0,
            nSM: 0,
            PhStart: 0,
            Plength: 0,
            Creg: 0,
            Sreg: 0,
            Activate: 0,
            PDIctrl: 0,
        },
        eepFMMU: ec_eepromFMMUt {
            Startpos: 0,
            nFMMU: 0,
            FMMU0: 0,
            FMMU1: 0,
            FMMU2: 0,
            FMMU3: 0,
        },
    };
    if argc != 2 as libc::c_int {
        let mut adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
        printf(
            b"Usage: simple_ng IFNAME1\nIFNAME1 is the NIC interface name, e.g. \'eth0\'\n\x00"
                as *const u8 as *const libc::c_char,
        );
        printf(b"\nAvailable adapters:\n\x00" as *const u8 as *const libc::c_char);
        adapter = ec_find_adapters();
        while !adapter.is_null() {
            printf(
                b"    - %s  (%s)\n\x00" as *const u8 as *const libc::c_char,
                (*adapter).name.as_mut_ptr(),
                (*adapter).desc.as_mut_ptr(),
            );
            adapter = (*adapter).next
        }
        ec_free_adapters(adapter);
        return 1 as libc::c_int;
    }
    fieldbus_initialize(&mut fieldbus, *argv.offset(1 as libc::c_int as isize));
    if fieldbus_start(&mut fieldbus) != 0 {
        let mut i: libc::c_int = 0;
        let mut min_time: libc::c_int = 0;
        let mut max_time: libc::c_int = 0;
        max_time = 0 as libc::c_int;
        min_time = max_time;
        i = 1 as libc::c_int;
        while i <= 10000 as libc::c_int {
            printf(b"Iteration %4d:\x00" as *const u8 as *const libc::c_char, i);
            if fieldbus_dump(&mut fieldbus) == 0 {
                fieldbus_check_state(&mut fieldbus);
            } else if i == 1 as libc::c_int {
                max_time = fieldbus.roundtrip_time;
                min_time = max_time
            } else if fieldbus.roundtrip_time < min_time {
                min_time = fieldbus.roundtrip_time
            } else if fieldbus.roundtrip_time > max_time {
                max_time = fieldbus.roundtrip_time
            }
            osal_usleep(5000 as libc::c_int as uint32);
            i += 1
        }
        printf(
            b"\nRoundtrip time (usec): min %d max %d\n\x00" as *const u8 as *const libc::c_char,
            min_time,
            max_time,
        );
        fieldbus_stop(&mut fieldbus);
    }
    return 0 as libc::c_int;
}
fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
