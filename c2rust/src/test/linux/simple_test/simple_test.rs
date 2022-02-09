use libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    #[no_mangle]
    fn osal_usleep(usec: uint32) -> libc::c_int;
    #[no_mangle]
    fn osal_thread_create(
        thandle: *mut libc::c_void,
        stacksize: libc::c_int,
        func: *mut libc::c_void,
        param: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    static mut ec_slave: [ec_slavet; 200];
    #[no_mangle]
    static mut ec_slavecount: libc::c_int;
    #[no_mangle]
    static mut ec_group: [ec_groupt; 2];
    #[no_mangle]
    static mut ec_DCtime: int64;
    #[no_mangle]
    fn ec_init(ifname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn ec_close();
    #[no_mangle]
    fn ec_readstate() -> libc::c_int;
    #[no_mangle]
    fn ec_writestate(slave: uint16) -> libc::c_int;
    #[no_mangle]
    fn ec_statecheck(slave: uint16, reqstate: uint16, timeout: libc::c_int) -> uint16;
    #[no_mangle]
    fn ec_send_processdata() -> libc::c_int;
    #[no_mangle]
    fn ec_receive_processdata(timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ec_find_adapters() -> *mut ec_adaptert;
    #[no_mangle]
    fn ec_free_adapters(adapter: *mut ec_adaptert);
    #[no_mangle]
    fn ec_configdc() -> boolean;
    #[no_mangle]
    fn ec_config_init(usetable: uint8) -> libc::c_int;
    #[no_mangle]
    fn ec_config_map(pIOmap: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn ec_recover_slave(slave: uint16, timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ec_reconfig_slave(slave: uint16, timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ec_ALstatuscode2string(ALstatuscode: uint16) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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

static mut IOmap: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut thread1: *mut pthread_t = 0 as *const pthread_t as *mut pthread_t;
#[no_mangle]
pub static mut expectedWKC: libc::c_int = 0;
#[no_mangle]
pub static mut needlf: boolean = 0;

static mut wkc: libc::c_int = 0;
#[no_mangle]
pub static mut inOP: boolean = 0;
#[no_mangle]
pub static mut currentgroup: uint8 = 0 as libc::c_int as uint8;
#[no_mangle]
pub unsafe extern "C" fn simpletest(mut ifname: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut oloop: libc::c_int = 0;
    let mut iloop: libc::c_int = 0;
    let mut chk: libc::c_int = 0;
    needlf = 0 as libc::c_int as boolean;
    inOP = 0 as libc::c_int as boolean;
    printf(b"Starting simple test\n\x00" as *const u8 as *const libc::c_char);
    /* initialise SOEM, bind socket to ifname */
    if ec_init(ifname) != 0 {
        printf(
            b"ec_init on %s succeeded.\n\x00" as *const u8 as *const libc::c_char,
            ifname,
        );
        /* find and auto-config slaves */
        if ec_config_init(0 as libc::c_int as uint8) > 0 as libc::c_int {
            printf(
                b"%d slaves found and configured.\n\x00" as *const u8 as *const libc::c_char,
                ec_slavecount,
            );
            ec_config_map(&mut IOmap as *mut [libc::c_char; 4096] as *mut libc::c_void);
            ec_configdc();
            printf(b"Slaves mapped, state to SAFE_OP.\n\x00" as *const u8 as *const libc::c_char);
            /* wait for all slaves to reach SAFE_OP state */
            ec_statecheck(
                0 as libc::c_int as uint16,
                EC_STATE_SAFE_OP as libc::c_int as uint16,
                2000000 as libc::c_int * 4 as libc::c_int,
            );
            oloop = ec_slave[0 as libc::c_int as usize].Obytes as libc::c_int;
            if oloop == 0 as libc::c_int
                && ec_slave[0 as libc::c_int as usize].Obits as libc::c_int > 0 as libc::c_int
            {
                oloop = 1 as libc::c_int
            }
            if oloop > 8 as libc::c_int {
                oloop = 8 as libc::c_int
            }
            iloop = ec_slave[0 as libc::c_int as usize].Ibytes as libc::c_int;
            if iloop == 0 as libc::c_int
                && ec_slave[0 as libc::c_int as usize].Ibits as libc::c_int > 0 as libc::c_int
            {
                iloop = 1 as libc::c_int
            }
            if iloop > 8 as libc::c_int {
                iloop = 8 as libc::c_int
            }
            printf(
                b"segments : %d : %d %d %d %d\n\x00" as *const u8 as *const libc::c_char,
                ec_group[0 as libc::c_int as usize].nsegments as libc::c_int,
                ec_group[0 as libc::c_int as usize].IOsegment[0 as libc::c_int as usize],
                ec_group[0 as libc::c_int as usize].IOsegment[1 as libc::c_int as usize],
                ec_group[0 as libc::c_int as usize].IOsegment[2 as libc::c_int as usize],
                ec_group[0 as libc::c_int as usize].IOsegment[3 as libc::c_int as usize],
            );
            printf(
                b"Request operational state for all slaves\n\x00" as *const u8
                    as *const libc::c_char,
            );
            expectedWKC = ec_group[0 as libc::c_int as usize].outputsWKC as libc::c_int
                * 2 as libc::c_int
                + ec_group[0 as libc::c_int as usize].inputsWKC as libc::c_int;
            printf(
                b"Calculated workcounter %d\n\x00" as *const u8 as *const libc::c_char,
                expectedWKC,
            );
            ec_slave[0 as libc::c_int as usize].state =
                EC_STATE_OPERATIONAL as libc::c_int as uint16;
            /* send one valid process data to make outputs in slaves happy*/
            ec_send_processdata();
            ec_receive_processdata(2000 as libc::c_int);
            /* request OP state for all slaves */
            ec_writestate(0 as libc::c_int as uint16);
            chk = 200 as libc::c_int;
            loop
            /* wait for all slaves to reach OP state */
            {
                ec_send_processdata();
                ec_receive_processdata(2000 as libc::c_int);
                ec_statecheck(
                    0 as libc::c_int as uint16,
                    EC_STATE_OPERATIONAL as libc::c_int as uint16,
                    50000 as libc::c_int,
                );
                let fresh0 = chk;
                chk = chk - 1;
                if !(fresh0 != 0
                    && ec_slave[0 as libc::c_int as usize].state as libc::c_int
                        != EC_STATE_OPERATIONAL as libc::c_int)
                {
                    break;
                }
            }
            if ec_slave[0 as libc::c_int as usize].state as libc::c_int
                == EC_STATE_OPERATIONAL as libc::c_int
            {
                printf(
                    b"Operational state reached for all slaves.\n\x00" as *const u8
                        as *const libc::c_char,
                );
                inOP = 1 as libc::c_int as boolean;
                /* cyclic loop */
                i = 1 as libc::c_int;
                while i <= 10000 as libc::c_int {
                    ec_send_processdata();
                    ::core::ptr::write_volatile(
                        &mut wkc as *mut libc::c_int,
                        ec_receive_processdata(2000 as libc::c_int),
                    );
                    if wkc >= expectedWKC {
                        printf(
                            b"Processdata cycle %4d, WKC %d , O:\x00" as *const u8
                                as *const libc::c_char,
                            i,
                            wkc,
                        );
                        j = 0 as libc::c_int;
                        while j < oloop {
                            printf(
                                b" %2.2x\x00" as *const u8 as *const libc::c_char,
                                *ec_slave[0 as libc::c_int as usize]
                                    .outputs
                                    .offset(j as isize)
                                    as libc::c_int,
                            );
                            j += 1
                        }
                        printf(b" I:\x00" as *const u8 as *const libc::c_char);
                        j = 0 as libc::c_int;
                        while j < iloop {
                            printf(
                                b" %2.2x\x00" as *const u8 as *const libc::c_char,
                                *ec_slave[0 as libc::c_int as usize]
                                    .inputs
                                    .offset(j as isize)
                                    as libc::c_int,
                            );
                            j += 1
                        }
                        printf(
                            b" T:%ld\r\x00" as *const u8 as *const libc::c_char,
                            ec_DCtime,
                        );
                        needlf = 1 as libc::c_int as boolean
                    }
                    osal_usleep(5000 as libc::c_int as uint32);
                    i += 1
                }
                inOP = 0 as libc::c_int as boolean
            } else {
                printf(
                    b"Not all slaves reached operational state.\n\x00" as *const u8
                        as *const libc::c_char,
                );
                ec_readstate();
                i = 1 as libc::c_int;
                while i <= ec_slavecount {
                    if ec_slave[i as usize].state as libc::c_int
                        != EC_STATE_OPERATIONAL as libc::c_int
                    {
                        printf(
                            b"Slave %d State=0x%2.2x StatusCode=0x%4.4x : %s\n\x00" as *const u8
                                as *const libc::c_char,
                            i,
                            ec_slave[i as usize].state as libc::c_int,
                            ec_slave[i as usize].ALstatuscode as libc::c_int,
                            ec_ALstatuscode2string(ec_slave[i as usize].ALstatuscode),
                        );
                    }
                    i += 1
                }
            }
            printf(
                b"\nRequest init state for all slaves\n\x00" as *const u8 as *const libc::c_char,
            );
            ec_slave[0 as libc::c_int as usize].state = EC_STATE_INIT as libc::c_int as uint16;
            /* request INIT state for all slaves */
            ec_writestate(0 as libc::c_int as uint16);
        } else {
            printf(b"No slaves found!\n\x00" as *const u8 as *const libc::c_char);
        }
        printf(b"End simple test, close socket\n\x00" as *const u8 as *const libc::c_char);
        /* stop SOEM, close socket */
        ec_close();
    } else {
        printf(
            b"No socket connection on %s\nExecute as root\n\x00" as *const u8
                as *const libc::c_char,
            ifname,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ecatcheck(mut ptr: *mut libc::c_void) {
    let mut slave: libc::c_int = 0;
    /* Not used */
    loop {
        if inOP as libc::c_int != 0
            && (wkc < expectedWKC
                || ec_group[currentgroup as usize].docheckstate as libc::c_int != 0)
        {
            if needlf != 0 {
                needlf = 0 as libc::c_int as boolean;
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
            /* one ore more slaves are not responding */
            ec_group[currentgroup as usize].docheckstate = 0 as libc::c_int as boolean;
            ec_readstate();
            slave = 1 as libc::c_int;
            while slave <= ec_slavecount {
                if ec_slave[slave as usize].group as libc::c_int == currentgroup as libc::c_int
                    && ec_slave[slave as usize].state as libc::c_int
                        != EC_STATE_OPERATIONAL as libc::c_int
                {
                    ec_group[currentgroup as usize].docheckstate = 1 as libc::c_int as boolean;
                    if ec_slave[slave as usize].state as libc::c_int
                        == EC_STATE_SAFE_OP as libc::c_int + EC_STATE_ERROR as libc::c_int
                    {
                        printf(
                            b"ERROR : slave %d is in SAFE_OP + ERROR, attempting ack.\n\x00"
                                as *const u8 as *const libc::c_char,
                            slave,
                        );
                        ec_slave[slave as usize].state = (EC_STATE_SAFE_OP as libc::c_int
                            + EC_STATE_ACK as libc::c_int)
                            as uint16;
                        ec_writestate(slave as uint16);
                    } else if ec_slave[slave as usize].state as libc::c_int
                        == EC_STATE_SAFE_OP as libc::c_int
                    {
                        printf(
                            b"WARNING : slave %d is in SAFE_OP, change to OPERATIONAL.\n\x00"
                                as *const u8 as *const libc::c_char,
                            slave,
                        );
                        ec_slave[slave as usize].state =
                            EC_STATE_OPERATIONAL as libc::c_int as uint16;
                        ec_writestate(slave as uint16);
                    } else if ec_slave[slave as usize].state as libc::c_int
                        > EC_STATE_NONE as libc::c_int
                    {
                        if ec_reconfig_slave(slave as uint16, 500 as libc::c_int) != 0 {
                            ec_slave[slave as usize].islost = 0 as libc::c_int as boolean;
                            printf(
                                b"MESSAGE : slave %d reconfigured\n\x00" as *const u8
                                    as *const libc::c_char,
                                slave,
                            );
                        }
                    } else if ec_slave[slave as usize].islost == 0 {
                        /* re-check state */
                        ec_statecheck(
                            slave as uint16,
                            EC_STATE_OPERATIONAL as libc::c_int as uint16,
                            2000 as libc::c_int,
                        );
                        if ec_slave[slave as usize].state as libc::c_int
                            == EC_STATE_NONE as libc::c_int
                        {
                            ec_slave[slave as usize].islost = 1 as libc::c_int as boolean;
                            printf(
                                b"ERROR : slave %d lost\n\x00" as *const u8 as *const libc::c_char,
                                slave,
                            );
                        }
                    }
                }
                if ec_slave[slave as usize].islost != 0 {
                    if ec_slave[slave as usize].state as libc::c_int == EC_STATE_NONE as libc::c_int
                    {
                        if ec_recover_slave(slave as uint16, 500 as libc::c_int) != 0 {
                            ec_slave[slave as usize].islost = 0 as libc::c_int as boolean;
                            printf(
                                b"MESSAGE : slave %d recovered\n\x00" as *const u8
                                    as *const libc::c_char,
                                slave,
                            );
                        }
                    } else {
                        ec_slave[slave as usize].islost = 0 as libc::c_int as boolean;
                        printf(
                            b"MESSAGE : slave %d found\n\x00" as *const u8 as *const libc::c_char,
                            slave,
                        );
                    }
                }
                slave += 1
            }
            if ec_group[currentgroup as usize].docheckstate == 0 {
                printf(
                    b"OK : all slaves resumed OPERATIONAL.\n\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        osal_usleep(10000 as libc::c_int as uint32);
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    printf(
        b"SOEM (Simple Open EtherCAT Master)\nSimple test\n\x00" as *const u8
            as *const libc::c_char,
    );
    if argc > 1 as libc::c_int {
        /* create thread to handle slave error handling in OP */
        //      pthread_create( &thread1, NULL, (void *) &ecatcheck, (void*) &ctime);
        osal_thread_create(
            &mut thread1 as *mut *mut pthread_t as *mut libc::c_void,
            128000 as libc::c_int,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
                *mut libc::c_void,
            >(Some(
                ecatcheck as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
            )),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(_: *const time_t) -> *mut libc::c_char>,
                *mut libc::c_void,
            >(Some(
                ctime as unsafe extern "C" fn(_: *const time_t) -> *mut libc::c_char,
            )),
        );
        /* start cyclic part */
        simpletest(*argv.offset(1 as libc::c_int as isize));
    } else {
        let mut adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
        printf(
            b"Usage: simple_test ifname1\nifname = eth0 for example\n\x00" as *const u8
                as *const libc::c_char,
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
    }
    printf(b"End program\n\x00" as *const u8 as *const libc::c_char);
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
