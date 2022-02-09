use crate::osal::linux::osal::{osal_timer_is_expired, osal_timer_start};
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
pub type __time_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;

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
