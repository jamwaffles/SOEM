use crate::{
    main::{
        ec_clearmbx, ec_mbxbuft, ec_mbxheadert, ec_nextmbxcnt, ecx_context, ecx_contextt,
        ecx_mbxreceive, ecx_mbxsend,
    },
    types::{ec_err_type, FoEOpCode, MailboxType, EC_TIMEOUTTXM},
};
use libc::{memcpy, strlen};

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_FOEt {
    pub MbxHeader: ec_mbxheadert,
    pub OpCode: u8,
    pub Reserved: u8,
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
}

#[derive(Copy, Clone)]
pub union C2RustUnnamed_3 {
    pub FileName: [libc::c_char; 512],
    pub Data: [u8; 512],
    pub ErrorText: [libc::c_char; 512],
}

#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
    pub Password: u32,
    pub PacketNumber: u32,
    pub ErrorCode: u32,
}
/* * FoE progress hook.
 *
 * @param[in]  context        = context struct
 * @param[in]     hook       = Pointer to hook function.
 * @return 1
 */
#[no_mangle]
pub unsafe fn ecx_FOEdefinehook(
    mut context: *mut ecx_contextt,
    hook: *mut libc::c_void,
) -> libc::c_int {
    (*context).FOEhook = ::core::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe fn(_: u16, _: libc::c_int, _: libc::c_int) -> libc::c_int>,
    >(hook);
    return 1i32;
}
/* * FoE read, blocking.
 *
 * @param[in]  context        = context struct
 * @param[in]     slave      = Slave number.
 * @param[in]     filename   = Filename of file to read.
 * @param[in]     password   = password.
 * @param[in,out] psize      = Size in bytes of file buffer, returns bytes read from file.
 * @param[out]    p          = Pointer to file buffer
 * @param[in]     timeout    = Timeout per mailbox cycle in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe fn ecx_FOEread(
    context: *mut ecx_contextt,
    slave: u16,
    filename: *mut libc::c_char,
    password: u32,
    psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut FOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut aFOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut wkc: libc::c_int = 0;
    let mut dataread: i32 = 0i32;
    let mut buffersize: i32 = 0;
    let mut packetnumber: i32 = 0;
    let mut prevpacket: i32 = 0i32;
    let mut fnsize: u16 = 0;
    let mut maxdata: u16 = 0;
    let mut segmentdata: u16 = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    let mut worktodo: bool = false;
    buffersize = *psize;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0);
    ec_clearmbx(&mut MbxOut);
    aFOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_FOEt;
    FOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_FOEt;
    fnsize = strlen(filename) as u16;
    maxdata = ((*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int - 12i32) as u16;
    if fnsize as libc::c_int > maxdata as libc::c_int {
        fnsize = maxdata
    }
    (*FOEp).MbxHeader.length = (0x6i32 + fnsize as libc::c_int) as u16;
    (*FOEp).MbxHeader.address = 0u16;
    (*FOEp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* FoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*FOEp).MbxHeader.mbxtype = (MailboxType::Foe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8;
    (*FOEp).OpCode = FoEOpCode::ECT_FOE_READ as u8;
    (*FOEp).c2rust_unnamed.Password = password;
    /* copy filename in mailbox */
    memcpy(
        &mut *(*FOEp)
            .c2rust_unnamed_0
            .FileName
            .as_mut_ptr()
            .offset(0isize) as *mut libc::c_char as *mut libc::c_void,
        filename as *const libc::c_void,
        fnsize as usize,
    );
    /* send FoE request to slave */
    wkc = ecx_mbxsend(
        context,
        slave,
        &mut MbxOut as *mut ec_mbxbuft,
        EC_TIMEOUTTXM,
    );
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        loop {
            worktodo = false;
            /* clean mailboxbuffer */
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* succeeded to read slave response ? */
                /* slave response should be FoE */
                if (*aFOEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::Foe as libc::c_int
                {
                    if (*aFOEp).OpCode as libc::c_int == FoEOpCode::ECT_FOE_DATA as libc::c_int {
                        segmentdata = ((*aFOEp).MbxHeader.length as libc::c_int - 0x6i32) as u16;
                        packetnumber = (*aFOEp).c2rust_unnamed.PacketNumber as i32;
                        prevpacket += 1;
                        if packetnumber == prevpacket
                            && dataread + segmentdata as libc::c_int <= buffersize
                        {
                            memcpy(
                                p,
                                &mut *(*aFOEp).c2rust_unnamed_0.Data.as_mut_ptr().offset(0isize)
                                    as *mut u8
                                    as *const libc::c_void,
                                segmentdata as usize,
                            );
                            dataread += segmentdata as libc::c_int;
                            p = (p as *mut u8).offset(segmentdata as libc::c_int as isize)
                                as *mut libc::c_void;
                            if segmentdata as libc::c_int == maxdata as libc::c_int {
                                worktodo = true;
                            }
                            (*FOEp).MbxHeader.length = 0x6u16;
                            (*FOEp).MbxHeader.address = 0u16;
                            (*FOEp).MbxHeader.priority = 0u8;
                            /* get new mailbox count value */
                            cnt = ec_nextmbxcnt(
                                (*(*context).slavelist.offset(slave as isize)).mbx_cnt,
                            ); /* FoE */
                            (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
                            (*FOEp).MbxHeader.mbxtype = (MailboxType::Foe as libc::c_int
                                + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
                                as u8;
                            (*FOEp).OpCode = FoEOpCode::ECT_FOE_ACK as u8;
                            (*FOEp).c2rust_unnamed.PacketNumber = packetnumber as u32;
                            /* send FoE ack to slave */
                            wkc = ecx_mbxsend(
                                context,
                                slave,
                                &mut MbxOut as *mut ec_mbxbuft,
                                EC_TIMEOUTTXM,
                            );
                            if wkc <= 0i32 {
                                worktodo = false;
                            }
                            if (*context).FOEhook.is_some() {
                                (*context).FOEhook.expect("non-null function pointer")(
                                    slave,
                                    packetnumber,
                                    dataread,
                                );
                            }
                        } else {
                            /* FoE error */
                            wkc = -(ec_err_type::EC_ERR_TYPE_FOE_BUF2SMALL as libc::c_int)
                        }
                    } else if (*aFOEp).OpCode as libc::c_int
                        == FoEOpCode::ECT_FOE_ERROR as libc::c_int
                    {
                        /* FoE error */
                        wkc = -(ec_err_type::EC_ERR_TYPE_FOE_ERROR as libc::c_int)
                    } else {
                        /* unexpected mailbox received */
                        wkc = -(ec_err_type::EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
                    }
                } else {
                    /* unexpected mailbox received */
                    wkc = -(ec_err_type::EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
                }
                *psize = dataread
            }
            if !(worktodo != false) {
                break;
            }
        }
    }
    return wkc;
}
/* * FoE write, blocking.
 *
 * @param[in]  context        = context struct
 * @param[in]  slave      = Slave number.
 * @param[in]  filename   = Filename of file to write.
 * @param[in]  password   = password.
 * @param[in]  psize      = Size in bytes of file buffer.
 * @param[out] p          = Pointer to file buffer
 * @param[in]  timeout    = Timeout per mailbox cycle in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe fn ecx_FOEwrite(
    context: *mut ecx_contextt,
    slave: u16,
    filename: *mut libc::c_char,
    password: u32,
    mut psize: libc::c_int,
    mut p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut FOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut aFOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut wkc: libc::c_int = 0;
    let mut packetnumber: i32 = 0;
    let mut sendpacket: i32 = 0i32;
    let mut fnsize: u16 = 0;
    let mut maxdata: u16 = 0;
    let mut segmentdata: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    let mut worktodo: bool = false;
    let mut dofinalzero: bool = false;
    let mut tsize: libc::c_int = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0);
    ec_clearmbx(&mut MbxOut);
    aFOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_FOEt;
    FOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_FOEt;
    dofinalzero = false;
    fnsize = strlen(filename) as u16;
    maxdata = ((*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int - 12i32) as u16;
    if fnsize as libc::c_int > maxdata as libc::c_int {
        fnsize = maxdata
    }
    (*FOEp).MbxHeader.length = (0x6i32 + fnsize as libc::c_int) as u16;
    (*FOEp).MbxHeader.address = 0u16;
    (*FOEp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* FoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*FOEp).MbxHeader.mbxtype = (MailboxType::Foe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8;
    (*FOEp).OpCode = FoEOpCode::ECT_FOE_WRITE as u8;
    (*FOEp).c2rust_unnamed.Password = password;
    /* copy filename in mailbox */
    memcpy(
        &mut *(*FOEp)
            .c2rust_unnamed_0
            .FileName
            .as_mut_ptr()
            .offset(0isize) as *mut libc::c_char as *mut libc::c_void,
        filename as *const libc::c_void,
        fnsize as usize,
    );
    /* send FoE request to slave */
    wkc = ecx_mbxsend(
        context,
        slave,
        &mut MbxOut as *mut ec_mbxbuft,
        EC_TIMEOUTTXM,
    );
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        loop {
            worktodo = false;
            /* clean mailboxbuffer */
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* succeeded to read slave response ? */
                /* slave response should be FoE */
                if (*aFOEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::Foe as libc::c_int
                {
                    match FoEOpCode::from_repr((*aFOEp).OpCode as usize).unwrap() {
                        FoEOpCode::ECT_FOE_ACK => {
                            packetnumber = (*aFOEp).c2rust_unnamed.PacketNumber as i32;
                            if packetnumber == sendpacket {
                                if (*context).FOEhook.is_some() {
                                    (*context).FOEhook.expect("non-null function pointer")(
                                        slave,
                                        packetnumber,
                                        psize,
                                    );
                                }
                                tsize = psize;
                                if tsize > maxdata as libc::c_int {
                                    tsize = maxdata as libc::c_int
                                }
                                if tsize != 0 || dofinalzero as libc::c_int != 0 {
                                    worktodo = true;
                                    dofinalzero = false;
                                    segmentdata = tsize;
                                    psize -= segmentdata;
                                    /* if last packet was full size, add a zero size packet as final */
                                    /* EOF is defined as packetsize < full packetsize */
                                    if psize == 0 && segmentdata == maxdata as libc::c_int {
                                        dofinalzero = true
                                    }
                                    (*FOEp).MbxHeader.length = (0x6i32 + segmentdata) as u16;
                                    (*FOEp).MbxHeader.address = 0u16;
                                    (*FOEp).MbxHeader.priority = 0u8;
                                    /* get new mailbox count value */
                                    cnt = ec_nextmbxcnt(
                                        (*(*context).slavelist.offset(slave as isize)).mbx_cnt,
                                    ); /* FoE */
                                    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
                                    (*FOEp).MbxHeader.mbxtype = (MailboxType::Foe as libc::c_int
                                        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
                                        as u8;
                                    (*FOEp).OpCode = FoEOpCode::ECT_FOE_DATA as u8;
                                    sendpacket += 1;
                                    (*FOEp).c2rust_unnamed.PacketNumber = sendpacket as u32;
                                    memcpy(
                                        &mut *(*FOEp)
                                            .c2rust_unnamed_0
                                            .Data
                                            .as_mut_ptr()
                                            .offset(0isize)
                                            as *mut u8
                                            as *mut libc::c_void,
                                        p,
                                        segmentdata as usize,
                                    );
                                    p = (p as *mut u8).offset(segmentdata as isize)
                                        as *mut libc::c_void;
                                    /* send FoE data to slave */
                                    wkc = ecx_mbxsend(
                                        context,
                                        slave,
                                        &mut MbxOut as *mut ec_mbxbuft,
                                        EC_TIMEOUTTXM,
                                    );
                                    if wkc <= 0i32 {
                                        worktodo = false
                                    }
                                }
                            } else {
                                /* FoE error */
                                wkc = -(ec_err_type::EC_ERR_TYPE_FOE_PACKETNUMBER as libc::c_int)
                            }
                        }
                        FoEOpCode::ECT_FOE_BUSY => {
                            /* resend if data has been send before */
                            /* otherwise ignore */
                            if sendpacket != 0 {
                                if psize == 0 {
                                    dofinalzero = true
                                }
                                psize += segmentdata;
                                p = (p as *mut u8).offset(-(segmentdata as isize))
                                    as *mut libc::c_void;
                                sendpacket -= 1
                            }
                        }
                        FoEOpCode::ECT_FOE_ERROR => {
                            /* FoE error */
                            if (*aFOEp).c2rust_unnamed.ErrorCode == 0x8001u32 {
                                wkc = -(ec_err_type::EC_ERR_TYPE_FOE_FILE_NOTFOUND as libc::c_int)
                            } else {
                                wkc = -(ec_err_type::EC_ERR_TYPE_FOE_ERROR as libc::c_int)
                            }
                        }
                        _ => {
                            /* unexpected mailbox received */
                            wkc = -(ec_err_type::EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
                        }
                    }
                } else {
                    /* unexpected mailbox received */
                    wkc = -(ec_err_type::EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
                }
            }
            if !(worktodo != false) {
                break;
            }
        }
    }
    return wkc;
}
#[no_mangle]
pub unsafe fn ec_FOEdefinehook(hook: *mut libc::c_void) -> libc::c_int {
    return ecx_FOEdefinehook(&mut ecx_context, hook);
}
#[no_mangle]
pub unsafe fn ec_FOEread(
    slave: u16,
    filename: *mut libc::c_char,
    password: u32,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_FOEread(
        &mut ecx_context,
        slave,
        filename,
        password,
        psize,
        p,
        timeout,
    );
}
#[no_mangle]
pub unsafe fn ec_FOEwrite(
    slave: u16,
    filename: *mut libc::c_char,
    password: u32,
    psize: libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_FOEwrite(
        &mut ecx_context,
        slave,
        filename,
        password,
        psize,
        p,
        timeout,
    );
}
