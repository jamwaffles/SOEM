use crate::{
    ethercatconfig::{ecx_config_init, ecx_config_map_group, ecx_reconfig_slave},
    ethercatmain::{
        ecx_init, ecx_readstate, ecx_receive_processdata, ecx_send_processdata, ecx_statecheck,
        ecx_writestate,
    },
    osal::linux::osal::{
        ec_timet, osal_current_time, osal_time_diff, osal_timer_is_expired, osal_timer_start,
        osal_usleep,
    },
};
use libc::{
    bind, ioctl, memcpy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t,
    pthread_mutex_unlock, pthread_mutexattr_init, pthread_mutexattr_t, recv, send, setsockopt,
    sockaddr, socket, strcpy, timeval,
};

pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;

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

pub type ec_eepromFMMUt = ec_eepromFMMU;

#[repr(C)]
#[derive(Copy, Clone)]
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
        0i32,
        core::mem::size_of::<Fieldbus>(),
    );
    (*fieldbus).iface = iface;
    (*fieldbus).group = 0u8;
    (*fieldbus).roundtrip_time = 0i32;
    (*fieldbus).ecaterror = 0u8;
    /* Initialize the ecx_contextt data structure */
    context = &mut (*fieldbus).context;
    (*context).port = &mut (*fieldbus).port;
    (*context).slavelist = (*fieldbus).slavelist.as_mut_ptr();
    (*context).slavecount = &mut (*fieldbus).slavecount;
    (*context).maxslave = 200i32;
    (*context).grouplist = (*fieldbus).grouplist.as_mut_ptr();
    (*context).maxgroup = 2i32;
    (*context).esibuf = (*fieldbus).esibuf.as_mut_ptr();
    (*context).esimap = (*fieldbus).esimap.as_mut_ptr();
    (*context).esislave = 0u16;
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
    (*context).manualstatechange = 0i32;
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
    wkc = ecx_receive_processdata(context, 2000i32);
    end = osal_current_time();
    osal_time_diff(&mut start, &mut end, &mut diff);
    (*fieldbus).roundtrip_time =
        diff.sec.wrapping_mul(1000000u32).wrapping_add(diff.usec) as libc::c_int;
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
    print!("Initializing SOEM on \'{:}\'... ", {
        std::ffi::CStr::from_ptr((*fieldbus).iface as *const libc::c_char)
            .to_str()
            .unwrap()
    });
    if ecx_init(context, (*fieldbus).iface) == 0 {
        println!("no socket connection");
        return 0u8;
    }

    println!("done");
    print!("Finding autoconfig slaves... ");
    if ecx_config_init(context, 0u8) <= 0i32 {
        println!("no slaves found");
        return 0u8;
    }

    println!("{:} slaves found", (*fieldbus).slavecount as libc::c_int);
    print!("Sequential mapping of I/O... ");
    ecx_config_map_group(
        context,
        (*fieldbus).map.as_mut_ptr() as *mut libc::c_void,
        (*fieldbus).group,
    );
    print!(
        "mapped {:}O+{:}I bytes from {:} segments",
        (*grp).Obytes as libc::c_int,
        (*grp).Ibytes as libc::c_int,
        (*grp).nsegments as libc::c_int as libc::c_int
    );
    if (*grp).nsegments as libc::c_int > 1i32 {
        /* Show how slaves are distrubuted */
        i = 0i32;
        while i < (*grp).nsegments as libc::c_int {
            print!(
                "{:}{:}",
                {
                    std::ffi::CStr::from_ptr(if i == 0i32 {
                        b" (\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"+\x00" as *const u8 as *const libc::c_char
                    } as *const libc::c_char)
                    .to_str()
                    .unwrap()
                },
                (*grp).IOsegment[i as usize] as libc::c_int
            );
            i += 1
        }
        print!(" slaves)");
    }

    println!("");
    print!("Configuring distributed clock... ");
    ecx_configdc(context);

    println!("done");
    print!("Waiting for all slaves in safe operational... ");
    ecx_statecheck(context, 0u16, EC_STATE_SAFE_OP as uint16, 2000000i32 * 4i32);

    println!("done");
    print!("Send a roundtrip to make outputs in slaves happy... ");
    fieldbus_roundtrip(fieldbus);

    println!("done");
    print!("Setting operational state..");
    /* Act on slave 0 (a virtual slave used for broadcasting) */
    slave = (*fieldbus).slavelist.as_mut_ptr();
    (*slave).state = EC_STATE_OPERATIONAL as uint16;
    ecx_writestate(context, 0u16);
    /* Poll the result ten times before giving up */
    i = 0i32;
    while i < 10i32 {
        print!(".");
        fieldbus_roundtrip(fieldbus);
        ecx_statecheck(
            context,
            0u16,
            EC_STATE_OPERATIONAL as uint16,
            2000000i32 / 10i32,
        );
        if (*slave).state as libc::c_int == EC_STATE_OPERATIONAL as libc::c_int {
            println!(" all slaves are now operational");
            return 1u8;
        }
        i += 1
    }
    print!(" failed,");
    ecx_readstate(context);
    i = 1i32;
    while i <= (*fieldbus).slavecount {
        slave = (*fieldbus).slavelist.as_mut_ptr().offset(i as isize);
        if (*slave).state as libc::c_int != EC_STATE_OPERATIONAL as libc::c_int {
            print!(
                " slave {:} is 0x{:4X} (AL-status=0x{:4X} {:})",
                i as libc::c_int,
                (*slave).state as libc::c_int as libc::c_uint,
                (*slave).ALstatuscode as libc::c_int as libc::c_uint,
                {
                    std::ffi::CStr::from_ptr(
                        ec_ALstatuscode2string((*slave).ALstatuscode) as *const libc::c_char
                    )
                    .to_str()
                    .unwrap()
                }
            );
        }
        i += 1
    }
    println!("");
    return 0u8;
}
unsafe extern "C" fn fieldbus_stop(mut fieldbus: *mut Fieldbus) {
    let mut context: *mut ecx_contextt = 0 as *mut ecx_contextt;
    let mut slave: *mut ec_slavet = 0 as *mut ec_slavet;
    context = &mut (*fieldbus).context;
    /* Act on slave 0 (a virtual slave used for broadcasting) */
    slave = (*fieldbus).slavelist.as_mut_ptr();
    print!("Requesting init state on all slaves... ");
    (*slave).state = EC_STATE_INIT as uint16;
    ecx_writestate(context, 0u16);

    println!("done");
    print!("Close socket... ");
    ecx_close(context);
    println!("done");
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
    expected_wkc = (*grp).outputsWKC as libc::c_int * 2i32 + (*grp).inputsWKC as libc::c_int;
    print!(
        "{:6} usec  WKC {:}",
        (*fieldbus).roundtrip_time as libc::c_int,
        wkc as libc::c_int
    );
    if wkc < expected_wkc {
        println!(" wrong (expected {:})", expected_wkc as libc::c_int);
        return 0u8;
    }
    print!("  O:");
    n = 0u32;
    while n < (*grp).Obytes {
        print!(
            " {:2X}",
            *(*grp).outputs.offset(n as isize) as libc::c_int as libc::c_uint
        );
        n = n.wrapping_add(1)
    }
    print!("  I:");
    n = 0u32;
    while n < (*grp).Ibytes {
        print!(
            " {:2X}",
            *(*grp).inputs.offset(n as isize) as libc::c_int as libc::c_uint
        );
        n = n.wrapping_add(1)
    }
    print!("  T: {:}\r", (*fieldbus).DCtime as libc::c_longlong);
    return 1u8;
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
    (*grp).docheckstate = 0u8;
    ecx_readstate(context);
    i = 1i32;
    while i <= (*fieldbus).slavecount {
        slave = (*context).slavelist.offset(i as isize);
        if !((*slave).group as libc::c_int != (*fieldbus).group as libc::c_int) {
            if (*slave).state as libc::c_int != EC_STATE_OPERATIONAL as libc::c_int {
                (*grp).docheckstate = 1u8;
                if (*slave).state as libc::c_int
                    == EC_STATE_SAFE_OP as libc::c_int + EC_STATE_ERROR as libc::c_int
                {
                    println!(
                        "* Slave {:} is in SAFE_OP+ERROR, attempting ACK",
                        i as libc::c_int
                    );
                    (*slave).state =
                        (EC_STATE_SAFE_OP as libc::c_int + EC_STATE_ACK as libc::c_int) as uint16;
                    ecx_writestate(context, i as uint16);
                } else if (*slave).state as libc::c_int == EC_STATE_SAFE_OP as libc::c_int {
                    println!(
                        "* Slave {:} is in SAFE_OP, change to OPERATIONAL",
                        i as libc::c_int
                    );
                    (*slave).state = EC_STATE_OPERATIONAL as uint16;
                    ecx_writestate(context, i as uint16);
                } else if (*slave).state as libc::c_int > EC_STATE_NONE as libc::c_int {
                    if ecx_reconfig_slave(context, i as uint16, 2000i32) != 0 {
                        (*slave).islost = 0u8;
                        println!("* Slave {:} reconfigured", i as libc::c_int);
                    }
                } else if (*slave).islost == 0 {
                    ecx_statecheck(
                        context,
                        i as uint16,
                        EC_STATE_OPERATIONAL as uint16,
                        2000i32,
                    );
                    if (*slave).state as libc::c_int == EC_STATE_NONE as libc::c_int {
                        (*slave).islost = 1u8;
                        println!("* Slave {:} lost", i as libc::c_int);
                    }
                }
            } else if (*slave).islost != 0 {
                if (*slave).state as libc::c_int != EC_STATE_NONE as libc::c_int {
                    (*slave).islost = 0u8;
                    println!("* Slave {:} found", i as libc::c_int);
                } else if ecx_recover_slave(context, i as uint16, 2000i32) != 0 {
                    (*slave).islost = 0u8;
                    println!("* Slave {:} recovered", i as libc::c_int);
                }
            }
        }
        i += 1
    }
    if (*grp).docheckstate == 0 {
        println!("All slaves resumed OPERATIONAL");
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
    if argc != 2i32 {
        let mut adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;

        println!("Usage: simple_ng IFNAME1\nIFNAME1 is the NIC interface name, e.g. \'eth0\'");
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
        return 1i32;
    }
    fieldbus_initialize(&mut fieldbus, *argv.offset(1isize));
    if fieldbus_start(&mut fieldbus) != 0 {
        let mut i: libc::c_int = 0;
        let mut min_time: libc::c_int = 0;
        let mut max_time: libc::c_int = 0;
        max_time = 0i32;
        min_time = max_time;
        i = 1i32;
        while i <= 10000i32 {
            print!("Iteration {:4}:", i as libc::c_int);
            if fieldbus_dump(&mut fieldbus) == 0 {
                fieldbus_check_state(&mut fieldbus);
            } else if i == 1i32 {
                max_time = fieldbus.roundtrip_time;
                min_time = max_time
            } else if fieldbus.roundtrip_time < min_time {
                min_time = fieldbus.roundtrip_time
            } else if fieldbus.roundtrip_time > max_time {
                max_time = fieldbus.roundtrip_time
            }
            osal_usleep(5000u32);
            i += 1
        }
        println!(
            "\nRoundtrip time (usec): min {:} max {:}",
            min_time as libc::c_int, max_time as libc::c_int
        );
        fieldbus_stop(&mut fieldbus);
    }
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
