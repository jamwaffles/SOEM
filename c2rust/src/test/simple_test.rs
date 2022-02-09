use libc;
extern "C" {

    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;

    fn ctime(__timer: *const time_t) -> *mut libc::c_char;

    fn osal_usleep(usec: uint32) -> libc::c_int;

    fn osal_thread_create(
        thandle: *mut libc::c_void,
        stacksize: libc::c_int,
        func: *mut libc::c_void,
        param: *mut libc::c_void,
    ) -> libc::c_int;

    static mut ec_slave: [ec_slavet; 200];

    static mut ec_slavecount: libc::c_int;

    static mut ec_group: [ec_groupt; 2];

    static mut ec_DCtime: int64;

    fn ec_init(ifname: *const libc::c_char) -> libc::c_int;

    fn ec_close();

    fn ec_readstate() -> libc::c_int;

    fn ec_writestate(slave: uint16) -> libc::c_int;

    fn ec_statecheck(slave: uint16, reqstate: uint16, timeout: libc::c_int) -> uint16;

    fn ec_send_processdata() -> libc::c_int;

    fn ec_receive_processdata(timeout: libc::c_int) -> libc::c_int;

    fn ec_find_adapters() -> *mut ec_adaptert;

    fn ec_free_adapters(adapter: *mut ec_adaptert);

    fn ec_configdc() -> boolean;

    fn ec_config_init(usetable: uint8) -> libc::c_int;

    fn ec_config_map(pIOmap: *mut libc::c_void) -> libc::c_int;

    fn ec_recover_slave(slave: uint16, timeout: libc::c_int) -> libc::c_int;

    fn ec_reconfig_slave(slave: uint16, timeout: libc::c_int) -> libc::c_int;

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
pub type pthread_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

static mut IOmap: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut thread1: *mut pthread_t = 0 as *mut pthread_t;
#[no_mangle]
pub static mut expectedWKC: libc::c_int = 0;
#[no_mangle]
pub static mut needlf: boolean = 0;

static mut wkc: libc::c_int = 0;
#[no_mangle]
pub static mut inOP: boolean = 0;
#[no_mangle]
pub static mut currentgroup: uint8 = 0u8;
#[no_mangle]
pub unsafe extern "C" fn simpletest(mut ifname: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut oloop: libc::c_int = 0;
    let mut iloop: libc::c_int = 0;
    let mut chk: libc::c_int = 0;
    needlf = 0u8;
    inOP = 0u8;
    println!("Starting simple test");
    /* initialise SOEM, bind socket to ifname */
    if ec_init(ifname) != 0 {
        println!("ec_init on {:} succeeded.", {
            std::ffi::CStr::from_ptr(ifname as *const libc::c_char)
                .to_str()
                .unwrap()
        });
        /* find and auto-config slaves */
        if ec_config_init(0u8) > 0i32 {
            println!(
                "{:} slaves found and configured.",
                ec_slavecount as libc::c_int
            );
            ec_config_map(&mut IOmap as *mut [libc::c_char; 4096] as *mut libc::c_void);
            ec_configdc();
            println!("Slaves mapped, state to SAFE_OP.");
            /* wait for all slaves to reach SAFE_OP state */
            ec_statecheck(0u16, EC_STATE_SAFE_OP as uint16, 2000000i32 * 4i32);
            oloop = ec_slave[0usize].Obytes as libc::c_int;
            if oloop == 0i32 && ec_slave[0usize].Obits as libc::c_int > 0i32 {
                oloop = 1i32
            }
            if oloop > 8i32 {
                oloop = 8i32
            }
            iloop = ec_slave[0usize].Ibytes as libc::c_int;
            if iloop == 0i32 && ec_slave[0usize].Ibits as libc::c_int > 0i32 {
                iloop = 1i32
            }
            if iloop > 8i32 {
                iloop = 8i32
            }

            println!(
                "segments : {:} : {:} {:} {:} {:}",
                ec_group[0usize].nsegments as libc::c_int as libc::c_int,
                ec_group[0usize].IOsegment[0usize] as libc::c_int,
                ec_group[0usize].IOsegment[1usize] as libc::c_int,
                ec_group[0usize].IOsegment[2usize] as libc::c_int,
                ec_group[0usize].IOsegment[3usize] as libc::c_int
            );
            println!("Request operational state for all slaves");
            expectedWKC = ec_group[0usize].outputsWKC as libc::c_int * 2i32
                + ec_group[0usize].inputsWKC as libc::c_int;
            println!("Calculated workcounter {:}", expectedWKC as libc::c_int);
            ec_slave[0usize].state = EC_STATE_OPERATIONAL as uint16;
            /* send one valid process data to make outputs in slaves happy*/
            ec_send_processdata();
            ec_receive_processdata(2000i32);
            /* request OP state for all slaves */
            ec_writestate(0u16);
            chk = 200i32;
            loop
            /* wait for all slaves to reach OP state */
            {
                ec_send_processdata();
                ec_receive_processdata(2000i32);
                ec_statecheck(0u16, EC_STATE_OPERATIONAL as uint16, 50000i32);
                let fresh0 = chk;
                chk = chk - 1;
                if !(fresh0 != 0
                    && ec_slave[0usize].state as libc::c_int != EC_STATE_OPERATIONAL as libc::c_int)
                {
                    break;
                }
            }
            if ec_slave[0usize].state as libc::c_int == EC_STATE_OPERATIONAL as libc::c_int {
                println!("Operational state reached for all slaves.");
                inOP = 1u8;
                /* cyclic loop */
                i = 1i32;
                while i <= 10000i32 {
                    ec_send_processdata();
                    ::core::ptr::write_volatile(
                        &mut wkc as *mut libc::c_int,
                        ec_receive_processdata(2000i32),
                    );
                    if wkc >= expectedWKC {
                        print!(
                            "Processdata cycle {:4}, WKC {:} , O:",
                            i as libc::c_int, wkc as libc::c_int
                        );
                        j = 0i32;
                        while j < oloop {
                            print!(
                                " {:2.2x}",
                                *ec_slave[0usize].outputs.offset(j as isize) as libc::c_int
                                    as libc::c_uint
                            );
                            j += 1
                        }
                        print!(" I:");
                        j = 0i32;
                        while j < iloop {
                            print!(
                                " {:2.2x}",
                                *ec_slave[0usize].inputs.offset(j as isize) as libc::c_int
                                    as libc::c_uint
                            );
                            j += 1
                        }
                        print!(" T:{:}\r", ec_DCtime as libc::c_long);
                        needlf = 1u8
                    }
                    osal_usleep(5000u32);
                    i += 1
                }
                inOP = 0u8
            } else {
                println!("Not all slaves reached operational state.");
                ec_readstate();
                i = 1i32;
                while i <= ec_slavecount {
                    if ec_slave[i as usize].state as libc::c_int
                        != EC_STATE_OPERATIONAL as libc::c_int
                    {
                        println!(
                            "Slave {:} State=0x{:2.2x} StatusCode=0x{:4.4x} : {:}",
                            i as libc::c_int,
                            ec_slave[i as usize].state as libc::c_int as libc::c_uint,
                            ec_slave[i as usize].ALstatuscode as libc::c_int as libc::c_uint,
                            {
                                std::ffi::CStr::from_ptr(ec_ALstatuscode2string(
                                    ec_slave[i as usize].ALstatuscode,
                                )
                                    as *const libc::c_char)
                                .to_str()
                                .unwrap()
                            }
                        );
                    }
                    i += 1
                }
            }
            println!("\nRequest init state for all slaves");
            ec_slave[0usize].state = EC_STATE_INIT as uint16;
            /* request INIT state for all slaves */
            ec_writestate(0u16);
        } else {
            println!("No slaves found!");
        }
        println!("End simple test, close socket");
        /* stop SOEM, close socket */
        ec_close();
    } else {
        println!("No socket connection on {:}\nExecute as root", {
            std::ffi::CStr::from_ptr(ifname as *const libc::c_char)
                .to_str()
                .unwrap()
        });
    };
}
#[no_mangle]
pub unsafe extern "C" fn ecatcheck(mut _ptr: *mut libc::c_void) {
    let mut slave: libc::c_int = 0;
    /* Not used */
    loop {
        if inOP as libc::c_int != 0
            && (wkc < expectedWKC
                || ec_group[currentgroup as usize].docheckstate as libc::c_int != 0)
        {
            if needlf != 0 {
                needlf = 0u8;
                println!("");
            }
            /* one ore more slaves are not responding */
            ec_group[currentgroup as usize].docheckstate = 0u8;
            ec_readstate();
            slave = 1i32;
            while slave <= ec_slavecount {
                if ec_slave[slave as usize].group as libc::c_int == currentgroup as libc::c_int
                    && ec_slave[slave as usize].state as libc::c_int
                        != EC_STATE_OPERATIONAL as libc::c_int
                {
                    ec_group[currentgroup as usize].docheckstate = 1u8;
                    if ec_slave[slave as usize].state as libc::c_int
                        == EC_STATE_SAFE_OP as libc::c_int + EC_STATE_ERROR as libc::c_int
                    {
                        println!(
                            "ERROR : slave {:} is in SAFE_OP + ERROR, attempting ack.",
                            slave as libc::c_int
                        );
                        ec_slave[slave as usize].state = (EC_STATE_SAFE_OP as libc::c_int
                            + EC_STATE_ACK as libc::c_int)
                            as uint16;
                        ec_writestate(slave as uint16);
                    } else if ec_slave[slave as usize].state as libc::c_int
                        == EC_STATE_SAFE_OP as libc::c_int
                    {
                        println!(
                            "WARNING : slave {:} is in SAFE_OP, change to OPERATIONAL.",
                            slave as libc::c_int
                        );
                        ec_slave[slave as usize].state = EC_STATE_OPERATIONAL as uint16;
                        ec_writestate(slave as uint16);
                    } else if ec_slave[slave as usize].state as libc::c_int
                        > EC_STATE_NONE as libc::c_int
                    {
                        if ec_reconfig_slave(slave as uint16, 500i32) != 0 {
                            ec_slave[slave as usize].islost = 0u8;
                            println!("MESSAGE : slave {:} reconfigured", slave as libc::c_int);
                        }
                    } else if ec_slave[slave as usize].islost == 0 {
                        /* re-check state */
                        ec_statecheck(slave as uint16, EC_STATE_OPERATIONAL as uint16, 2000i32);
                        if ec_slave[slave as usize].state as libc::c_int
                            == EC_STATE_NONE as libc::c_int
                        {
                            ec_slave[slave as usize].islost = 1u8;
                            println!("ERROR : slave {:} lost", slave as libc::c_int);
                        }
                    }
                }
                if ec_slave[slave as usize].islost != 0 {
                    if ec_slave[slave as usize].state as libc::c_int == EC_STATE_NONE as libc::c_int
                    {
                        if ec_recover_slave(slave as uint16, 500i32) != 0 {
                            ec_slave[slave as usize].islost = 0u8;
                            println!("MESSAGE : slave {:} recovered", slave as libc::c_int);
                        }
                    } else {
                        ec_slave[slave as usize].islost = 0u8;
                        println!("MESSAGE : slave {:} found", slave as libc::c_int);
                    }
                }
                slave += 1
            }
            if ec_group[currentgroup as usize].docheckstate == 0 {
                println!("OK : all slaves resumed OPERATIONAL.");
            }
        }
        osal_usleep(10000u32);
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    println!("SOEM (Simple Open EtherCAT Master)\nSimple test");
    if argc > 1i32 {
        /* create thread to handle slave error handling in OP */
        //      pthread_create( &thread1, NULL, (void *) &ecatcheck, (void*) &ctime);
        osal_thread_create(
            &mut thread1 as *mut *mut pthread_t as *mut libc::c_void,
            128000i32,
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
        simpletest(*argv.offset(1isize));
    } else {
        let mut adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;

        println!("Usage: simple_test ifname1\nifname = eth0 for example");
        println!("\nAvailable adapters:");
        adapter = ec_find_adapters();
        while !adapter.is_null() {
            println!(
                "    - {:}  ({:})",
                {
                    std::ffi::CStr::from_ptr((*adapter).name.as_mut_ptr() as *const libc::c_char)
                        .to_str()
                        .unwrap()
                },
                {
                    std::ffi::CStr::from_ptr((*adapter).desc.as_mut_ptr() as *const libc::c_char)
                        .to_str()
                        .unwrap()
                }
            );
            adapter = (*adapter).next
        }
        ec_free_adapters(adapter);
    }
    println!("End program");
    return 0i32;
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
    unsafe { ::std::process::exit(main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr())) }
}
