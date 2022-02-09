use crate::{
    ethercatbase::{ecx_BWR, ecx_FPRD, ecx_FPWR},
    ethercatmain::{ecx_context, ecx_contextt},
    ethercattype::EthercatRegister,
    osal::linux::osal::{ec_timet, osal_current_time, osal_timer_is_expired, osal_timer_start},
};
use libc::{
    bind, ioctl, memcpy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t,
    pthread_mutex_unlock, pthread_mutexattr_init, pthread_mutexattr_t, recv, send, setsockopt,
    sockaddr, socket, strcpy, timeval,
};

pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;

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
        EthercatRegister::ec_err_type::EC_REG_DCSYNCACT as uint16,
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
        EthercatRegister::ec_err_type::EC_REG_DCCUC as uint16,
        ::core::mem::size_of::<uint8>() as uint16,
        &mut h as *mut uint8 as *mut libc::c_void,
        2000i32,
    );
    t1 = 0i64;
    ecx_FPRD(
        (*context).port,
        slaveh,
        EthercatRegister::ec_err_type::EC_REG_DCSYSTIME as uint16,
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
        EthercatRegister::ec_err_type::EC_REG_DCSTART0 as uint16,
        ::core::mem::size_of::<int64>() as uint16,
        &mut t as *mut int64 as *mut libc::c_void,
        2000i32,
    ); /* activate cyclic operation */
    tc = CyclTime as int32;
    ecx_FPWR(
        (*context).port,
        slaveh,
        EthercatRegister::ec_err_type::EC_REG_DCCYCLE0 as uint16,
        ::core::mem::size_of::<int32>() as uint16,
        &mut tc as *mut int32 as *mut libc::c_void,
        2000i32,
    );
    ecx_FPWR(
        (*context).port,
        slaveh,
        EthercatRegister::ec_err_type::EC_REG_DCSYNCACT as uint16,
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
        EthercatRegister::ec_err_type::EC_REG_DCSYNCACT as uint16,
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
        EthercatRegister::ec_err_type::EC_REG_DCCUC as uint16,
        ::core::mem::size_of::<uint8>() as uint16,
        &mut h as *mut uint8 as *mut libc::c_void,
        2000i32,
    );
    t1 = 0i64;
    ecx_FPRD(
        (*context).port,
        slaveh,
        EthercatRegister::ec_err_type::EC_REG_DCSYSTIME as uint16,
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
        EthercatRegister::ec_err_type::EC_REG_DCSTART0 as uint16,
        ::core::mem::size_of::<int64>() as uint16,
        &mut t as *mut int64 as *mut libc::c_void,
        2000i32,
    ); /* SYNC1 cycle time */
    tc = CyclTime0 as int32; /* activate cyclic operation */
    ecx_FPWR(
        (*context).port,
        slaveh,
        EthercatRegister::ec_err_type::EC_REG_DCCYCLE0 as uint16,
        ::core::mem::size_of::<int32>() as uint16,
        &mut tc as *mut int32 as *mut libc::c_void,
        2000i32,
    );
    tc = CyclTime1 as int32;
    ecx_FPWR(
        (*context).port,
        slaveh,
        EthercatRegister::ec_err_type::EC_REG_DCCYCLE1 as uint16,
        ::core::mem::size_of::<int32>() as uint16,
        &mut tc as *mut int32 as *mut libc::c_void,
        2000i32,
    );
    ecx_FPWR(
        (*context).port,
        slaveh,
        EthercatRegister::ec_err_type::EC_REG_DCSYNCACT as uint16,
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
        EthercatRegister::ec_err_type::EC_REG_DCTIME0 as uint16,
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
                EthercatRegister::ec_err_type::EC_REG_DCTIME0 as uint16,
                ::core::mem::size_of::<int32>() as uint16,
                &mut ht as *mut int32 as *mut libc::c_void,
                2000i32,
            );
            (*(*context).slavelist.offset(i as isize)).DCrtA = ht;
            /* 64bit latched DCrecvTimeA of each specific slave */
            ecx_FPRD(
                (*context).port,
                slaveh,
                EthercatRegister::ec_err_type::EC_REG_DCSOF as uint16,
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
                EthercatRegister::ec_err_type::EC_REG_DCSYSOFFSET as uint16,
                ::core::mem::size_of::<int64>() as uint16,
                &mut hrt as *mut int64 as *mut libc::c_void,
                2000i32,
            );
            ecx_FPRD(
                (*context).port,
                slaveh,
                EthercatRegister::ec_err_type::EC_REG_DCTIME1 as uint16,
                ::core::mem::size_of::<int32>() as uint16,
                &mut ht as *mut int32 as *mut libc::c_void,
                2000i32,
            );
            (*(*context).slavelist.offset(i as isize)).DCrtB = ht;
            ecx_FPRD(
                (*context).port,
                slaveh,
                EthercatRegister::ec_err_type::EC_REG_DCTIME2 as uint16,
                ::core::mem::size_of::<int32>() as uint16,
                &mut ht as *mut int32 as *mut libc::c_void,
                2000i32,
            );
            (*(*context).slavelist.offset(i as isize)).DCrtC = ht;
            ecx_FPRD(
                (*context).port,
                slaveh,
                EthercatRegister::ec_err_type::EC_REG_DCTIME3 as uint16,
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
                    EthercatRegister::ec_err_type::EC_REG_DCSYSDELAY as uint16,
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
