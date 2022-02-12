use libc::{c_void, pthread_t};
use soem::{
    config::{ec_config_init, ec_config_map, ec_reconfig_slave, ec_recover_slave},
    dc::ec_configdc,
    main::{
        ec_DCtime, ec_adaptert, ec_close, ec_find_adapters, ec_free_adapters, ec_group, ec_init,
        ec_readstate, ec_receive_processdata, ec_send_processdata, ec_slave, ec_slavecount,
        ec_statecheck, ec_writestate,
    },
    osal::linux::osal::{osal_thread_create, osal_usleep},
    print::ec_ALstatuscode2string,
    types::{self, ec_state, EC_TIMEOUTRET, EC_TIMEOUTSTATE},
};

const EC_TIMEOUTMON: u32 = 500;

static mut IO_MAP: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut thread1: *mut pthread_t = 0 as *mut pthread_t;
#[no_mangle]
pub static mut expectedWKC: libc::c_int = 0;
#[no_mangle]
pub static mut needlf: bool = false;

static mut WKC: libc::c_int = 0;
#[no_mangle]
pub static mut inOP: bool = false;
#[no_mangle]
pub static mut currentgroup: u8 = 0u8;
#[no_mangle]
pub unsafe fn simpletest(ifname: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut oloop: libc::c_int = 0;
    let mut iloop: libc::c_int = 0;
    let mut chk: libc::c_int = 0;
    needlf = false;
    inOP = false;
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
            ec_config_map(&mut IO_MAP as *mut [libc::c_char; 4096] as *mut libc::c_void);
            ec_configdc();
            println!("Slaves mapped, state to SAFE_OP.");
            /* wait for all slaves to reach SAFE_OP state */
            ec_statecheck(0u16, ec_state::EC_STATE_SAFE_OP as u16, EC_TIMEOUTSTATE * 4);
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
            ec_slave[0usize].state = ec_state::EC_STATE_OPERATIONAL as u16;
            /* send one valid process data to make outputs in slaves happy*/
            ec_send_processdata();
            ec_receive_processdata(EC_TIMEOUTRET);
            /* request OP state for all slaves */
            ec_writestate(0u16);
            chk = 200i32;
            loop
            /* wait for all slaves to reach OP state */
            {
                ec_send_processdata();
                ec_receive_processdata(EC_TIMEOUTRET);
                ec_statecheck(0u16, ec_state::EC_STATE_OPERATIONAL as u16, 50000);
                let fresh0 = chk;
                chk = chk - 1;
                if !(fresh0 != 0
                    && ec_slave[0usize].state as libc::c_int
                        != ec_state::EC_STATE_OPERATIONAL as libc::c_int)
                {
                    break;
                }
            }
            if ec_slave[0usize].state as libc::c_int
                == ec_state::EC_STATE_OPERATIONAL as libc::c_int
            {
                println!("Operational state reached for all slaves.");
                inOP = true;
                /* cyclic loop */
                i = 1i32;
                while i <= 10000i32 {
                    ec_send_processdata();
                    ::core::ptr::write_volatile(
                        &mut WKC as *mut libc::c_int,
                        ec_receive_processdata(EC_TIMEOUTRET),
                    );
                    if WKC >= expectedWKC {
                        print!(
                            "Processdata cycle {:4}, WKC {:} , O:",
                            i as libc::c_int, WKC as libc::c_int
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
                        needlf = true;
                    }
                    osal_usleep(5000u32);
                    i += 1
                }
                inOP = false;
            } else {
                println!("Not all slaves reached operational state.");
                ec_readstate();
                i = 1i32;
                while i <= ec_slavecount {
                    if ec_slave[i as usize].state as libc::c_int
                        != ec_state::EC_STATE_OPERATIONAL as libc::c_int
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
            ec_slave[0usize].state = ec_state::EC_STATE_INIT as u16;
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
pub unsafe fn ecatcheck(mut _ptr: *mut libc::c_void) {
    let mut slave: libc::c_int = 0;

    loop {
        if inOP == true
            && (WKC < expectedWKC
                || ec_group[currentgroup as usize].docheckstate as libc::c_int != 0)
        {
            if needlf == true {
                needlf = false;
                println!("");
            }
            /* one ore more slaves are not responding */
            ec_group[currentgroup as usize].docheckstate = false;
            ec_readstate();
            slave = 1i32;
            while slave <= ec_slavecount {
                if ec_slave[slave as usize].group as libc::c_int == currentgroup as libc::c_int
                    && ec_slave[slave as usize].state as libc::c_int
                        != ec_state::EC_STATE_OPERATIONAL as libc::c_int
                {
                    ec_group[currentgroup as usize].docheckstate = true;
                    if ec_slave[slave as usize].state as libc::c_int
                        == ec_state::EC_STATE_SAFE_OP as libc::c_int
                            + ec_state::EC_STATE_ERROR as libc::c_int
                    {
                        println!(
                            "ERROR : slave {:} is in SAFE_OP + ERROR, attempting ack.",
                            slave as libc::c_int
                        );
                        ec_slave[slave as usize].state = (ec_state::EC_STATE_SAFE_OP as libc::c_int
                            + types::EC_STATE_ACK as libc::c_int)
                            as u16;
                        ec_writestate(slave as u16);
                    } else if ec_slave[slave as usize].state as libc::c_int
                        == ec_state::EC_STATE_SAFE_OP as libc::c_int
                    {
                        println!(
                            "WARNING : slave {:} is in SAFE_OP, change to OPERATIONAL.",
                            slave as libc::c_int
                        );
                        ec_slave[slave as usize].state = ec_state::EC_STATE_OPERATIONAL as u16;
                        ec_writestate(slave as u16);
                    } else if ec_slave[slave as usize].state as libc::c_int
                        > ec_state::EC_STATE_NONE as libc::c_int
                    {
                        if ec_reconfig_slave(slave as u16, EC_TIMEOUTMON) != 0 {
                            ec_slave[slave as usize].islost = false;
                            println!("MESSAGE : slave {:} reconfigured", slave as libc::c_int);
                        }
                    } else if ec_slave[slave as usize].islost == true {
                        /* re-check state */
                        ec_statecheck(
                            slave as u16,
                            ec_state::EC_STATE_OPERATIONAL as u16,
                            EC_TIMEOUTRET,
                        );
                        if ec_slave[slave as usize].state as libc::c_int
                            == ec_state::EC_STATE_NONE as libc::c_int
                        {
                            ec_slave[slave as usize].islost = true;
                            println!("ERROR : slave {:} lost", slave as libc::c_int);
                        }
                    }
                }
                if ec_slave[slave as usize].islost == true {
                    if ec_slave[slave as usize].state as libc::c_int
                        == ec_state::EC_STATE_NONE as libc::c_int
                    {
                        if ec_recover_slave(slave as u16, EC_TIMEOUTMON) != 0 {
                            ec_slave[slave as usize].islost = false;
                            println!("MESSAGE : slave {:} recovered", slave as libc::c_int);
                        }
                    } else {
                        ec_slave[slave as usize].islost = false;
                        println!("MESSAGE : slave {:} found", slave as libc::c_int);
                    }
                }
                slave += 1
            }
            if ec_group[currentgroup as usize].docheckstate == false {
                println!("OK : all slaves resumed OPERATIONAL.");
            }
        }
        osal_usleep(10000u32);
    }
}
unsafe fn main_0(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    println!("SOEM (Simple Open EtherCAT Master)\nSimple test");
    if argc > 1i32 {
        /* create thread to handle slave error handling in OP */
        osal_thread_create(
            &mut thread1 as *mut *mut pthread_t as *mut libc::c_void,
            128000i32,
            ::core::mem::transmute::<
                Option<unsafe fn(_: *mut libc::c_void) -> ()>,
                *mut libc::c_void,
            >(Some(ecatcheck as unsafe fn(_: *mut libc::c_void) -> ())),
            &mut None::<()> as *mut _ as *mut c_void,
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
