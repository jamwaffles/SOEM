use crate::{
    base::{ecx_BWR, ecx_FPRD, ecx_FPWR},
    main::{ecx_context, ecx_contextt},
    osal::linux::osal::{ec_timet, osal_current_time},
    types::{EthercatRegister, EC_TIMEOUTRET},
};

/** 1st sync pulse delay in ns here 100ms */
pub const SYNC_DELAY: i32 = 100_000_000;

const PORTM0: u8 = 0x01;
const PORTM1: u8 = 0x02;
const PORTM2: u8 = 0x04;
const PORTM3: u8 = 0x08;

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
pub unsafe fn ecx_dcsync0(
    context: &mut ecx_contextt,
    slave: u16,
    act: bool,
    CyclTime: u32,
    CyclShift: i32,
) {
    let mut h: u8 = 0;
    let mut RA: u8 = 0;
    let mut slaveh: u16 = 0;
    let mut t: i64 = 0;
    let mut t1: i64 = 0;
    let mut tc: i32 = 0;
    slaveh = context.slavelist[slave as usize].configadr;
    RA = 0u8;
    /* stop cyclic operation, ready for next trigger */
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCSYNCACT as u16,
        ::core::mem::size_of::<u8>(),
        &mut RA as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    if act == true {
        RA = 1 + 2; /* act cyclic operation and sync0, sync1 deactivated */
    } /* write access to ethercat */
    h = 0u8; /* read local time of slave */
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCCUC as u16,
        ::core::mem::size_of::<u8>(),
        &mut h as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    t1 = 0i64;
    ecx_FPRD(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCSYSTIME as u16,
        ::core::mem::size_of::<i64>(),
        &mut t1 as *mut i64 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    t1 = t1;
    /* Calculate first trigger time, always a whole multiple of CyclTime rounded up
    plus the shifttime (can be negative)
    This insures best synchronization between slaves, slaves with the same CyclTime
    will sync at the same moment (you can use CyclShift to shift the sync) */
    if CyclTime > 0u32 {
        t = (t1 + SYNC_DELAY as i64) / CyclTime as libc::c_long * CyclTime as libc::c_long
            + CyclTime as libc::c_long
            + CyclShift as libc::c_long
    } else {
        t = t1 + SYNC_DELAY as i64 + CyclShift as libc::c_long
        /* first trigger at T1 + CyclTime + SyncDelay + CyclShift in ns */
    } /* SYNC0 start time */
    t = t; /* SYNC0 cycle time */
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCSTART0 as u16,
        ::core::mem::size_of::<i64>(),
        &mut t as *mut i64 as *mut libc::c_void,
        EC_TIMEOUTRET,
    ); /* activate cyclic operation */
    tc = CyclTime as i32;
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCCYCLE0 as u16,
        ::core::mem::size_of::<i32>(),
        &mut tc as *mut i32 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCSYNCACT as u16,
        ::core::mem::size_of::<u8>(),
        &mut RA as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    // update ec_slave state
    context.slavelist[slave as usize].DCactive = act;
    context.slavelist[slave as usize].DCshift = CyclShift;
    context.slavelist[slave as usize].DCcycle = CyclTime as i32;
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
pub unsafe fn ecx_dcsync01(
    context: &mut ecx_contextt,
    slave: u16,
    act: bool,
    CyclTime0: u32,
    CyclTime1: u32,
    CyclShift: i32,
) {
    let mut h: u8 = 0;
    let mut RA: u8 = 0;
    let mut slaveh: u16 = 0;
    let mut t: i64 = 0;
    let mut t1: i64 = 0;
    let mut tc: i32 = 0;
    let mut TrueCyclTime: u32 = 0;
    /* Sync1 can be used as a multiple of Sync0, use true cycle time */
    TrueCyclTime = CyclTime1
        .wrapping_div(CyclTime0)
        .wrapping_add(1u32)
        .wrapping_mul(CyclTime0);
    slaveh = context.slavelist[slave as usize].configadr;
    RA = 0u8;
    /* stop cyclic operation, ready for next trigger */
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCSYNCACT as u16,
        ::core::mem::size_of::<u8>(),
        &mut RA as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    if act == true {
        RA = 1 + 2 + 4; /* act cyclic operation and sync0 + sync1 */
    } /* write access to ethercat */
    h = 0u8; /* read local time of slave */
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCCUC as u16,
        ::core::mem::size_of::<u8>(),
        &mut h as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    t1 = 0i64;
    ecx_FPRD(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCSYSTIME as u16,
        ::core::mem::size_of::<i64>(),
        &mut t1 as *mut i64 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    t1 = t1;
    /* Calculate first trigger time, always a whole multiple of TrueCyclTime rounded up
    plus the shifttime (can be negative)
    This insures best synchronization between slaves, slaves with the same CyclTime
    will sync at the same moment (you can use CyclShift to shift the sync) */
    if CyclTime0 > 0u32 {
        t = (t1 + SYNC_DELAY as i64) / TrueCyclTime as libc::c_long * TrueCyclTime as libc::c_long
            + TrueCyclTime as libc::c_long
            + CyclShift as libc::c_long
    } else {
        t = t1 + SYNC_DELAY as i64 + CyclShift as libc::c_long
        /* first trigger at T1 + CyclTime + SyncDelay + CyclShift in ns */
    } /* SYNC0 start time */
    t = t; /* SYNC0 cycle time */
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCSTART0 as u16,
        ::core::mem::size_of::<i64>(),
        &mut t as *mut i64 as *mut libc::c_void,
        EC_TIMEOUTRET,
    ); /* SYNC1 cycle time */
    tc = CyclTime0 as i32; /* activate cyclic operation */
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCCYCLE0 as u16,
        ::core::mem::size_of::<i32>(),
        &mut tc as *mut i32 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    tc = CyclTime1 as i32;
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCCYCLE1 as u16,
        ::core::mem::size_of::<i32>(),
        &mut tc as *mut i32 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    ecx_FPWR(
        &mut context.port,
        slaveh,
        EthercatRegister::ECT_REG_DCSYNCACT as u16,
        ::core::mem::size_of::<u8>(),
        &mut RA as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    // update ec_slave state
    context.slavelist[slave as usize].DCactive = act;
    context.slavelist[slave as usize].DCshift = CyclShift;
    context.slavelist[slave as usize].DCcycle = CyclTime0 as i32;
}
/* latched port time of slave */
fn ecx_porttime(context: &ecx_contextt, slave: u16, port: u8) -> i32 {
    let mut ts: i32 = 0;
    match port as libc::c_int {
        0 => ts = context.slavelist[slave as usize].DCrtA,
        1 => ts = context.slavelist[slave as usize].DCrtB,
        2 => ts = context.slavelist[slave as usize].DCrtC,
        3 => ts = context.slavelist[slave as usize].DCrtD,
        _ => ts = 0i32,
    }
    return ts;
}
/* calculate previous active port of a slave */
fn ecx_prevport(context: &ecx_contextt, slave: u16, port: u8) -> u8 {
    let mut pport: u8 = port;
    let aport: u8 = context.slavelist[slave as usize].activeports;
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
unsafe fn ecx_parentport(context: &mut ecx_contextt, parent: u16) -> u8 {
    let mut parentport: u8 = 0u8;
    let mut b: u8 = 0;
    /* search order is important, here 3 - 1 - 2 - 0 */
    b = context.slavelist[parent as usize].consumedports;
    if b & PORTM3 != 0 {
        parentport = 3u8;
        b &= !(PORTM3)
    } else if b & PORTM1 != 0 {
        parentport = 1u8;
        b &= !(PORTM1)
    } else if b & PORTM2 != 0 {
        parentport = 2u8;
        b &= !(PORTM2)
    } else if b & PORTM0 != 0 {
        parentport = 0u8;
        b &= !(PORTM0)
    }
    context.slavelist[parent as usize].consumedports = b;
    return parentport;
}

#[test]
fn check_ecx_parentport() {
    // TODO
}

/* *
 * Locate DC slaves, measure propagation delays.
 *
 * @param[in]  context        = context struct
 * @return bool if slaves are found with DC
 */
#[no_mangle]
pub unsafe fn ecx_configdc(context: &mut ecx_contextt) -> bool {
    let mut i: u16 = 0; /* latch DCrecvTimeA of all slaves */
    let mut slaveh: u16 = 0; /* EtherCAT uses 2000-01-01 as epoch start instead of 1970-01-01 */
    let mut parent: u16 = 0;
    let mut child: u16 = 0;
    let mut parenthold: u16 = 0u16;
    let mut prevDCslave: u16 = 0u16;
    let mut ht: i32 = 0;
    let mut dt1: i32 = 0;
    let mut dt2: i32 = 0;
    let mut dt3: i32 = 0;
    let mut hrt: i64 = 0;
    let mut entryport: u8 = 0;
    let mut nlist: i8 = 0;
    let mut plist: [i8; 4] = [0; 4];
    let mut tlist: [i32; 4] = [0; 4];
    let mut mastertime: ec_timet = ec_timet { sec: 0, usec: 0 };
    let mut mastertime64: u64 = 0;
    context.slavelist[0].hasdc = false;
    (*(*context).grouplist.offset(0isize)).hasdc = false;
    ht = 0i32;
    ecx_BWR(
        &mut context.port,
        0u16,
        EthercatRegister::ECT_REG_DCTIME0 as u16,
        ::core::mem::size_of::<i32>(),
        &mut ht as *mut i32 as *mut libc::c_void,
        EC_TIMEOUTRET,
    );
    mastertime = osal_current_time();
    /* EtherCAT uses 2000-01-01 as epoch start instead of 1970-01-01 */
    mastertime.sec = (mastertime.sec as libc::c_ulong).wrapping_sub(946684800u64) as u32;
    mastertime64 = (mastertime.sec as u64)
        .wrapping_mul(1000000u64)
        .wrapping_add(mastertime.usec as u64)
        .wrapping_mul(1000u64);
    i = 1u16;
    while i as libc::c_int <= context.slavelist.len() as i32 {
        context.slavelist[i as usize].consumedports = context.slavelist[i as usize].activeports;
        if context.slavelist[i as usize].hasdc == true {
            if context.slavelist[0].hasdc == false {
                context.slavelist[0].hasdc = true;
                context.slavelist[0].DCnext = i;
                context.slavelist[i as usize].DCprevious = 0u16;
                (*(*context)
                    .grouplist
                    .offset(context.slavelist[i as usize].group as isize))
                .hasdc = true;
                (*(*context)
                    .grouplist
                    .offset(context.slavelist[i as usize].group as isize))
                .DCnext = i
            } else {
                context.slavelist[prevDCslave as usize].DCnext = i;
                context.slavelist[i as usize].DCprevious = prevDCslave
            }
            /* this branch has DC slave so remove parenthold */
            parenthold = 0u16;
            prevDCslave = i;
            slaveh = context.slavelist[i as usize].configadr;
            ecx_FPRD(
                &mut context.port,
                slaveh,
                EthercatRegister::ECT_REG_DCTIME0 as u16,
                ::core::mem::size_of::<i32>(),
                &mut ht as *mut i32 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            context.slavelist[i as usize].DCrtA = ht;
            /* 64bit latched DCrecvTimeA of each specific slave */
            ecx_FPRD(
                &mut context.port,
                slaveh,
                EthercatRegister::ECT_REG_DCSOF as u16,
                ::core::mem::size_of::<i64>(),
                &mut hrt as *mut i64 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            /* use it as offset in order to set local time around 0 + mastertime */
            hrt = (-hrt as libc::c_ulong).wrapping_add(mastertime64) as i64;
            /* save it in the offset register */
            ecx_FPWR(
                &mut context.port,
                slaveh,
                EthercatRegister::ECT_REG_DCSYSOFFSET as u16,
                ::core::mem::size_of::<i64>(),
                &mut hrt as *mut i64 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            ecx_FPRD(
                &mut context.port,
                slaveh,
                EthercatRegister::ECT_REG_DCTIME1 as u16,
                ::core::mem::size_of::<i32>(),
                &mut ht as *mut i32 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            context.slavelist[i as usize].DCrtB = ht;
            ecx_FPRD(
                &mut context.port,
                slaveh,
                EthercatRegister::ECT_REG_DCTIME2 as u16,
                ::core::mem::size_of::<i32>(),
                &mut ht as *mut i32 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            context.slavelist[i as usize].DCrtC = ht;
            ecx_FPRD(
                &mut context.port,
                slaveh,
                EthercatRegister::ECT_REG_DCTIME3 as u16,
                ::core::mem::size_of::<i32>(),
                &mut ht as *mut i32 as *mut libc::c_void,
                EC_TIMEOUTRET,
            );
            context.slavelist[i as usize].DCrtD = ht;
            /* make list of active ports and their time stamps */
            nlist = 0i8;
            if context.slavelist[i as usize].activeports as libc::c_int & 0x1i32 != 0 {
                plist[nlist as usize] = 0i8;
                tlist[nlist as usize] = context.slavelist[i as usize].DCrtA;
                nlist += 1
            }
            if context.slavelist[i as usize].activeports as libc::c_int & 0x8i32 != 0 {
                plist[nlist as usize] = 3i8;
                tlist[nlist as usize] = context.slavelist[i as usize].DCrtD;
                nlist += 1
            }
            if context.slavelist[i as usize].activeports as libc::c_int & 0x2i32 != 0 {
                plist[nlist as usize] = 1i8;
                tlist[nlist as usize] = context.slavelist[i as usize].DCrtB;
                nlist += 1
            }
            if context.slavelist[i as usize].activeports as libc::c_int & 0x4i32 != 0 {
                plist[nlist as usize] = 2i8;
                tlist[nlist as usize] = context.slavelist[i as usize].DCrtC;
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
            entryport = plist[entryport as usize] as u8;
            context.slavelist[i as usize].entryport = entryport;
            /* consume entryport from activeports */
            let ref mut fresh0 = context.slavelist[i as usize].consumedports;
            *fresh0 = (*fresh0 as libc::c_int
                & !((1i32) << entryport as libc::c_int) as u8 as libc::c_int)
                as u8;
            /* finding DC parent of current */
            parent = i;
            loop {
                child = parent;
                parent = context.slavelist[parent as usize].parent;
                if parent as libc::c_int == 0i32
                    || context.slavelist[parent as usize].hasdc as libc::c_int != 0
                {
                    break;
                }
            }
            /* only calculate propagation delay if slave is not the first */
            if parent as libc::c_int > 0i32 {
                /* find port on parent this slave is connected to */
                context.slavelist[i as usize].parentport = ecx_parentport(context, parent);
                if context.slavelist[parent as usize].topology as libc::c_int == 1i32 {
                    context.slavelist[i as usize].parentport =
                        context.slavelist[parent as usize].entryport
                }
                dt1 = 0i32;
                dt2 = 0i32;
                /* delta time of (parentport - 1) - parentport */
                /* note: order of ports is 0 - 3 - 1 -2 */
                /* non active ports are skipped */
                dt3 = ecx_porttime(context, parent, context.slavelist[i as usize].parentport)
                    - ecx_porttime(
                        context,
                        parent,
                        ecx_prevport(context, parent, context.slavelist[i as usize].parentport),
                    );
                /* current slave has children */
                /* those children's delays need to be subtracted */
                if context.slavelist[i as usize].topology as libc::c_int > 1i32 {
                    dt1 = ecx_porttime(
                        context,
                        i,
                        ecx_prevport(context, i, context.slavelist[i as usize].entryport),
                    ) - ecx_porttime(context, i, context.slavelist[i as usize].entryport)
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
                        ecx_prevport(context, parent, context.slavelist[i as usize].parentport),
                    ) - ecx_porttime(
                        context,
                        parent,
                        context.slavelist[parent as usize].entryport,
                    )
                }
                if dt2 < 0i32 {
                    dt2 = -dt2
                }
                /* calculate current slave delay from delta times */
                /* assumption : forward delay equals return delay */
                context.slavelist[i as usize].pdelay =
                    (dt3 - dt1) / 2i32 + dt2 + context.slavelist[parent as usize].pdelay;
                ht = context.slavelist[i as usize].pdelay;
                /* write propagation delay*/
                ecx_FPWR(
                    &mut context.port,
                    slaveh,
                    EthercatRegister::ECT_REG_DCSYSDELAY as u16,
                    ::core::mem::size_of::<i32>(),
                    &mut ht as *mut i32 as *mut libc::c_void,
                    EC_TIMEOUTRET,
                );
            }
        } else {
            context.slavelist[i as usize].DCrtA = 0i32;
            context.slavelist[i as usize].DCrtB = 0i32;
            context.slavelist[i as usize].DCrtC = 0i32;
            context.slavelist[i as usize].DCrtD = 0i32;
            parent = context.slavelist[i as usize].parent;
            /* if non DC slave found on first position on branch hold root parent */
            if parent as libc::c_int > 0i32
                && context.slavelist[parent as usize].topology as libc::c_int > 2i32
            {
                parenthold = parent
            }
            /* if branch has no DC slaves consume port on root parent */
            if parenthold as libc::c_int != 0
                && context.slavelist[i as usize].topology as libc::c_int == 1i32
            {
                ecx_parentport(context, parenthold);
                parenthold = 0u16
            }
        }
        i = i.wrapping_add(1)
    }
    return context.slavelist[0].hasdc;
}
#[no_mangle]
pub unsafe fn ec_dcsync0(slave: u16, act: bool, CyclTime: u32, CyclShift: i32) {
    ecx_dcsync0(&mut ecx_context, slave, act, CyclTime, CyclShift);
}
#[no_mangle]
pub unsafe fn ec_dcsync01(slave: u16, act: bool, CyclTime0: u32, CyclTime1: u32, CyclShift: i32) {
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
pub unsafe fn ec_configdc() -> bool {
    return ecx_configdc(&mut ecx_context);
}
