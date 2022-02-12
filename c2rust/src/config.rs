use crate::{
    base::{
        ecx_APRD, ecx_APRDw, ecx_APWRw, ecx_BRD, ecx_BWR, ecx_FPRD, ecx_FPRDw, ecx_FPWR, ecx_FPWRw,
    },
    coe::{ecx_readPDOmap, ecx_readPDOmapCA},
    main::{
        ec_eepromPDOt, ec_fmmut, ec_groupt, ec_slavet, ec_smt, ecx_context, ecx_contextt,
        ecx_eeprom2master, ecx_eeprom2pdi, ecx_readeeprom, ecx_readeeprom1, ecx_readeeprom2,
        ecx_siiFMMU, ecx_siiPDO, ecx_siiSM, ecx_siiSMnext, ecx_siifind, ecx_siigetbyte,
        ecx_siistring, ecx_statecheck, EC_MAXSLAVE,
    },
    osal::linux::osal::osal_usleep,
    soe::ecx_readIDNmap,
    types::{
        self, EthercatRegister, SiiCategory, SiiGeneral, SlaveState, EC_SLAVECOUNTEXCEEDED,
        EC_TIMEOUTEEP, EC_TIMEOUTRET3, EC_TIMEOUTSAFE, EC_TIMEOUTSTATE,
    },
};
use libc::{memcpy, memset, sprintf, strcpy};

#[derive(Copy, Clone)]
struct ec_configlist_t {
    pub man: u32,
    pub id: u32,
    pub name: &'static str,
    pub Dtype: u8,
    pub Ibits: u16,
    pub Obits: u16,
    pub SM2a: u16,
    pub SM2f: u32,
    pub SM3a: u16,
    pub SM3f: u32,
    pub FM0ac: u8,
    pub FM1ac: u8,
}

#[derive(Copy, Clone)]
pub struct ecx_mapt_t {
    pub thread_n: libc::c_int,
    pub running: libc::c_int,
    pub context: *mut ecx_contextt,
    pub slave: u16,
}
#[no_mangle]
pub static mut ecx_mapt: [ecx_mapt_t; 1] = [ecx_mapt_t {
    thread_n: 0,
    running: 0,
    context: 0 as *mut ecx_contextt,
    slave: 0,
}; 1];
#[no_mangle]
static mut ec_configlist: [ec_configlist_t; 24] = {
    [
        {
            let init = ec_configlist_t {
                man: 0u32,
                id: 0u32,
                name: "",
                Dtype: 0u8,
                Ibits: 0u16,
                Obits: 0u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x44c2c52u32,
                name: "EK1100",
                Dtype: 1u8,
                Ibits: 0u16,
                Obits: 0u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x3ea3052u32,
                name: "EL1002",
                Dtype: 2u8,
                Ibits: 2u16,
                Obits: 0u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x3ec3052u32,
                name: "EL1004",
                Dtype: 2u8,
                Ibits: 4u16,
                Obits: 0u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x3f43052u32,
                name: "EL1012",
                Dtype: 2u8,
                Ibits: 2u16,
                Obits: 0u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x3f63052u32,
                name: "EL1014",
                Dtype: 2u8,
                Ibits: 4u16,
                Obits: 0u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x3fa3052u32,
                name: "EL1018",
                Dtype: 2u8,
                Ibits: 8u16,
                Obits: 0u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x7d23052u32,
                name: "EL2002",
                Dtype: 3u8,
                Ibits: 0u16,
                Obits: 2u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x7d43052u32,
                name: "EL2004",
                Dtype: 3u8,
                Ibits: 0u16,
                Obits: 4u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x7d83052u32,
                name: "EL2008",
                Dtype: 3u8,
                Ibits: 0u16,
                Obits: 8u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x7f03052u32,
                name: "EL2032",
                Dtype: 6u8,
                Ibits: 2u16,
                Obits: 2u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0xc1e3052u32,
                name: "EL3102",
                Dtype: 4u8,
                Ibits: 48u16,
                Obits: 0u16,
                SM2a: 0x1000u16,
                SM2f: 0x24u32,
                SM3a: 0x1100u16,
                SM3f: 0x10020u32,
                FM0ac: 0u8,
                FM1ac: 1u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0xc283052u32,
                name: "EL3112",
                Dtype: 4u8,
                Ibits: 48u16,
                Obits: 0u16,
                SM2a: 0x1000u16,
                SM2f: 0x24u32,
                SM3a: 0x1100u16,
                SM3f: 0x10020u32,
                FM0ac: 0u8,
                FM1ac: 1u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0xc323052u32,
                name: "EL3122",
                Dtype: 4u8,
                Ibits: 48u16,
                Obits: 0u16,
                SM2a: 0x1000u16,
                SM2f: 0x24u32,
                SM3a: 0x1100u16,
                SM3f: 0x10020u32,
                FM0ac: 0u8,
                FM1ac: 1u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0xc463052u32,
                name: "EL3142",
                Dtype: 4u8,
                Ibits: 48u16,
                Obits: 0u16,
                SM2a: 0x1000u16,
                SM2f: 0x24u32,
                SM3a: 0x1100u16,
                SM3f: 0x10020u32,
                FM0ac: 0u8,
                FM1ac: 1u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0xc503052u32,
                name: "EL3152",
                Dtype: 4u8,
                Ibits: 48u16,
                Obits: 0u16,
                SM2a: 0x1000u16,
                SM2f: 0x24u32,
                SM3a: 0x1100u16,
                SM3f: 0x10020u32,
                FM0ac: 0u8,
                FM1ac: 1u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0xc5a3052u32,
                name: "EL3162",
                Dtype: 4u8,
                Ibits: 48u16,
                Obits: 0u16,
                SM2a: 0x1000u16,
                SM2f: 0x24u32,
                SM3a: 0x1100u16,
                SM3f: 0x10020u32,
                FM0ac: 0u8,
                FM1ac: 1u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0xfc03052u32,
                name: "EL4032",
                Dtype: 5u8,
                Ibits: 0u16,
                Obits: 32u16,
                SM2a: 0x1100u16,
                SM2f: 0x10024u32,
                SM3a: 0x1180u16,
                SM3f: 0x22u32,
                FM0ac: 1u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x10063052u32,
                name: "EL4102",
                Dtype: 5u8,
                Ibits: 0u16,
                Obits: 32u16,
                SM2a: 0x1000u16,
                SM2f: 0x10024u32,
                SM3a: 0x1100u16,
                SM3f: 0x22u32,
                FM0ac: 1u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x10103052u32,
                name: "EL4112",
                Dtype: 5u8,
                Ibits: 0u16,
                Obits: 32u16,
                SM2a: 0x1000u16,
                SM2f: 0x10024u32,
                SM3a: 0x1100u16,
                SM3f: 0x22u32,
                FM0ac: 1u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x101a3052u32,
                name: "EL4122",
                Dtype: 5u8,
                Ibits: 0u16,
                Obits: 32u16,
                SM2a: 0x1000u16,
                SM2f: 0x10024u32,
                SM3a: 0x1100u16,
                SM3f: 0x22u32,
                FM0ac: 1u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x10243052u32,
                name: "EL4132",
                Dtype: 5u8,
                Ibits: 0u16,
                Obits: 32u16,
                SM2a: 0x1000u16,
                SM2f: 0x10024u32,
                SM3a: 0x1100u16,
                SM3f: 0x22u32,
                FM0ac: 1u8,
                FM1ac: 0u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0x2u32,
                id: 0x13ed3052u32,
                name: "EL5101",
                Dtype: 7u8,
                Ibits: 40u16,
                Obits: 24u16,
                SM2a: 0x1000u16,
                SM2f: 0x10024u32,
                SM3a: 0x1100u16,
                SM3f: 0x10020u32,
                FM0ac: 1u8,
                FM1ac: 1u8,
            };
            init
        },
        {
            let init = ec_configlist_t {
                man: 0xffffffffu32,
                id: 0u32,
                name: "",
                Dtype: 0u8,
                Ibits: 0u16,
                Obits: 0u16,
                SM2a: 0u16,
                SM2f: 0u32,
                SM3a: 0u16,
                SM3f: 0u32,
                FM0ac: 0u8,
                FM1ac: 0u8,
            };
            init
        },
    ]
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
pub unsafe fn ec_findconfig(man: u32, id: u32) -> libc::c_int {
    let mut i: libc::c_int = 0i32;
    loop {
        i += 1;
        if !(ec_configlist[i as usize].man != 0xffffffffu32
            && (ec_configlist[i as usize].man != man || ec_configlist[i as usize].id != id))
        {
            break;
        }
    }
    if ec_configlist[i as usize].man == 0xffffffffu32 {
        i = 0i32
    }
    return i;
}
#[no_mangle]
pub unsafe fn ecx_init_context(context: *mut ecx_contextt) {
    let mut lp: libc::c_int = 0;
    *(*context).slavecount = 0i32;
    /* clean ec_slave array */
    memset(
        (*context).slavelist as *mut libc::c_void,
        0i32,
        core::mem::size_of::<ec_slavet>().wrapping_mul((*context).maxslave as usize),
    );
    memset(
        (*context).grouplist as *mut libc::c_void,
        0i32,
        core::mem::size_of::<ec_groupt>().wrapping_mul((*context).maxgroup as usize),
    );
    /* clear slave eeprom cache, does not actually read any eeprom */
    ecx_siigetbyte(context, 0u16, ((128i32) << 5i32) as u16);
    lp = 0i32;
    while lp < (*context).maxgroup {
        /* default start address per group entry */
        (*(*context).grouplist.offset(lp as isize)).logstartaddr = (lp << 16i32) as u32;
        lp += 1
    }
}
#[no_mangle]
pub unsafe fn ecx_detect_slaves(context: *mut ecx_contextt) -> libc::c_int {
    let mut b: u8 = 0;
    let mut w: u16 = 0;
    let mut wkc: libc::c_int = 0;
    /* make special pre-init register writes to enable MAC[1] local administered bit *
     * setting for old netX100 slaves */
    b = 0u8; /* Ignore Alias register */
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_DLALIAS as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    ); /* Reset all slaves to Init */
    b = (SlaveState::Init as libc::c_int | types::EC_STATE_ACK as libc::c_int) as u8;
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_ALCTL as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    /* netX100 should now be happy */
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_ALCTL as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    ); /* Reset all slaves to Init */
    wkc = ecx_BRD(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_TYPE as u16,
        ::core::mem::size_of::<u16>(),
        &mut w as *mut u16 as *mut libc::c_void,
        EC_TIMEOUTSAFE,
    ); /* detect number of slaves */
    if wkc > 0i32 {
        /* this is strictly "less than" since the master is "slave 0" */
        if wkc < EC_MAXSLAVE as i32 {
            *(*context).slavecount = wkc
        } else {
            // FIXME: Find other `EC_PRINT` and convert to eprintln. This should be a log::error in
            // the future.
            eprintln!(
                "Error: too many slaves on network: num_slaves={}, EC_MAXSLAVE={}",
                wkc, EC_MAXSLAVE,
            );
            return EC_SLAVECOUNTEXCEEDED;
        }
    } /* deact loop manual */
    return wkc; /* set IRQ mask */
}
unsafe fn ecx_set_slaves_to_default(context: *mut ecx_contextt) {
    let mut b: u8 = 0; /* reset CRC counters */
    let mut w: u16 = 0; /* reset FMMU's */
    let mut zbuf: [u8; 64] = [0; 64]; /* reset SyncM */
    memset(
        &mut zbuf as *mut [u8; 64] as *mut libc::c_void,
        0i32,
        core::mem::size_of::<[u8; 64]>(),
    ); /* reset activation register */
    b = 0u8; /* reset system time+ofs */
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_DLPORT as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    ); /* DC speedstart */
    w = 0x4u16; /* DC filt expr */
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_IRQMASK as u16,
        ::core::mem::size_of::<u16>(),
        &mut w as *mut u16 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    ); /* Ignore Alias register */
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_RXERR as u16,
        8,
        &mut zbuf as *mut [u8; 64] as *mut libc::c_void,
        EC_TIMEOUTRET3,
    ); /* Reset all slaves to Init */
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_FMMU0 as u16,
        16 * 3,
        &mut zbuf as *mut [u8; 64] as *mut libc::c_void,
        EC_TIMEOUTRET3,
    ); /* force Eeprom from PDI */
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_SM0 as u16,
        8 * 4,
        &mut zbuf as *mut [u8; 64] as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    b = 0u8;
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_DCSYNCACT as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_DCSYSTIME as u16,
        4,
        &mut zbuf as *mut [u8; 64] as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    w = 0x1000u16;
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_DCSPEEDCNT as u16,
        ::core::mem::size_of::<u16>(),
        &mut w as *mut u16 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    w = 0xc00u16;
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_DCTIMEFILT as u16,
        ::core::mem::size_of::<u16>(),
        &mut w as *mut u16 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    b = 0u8;
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_DLALIAS as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    b = (SlaveState::Init as libc::c_int | types::EC_STATE_ACK as libc::c_int) as u8;
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_ALCTL as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    b = 2u8;
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_EEPCFG as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    b = 0u8;
    ecx_BWR(
        (*context).port.as_mut().unwrap(),
        0u16,
        EthercatRegister::ECT_REG_EEPCFG as u16,
        ::core::mem::size_of::<u8>(),
        &mut b as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET3,
    );
    /* set Eeprom to master */
}
unsafe fn ecx_config_from_table(context: *mut ecx_contextt, slave: u16) -> libc::c_int {
    let mut cindex: libc::c_int = 0;
    let mut csl: *mut ec_slavet = 0 as *mut ec_slavet;
    csl = &mut *(*context).slavelist.offset(slave as isize) as *mut ec_slavet;
    cindex = ec_findconfig((*csl).eep_man, (*csl).eep_id);
    (*csl).configindex = cindex as u16;
    /* slave found in configuration table ? */
    if cindex != 0 {
        (*csl).Dtype = ec_configlist[cindex as usize].Dtype as u16;
        strcpy(
            (*csl).name.as_mut_ptr(),
            ec_configlist[cindex as usize].name.as_ptr() as *const i8,
        );
        (*csl).Ibits = ec_configlist[cindex as usize].Ibits;
        (*csl).Obits = ec_configlist[cindex as usize].Obits;
        if (*csl).Obits != 0 {
            (*csl).FMMU0func = 1u8
        }
        if (*csl).Ibits != 0 {
            (*csl).FMMU1func = 2u8
        }
        (*csl).FMMU[0usize].FMMUactive = ec_configlist[cindex as usize].FM0ac;
        (*csl).FMMU[1usize].FMMUactive = ec_configlist[cindex as usize].FM1ac;
        (*csl).SM[2usize].StartAddr = ec_configlist[cindex as usize].SM2a;
        (*csl).SM[2usize].SMflags = ec_configlist[cindex as usize].SM2f;
        /* simple (no mailbox) output slave found ? */
        if (*csl).Obits as libc::c_int != 0 && (*csl).SM[2usize].StartAddr == 0 {
            (*csl).SM[0usize].StartAddr = 0xf00u16;
            (*csl).SM[0usize].SMlength = (((*csl).Obits as libc::c_int + 7i32) / 8i32) as u16;
            (*csl).SM[0usize].SMflags = 0x10044u32;
            (*csl).FMMU[0usize].FMMUactive = 1u8;
            (*csl).FMMU[0usize].FMMUtype = 2u8;
            (*csl).SMtype[0usize] = 3u8
        } else {
            /* complex output slave */
            (*csl).SM[2usize].SMlength = (((*csl).Obits as libc::c_int + 7i32) / 8i32) as u16;
            (*csl).SMtype[2usize] = 3u8
        }
        (*csl).SM[3usize].StartAddr = ec_configlist[cindex as usize].SM3a;
        (*csl).SM[3usize].SMflags = ec_configlist[cindex as usize].SM3f;
        /* simple (no mailbox) input slave found ? */
        if (*csl).Ibits as libc::c_int != 0 && (*csl).SM[3usize].StartAddr == 0 {
            (*csl).SM[1usize].StartAddr = 0x1000u16;
            (*csl).SM[1usize].SMlength = (((*csl).Ibits as libc::c_int + 7i32) / 8i32) as u16;
            (*csl).SM[1usize].SMflags = 0u32;
            (*csl).FMMU[1usize].FMMUactive = 1u8;
            (*csl).FMMU[1usize].FMMUtype = 1u8;
            (*csl).SMtype[1usize] = 4u8
        } else {
            /* complex input slave */
            (*csl).SM[3usize].SMlength = (((*csl).Ibits as libc::c_int + 7i32) / 8i32) as u16;
            (*csl).SMtype[3usize] = 4u8
        }
    }
    return cindex;
}
/* If slave has SII and same slave ID done before, use previous data.
 * This is safe because SII is constant for same slave ID.
 */
unsafe fn ecx_lookup_prev_sii(context: *mut ecx_contextt, slave: u16) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nSM: libc::c_int = 0;
    if slave as libc::c_int > 1i32 && *(*context).slavecount > 0i32 {
        i = 1i32;
        while ((*(*context).slavelist.offset(i as isize)).eep_man
            != (*(*context).slavelist.offset(slave as isize)).eep_man
            || (*(*context).slavelist.offset(i as isize)).eep_id
                != (*(*context).slavelist.offset(slave as isize)).eep_id
            || (*(*context).slavelist.offset(i as isize)).eep_rev
                != (*(*context).slavelist.offset(slave as isize)).eep_rev)
            && i < slave as libc::c_int
        {
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
            if (*(*context).slavelist.offset(i as isize)).blockLRW as libc::c_int > 0i32 {
                (*(*context).slavelist.offset(slave as isize)).blockLRW = 1u8;
                let ref mut fresh0 = (*(*context).slavelist.offset(0isize)).blockLRW;
                *fresh0 = (*fresh0).wrapping_add(1)
            }
            (*(*context).slavelist.offset(slave as isize)).Ebuscurrent =
                (*(*context).slavelist.offset(i as isize)).Ebuscurrent;
            let ref mut fresh1 = (*(*context).slavelist.offset(0isize)).Ebuscurrent;
            *fresh1 = (*fresh1 as libc::c_int
                + (*(*context).slavelist.offset(slave as isize)).Ebuscurrent as libc::c_int)
                as i16;
            memcpy(
                (*(*context).slavelist.offset(slave as isize))
                    .name
                    .as_mut_ptr() as *mut libc::c_void,
                (*(*context).slavelist.offset(i as isize)).name.as_mut_ptr() as *const libc::c_void,
                (40i32 + 1i32) as usize,
            );
            nSM = 0i32;
            while nSM < 8i32 {
                (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].StartAddr =
                    (*(*context).slavelist.offset(i as isize)).SM[nSM as usize].StartAddr;
                (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMlength =
                    (*(*context).slavelist.offset(i as isize)).SM[nSM as usize].SMlength;
                (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMflags =
                    (*(*context).slavelist.offset(i as isize)).SM[nSM as usize].SMflags;
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
            return 1i32;
        }
    }
    return 0i32;
}
/* * Enumerate and init all slaves.
 *
 * @param[in] context      = context struct
 * @param[in] usetable     = TRUE when using configtable to init slaves, FALSE otherwise
 * @return Workcounter of slave discover datagram = number of slaves found
 */
#[no_mangle]
pub unsafe fn ecx_config_init(context: *mut ecx_contextt, usetable: u8) -> libc::c_int {
    let mut slave: u16 = 0;
    let mut ADPh: u16 = 0;
    let mut configadr: u16 = 0;
    let mut ssigen: u16 = 0;
    let mut topology: u16 = 0;
    let mut estat: u16 = 0;
    let mut topoc: i16 = 0;
    let mut slavec: i16 = 0;
    let mut aliasadr: i16 = 0;
    let mut b: u8 = 0;
    let mut h: u8 = 0;
    let mut SMc: u8 = 0;
    let mut eedat: u32 = 0;
    let mut wkc: libc::c_int = 0;
    let mut cindex: libc::c_int = 0;
    let mut nSM: libc::c_int = 0;
    let mut val16: u16 = 0;
    ecx_init_context(context);
    wkc = ecx_detect_slaves(context);
    if wkc > 0i32 {
        ecx_set_slaves_to_default(context);
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            ADPh = (1i32 - slave as libc::c_int) as u16;
            /* Manuf */
            val16 = ecx_APRDw(
                (*context).port.as_mut().unwrap(),
                ADPh,
                EthercatRegister::ECT_REG_PDICTL as u16,
                EC_TIMEOUTRET3,
            ); /* read interface type of slave */
            (*(*context).slavelist.offset(slave as isize)).Itype = val16;
            ecx_APWRw(
                (*context).port.as_mut().unwrap(),
                ADPh,
                EthercatRegister::ECT_REG_STADR as u16,
                (slave as libc::c_int + 0x1000i32) as u16,
                EC_TIMEOUTRET3,
            );
            if slave as libc::c_int == 1i32 {
                b = 1u8
            /* a node offset is used to improve readability of network frames */
            /* this has no impact on the number of addressable slaves (auto wrap around) */
            /* set node address of slave */
            /* kill non ecat frames for first slave */
            } else {
                b = 0u8
                /* pass all frames for following slaves */
            } /* set non ecat frame behaviour */
            ecx_APWRw(
                (*context).port.as_mut().unwrap(),
                ADPh,
                EthercatRegister::ECT_REG_DLCTL as u16,
                b as u16,
                EC_TIMEOUTRET3,
            );
            configadr = ecx_APRDw(
                (*context).port.as_mut().unwrap(),
                ADPh,
                EthercatRegister::ECT_REG_STADR as u16,
                EC_TIMEOUTRET3,
            );
            configadr = configadr;
            (*(*context).slavelist.offset(slave as isize)).configadr = configadr;
            ecx_FPRD(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_ALIAS as u16,
                ::core::mem::size_of::<i16>(),
                &mut aliasadr as *mut i16 as *mut libc::c_void,
                EC_TIMEOUTRET3,
            );
            (*(*context).slavelist.offset(slave as isize)).aliasadr = aliasadr as u16;
            ecx_FPRD(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_EEPSTAT as u16,
                ::core::mem::size_of::<u16>(),
                &mut estat as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRET3,
            );
            estat = estat;
            if estat as libc::c_int & 0x40i32 != 0 {
                /* check if slave can read 8 byte chunks */
                (*(*context).slavelist.offset(slave as isize)).eep_8byte = 1u8
            } /* Manuf */
            ecx_readeeprom1(context, slave, SiiGeneral::Manufacturer as u16);
            slave = slave.wrapping_add(1)
        }
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            eedat = ecx_readeeprom2(context, slave, EC_TIMEOUTEEP);
            (*(*context).slavelist.offset(slave as isize)).eep_man = eedat;
            ecx_readeeprom1(context, slave, SiiGeneral::Id as u16);
            slave = slave.wrapping_add(1)
            /* ID */
        } /* ID */
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            eedat = ecx_readeeprom2(context, slave, EC_TIMEOUTEEP);
            (*(*context).slavelist.offset(slave as isize)).eep_id = eedat;
            ecx_readeeprom1(context, slave, SiiGeneral::Revision as u16);
            slave = slave.wrapping_add(1)
            /* revision */
        } /* revision */
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            eedat = ecx_readeeprom2(context, slave, EC_TIMEOUTEEP);
            (*(*context).slavelist.offset(slave as isize)).eep_rev = eedat;
            ecx_readeeprom1(context, slave, SiiGeneral::RxMailboxAddress as u16);
            slave = slave.wrapping_add(1)
            /* write mailbox address + mailboxsize */
        } /* write mailbox address and mailboxsize */
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            eedat = ecx_readeeprom2(context, slave, EC_TIMEOUTEEP);
            (*(*context).slavelist.offset(slave as isize)).mbx_wo = (eedat & 0xffffu32) as u16;
            (*(*context).slavelist.offset(slave as isize)).mbx_l = (eedat >> 16i32) as u16;
            if (*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int > 0i32 {
                ecx_readeeprom1(context, slave, SiiGeneral::TxMailboxAddress as u16);
                /* read mailbox offset */
            } /* read mailbox offset */
            slave = slave.wrapping_add(1)
        } /* read mailbox offset */
        slave = 1u16; /*read mailbox length */
        while slave as libc::c_int <= *(*context).slavecount {
            if (*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int > 0i32 {
                eedat = ecx_readeeprom2(context, slave, EC_TIMEOUTEEP);
                (*(*context).slavelist.offset(slave as isize)).mbx_ro = (eedat & 0xffffu32) as u16;
                (*(*context).slavelist.offset(slave as isize)).mbx_rl = (eedat >> 16i32) as u16;
                if (*(*context).slavelist.offset(slave as isize)).mbx_rl as libc::c_int == 0i32 {
                    (*(*context).slavelist.offset(slave as isize)).mbx_rl =
                        (*(*context).slavelist.offset(slave as isize)).mbx_l
                }
                ecx_readeeprom1(context, slave, SiiGeneral::MailboxProtocol as u16);
            }
            configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
            val16 = ecx_FPRDw(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_ESCSUP as u16,
                EC_TIMEOUTRET3,
            );
            /* Support DC? */
            if val16 & 0x4 > 0 {
                (*(*context).slavelist.offset(slave as isize)).hasdc = true;
            } else {
                (*(*context).slavelist.offset(slave as isize)).hasdc = false;
            } /* extract topology from DL status */
            topology = ecx_FPRDw(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_DLSTAT as u16,
                EC_TIMEOUTRET3,
            );
            topology = topology;
            h = 0u8;
            b = 0u8;
            /* port0 open and communication established */
            if topology as libc::c_int & 0x300i32 == 0x200i32 {
                h = h.wrapping_add(1);
                b = (b as libc::c_int | 0x1i32) as u8
            }
            /* port1 open and communication established */
            if topology as libc::c_int & 0xc00i32 == 0x800i32 {
                h = h.wrapping_add(1);
                b = (b as libc::c_int | 0x2i32) as u8
            }
            /* port2 open and communication established */
            if topology as libc::c_int & 0x3000i32 == 0x2000i32 {
                h = h.wrapping_add(1);
                b = (b as libc::c_int | 0x4i32) as u8
            }
            /* port3 open and communication established */
            if topology as libc::c_int & 0xc000i32 == 0x8000i32 {
                h = h.wrapping_add(1);
                b = (b as libc::c_int | 0x8i32) as u8
            }
            /* ptype = Physical type*/
            val16 = ecx_FPRDw(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_PORTDES as u16,
                EC_TIMEOUTRET3,
            );
            (*(*context).slavelist.offset(slave as isize)).ptype =
                (val16 as libc::c_int & 0xffi32) as u8;
            (*(*context).slavelist.offset(slave as isize)).topology = h;
            (*(*context).slavelist.offset(slave as isize)).activeports = b;
            /* 0=no links, not possible             */
            /* 1=1 link  , end of line              */
            /* 2=2 links , one before and one after */
            /* 3=3 links , split point              */
            /* 4=4 links , cross point              */
            /* search for parent */
            (*(*context).slavelist.offset(slave as isize)).parent = 0u16; /* parent is master */
            if slave as libc::c_int > 1i32 {
                topoc = 0i16;
                slavec = (slave as libc::c_int - 1i32) as i16;
                loop {
                    topology = (*(*context).slavelist.offset(slavec as isize)).topology as u16;
                    if topology as libc::c_int == 1i32 {
                        topoc -= 1
                        /* endpoint found */
                    }
                    if topology as libc::c_int == 3i32 {
                        topoc += 1
                        /* split found */
                    }
                    if topology as libc::c_int == 4i32 {
                        topoc = (topoc as libc::c_int + 2i32) as i16
                        /* cross found */
                    }
                    if topoc as libc::c_int >= 0i32 && topology as libc::c_int > 1i32
                        || slavec as libc::c_int == 1i32
                    {
                        /* parent found */
                        (*(*context).slavelist.offset(slave as isize)).parent = slavec as u16; //* check state change Init */
                        slavec = 1i16
                    }
                    slavec -= 1;
                    if !(slavec as libc::c_int > 0i32) {
                        break;
                    }
                }
            }
            ecx_statecheck(context, slave, SlaveState::Init as u16, EC_TIMEOUTSTATE);
            /* set default mailbox configuration if slave has mailbox */
            if (*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int > 0i32 {
                (*(*context).slavelist.offset(slave as isize)).SMtype[0usize] = 1u8;
                (*(*context).slavelist.offset(slave as isize)).SMtype[1usize] = 2u8;
                (*(*context).slavelist.offset(slave as isize)).SMtype[2usize] = 3u8;
                (*(*context).slavelist.offset(slave as isize)).SMtype[3usize] = 4u8;
                (*(*context).slavelist.offset(slave as isize)).SM[0usize].StartAddr =
                    (*(*context).slavelist.offset(slave as isize)).mbx_wo;
                (*(*context).slavelist.offset(slave as isize)).SM[0usize].SMlength =
                    (*(*context).slavelist.offset(slave as isize)).mbx_l;
                (*(*context).slavelist.offset(slave as isize)).SM[0usize].SMflags = 0x10026u32;
                (*(*context).slavelist.offset(slave as isize)).SM[1usize].StartAddr =
                    (*(*context).slavelist.offset(slave as isize)).mbx_ro;
                (*(*context).slavelist.offset(slave as isize)).SM[1usize].SMlength =
                    (*(*context).slavelist.offset(slave as isize)).mbx_rl;
                (*(*context).slavelist.offset(slave as isize)).SM[1usize].SMflags = 0x10022u32;
                eedat = ecx_readeeprom2(context, slave, EC_TIMEOUTEEP);
                (*(*context).slavelist.offset(slave as isize)).mbx_proto = eedat as u16
            }
            cindex = 0i32;
            /* use configuration table ? */
            if usetable as libc::c_int == 1i32 {
                cindex = ecx_config_from_table(context, slave)
            }
            /* slave not in configuration table, find out via SII */
            if cindex == 0 && ecx_lookup_prev_sii(context, slave) == 0 {
                ssigen = ecx_siifind(context, slave, SiiCategory::General as u16) as u16;
                /* SII general section */
                if ssigen != 0 {
                    (*(*context).slavelist.offset(slave as isize)).CoEdetails =
                        ecx_siigetbyte(context, slave, (ssigen as libc::c_int + 0x7i32) as u16);
                    (*(*context).slavelist.offset(slave as isize)).FoEdetails =
                        ecx_siigetbyte(context, slave, (ssigen as libc::c_int + 0x8i32) as u16);
                    (*(*context).slavelist.offset(slave as isize)).EoEdetails =
                        ecx_siigetbyte(context, slave, (ssigen as libc::c_int + 0x9i32) as u16);
                    (*(*context).slavelist.offset(slave as isize)).SoEdetails =
                        ecx_siigetbyte(context, slave, (ssigen as libc::c_int + 0xai32) as u16);
                    if ecx_siigetbyte(context, slave, (ssigen as libc::c_int + 0xdi32) as u16)
                        as libc::c_int
                        & 0x2i32
                        > 0i32
                    {
                        (*(*context).slavelist.offset(slave as isize)).blockLRW = 1u8;
                        let ref mut fresh2 = (*(*context).slavelist.offset(0isize)).blockLRW;
                        *fresh2 = (*fresh2).wrapping_add(1)
                    }
                    (*(*context).slavelist.offset(slave as isize)).Ebuscurrent =
                        ecx_siigetbyte(context, slave, (ssigen as libc::c_int + 0xei32) as u16)
                            as i16;
                    let ref mut fresh3 = (*(*context).slavelist.offset(slave as isize)).Ebuscurrent;
                    *fresh3 = (*fresh3 as libc::c_int
                        + ((ecx_siigetbyte(context, slave, (ssigen as libc::c_int + 0xfi32) as u16)
                            as libc::c_int)
                            << 8i32)) as i16;
                    let ref mut fresh4 = (*(*context).slavelist.offset(0isize)).Ebuscurrent;
                    *fresh4 = (*fresh4 as libc::c_int
                        + (*(*context).slavelist.offset(slave as isize)).Ebuscurrent as libc::c_int)
                        as i16
                }
                /* SII strings section */
                if ecx_siifind(context, slave, SiiCategory::String as u16) as libc::c_int > 0i32 {
                    ecx_siistring(
                        context,
                        (*(*context).slavelist.offset(slave as isize))
                            .name
                            .as_mut_ptr(),
                        slave,
                        1u16,
                    );
                } else {
                    /* no name for slave found, use constructed name */
                    sprintf(
                        (*(*context).slavelist.offset(slave as isize))
                            .name
                            .as_mut_ptr(),
                        b"? M:%8.8x I:%8.8x" as *const u8 as *const libc::c_char,
                        (*(*context).slavelist.offset(slave as isize)).eep_man,
                        (*(*context).slavelist.offset(slave as isize)).eep_id,
                    );
                }
                /* SII SM section */
                nSM = ecx_siiSM(context, slave, (*context).eepSM) as libc::c_int;
                if nSM > 0i32 {
                    (*(*context).slavelist.offset(slave as isize)).SM[0usize].StartAddr =
                        (*(*context).eepSM).PhStart;
                    (*(*context).slavelist.offset(slave as isize)).SM[0usize].SMlength =
                        (*(*context).eepSM).Plength;
                    (*(*context).slavelist.offset(slave as isize)).SM[0usize].SMflags =
                        ((*(*context).eepSM).Creg as libc::c_int
                            + (((*(*context).eepSM).Activate as libc::c_int) << 16i32))
                            as u32;
                    SMc = 1u8;
                    while (SMc as libc::c_int) < 8i32
                        && ecx_siiSMnext(context, slave, (*context).eepSM, SMc as u16)
                            as libc::c_int
                            != 0
                    {
                        (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr =
                            (*(*context).eepSM).PhStart;
                        (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].SMlength =
                            (*(*context).eepSM).Plength;
                        (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].SMflags =
                            ((*(*context).eepSM).Creg as libc::c_int
                                + (((*(*context).eepSM).Activate as libc::c_int) << 16i32))
                                as u32;
                        SMc = SMc.wrapping_add(1)
                    }
                }
                /* SII FMMU section */
                if ecx_siiFMMU(context, slave, (*context).eepFMMU) != 0 {
                    if (*(*context).eepFMMU).FMMU0 as libc::c_int != 0xffi32 {
                        (*(*context).slavelist.offset(slave as isize)).FMMU0func =
                            (*(*context).eepFMMU).FMMU0
                    }
                    if (*(*context).eepFMMU).FMMU1 as libc::c_int != 0xffi32 {
                        (*(*context).slavelist.offset(slave as isize)).FMMU1func =
                            (*(*context).eepFMMU).FMMU1
                    }
                    if (*(*context).eepFMMU).FMMU2 as libc::c_int != 0xffi32 {
                        (*(*context).slavelist.offset(slave as isize)).FMMU2func =
                            (*(*context).eepFMMU).FMMU2
                    }
                    if (*(*context).eepFMMU).FMMU3 as libc::c_int != 0xffi32 {
                        (*(*context).slavelist.offset(slave as isize)).FMMU3func =
                            (*(*context).eepFMMU).FMMU3
                    }
                }
            }
            if (*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int > 0i32 {
                if (*(*context).slavelist.offset(slave as isize)).SM[0usize].StartAddr
                    as libc::c_int
                    == 0i32
                {
                    /* should never happen */
                    (*(*context).slavelist.offset(slave as isize)).SM[0usize].StartAddr = 0x1000u16;
                    (*(*context).slavelist.offset(slave as isize)).SM[0usize].SMlength = 0x80u16;
                    (*(*context).slavelist.offset(slave as isize)).SM[0usize].SMflags = 0x10026u32;
                    (*(*context).slavelist.offset(slave as isize)).SMtype[0usize] = 1u8
                }
                if (*(*context).slavelist.offset(slave as isize)).SM[1usize].StartAddr
                    as libc::c_int
                    == 0i32
                {
                    /* should never happen */
                    (*(*context).slavelist.offset(slave as isize)).SM[1usize].StartAddr = 0x1080u16;
                    (*(*context).slavelist.offset(slave as isize)).SM[1usize].SMlength = 0x80u16;
                    (*(*context).slavelist.offset(slave as isize)).SM[1usize].SMflags = 0x10022u32;
                    (*(*context).slavelist.offset(slave as isize)).SMtype[1usize] = 2u8
                }
                /* program SM0 mailbox in and SM1 mailbox out for slave */
                /* writing both SM in one datagram will solve timing issue in old NETX */
                ecx_FPWR(
                    (*context).port.as_mut().unwrap(),
                    configadr,
                    EthercatRegister::ECT_REG_SM0 as u16,
                    core::mem::size_of::<ec_smt>().wrapping_mul(2usize),
                    &mut *(*(*context).slavelist.offset(slave as isize))
                        .SM
                        .as_mut_ptr()
                        .offset(0isize) as *mut ec_smt as *mut libc::c_void,
                    EC_TIMEOUTRET3,
                );
            }
            /* some slaves need eeprom available to PDI in init->preop transition */
            ecx_eeprom2pdi(context, slave);
            /* User may override automatic state change */
            if (*context).manualstatechange == 0i32 {
                /* request pre_op for slave */
                ecx_FPWRw(
                    (*context).port.as_mut().unwrap(),
                    configadr,
                    EthercatRegister::ECT_REG_ALCTL as u16,
                    (SlaveState::PreOp as libc::c_int | types::EC_STATE_ACK as libc::c_int) as u16,
                    EC_TIMEOUTRET3,
                );
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
unsafe fn ecx_lookup_mapping(
    context: *mut ecx_contextt,
    slave: u16,
    Osize: *mut u32,
    Isize: *mut u32,
) -> libc::c_int {
    let mut i: libc::c_int = 0; /* check state change pre-op */
    let mut nSM: libc::c_int = 0;
    if slave as libc::c_int > 1i32 && *(*context).slavecount > 0i32 {
        i = 1i32;
        while ((*(*context).slavelist.offset(i as isize)).eep_man
            != (*(*context).slavelist.offset(slave as isize)).eep_man
            || (*(*context).slavelist.offset(i as isize)).eep_id
                != (*(*context).slavelist.offset(slave as isize)).eep_id
            || (*(*context).slavelist.offset(i as isize)).eep_rev
                != (*(*context).slavelist.offset(slave as isize)).eep_rev)
            && i < slave as libc::c_int
        {
            i += 1
        }
        if i < slave as libc::c_int {
            nSM = 0i32;
            while nSM < 8i32 {
                (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMlength =
                    (*(*context).slavelist.offset(i as isize)).SM[nSM as usize].SMlength;
                (*(*context).slavelist.offset(slave as isize)).SMtype[nSM as usize] =
                    (*(*context).slavelist.offset(i as isize)).SMtype[nSM as usize];
                nSM += 1
            }
            *Osize = (*(*context).slavelist.offset(i as isize)).Obits as u32;
            *Isize = (*(*context).slavelist.offset(i as isize)).Ibits as u32;
            (*(*context).slavelist.offset(slave as isize)).Obits = *Osize as u16;
            (*(*context).slavelist.offset(slave as isize)).Ibits = *Isize as u16;
            return 1i32;
        }
    }
    return 0i32;
}
unsafe fn ecx_map_coe_soe(
    context: *mut ecx_contextt,
    slave: u16,
    thread_n: libc::c_int,
) -> libc::c_int {
    let mut Isize: u32 = 0;
    let mut Osize: u32 = 0;
    let mut rval: libc::c_int = 0;
    ecx_statecheck(context, slave, SlaveState::PreOp as u16, EC_TIMEOUTSTATE);
    /* execute special slave configuration hook Pre-Op to Safe-OP */
    if (*(*context).slavelist.offset(slave as isize))
        .PO2SOconfig
        .is_some()
    {
        /* only if registered */
        (*(*context).slavelist.offset(slave as isize))
            .PO2SOconfig
            .expect("non-null function pointer")(slave);
    }
    if (*(*context).slavelist.offset(slave as isize))
        .PO2SOconfigx
        .is_some()
    {
        /* only if registered */
        (*(*context).slavelist.offset(slave as isize))
            .PO2SOconfigx
            .expect("non-null function pointer")(context, slave);
    }
    /* if slave not found in configlist find IO mapping in slave self */
    if (*(*context).slavelist.offset(slave as isize)).configindex == 0 {
        Isize = 0u32;
        Osize = 0u32;
        if (*(*context).slavelist.offset(slave as isize)).mbx_proto as libc::c_int & 0x4i32 != 0 {
            /* has CoE */
            rval = 0i32;
            if (*(*context).slavelist.offset(slave as isize)).CoEdetails as libc::c_int & 0x20i32
                != 0
            {
                /* has Complete Access */
                /* read PDO mapping via CoE and use Complete Access */
                rval = ecx_readPDOmapCA(context, slave, thread_n, &mut Osize, &mut Isize)
            }
            if rval == 0 {
                /* CA not available or not succeeded */
                /* read PDO mapping via CoE */
                rval = ecx_readPDOmap(context, slave, &mut Osize, &mut Isize)
            }
        }
        if Isize == 0
            && Osize == 0
            && (*(*context).slavelist.offset(slave as isize)).mbx_proto as libc::c_int & 0x10i32
                != 0
        {
            /* has SoE */
            /* read AT / MDT mapping via SoE */
            rval = ecx_readIDNmap(context, slave, &mut Osize, &mut Isize);
            (*(*context).slavelist.offset(slave as isize)).SM[2usize].SMlength =
                Osize.wrapping_add(7u32).wrapping_div(8u32) as u16;
            (*(*context).slavelist.offset(slave as isize)).SM[3usize].SMlength =
                Isize.wrapping_add(7u32).wrapping_div(8u32) as u16
        }
        (*(*context).slavelist.offset(slave as isize)).Obits = Osize as u16;
        (*(*context).slavelist.offset(slave as isize)).Ibits = Isize as u16
    }
    return 1i32;
}
unsafe fn ecx_map_sii(context: *mut ecx_contextt, slave: u16) -> libc::c_int {
    let mut Isize: u32 = 0;
    let mut Osize: u32 = 0;
    let mut nSM: libc::c_int = 0;
    let mut eepPDO: ec_eepromPDOt = ec_eepromPDOt {
        Startpos: 0,
        Length: 0,
        nPDO: 0,
        Index: [0; 512],
        SyncM: [0; 512],
        BitSize: [0; 512],
        SMbitsize: [0; 8],
    };
    Osize = (*(*context).slavelist.offset(slave as isize)).Obits as u32;
    Isize = (*(*context).slavelist.offset(slave as isize)).Ibits as u32;
    if Isize == 0 && Osize == 0 {
        /* find PDO in previous slave with same ID */
        ecx_lookup_mapping(context, slave, &mut Osize, &mut Isize);
    }
    if Isize == 0 && Osize == 0 {
        /* find PDO mapping by SII */
        memset(
            &mut eepPDO as *mut ec_eepromPDOt as *mut libc::c_void,
            0i32,
            core::mem::size_of::<ec_eepromPDOt>(),
        );
        Isize = ecx_siiPDO(context, slave, &mut eepPDO, 0u8);
        nSM = 0i32;
        while nSM < 8i32 {
            if eepPDO.SMbitsize[nSM as usize] as libc::c_int > 0i32 {
                (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMlength =
                    ((eepPDO.SMbitsize[nSM as usize] as libc::c_int + 7i32) / 8i32) as u16;
                (*(*context).slavelist.offset(slave as isize)).SMtype[nSM as usize] = 4u8
            }
            nSM += 1
        }
        Osize = ecx_siiPDO(context, slave, &mut eepPDO, 1u8);
        nSM = 0i32;
        while nSM < 8i32 {
            if eepPDO.SMbitsize[nSM as usize] as libc::c_int > 0i32 {
                (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMlength =
                    ((eepPDO.SMbitsize[nSM as usize] as libc::c_int + 7i32) / 8i32) as u16;
                (*(*context).slavelist.offset(slave as isize)).SMtype[nSM as usize] = 3u8
            }
            nSM += 1
        }
    }
    (*(*context).slavelist.offset(slave as isize)).Obits = Osize as u16;
    (*(*context).slavelist.offset(slave as isize)).Ibits = Isize as u16;
    return 1i32;
}
unsafe fn ecx_map_sm(context: *mut ecx_contextt, slave: u16) -> libc::c_int {
    let mut configadr: u16 = 0;
    let mut nSM: libc::c_int = 0;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    if (*(*context).slavelist.offset(slave as isize)).mbx_l == 0
        && (*(*context).slavelist.offset(slave as isize)).SM[0usize].StartAddr as libc::c_int != 0
    {
        ecx_FPWR(
            (*context).port.as_mut().unwrap(),
            configadr,
            EthercatRegister::ECT_REG_SM0 as u16,
            ::core::mem::size_of::<ec_smt>(),
            &mut *(*(*context).slavelist.offset(slave as isize))
                .SM
                .as_mut_ptr()
                .offset(0isize) as *mut ec_smt as *mut libc::c_void,
            EC_TIMEOUTRET3,
        );
    }
    if (*(*context).slavelist.offset(slave as isize)).mbx_l == 0
        && (*(*context).slavelist.offset(slave as isize)).SM[1usize].StartAddr as libc::c_int != 0
    {
        ecx_FPWR(
            (*context).port.as_mut().unwrap(),
            configadr,
            EthercatRegister::ECT_REG_SM1 as u16,
            ::core::mem::size_of::<ec_smt>(),
            &mut *(*(*context).slavelist.offset(slave as isize))
                .SM
                .as_mut_ptr()
                .offset(1isize) as *mut ec_smt as *mut libc::c_void,
            EC_TIMEOUTRET3,
        );
    }
    /* program SM2 to SMx */
    nSM = 2i32;
    while nSM < 8i32 {
        if (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].StartAddr != 0 {
            /* check if SM length is zero -> clear enable flag */
            if (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMlength
                as libc::c_int
                == 0i32
            {
                (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMflags =
                    (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMflags
                        & 0xfffeffffu32
            } else {
                /* if SM length is non zero always set enable flag */
                (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMflags =
                    (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].SMflags
                        | !(0xfffeffffu32)
            }
            ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                (EthercatRegister::ECT_REG_SM0 as libc::c_int as libc::c_ulong).wrapping_add(
                    (nSM as libc::c_ulong).wrapping_mul(core::mem::size_of::<ec_smt>() as u64),
                ) as u16,
                ::core::mem::size_of::<ec_smt>(),
                &mut *(*(*context).slavelist.offset(slave as isize))
                    .SM
                    .as_mut_ptr()
                    .offset(nSM as isize) as *mut ec_smt as *mut libc::c_void,
                EC_TIMEOUTRET3,
            );
        }
        nSM += 1
    }
    if (*(*context).slavelist.offset(slave as isize)).Ibits as libc::c_int > 7i32 {
        (*(*context).slavelist.offset(slave as isize)).Ibytes =
            (((*(*context).slavelist.offset(slave as isize)).Ibits as libc::c_int + 7i32) / 8i32)
                as u32
    }
    if (*(*context).slavelist.offset(slave as isize)).Obits as libc::c_int > 7i32 {
        (*(*context).slavelist.offset(slave as isize)).Obytes =
            (((*(*context).slavelist.offset(slave as isize)).Obits as libc::c_int + 7i32) / 8i32)
                as u32
    }
    return 1i32;
}
unsafe fn ecx_get_threadcount() -> libc::c_int {
    let mut thrc: libc::c_int = 0;
    let mut thrn: libc::c_int = 0;
    thrc = 0i32;
    thrn = 0i32;
    while thrn < 1i32 {
        thrc += ecx_mapt[thrn as usize].running;
        thrn += 1
    }
    return thrc;
}
unsafe fn ecx_config_find_mappings(context: *mut ecx_contextt, group: u8) {
    let mut thrn: libc::c_int = 0;
    let mut thrc: libc::c_int = 0;
    let mut slave: u16 = 0;
    thrn = 0i32;
    while thrn < 1i32 {
        ecx_mapt[thrn as usize].running = 0i32;
        thrn += 1
    }
    /* find CoE and SoE mapping of slaves in multiple threads */
    slave = 1u16;
    while slave as libc::c_int <= *(*context).slavecount {
        if group == 0
            || group as libc::c_int
                == (*(*context).slavelist.offset(slave as isize)).group as libc::c_int
        {
            /* serialised version */
            ecx_map_coe_soe(context, slave, 0i32);
        }
        slave = slave.wrapping_add(1)
    }
    loop
    /* wait for all threads to finish */
    {
        thrc = ecx_get_threadcount();
        if thrc != 0 {
            osal_usleep(1000u32);
        }
        if !(thrc != 0) {
            break;
        }
    }
    /* find SII mapping of slave and program SM */
    slave = 1u16;
    while slave as libc::c_int <= *(*context).slavecount {
        if group == 0
            || group as libc::c_int
                == (*(*context).slavelist.offset(slave as isize)).group as libc::c_int
        {
            ecx_map_sii(context, slave);
            ecx_map_sm(context, slave);
        }
        slave = slave.wrapping_add(1)
    }
}
unsafe fn ecx_config_create_input_mappings(
    context: *mut ecx_contextt,
    pIOmap: *mut libc::c_void,
    group: u8,
    slave: i16,
    LogAddr: *mut u32,
    BitPos: *mut u8,
) {
    let mut BitCount: libc::c_int = 0i32;
    let mut FMMUdone: libc::c_int = 0i32;
    let mut AddToInputsWKC: libc::c_int = 0i32;
    let mut ByteCount: u16 = 0u16;
    let mut FMMUsize: u16 = 0u16;
    let mut SMc: u8 = 0u8;
    let mut EndAddr: u16 = 0;
    let mut SMlength: u16 = 0;
    let mut configadr: u16 = 0;
    let mut FMMUc: u8 = 0;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    FMMUc = (*(*context).slavelist.offset(slave as isize)).FMMUunused;
    if (*(*context).slavelist.offset(slave as isize)).Obits != 0 {
        /* find free FMMU */
        while (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart != 0 {
            FMMUc = FMMUc.wrapping_add(1)
        }
    }
    /* search for SM that contribute to the input mapping */
    while (SMc as libc::c_int) < 8i32 - 1i32
        && FMMUdone
            < ((*(*context).slavelist.offset(slave as isize)).Ibits as libc::c_int + 7i32) / 8i32
    {
        while (SMc as libc::c_int) < 8i32 - 1i32
            && (*(*context).slavelist.offset(slave as isize)).SMtype[SMc as usize] as libc::c_int
                != 4i32
        {
            SMc = SMc.wrapping_add(1)
        }
        (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].PhysStart =
            (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr;
        SMlength = (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].SMlength;
        ByteCount = (ByteCount as libc::c_int + SMlength as libc::c_int) as u16;
        BitCount += SMlength as libc::c_int * 8i32;
        EndAddr = ((*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr
            as libc::c_int
            + SMlength as libc::c_int) as u16;
        while BitCount < (*(*context).slavelist.offset(slave as isize)).Ibits as libc::c_int
            && (SMc as libc::c_int) < 8i32 - 1i32
        {
            /* more SM for input */
            SMc = SMc.wrapping_add(1);
            while (SMc as libc::c_int) < 8i32 - 1i32
                && (*(*context).slavelist.offset(slave as isize)).SMtype[SMc as usize]
                    as libc::c_int
                    != 4i32
            {
                SMc = SMc.wrapping_add(1)
            }
            /* if addresses from more SM connect use one FMMU otherwise break up in multiple FMMU */
            if (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr
                as libc::c_int
                > EndAddr as libc::c_int
            {
                break;
            }
            SMlength = (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].SMlength;
            ByteCount = (ByteCount as libc::c_int + SMlength as libc::c_int) as u16;
            BitCount += SMlength as libc::c_int * 8i32;
            EndAddr = ((*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr
                as libc::c_int
                + SMlength as libc::c_int) as u16
        }
        /* bit oriented slave */
        if (*(*context).slavelist.offset(slave as isize)).Ibytes == 0 {
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart = *LogAddr;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStartbit =
                *BitPos;
            *BitPos = (*BitPos as libc::c_int
                + ((*(*context).slavelist.offset(slave as isize)).Ibits as libc::c_int - 1i32))
                as u8;
            if *BitPos as libc::c_int > 7i32 {
                *LogAddr = (*LogAddr).wrapping_add(1u32);
                *BitPos = (*BitPos as libc::c_int - 8i32) as u8
            }
            FMMUsize = (*LogAddr)
                .wrapping_sub(
                    (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart,
                )
                .wrapping_add(1u32) as u16;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogLength =
                FMMUsize;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogEndbit = *BitPos;
            *BitPos = (*BitPos as libc::c_int + 1i32) as u8;
            if *BitPos as libc::c_int > 7i32 {
                *LogAddr = (*LogAddr).wrapping_add(1u32);
                *BitPos = (*BitPos as libc::c_int - 8i32) as u8
            }
        } else {
            /* byte oriented slave */
            if *BitPos != 0 {
                *LogAddr = (*LogAddr).wrapping_add(1u32);
                *BitPos = 0u8
            }
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart = *LogAddr;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStartbit =
                *BitPos;
            *BitPos = 7u8;
            FMMUsize = ByteCount;
            if FMMUsize as libc::c_int + FMMUdone
                > (*(*context).slavelist.offset(slave as isize)).Ibytes as libc::c_int
            {
                FMMUsize = (*(*context).slavelist.offset(slave as isize))
                    .Ibytes
                    .wrapping_sub(FMMUdone as libc::c_uint) as u16
            }
            *LogAddr = (*LogAddr).wrapping_add(FMMUsize as libc::c_uint);
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogLength =
                FMMUsize;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogEndbit = *BitPos;
            *BitPos = 0u8
        }
        FMMUdone += FMMUsize as libc::c_int;
        if (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogLength != 0 {
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].PhysStartBit = 0u8;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].FMMUtype = 1u8;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].FMMUactive = 1u8;
            /* program FMMU for input */
            ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                (EthercatRegister::ECT_REG_FMMU0 as libc::c_int as libc::c_ulong).wrapping_add(
                    core::mem::size_of::<ec_fmmut>().wrapping_mul(FMMUc as usize) as u64,
                ) as u16,
                ::core::mem::size_of::<ec_fmmut>(),
                &mut *(*(*context).slavelist.offset(slave as isize))
                    .FMMU
                    .as_mut_ptr()
                    .offset(FMMUc as isize) as *mut ec_fmmut as *mut libc::c_void,
                EC_TIMEOUTRET3,
            );
            /* Set flag to add one for an input FMMU,
            a single ESC can only contribute once */
            AddToInputsWKC = 1i32
        }
        if (*(*context).slavelist.offset(slave as isize))
            .inputs
            .is_null()
        {
            if group != 0 {
                let ref mut fresh5 = (*(*context).slavelist.offset(slave as isize)).inputs;
                *fresh5 = (pIOmap as *mut u8)
                    .offset(
                        (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart
                            as isize,
                    )
                    .offset(-((*(*context).grouplist.offset(group as isize)).logstartaddr as isize))
            } else {
                let ref mut fresh6 = (*(*context).slavelist.offset(slave as isize)).inputs;
                *fresh6 = (pIOmap as *mut u8).offset(
                    (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart
                        as isize,
                )
            }
            (*(*context).slavelist.offset(slave as isize)).Istartbit =
                (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStartbit
        }
        FMMUc = FMMUc.wrapping_add(1)
    }
    (*(*context).slavelist.offset(slave as isize)).FMMUunused = FMMUc;
    /* Add one WKC for an input if flag is true */
    if AddToInputsWKC != 0 {
        let ref mut fresh7 = (*(*context).grouplist.offset(group as isize)).inputsWKC;
        *fresh7 = (*fresh7).wrapping_add(1)
    };
}
unsafe fn ecx_config_create_output_mappings(
    context: *mut ecx_contextt,
    pIOmap: *mut libc::c_void,
    group: u8,
    slave: i16,
    LogAddr: *mut u32,
    BitPos: *mut u8,
) {
    let mut BitCount: libc::c_int = 0i32;
    let mut FMMUdone: libc::c_int = 0i32;
    let mut AddToOutputsWKC: libc::c_int = 0i32;
    let mut ByteCount: u16 = 0u16;
    let mut FMMUsize: u16 = 0u16;
    let mut SMc: u8 = 0u8;
    let mut EndAddr: u16 = 0;
    let mut SMlength: u16 = 0;
    let mut configadr: u16 = 0;
    let mut FMMUc: u8 = 0;
    FMMUc = (*(*context).slavelist.offset(slave as isize)).FMMUunused;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    /* search for SM that contribute to the output mapping */
    while (SMc as libc::c_int) < 8i32 - 1i32
        && FMMUdone
            < ((*(*context).slavelist.offset(slave as isize)).Obits as libc::c_int + 7i32) / 8i32
    {
        while (SMc as libc::c_int) < 8i32 - 1i32
            && (*(*context).slavelist.offset(slave as isize)).SMtype[SMc as usize] as libc::c_int
                != 3i32
        {
            SMc = SMc.wrapping_add(1)
        }
        (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].PhysStart =
            (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr;
        SMlength = (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].SMlength;
        ByteCount = (ByteCount as libc::c_int + SMlength as libc::c_int) as u16;
        BitCount += SMlength as libc::c_int * 8i32;
        EndAddr = ((*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr
            as libc::c_int
            + SMlength as libc::c_int) as u16;
        while BitCount < (*(*context).slavelist.offset(slave as isize)).Obits as libc::c_int
            && (SMc as libc::c_int) < 8i32 - 1i32
        {
            /* more SM for output */
            SMc = SMc.wrapping_add(1);
            while (SMc as libc::c_int) < 8i32 - 1i32
                && (*(*context).slavelist.offset(slave as isize)).SMtype[SMc as usize]
                    as libc::c_int
                    != 3i32
            {
                SMc = SMc.wrapping_add(1)
            }
            /* if addresses from more SM connect use one FMMU otherwise break up in multiple FMMU */
            if (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr
                as libc::c_int
                > EndAddr as libc::c_int
            {
                break;
            }
            SMlength = (*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].SMlength;
            ByteCount = (ByteCount as libc::c_int + SMlength as libc::c_int) as u16;
            BitCount += SMlength as libc::c_int * 8i32;
            EndAddr = ((*(*context).slavelist.offset(slave as isize)).SM[SMc as usize].StartAddr
                as libc::c_int
                + SMlength as libc::c_int) as u16
        }
        /* bit oriented slave */
        if (*(*context).slavelist.offset(slave as isize)).Obytes == 0 {
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart = *LogAddr;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStartbit =
                *BitPos;
            *BitPos = (*BitPos as libc::c_int
                + ((*(*context).slavelist.offset(slave as isize)).Obits as libc::c_int - 1i32))
                as u8;
            if *BitPos as libc::c_int > 7i32 {
                *LogAddr = (*LogAddr).wrapping_add(1u32);
                *BitPos = (*BitPos as libc::c_int - 8i32) as u8
            }
            FMMUsize = (*LogAddr)
                .wrapping_sub(
                    (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart,
                )
                .wrapping_add(1u32) as u16;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogLength =
                FMMUsize;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogEndbit = *BitPos;
            *BitPos = (*BitPos as libc::c_int + 1i32) as u8;
            if *BitPos as libc::c_int > 7i32 {
                *LogAddr = (*LogAddr).wrapping_add(1u32);
                *BitPos = (*BitPos as libc::c_int - 8i32) as u8
            }
        } else {
            /* byte oriented slave */
            if *BitPos != 0 {
                *LogAddr = (*LogAddr).wrapping_add(1u32);
                *BitPos = 0u8
            }
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart = *LogAddr;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStartbit =
                *BitPos;
            *BitPos = 7u8;
            FMMUsize = ByteCount;
            if FMMUsize as libc::c_int + FMMUdone
                > (*(*context).slavelist.offset(slave as isize)).Obytes as libc::c_int
            {
                FMMUsize = (*(*context).slavelist.offset(slave as isize))
                    .Obytes
                    .wrapping_sub(FMMUdone as libc::c_uint) as u16
            }
            *LogAddr = (*LogAddr).wrapping_add(FMMUsize as libc::c_uint);
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogLength =
                FMMUsize;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogEndbit = *BitPos;
            *BitPos = 0u8
        }
        FMMUdone += FMMUsize as libc::c_int;
        if (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogLength != 0 {
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].PhysStartBit = 0u8;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].FMMUtype = 2u8;
            (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].FMMUactive = 1u8;
            /* program FMMU for output */
            ecx_FPWR(
                (*context).port.as_mut().unwrap(),
                configadr,
                (EthercatRegister::ECT_REG_FMMU0 as libc::c_int as libc::c_ulong).wrapping_add(
                    core::mem::size_of::<ec_fmmut>().wrapping_mul(FMMUc as usize) as u64,
                ) as u16,
                ::core::mem::size_of::<ec_fmmut>(),
                &mut *(*(*context).slavelist.offset(slave as isize))
                    .FMMU
                    .as_mut_ptr()
                    .offset(FMMUc as isize) as *mut ec_fmmut as *mut libc::c_void,
                EC_TIMEOUTRET3,
            );
            /* Set flag to add one for an output FMMU,
            a single ESC can only contribute once */
            AddToOutputsWKC = 1i32
        }
        if (*(*context).slavelist.offset(slave as isize))
            .outputs
            .is_null()
        {
            if group != 0 {
                let ref mut fresh8 = (*(*context).slavelist.offset(slave as isize)).outputs;
                *fresh8 = (pIOmap as *mut u8)
                    .offset(
                        (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart
                            as isize,
                    )
                    .offset(-((*(*context).grouplist.offset(group as isize)).logstartaddr as isize))
            } else {
                let ref mut fresh9 = (*(*context).slavelist.offset(slave as isize)).outputs;
                *fresh9 = (pIOmap as *mut u8).offset(
                    (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStart
                        as isize,
                )
            }
            (*(*context).slavelist.offset(slave as isize)).Ostartbit =
                (*(*context).slavelist.offset(slave as isize)).FMMU[FMMUc as usize].LogStartbit
        }
        FMMUc = FMMUc.wrapping_add(1)
    }
    (*(*context).slavelist.offset(slave as isize)).FMMUunused = FMMUc;
    /* Add one WKC for an output if flag is true */
    if AddToOutputsWKC != 0 {
        let ref mut fresh10 = (*(*context).grouplist.offset(group as isize)).outputsWKC;
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
pub unsafe fn ecx_config_map_group(
    context: *mut ecx_contextt,
    pIOmap: *mut libc::c_void,
    group: u8,
) -> libc::c_int {
    let mut slave: u16 = 0;
    let mut configadr: u16 = 0;
    let mut BitPos: u8 = 0;
    let mut LogAddr: u32 = 0u32;
    let mut oLogAddr: u32 = 0u32;
    let mut diff: u32 = 0;
    let mut currentsegment: u16 = 0u16;
    let mut segmentsize: u32 = 0u32;
    if *(*context).slavecount > 0i32 && (group as libc::c_int) < (*context).maxgroup {
        LogAddr = (*(*context).grouplist.offset(group as isize)).logstartaddr;
        oLogAddr = LogAddr;
        BitPos = 0u8;
        (*(*context).grouplist.offset(group as isize)).nsegments = 0u16;
        (*(*context).grouplist.offset(group as isize)).outputsWKC = 0u16;
        (*(*context).grouplist.offset(group as isize)).inputsWKC = 0u16;
        /* Find mappings and program syncmanagers */
        ecx_config_find_mappings(context, group);
        /* do output mapping of slave and program FMMUs */
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
            if group == 0
                || group as libc::c_int
                    == (*(*context).slavelist.offset(slave as isize)).group as libc::c_int
            {
                /* create output mapping */
                if (*(*context).slavelist.offset(slave as isize)).Obits != 0 {
                    ecx_config_create_output_mappings(
                        context,
                        pIOmap,
                        group,
                        slave as i16,
                        &mut LogAddr,
                        &mut BitPos,
                    );
                    diff = LogAddr.wrapping_sub(oLogAddr);
                    oLogAddr = LogAddr;
                    if segmentsize.wrapping_add(diff)
                        > (1518i32 - 14i32 - 2i32 - 10i32 - 2i32 - 4i32 - 20i32) as libc::c_uint
                    {
                        (*(*context).grouplist.offset(group as isize)).IOsegment
                            [currentsegment as usize] = segmentsize;
                        if (currentsegment as libc::c_int) < 64i32 - 1i32 {
                            currentsegment = currentsegment.wrapping_add(1);
                            segmentsize = diff
                        }
                    } else {
                        segmentsize = (segmentsize).wrapping_add(diff)
                    }
                }
            }
            slave = slave.wrapping_add(1)
        }
        if BitPos != 0 {
            LogAddr = LogAddr.wrapping_add(1);
            oLogAddr = LogAddr;
            BitPos = 0u8;
            if segmentsize.wrapping_add(1u32)
                > (1518i32 - 14i32 - 2i32 - 10i32 - 2i32 - 4i32 - 20i32) as libc::c_uint
            {
                (*(*context).grouplist.offset(group as isize)).IOsegment[currentsegment as usize] =
                    segmentsize;
                if (currentsegment as libc::c_int) < 64i32 - 1i32 {
                    currentsegment = currentsegment.wrapping_add(1);
                    segmentsize = 1u32
                }
            } else {
                segmentsize = (segmentsize).wrapping_add(1u32)
            }
        }
        let ref mut fresh11 = (*(*context).grouplist.offset(group as isize)).outputs;
        *fresh11 = pIOmap as *mut u8;
        (*(*context).grouplist.offset(group as isize)).Obytes =
            LogAddr.wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr);
        (*(*context).grouplist.offset(group as isize)).nsegments =
            (currentsegment as libc::c_int + 1i32) as u16;
        (*(*context).grouplist.offset(group as isize)).Isegment = currentsegment;
        (*(*context).grouplist.offset(group as isize)).Ioffset = segmentsize as u16;
        if group == 0 {
            let ref mut fresh12 = (*(*context).slavelist.offset(0isize)).outputs;
            *fresh12 = pIOmap as *mut u8;
            (*(*context).slavelist.offset(0isize)).Obytes =
                LogAddr.wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr)
            /* store output bytes in master record */
        }
        /* do input mapping of slave and program FMMUs */
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
            if group == 0
                || group as libc::c_int
                    == (*(*context).slavelist.offset(slave as isize)).group as libc::c_int
            {
                /* create input mapping */
                if (*(*context).slavelist.offset(slave as isize)).Ibits != 0 {
                    ecx_config_create_input_mappings(
                        context,
                        pIOmap,
                        group,
                        slave as i16,
                        &mut LogAddr,
                        &mut BitPos,
                    ); /* set Eeprom control to PDI */
                    diff = LogAddr.wrapping_sub(oLogAddr);
                    oLogAddr = LogAddr;
                    if segmentsize.wrapping_add(diff)
                        > (1518i32 - 14i32 - 2i32 - 10i32 - 2i32 - 4i32 - 20i32) as libc::c_uint
                    {
                        (*(*context).grouplist.offset(group as isize)).IOsegment
                            [currentsegment as usize] = segmentsize;
                        if (currentsegment as libc::c_int) < 64i32 - 1i32 {
                            currentsegment = currentsegment.wrapping_add(1);
                            segmentsize = diff
                        }
                    } else {
                        segmentsize = (segmentsize).wrapping_add(diff)
                    }
                }
                ecx_eeprom2pdi(context, slave);
                /* User may override automatic state change */
                if (*context).manualstatechange == 0i32 {
                    /* request safe_op for slave */
                    ecx_FPWRw(
                        (*context).port.as_mut().unwrap(),
                        configadr,
                        EthercatRegister::ECT_REG_ALCTL as u16,
                        SlaveState::SafeOp as u16,
                        EC_TIMEOUTRET3,
                    );
                    /* set safeop status */
                }
                if (*(*context).slavelist.offset(slave as isize)).blockLRW != 0 {
                    let ref mut fresh13 = (*(*context).grouplist.offset(group as isize)).blockLRW;
                    *fresh13 = (*fresh13).wrapping_add(1)
                }
                let ref mut fresh14 = (*(*context).grouplist.offset(group as isize)).Ebuscurrent;
                *fresh14 = (*fresh14 as libc::c_int
                    + (*(*context).slavelist.offset(slave as isize)).Ebuscurrent as libc::c_int)
                    as i16
            }
            slave = slave.wrapping_add(1)
        }
        if BitPos != 0 {
            LogAddr = LogAddr.wrapping_add(1);
            oLogAddr = LogAddr;
            BitPos = 0u8;
            if segmentsize.wrapping_add(1u32)
                > (1518i32 - 14i32 - 2i32 - 10i32 - 2i32 - 4i32 - 20i32) as libc::c_uint
            {
                (*(*context).grouplist.offset(group as isize)).IOsegment[currentsegment as usize] =
                    segmentsize;
                if (currentsegment as libc::c_int) < 64i32 - 1i32 {
                    currentsegment = currentsegment.wrapping_add(1);
                    segmentsize = 1u32
                }
            } else {
                segmentsize = (segmentsize).wrapping_add(1u32)
            }
        }
        (*(*context).grouplist.offset(group as isize)).IOsegment[currentsegment as usize] =
            segmentsize;
        (*(*context).grouplist.offset(group as isize)).nsegments =
            (currentsegment as libc::c_int + 1i32) as u16;
        let ref mut fresh15 = (*(*context).grouplist.offset(group as isize)).inputs;
        *fresh15 = (pIOmap as *mut u8)
            .offset((*(*context).grouplist.offset(group as isize)).Obytes as isize);
        (*(*context).grouplist.offset(group as isize)).Ibytes = LogAddr
            .wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr)
            .wrapping_sub((*(*context).grouplist.offset(group as isize)).Obytes);
        if group == 0 {
            let ref mut fresh16 = (*(*context).slavelist.offset(0isize)).inputs;
            *fresh16 =
                (pIOmap as *mut u8).offset((*(*context).slavelist.offset(0isize)).Obytes as isize);
            (*(*context).slavelist.offset(0isize)).Ibytes = LogAddr
                .wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr)
                .wrapping_sub((*(*context).slavelist.offset(0isize)).Obytes)
            /* store input bytes in master record */
        }
        return LogAddr.wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr)
            as libc::c_int;
    }
    return 0i32;
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
pub unsafe fn ecx_config_overlap_map_group(
    context: *mut ecx_contextt,
    pIOmap: *mut libc::c_void,
    group: u8,
) -> libc::c_int {
    let mut slave: u16 = 0;
    let mut configadr: u16 = 0;
    let mut BitPos: u8 = 0;
    let mut mLogAddr: u32 = 0u32;
    let mut siLogAddr: u32 = 0u32;
    let mut soLogAddr: u32 = 0u32;
    let mut tempLogAddr: u32 = 0;
    let mut diff: u32 = 0;
    let mut currentsegment: u16 = 0u16;
    let mut segmentsize: u32 = 0u32;
    if *(*context).slavecount > 0i32 && (group as libc::c_int) < (*context).maxgroup {
        mLogAddr = (*(*context).grouplist.offset(group as isize)).logstartaddr;
        siLogAddr = mLogAddr;
        soLogAddr = mLogAddr;
        BitPos = 0u8;
        (*(*context).grouplist.offset(group as isize)).nsegments = 0u16;
        (*(*context).grouplist.offset(group as isize)).outputsWKC = 0u16;
        (*(*context).grouplist.offset(group as isize)).inputsWKC = 0u16;
        /* Find mappings and program syncmanagers */
        ecx_config_find_mappings(context, group);
        /* do IO mapping of slave and program FMMUs */
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
            soLogAddr = mLogAddr;
            siLogAddr = soLogAddr;
            if group == 0
                || group as libc::c_int
                    == (*(*context).slavelist.offset(slave as isize)).group as libc::c_int
            {
                /* create output mapping */
                if (*(*context).slavelist.offset(slave as isize)).Obits != 0 {
                    ecx_config_create_output_mappings(
                        context,
                        pIOmap,
                        group,
                        slave as i16,
                        &mut soLogAddr,
                        &mut BitPos,
                    );
                    if BitPos != 0 {
                        soLogAddr = soLogAddr.wrapping_add(1);
                        BitPos = 0u8
                    }
                }
                /* create input mapping */
                if (*(*context).slavelist.offset(slave as isize)).Ibits != 0 {
                    ecx_config_create_input_mappings(
                        context,
                        pIOmap,
                        group,
                        slave as i16,
                        &mut siLogAddr,
                        &mut BitPos,
                    ); /* set Eeprom control to PDI */
                    if BitPos != 0 {
                        siLogAddr = siLogAddr.wrapping_add(1);
                        BitPos = 0u8
                    }
                }
                tempLogAddr = if siLogAddr > soLogAddr {
                    siLogAddr
                } else {
                    soLogAddr
                };
                diff = tempLogAddr.wrapping_sub(mLogAddr);
                mLogAddr = tempLogAddr;
                if segmentsize.wrapping_add(diff)
                    > (1518i32 - 14i32 - 2i32 - 10i32 - 2i32 - 4i32 - 20i32) as libc::c_uint
                {
                    (*(*context).grouplist.offset(group as isize)).IOsegment
                        [currentsegment as usize] = segmentsize;
                    if (currentsegment as libc::c_int) < 64i32 - 1i32 {
                        currentsegment = currentsegment.wrapping_add(1);
                        segmentsize = diff
                    }
                } else {
                    segmentsize = (segmentsize).wrapping_add(diff)
                }
                ecx_eeprom2pdi(context, slave);
                /* User may override automatic state change */
                if (*context).manualstatechange == 0i32 {
                    /* request safe_op for slave */
                    ecx_FPWRw(
                        (*context).port.as_mut().unwrap(),
                        configadr,
                        EthercatRegister::ECT_REG_ALCTL as u16,
                        SlaveState::SafeOp as u16,
                        EC_TIMEOUTRET3,
                    );
                }
                if (*(*context).slavelist.offset(slave as isize)).blockLRW != 0 {
                    let ref mut fresh17 = (*(*context).grouplist.offset(group as isize)).blockLRW;
                    *fresh17 = (*fresh17).wrapping_add(1)
                }
                let ref mut fresh18 = (*(*context).grouplist.offset(group as isize)).Ebuscurrent;
                *fresh18 = (*fresh18 as libc::c_int
                    + (*(*context).slavelist.offset(slave as isize)).Ebuscurrent as libc::c_int)
                    as i16
            }
            slave = slave.wrapping_add(1)
        }
        (*(*context).grouplist.offset(group as isize)).IOsegment[currentsegment as usize] =
            segmentsize;
        (*(*context).grouplist.offset(group as isize)).nsegments =
            (currentsegment as libc::c_int + 1i32) as u16;
        (*(*context).grouplist.offset(group as isize)).Isegment = 0u16;
        (*(*context).grouplist.offset(group as isize)).Ioffset = 0u16;
        (*(*context).grouplist.offset(group as isize)).Obytes =
            soLogAddr.wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr);
        (*(*context).grouplist.offset(group as isize)).Ibytes =
            siLogAddr.wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr);
        let ref mut fresh19 = (*(*context).grouplist.offset(group as isize)).outputs;
        *fresh19 = pIOmap as *mut u8;
        let ref mut fresh20 = (*(*context).grouplist.offset(group as isize)).inputs;
        *fresh20 = (pIOmap as *mut u8)
            .offset((*(*context).grouplist.offset(group as isize)).Obytes as isize);
        /* Move calculated inputs with OBytes offset*/
        slave = 1u16;
        while slave as libc::c_int <= *(*context).slavecount {
            if group == 0
                || group as libc::c_int
                    == (*(*context).slavelist.offset(slave as isize)).group as libc::c_int
            {
                if (*(*context).slavelist.offset(slave as isize)).Ibits as libc::c_int > 0i32 {
                    let ref mut fresh21 = (*(*context).slavelist.offset(slave as isize)).inputs;
                    *fresh21 = (*fresh21)
                        .offset((*(*context).grouplist.offset(group as isize)).Obytes as isize)
                }
            }
            slave = slave.wrapping_add(1)
        }
        if group == 0 {
            /* store output bytes in master record */
            let ref mut fresh22 = (*(*context).slavelist.offset(0isize)).outputs;
            *fresh22 = pIOmap as *mut u8;
            (*(*context).slavelist.offset(0isize)).Obytes =
                soLogAddr.wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr);
            let ref mut fresh23 = (*(*context).slavelist.offset(0isize)).inputs;
            *fresh23 =
                (pIOmap as *mut u8).offset((*(*context).slavelist.offset(0isize)).Obytes as isize);
            (*(*context).slavelist.offset(0isize)).Ibytes =
                siLogAddr.wrapping_sub((*(*context).grouplist.offset(group as isize)).logstartaddr)
        }
        return (*(*context).grouplist.offset(group as isize))
            .Obytes
            .wrapping_add((*(*context).grouplist.offset(group as isize)).Ibytes)
            as libc::c_int;
    }
    return 0i32;
}
/* * Recover slave.
 *
 * @param[in] context = context struct
 * @param[in] slave   = slave to recover
 * @param[in] timeout = local timeout f.e. EC_TIMEOUTRET3
 * @return >0 if successful
 */
#[no_mangle]
pub unsafe fn ecx_recover_slave(
    context: *mut ecx_contextt,
    slave: u16,
    timeout: u32,
) -> libc::c_int {
    let mut rval: libc::c_int = 0;
    let mut wkc: libc::c_int = 0;
    let mut ADPh: u16 = 0;
    let mut configadr: u16 = 0;
    let mut readadr: u16 = 0;
    rval = 0i32;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    ADPh = (1i32 - slave as libc::c_int) as u16;
    /* check if we found another slave than the requested */
    readadr = 0xfffeu16;
    wkc = ecx_APRD(
        (*context).port.as_mut().unwrap(),
        ADPh,
        EthercatRegister::ECT_REG_STADR as u16,
        ::core::mem::size_of::<u16>(),
        &mut readadr as *mut u16 as *mut libc::c_void,
        timeout,
    );
    /* correct slave found, finished */
    if readadr as libc::c_int == configadr as libc::c_int {
        return 1i32;
    }
    /* only try if no config address*/
    if wkc > 0i32 && readadr as libc::c_int == 0i32 {
        /* clear possible slaves at EC_TEMPNODE */
        ecx_FPWRw(
            (*context).port.as_mut().unwrap(),
            0xffffu16,
            EthercatRegister::ECT_REG_STADR as u16,
            0u16,
            0,
        );
        /* set temporary node address of slave */
        if ecx_APWRw(
            (*context).port.as_mut().unwrap(),
            ADPh,
            EthercatRegister::ECT_REG_STADR as u16,
            0xffffu16,
            timeout,
        ) <= 0i32
        {
            ecx_FPWRw(
                (*context).port.as_mut().unwrap(),
                0xffffu16,
                EthercatRegister::ECT_REG_STADR as u16,
                0u16,
                0,
            );
            return 0i32;
            /* slave fails to respond */
        } /* temporary config address */
        (*(*context).slavelist.offset(slave as isize)).configadr = 0xffffu16; /* set Eeprom control to master */
        ecx_eeprom2master(context, slave);
        /* check if slave is the same as configured before */
        if ecx_FPRDw(
            (*context).port.as_mut().unwrap(),
            0xffffu16,
            EthercatRegister::ECT_REG_ALIAS as u16,
            timeout,
        ) as libc::c_int
            == (*(*context).slavelist.offset(slave as isize)).aliasadr as libc::c_int
            && ecx_readeeprom(context, slave, SiiGeneral::Id as u16, EC_TIMEOUTEEP)
                == (*(*context).slavelist.offset(slave as isize)).eep_id
            && ecx_readeeprom(
                context,
                slave,
                SiiGeneral::Manufacturer as u16,
                EC_TIMEOUTEEP,
            ) == (*(*context).slavelist.offset(slave as isize)).eep_man
            && ecx_readeeprom(context, slave, SiiGeneral::Revision as u16, EC_TIMEOUTEEP)
                == (*(*context).slavelist.offset(slave as isize)).eep_rev
        {
            rval = ecx_FPWRw(
                (*context).port.as_mut().unwrap(),
                0xffffu16,
                EthercatRegister::ECT_REG_STADR as u16,
                configadr,
                timeout,
            );
            (*(*context).slavelist.offset(slave as isize)).configadr = configadr
        } else {
            /* slave is not the expected one, remove config address*/
            ecx_FPWRw(
                (*context).port.as_mut().unwrap(),
                0xffffu16,
                EthercatRegister::ECT_REG_STADR as u16,
                0u16,
                timeout,
            );
            (*(*context).slavelist.offset(slave as isize)).configadr = configadr
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
pub unsafe fn ecx_reconfig_slave(
    context: *mut ecx_contextt,
    slave: u16,
    timeout: u32,
) -> libc::c_int {
    let mut state: libc::c_int = 0; /* set Eeprom control to PDI */
    let mut nSM: libc::c_int = 0;
    let mut FMMUc: libc::c_int = 0;
    let mut configadr: u16 = 0;
    configadr = (*(*context).slavelist.offset(slave as isize)).configadr;
    if ecx_FPWRw(
        (*context).port.as_mut().unwrap(),
        configadr,
        EthercatRegister::ECT_REG_ALCTL as u16,
        SlaveState::Init as u16,
        timeout,
    ) <= 0i32
    {
        return 0i32;
    }
    state = 0i32;
    ecx_eeprom2pdi(context, slave);
    /* check state change init */
    state = ecx_statecheck(context, slave, SlaveState::Init as u16, EC_TIMEOUTSTATE) as libc::c_int;
    if state == SlaveState::Init as libc::c_int {
        /* program all enabled SM */
        nSM = 0i32; /* check state change pre-op */
        while nSM < 8i32 {
            if (*(*context).slavelist.offset(slave as isize)).SM[nSM as usize].StartAddr != 0 {
                ecx_FPWR(
                    (*context).port.as_mut().unwrap(),
                    configadr,
                    (EthercatRegister::ECT_REG_SM0 as libc::c_int as libc::c_ulong).wrapping_add(
                        (nSM as libc::c_ulong).wrapping_mul(core::mem::size_of::<ec_smt>() as u64)
                            as u64,
                    ) as u16,
                    ::core::mem::size_of::<ec_smt>(),
                    &mut *(*(*context).slavelist.offset(slave as isize))
                        .SM
                        .as_mut_ptr()
                        .offset(nSM as isize) as *mut ec_smt
                        as *mut libc::c_void,
                    timeout,
                );
            }
            nSM += 1
        }
        ecx_FPWRw(
            (*context).port.as_mut().unwrap(),
            configadr,
            EthercatRegister::ECT_REG_ALCTL as u16,
            SlaveState::PreOp as u16,
            timeout,
        );
        state = ecx_statecheck(context, slave, SlaveState::PreOp as u16, EC_TIMEOUTSTATE)
            as libc::c_int;
        if state == SlaveState::PreOp as libc::c_int {
            /* execute special slave configuration hook Pre-Op to Safe-OP */
            if (*(*context).slavelist.offset(slave as isize))
                .PO2SOconfig
                .is_some()
            {
                /* only if registered */
                (*(*context).slavelist.offset(slave as isize))
                    .PO2SOconfig
                    .expect("non-null function pointer")(slave);
            }
            if (*(*context).slavelist.offset(slave as isize))
                .PO2SOconfigx
                .is_some()
            {
                /* only if registered */
                (*(*context).slavelist.offset(slave as isize))
                    .PO2SOconfigx
                    .expect("non-null function pointer")(context, slave); /* set safeop status */
            } /* check state change safe-op */
            ecx_FPWRw(
                (*context).port.as_mut().unwrap(),
                configadr,
                EthercatRegister::ECT_REG_ALCTL as u16,
                SlaveState::SafeOp as u16,
                timeout,
            );
            state = ecx_statecheck(context, slave, SlaveState::SafeOp as u16, EC_TIMEOUTSTATE)
                as libc::c_int;
            /* program configured FMMU */
            FMMUc = 0i32;
            while FMMUc < (*(*context).slavelist.offset(slave as isize)).FMMUunused as libc::c_int {
                ecx_FPWR(
                    (*context).port.as_mut().unwrap(),
                    configadr,
                    (EthercatRegister::ECT_REG_FMMU0 as libc::c_int as libc::c_ulong).wrapping_add(
                        core::mem::size_of::<ec_fmmut>().wrapping_mul(FMMUc as usize) as u64,
                    ) as u16,
                    ::core::mem::size_of::<ec_fmmut>(),
                    &mut *(*(*context).slavelist.offset(slave as isize))
                        .FMMU
                        .as_mut_ptr()
                        .offset(FMMUc as isize) as *mut ec_fmmut
                        as *mut libc::c_void,
                    timeout,
                );
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
pub unsafe fn ec_config_init(usetable: u8) -> libc::c_int {
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
pub unsafe fn ec_config_map_group(pIOmap: *mut libc::c_void, group: u8) -> libc::c_int {
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
pub unsafe fn ec_config_overlap_map_group(pIOmap: *mut libc::c_void, group: u8) -> libc::c_int {
    return ecx_config_overlap_map_group(&mut ecx_context, pIOmap, group);
}
/* * Map all PDOs from slaves to IOmap with Outputs/Inputs
 * in sequential order (legacy SOEM way).
 *
 * @param[out] pIOmap     = pointer to IOmap
 * @return IOmap size
 */
#[no_mangle]
pub unsafe fn ec_config_map(pIOmap: *mut libc::c_void) -> libc::c_int {
    return ec_config_map_group(pIOmap, 0u8);
}
/* * Map all PDOs from slaves to IOmap with Outputs/Inputs
* overlapping. NOTE: Must use this for TI ESC when using LRW.
*
* @param[out] pIOmap     = pointer to IOmap
* @return IOmap size
*/
#[no_mangle]
pub unsafe fn ec_config_overlap_map(pIOmap: *mut libc::c_void) -> libc::c_int {
    return ec_config_overlap_map_group(pIOmap, 0u8);
}
/* * Enumerate / map and init all slaves.
 *
 * @param[in] usetable    = TRUE when using configtable to init slaves, FALSE otherwise
 * @param[out] pIOmap     = pointer to IOmap
 * @return Workcounter of slave discover datagram = number of slaves found
 */
#[no_mangle]
pub unsafe fn ec_config(usetable: u8, pIOmap: *mut libc::c_void) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    wkc = ec_config_init(usetable);
    if wkc != 0 {
        ec_config_map(pIOmap);
    }
    return wkc;
}
/* * Enumerate / map and init all slaves.
*
* @param[in] usetable    = TRUE when using configtable to init slaves, FALSE otherwise
* @param[out] pIOmap     = pointer to IOmap
* @return Workcounter of slave discover datagram = number of slaves found
*/
#[no_mangle]
pub unsafe fn ec_config_overlap(usetable: u8, pIOmap: *mut libc::c_void) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    wkc = ec_config_init(usetable);
    if wkc != 0 {
        ec_config_overlap_map(pIOmap);
    }
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
pub unsafe fn ec_recover_slave(slave: u16, timeout: u32) -> libc::c_int {
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
pub unsafe fn ec_reconfig_slave(slave: u16, timeout: u32) -> libc::c_int {
    return ecx_reconfig_slave(&mut ecx_context, slave, timeout);
}
