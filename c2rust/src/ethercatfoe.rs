use crate::{
    ethercatmain::{
        ec_clearmbx, ec_mbxbuft, ec_mbxheadert, ec_nextmbxcnt, ecx_context, ecx_contextt,
        ecx_mbxreceive, ecx_mbxsend,
    },
    ethercattype::{ec_err_type, FoEOpCode, MailboxType, EC_TIMEOUTTXM},
};
use libc::{memcpy, strlen};

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

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_FOEt {
    pub MbxHeader: ec_mbxheadert,
    pub OpCode: uint8,
    pub Reserved: uint8,
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_3 {
    pub FileName: [libc::c_char; 512],
    pub Data: [uint8; 512],
    pub ErrorText: [libc::c_char; 512],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
    pub Password: uint32,
    pub PacketNumber: uint32,
    pub ErrorCode: uint32,
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
    mut hook: *mut libc::c_void,
) -> libc::c_int {
    (*context).FOEhook = ::core::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe fn(_: uint16, _: libc::c_int, _: libc::c_int) -> libc::c_int>,
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
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut filename: *mut libc::c_char,
    mut password: uint32,
    mut psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut FOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut aFOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut wkc: libc::c_int = 0;
    let mut dataread: int32 = 0i32;
    let mut buffersize: int32 = 0;
    let mut packetnumber: int32 = 0;
    let mut prevpacket: int32 = 0i32;
    let mut fnsize: uint16 = 0;
    let mut maxdata: uint16 = 0;
    let mut segmentdata: uint16 = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut worktodo: boolean = 0;
    buffersize = *psize;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0i32);
    ec_clearmbx(&mut MbxOut);
    aFOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_FOEt;
    FOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_FOEt;
    fnsize = strlen(filename) as uint16;
    maxdata =
        ((*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int - 12i32) as uint16;
    if fnsize as libc::c_int > maxdata as libc::c_int {
        fnsize = maxdata
    }
    (*FOEp).MbxHeader.length = (0x6i32 + fnsize as libc::c_int) as uint16;
    (*FOEp).MbxHeader.address = 0u16;
    (*FOEp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* FoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*FOEp).MbxHeader.mbxtype = (MailboxType::ECT_MBXT_FOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
        as uint8;
    (*FOEp).OpCode = FoEOpCode::ECT_FOE_READ as uint8;
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
            worktodo = 0u8;
            /* clean mailboxbuffer */
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* succeeded to read slave response ? */
                /* slave response should be FoE */
                if (*aFOEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::ECT_MBXT_FOE as libc::c_int
                {
                    if (*aFOEp).OpCode as libc::c_int == FoEOpCode::ECT_FOE_DATA as libc::c_int {
                        segmentdata = ((*aFOEp).MbxHeader.length as libc::c_int - 0x6i32) as uint16;
                        packetnumber = (*aFOEp).c2rust_unnamed.PacketNumber as int32;
                        prevpacket += 1;
                        if packetnumber == prevpacket
                            && dataread + segmentdata as libc::c_int <= buffersize
                        {
                            memcpy(
                                p,
                                &mut *(*aFOEp).c2rust_unnamed_0.Data.as_mut_ptr().offset(0isize)
                                    as *mut uint8
                                    as *const libc::c_void,
                                segmentdata as usize,
                            );
                            dataread += segmentdata as libc::c_int;
                            p = (p as *mut uint8).offset(segmentdata as libc::c_int as isize)
                                as *mut libc::c_void;
                            if segmentdata as libc::c_int == maxdata as libc::c_int {
                                worktodo = 1u8
                            }
                            (*FOEp).MbxHeader.length = 0x6u16;
                            (*FOEp).MbxHeader.address = 0u16;
                            (*FOEp).MbxHeader.priority = 0u8;
                            /* get new mailbox count value */
                            cnt = ec_nextmbxcnt(
                                (*(*context).slavelist.offset(slave as isize)).mbx_cnt,
                            ); /* FoE */
                            (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
                            (*FOEp).MbxHeader.mbxtype = (MailboxType::ECT_MBXT_FOE as libc::c_int
                                + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
                                as uint8;
                            (*FOEp).OpCode = FoEOpCode::ECT_FOE_ACK as uint8;
                            (*FOEp).c2rust_unnamed.PacketNumber = packetnumber as uint32;
                            /* send FoE ack to slave */
                            wkc = ecx_mbxsend(
                                context,
                                slave,
                                &mut MbxOut as *mut ec_mbxbuft,
                                EC_TIMEOUTTXM,
                            );
                            if wkc <= 0i32 {
                                worktodo = 0u8
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
            if !(worktodo != 0) {
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
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut filename: *mut libc::c_char,
    mut password: uint32,
    mut psize: libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut FOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut aFOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut wkc: libc::c_int = 0;
    let mut packetnumber: int32 = 0;
    let mut sendpacket: int32 = 0i32;
    let mut fnsize: uint16 = 0;
    let mut maxdata: uint16 = 0;
    let mut segmentdata: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut worktodo: boolean = 0;
    let mut dofinalzero: boolean = 0;
    let mut tsize: libc::c_int = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0i32);
    ec_clearmbx(&mut MbxOut);
    aFOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_FOEt;
    FOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_FOEt;
    dofinalzero = 0u8;
    fnsize = strlen(filename) as uint16;
    maxdata =
        ((*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int - 12i32) as uint16;
    if fnsize as libc::c_int > maxdata as libc::c_int {
        fnsize = maxdata
    }
    (*FOEp).MbxHeader.length = (0x6i32 + fnsize as libc::c_int) as uint16;
    (*FOEp).MbxHeader.address = 0u16;
    (*FOEp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* FoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*FOEp).MbxHeader.mbxtype = (MailboxType::ECT_MBXT_FOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
        as uint8;
    (*FOEp).OpCode = FoEOpCode::ECT_FOE_WRITE as uint8;
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
            worktodo = 0u8;
            /* clean mailboxbuffer */
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* succeeded to read slave response ? */
                /* slave response should be FoE */
                if (*aFOEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::ECT_MBXT_FOE as libc::c_int
                {
                    match (*aFOEp).OpCode as libc::c_int {
                        4 => {
                            packetnumber = (*aFOEp).c2rust_unnamed.PacketNumber as int32;
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
                                    worktodo = 1u8;
                                    dofinalzero = 0u8;
                                    segmentdata = tsize;
                                    psize -= segmentdata;
                                    /* if last packet was full size, add a zero size packet as final */
                                    /* EOF is defined as packetsize < full packetsize */
                                    if psize == 0 && segmentdata == maxdata as libc::c_int {
                                        dofinalzero = 1u8
                                    }
                                    (*FOEp).MbxHeader.length = (0x6i32 + segmentdata) as uint16;
                                    (*FOEp).MbxHeader.address = 0u16;
                                    (*FOEp).MbxHeader.priority = 0u8;
                                    /* get new mailbox count value */
                                    cnt = ec_nextmbxcnt(
                                        (*(*context).slavelist.offset(slave as isize)).mbx_cnt,
                                    ); /* FoE */
                                    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
                                    (*FOEp).MbxHeader.mbxtype = (MailboxType::ECT_MBXT_FOE
                                        as libc::c_int
                                        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
                                        as uint8;
                                    (*FOEp).OpCode = FoEOpCode::ECT_FOE_DATA as uint8;
                                    sendpacket += 1;
                                    (*FOEp).c2rust_unnamed.PacketNumber = sendpacket as uint32;
                                    memcpy(
                                        &mut *(*FOEp)
                                            .c2rust_unnamed_0
                                            .Data
                                            .as_mut_ptr()
                                            .offset(0isize)
                                            as *mut uint8
                                            as *mut libc::c_void,
                                        p,
                                        segmentdata as usize,
                                    );
                                    p = (p as *mut uint8).offset(segmentdata as isize)
                                        as *mut libc::c_void;
                                    /* send FoE data to slave */
                                    wkc = ecx_mbxsend(
                                        context,
                                        slave,
                                        &mut MbxOut as *mut ec_mbxbuft,
                                        EC_TIMEOUTTXM,
                                    );
                                    if wkc <= 0i32 {
                                        worktodo = 0u8
                                    }
                                }
                            } else {
                                /* FoE error */
                                wkc = -(ec_err_type::EC_ERR_TYPE_FOE_PACKETNUMBER as libc::c_int)
                            }
                        }
                        6 => {
                            /* resend if data has been send before */
                            /* otherwise ignore */
                            if sendpacket != 0 {
                                if psize == 0 {
                                    dofinalzero = 1u8
                                }
                                psize += segmentdata;
                                p = (p as *mut uint8).offset(-(segmentdata as isize))
                                    as *mut libc::c_void;
                                sendpacket -= 1
                            }
                        }
                        5 => {
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
            if !(worktodo != 0) {
                break;
            }
        }
    }
    return wkc;
}
#[no_mangle]
pub unsafe fn ec_FOEdefinehook(mut hook: *mut libc::c_void) -> libc::c_int {
    return ecx_FOEdefinehook(&mut ecx_context, hook);
}
#[no_mangle]
pub unsafe fn ec_FOEread(
    mut slave: uint16,
    mut filename: *mut libc::c_char,
    mut password: uint32,
    mut psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
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
    mut slave: uint16,
    mut filename: *mut libc::c_char,
    mut password: uint32,
    mut psize: libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
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
