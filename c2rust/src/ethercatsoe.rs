use crate::{
    ethercatmain::{
        ec_clearmbx, ec_mbxbuft, ec_mbxheadert, ec_nextmbxcnt, ecx_context, ecx_contextt,
        ecx_mbxempty, ecx_mbxreceive, ecx_mbxsend, ecx_packeterror, ecx_pusherror,
    },
    ethercattype::{ec_err_type, ec_errort, C2RustUnnamed_0, MailboxType, SoEOpCode},
    osal::linux::osal::{ec_timet, osal_current_time},
};
use libc::{memcpy, memset};

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
pub struct ec_SoEmappingt {
    pub currentlength: uint16,
    pub maxlength: uint16,
    pub idn: [uint16; 64],
}

#[repr(C, packed)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct ec_SoEattributet {
    #[bitfield(name = "evafactor", ty = "uint32", bits = "0..=15")]
    #[bitfield(name = "length", ty = "uint32", bits = "16..=17")]
    #[bitfield(name = "list", ty = "uint32", bits = "18..=18")]
    #[bitfield(name = "command", ty = "uint32", bits = "19..=19")]
    #[bitfield(name = "datatype", ty = "uint32", bits = "20..=22")]
    #[bitfield(name = "reserved1", ty = "uint32", bits = "23..=23")]
    #[bitfield(name = "decimals", ty = "uint32", bits = "24..=27")]
    #[bitfield(name = "wppreop", ty = "uint32", bits = "28..=28")]
    #[bitfield(name = "wpsafeop", ty = "uint32", bits = "29..=29")]
    #[bitfield(name = "wpop", ty = "uint32", bits = "30..=30")]
    #[bitfield(name = "reserved2", ty = "uint32", bits = "31..=31")]
    pub evafactor_length_list_command_datatype_reserved1_decimals_wppreop_wpsafeop_wpop_reserved2:
        [u8; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct ec_SoEt {
    pub MbxHeader: ec_mbxheadert,
    #[bitfield(name = "opCode", ty = "uint8", bits = "0..=2")]
    #[bitfield(name = "incomplete", ty = "uint8", bits = "3..=3")]
    #[bitfield(name = "error", ty = "uint8", bits = "4..=4")]
    #[bitfield(name = "driveNo", ty = "uint8", bits = "5..=7")]
    pub opCode_incomplete_error_driveNo: [u8; 1],
    pub elementflags: uint8,
    pub c2rust_unnamed: C2RustUnnamed_3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_3 {
    pub idn: uint16,
    pub fragmentsleft: uint16,
}
/* * Report SoE error.
 *
 * @param[in]  context        = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  idn        = IDN that generated error
 * @param[in]  Error      = Error code, see EtherCAT documentation for list
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_SoEerror(
    mut context: *mut ecx_contextt,
    mut Slave: uint16,
    mut idn: uint16,
    mut Error: uint16,
) {
    let mut Ec: ec_errort = ec_errort {
        Time: ec_timet { sec: 0, usec: 0 },
        Signal: 0,
        Slave: 0,
        Index: 0,
        SubIdx: 0,
        Etype: ec_err_type::EC_ERR_TYPE_SDO_ERROR,
        c2rust_unnamed: C2RustUnnamed_0 { AbortCode: 0 },
    };
    memset(
        &mut Ec as *mut ec_errort as *mut libc::c_void,
        0i32,
        core::mem::size_of::<ec_errort>(),
    );
    Ec.Time = osal_current_time();
    Ec.Slave = Slave;
    Ec.Index = idn;
    Ec.SubIdx = 0u8;
    *(*context).ecaterror = 1u8;
    Ec.Etype = ec_err_type::EC_CMD_SOE_ERROR;
    Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode = Error;
    ecx_pusherror(context, &mut Ec);
}
/* * SoE read, blocking.
 *
 * The IDN object of the selected slave and DriveNo is read. If a response
 * is larger than the mailbox size then the response is segmented. The function
 * will combine all segments and copy them to the parameter buffer.
 *
 * @param[in]  context        = context struct
 * @param[in]  slave         = Slave number
 * @param[in]  driveNo       = Drive number in slave
 * @param[in]  elementflags  = Flags to select what properties of IDN are to be transferred.
 * @param[in]  idn           = IDN.
 * @param[in,out] psize      = Size in bytes of parameter buffer, returns bytes read from SoE.
 * @param[out] p             = Pointer to parameter buffer
 * @param[in]  timeout       = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_SoEread(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut driveNo: uint8,
    mut elementflags: uint8,
    mut idn: uint16,
    mut psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut SoEp: *mut ec_SoEt = 0 as *mut ec_SoEt;
    let mut aSoEp: *mut ec_SoEt = 0 as *mut ec_SoEt;
    let mut totalsize: libc::c_int = 0;
    let mut framedatasize: libc::c_int = 0;
    let mut wkc: libc::c_int = 0;
    let mut bp: *mut uint8 = 0 as *mut uint8;
    let mut mp: *mut uint8 = 0 as *mut uint8;
    let mut errorcode: *mut uint16 = 0 as *mut uint16;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut NotLast: boolean = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0i32);
    ec_clearmbx(&mut MbxOut);
    aSoEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SoEt;
    SoEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SoEt;
    (*SoEp).MbxHeader.length = core::mem::size_of::<ec_SoEt>()
        .wrapping_sub(core::mem::size_of::<ec_mbxheadert>())
        as uint16;
    (*SoEp).MbxHeader.address = 0u16;
    (*SoEp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* SoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*SoEp).MbxHeader.mbxtype = (MailboxType::ECT_MBXT_SOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
        as uint8;
    (*SoEp).set_opCode(SoEOpCode::ECT_SOE_READREQ as uint8);
    (*SoEp).set_incomplete(0u8);
    (*SoEp).set_error(0u8);
    (*SoEp).set_driveNo(driveNo);
    (*SoEp).elementflags = elementflags;
    (*SoEp).c2rust_unnamed.idn = idn;
    totalsize = 0i32;
    bp = p as *mut uint8;
    mp = (&mut MbxIn as *mut ec_mbxbuft as *mut uint8)
        .offset(::core::mem::size_of::<ec_SoEt>() as isize);
    NotLast = 1u8;
    /* send SoE request to slave */
    wkc = ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft, 20000i32);
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        while NotLast != 0 {
            /* clean mailboxbuffer */
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* succeeded to read slave response ? */
                /* slave response should be SoE, ReadRes */
                if (*aSoEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::ECT_MBXT_SOE as libc::c_int
                    && (*aSoEp).opCode() as libc::c_int == SoEOpCode::ECT_SOE_READRES as libc::c_int
                    && (*aSoEp).error() as libc::c_int == 0i32
                    && (*aSoEp).driveNo() as libc::c_int == driveNo as libc::c_int
                    && (*aSoEp).elementflags as libc::c_int == elementflags as libc::c_int
                {
                    framedatasize = ((*aSoEp).MbxHeader.length as usize)
                        .wrapping_sub(core::mem::size_of::<ec_SoEt>() as u64)
                        .wrapping_add(core::mem::size_of::<ec_mbxheadert>() as u64)
                        as libc::c_int;
                    totalsize += framedatasize;
                    /* Does parameter fit in parameter buffer ? */
                    if totalsize <= *psize {
                        /* copy parameter data in parameter buffer */
                        memcpy(
                            bp as *mut libc::c_void,
                            mp as *const libc::c_void,
                            framedatasize as usize,
                        );
                        /* increment buffer pointer */
                        bp = bp.offset(framedatasize as isize)
                    } else {
                        framedatasize -= totalsize - *psize;
                        totalsize = *psize;
                        /* copy parameter data in parameter buffer */
                        if framedatasize > 0i32 {
                            memcpy(
                                bp as *mut libc::c_void,
                                mp as *const libc::c_void,
                                framedatasize as usize,
                            );
                        }
                    }
                    if (*aSoEp).incomplete() == 0 {
                        NotLast = 0u8;
                        *psize = totalsize
                    }
                } else {
                    /* other slave response */
                    NotLast = 0u8;
                    if (*aSoEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                        == MailboxType::ECT_MBXT_SOE as libc::c_int
                        && (*aSoEp).opCode() as libc::c_int
                            == SoEOpCode::ECT_SOE_READRES as libc::c_int
                        && (*aSoEp).error() as libc::c_int == 1i32
                    {
                        mp = (&mut MbxIn as *mut ec_mbxbuft as *mut uint8).offset(
                            ((*aSoEp).MbxHeader.length as usize)
                                .wrapping_add(core::mem::size_of::<ec_mbxheadert>() as u64)
                                .wrapping_sub(core::mem::size_of::<uint16>() as u64)
                                as isize,
                        );
                        errorcode = mp as *mut uint16;
                        ecx_SoEerror(context, slave, idn, *errorcode);
                    } else {
                        ecx_packeterror(context, slave, idn, 0u8, 1u16);
                        /* Unexpected frame returned */
                    }
                    wkc = 0i32
                }
            } else {
                NotLast = 0u8;
                ecx_packeterror(context, slave, idn, 0u8, 4u16);
                /* no response */
            }
        }
    }
    return wkc;
}
/* * SoE write, blocking.
 *
 * The IDN object of the selected slave and DriveNo is written. If a response
 * is larger than the mailbox size then the response is segmented.
 *
 * @param[in]  context        = context struct
 * @param[in]  slave         = Slave number
 * @param[in]  driveNo       = Drive number in slave
 * @param[in]  elementflags  = Flags to select what properties of IDN are to be transferred.
 * @param[in]  idn           = IDN.
 * @param[in]  psize         = Size in bytes of parameter buffer.
 * @param[out] p             = Pointer to parameter buffer
 * @param[in]  timeout       = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_SoEwrite(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut driveNo: uint8,
    mut elementflags: uint8,
    mut idn: uint16,
    mut psize: libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut SoEp: *mut ec_SoEt = 0 as *mut ec_SoEt;
    let mut aSoEp: *mut ec_SoEt = 0 as *mut ec_SoEt;
    let mut framedatasize: libc::c_int = 0;
    let mut maxdata: libc::c_int = 0;
    let mut wkc: libc::c_int = 0;
    let mut mp: *mut uint8 = 0 as *mut uint8;
    let mut hp: *mut uint8 = 0 as *mut uint8;
    let mut errorcode: *mut uint16 = 0 as *mut uint16;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut NotLast: boolean = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0i32); /*  segmented transfer needed  */
    ec_clearmbx(&mut MbxOut);
    aSoEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SoEt;
    SoEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SoEt;
    (*SoEp).MbxHeader.address = 0u16;
    (*SoEp).MbxHeader.priority = 0u8;
    (*SoEp).set_opCode(SoEOpCode::ECT_SOE_WRITEREQ as uint8);
    (*SoEp).set_error(0u8);
    (*SoEp).set_driveNo(driveNo);
    (*SoEp).elementflags = elementflags;
    hp = p as *mut uint8;
    mp = (&mut MbxOut as *mut ec_mbxbuft as *mut uint8)
        .offset(::core::mem::size_of::<ec_SoEt>() as isize);
    maxdata = ((*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_ulong)
        .wrapping_sub(core::mem::size_of::<ec_SoEt>() as u64) as libc::c_int;
    NotLast = 1u8;
    while NotLast != 0 {
        framedatasize = psize;
        NotLast = 0u8;
        (*SoEp).c2rust_unnamed.idn = idn;
        (*SoEp).set_incomplete(0u8);
        if framedatasize > maxdata {
            framedatasize = maxdata;
            NotLast = 1u8;
            (*SoEp).set_incomplete(1u8);
            (*SoEp).c2rust_unnamed.fragmentsleft = (psize / maxdata) as uint16
        }
        (*SoEp).MbxHeader.length = core::mem::size_of::<ec_SoEt>()
            .wrapping_sub(core::mem::size_of::<ec_mbxheadert>())
            .wrapping_add(framedatasize as usize) as uint16;
        /* get new mailbox counter, used for session handle */
        cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* SoE */
        (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
        (*SoEp).MbxHeader.mbxtype = (MailboxType::ECT_MBXT_SOE as libc::c_int
            + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
            as uint8;
        /* copy parameter data to mailbox */
        memcpy(
            mp as *mut libc::c_void,
            hp as *const libc::c_void,
            framedatasize as usize,
        );
        hp = hp.offset(framedatasize as isize);
        psize -= framedatasize;
        /* send SoE request to slave */
        wkc = ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft, 20000i32);
        if wkc > 0i32 {
            /* succeeded to place mailbox in slave ? */
            if NotLast == 0 || ecx_mbxempty(context, slave, timeout) == 0 {
                /* clean mailboxbuffer */
                ec_clearmbx(&mut MbxIn);
                /* read slave response */
                wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
                if wkc > 0i32 {
                    /* succeeded to read slave response ? */
                    NotLast = 0u8;
                    /* slave response should be SoE, WriteRes */
                    if !((*aSoEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                        == MailboxType::ECT_MBXT_SOE as libc::c_int
                        && (*aSoEp).opCode() as libc::c_int
                            == SoEOpCode::ECT_SOE_WRITERES as libc::c_int
                        && (*aSoEp).error() as libc::c_int == 0i32
                        && (*aSoEp).driveNo() as libc::c_int == driveNo as libc::c_int
                        && (*aSoEp).elementflags as libc::c_int == elementflags as libc::c_int)
                    {
                        /* other slave response */
                        if (*aSoEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                            == MailboxType::ECT_MBXT_SOE as libc::c_int
                            && (*aSoEp).opCode() as libc::c_int
                                == SoEOpCode::ECT_SOE_READRES as libc::c_int
                            && (*aSoEp).error() as libc::c_int == 1i32
                        {
                            mp = (&mut MbxIn as *mut ec_mbxbuft as *mut uint8).offset(
                                ((*aSoEp).MbxHeader.length as usize)
                                    .wrapping_add(core::mem::size_of::<ec_mbxheadert>() as u64)
                                    .wrapping_sub(core::mem::size_of::<uint16>() as u64)
                                    as isize,
                            );
                            errorcode = mp as *mut uint16;
                            ecx_SoEerror(context, slave, idn, *errorcode);
                        } else {
                            ecx_packeterror(context, slave, idn, 0u8, 1u16);
                            /* Unexpected frame returned */
                        }
                        wkc = 0i32
                    }
                } else {
                    ecx_packeterror(context, slave, idn, 0u8, 4u16);
                    /* no response */
                }
            }
        }
    }
    return wkc;
}
/* * SoE read AT and MTD mapping.
 *
 * SoE has standard indexes defined for mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave.
 *
 * @param[in]  context = context struct
 * @param[in]  slave   = Slave number
 * @param[out] Osize   = Size in bits of output mapping (MTD) found
 * @param[out] Isize   = Size in bits of input mapping (AT) found
 * @return >0 if mapping successful.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_readIDNmap(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut Osize: *mut uint32,
    mut Isize: *mut uint32,
) -> libc::c_int {
    let mut retVal: libc::c_int = 0i32;
    let mut wkc: libc::c_int = 0;
    let mut psize: libc::c_int = 0;
    let mut driveNr: uint8 = 0;
    let mut entries: uint16 = 0;
    let mut itemcount: uint16 = 0;
    let mut SoEmapping: ec_SoEmappingt = ec_SoEmappingt {
        currentlength: 0,
        maxlength: 0,
        idn: [0; 64],
    };
    let mut SoEattribute: ec_SoEattributet = ec_SoEattributet {
        evafactor_length_list_command_datatype_reserved1_decimals_wppreop_wpsafeop_wpop_reserved2:
            [0; 4],
    };
    *Isize = 0u32;
    *Osize = 0u32;
    driveNr = 0u8;
    while (driveNr as libc::c_int) < 8i32 {
        psize = ::core::mem::size_of::<ec_SoEmappingt>() as libc::c_int;
        /* read output mapping via SoE */
        wkc = ecx_SoEread(
            context,
            slave,
            driveNr,
            0x40u8,
            24u16,
            &mut psize,
            &mut SoEmapping as *mut ec_SoEmappingt as *mut libc::c_void,
            700000i32,
        );
        if wkc > 0i32
            && psize >= 4i32
            && {
                entries = (SoEmapping.currentlength as libc::c_int / 2i32) as uint16;
                (entries as libc::c_int) > 0i32
            }
            && entries as libc::c_int <= 64i32
        {
            /* command word (uint16) is always mapped but not in list */
            *Osize = (*Osize).wrapping_add(16u32);
            itemcount = 0u16;
            while (itemcount as libc::c_int) < entries as libc::c_int {
                psize = ::core::mem::size_of::<ec_SoEattributet>() as libc::c_int;
                /* read attribute of each IDN in mapping list */
                wkc = ecx_SoEread(
                    context,
                    slave,
                    driveNr,
                    0x4u8,
                    SoEmapping.idn[itemcount as usize],
                    &mut psize,
                    &mut SoEattribute as *mut ec_SoEattributet as *mut libc::c_void,
                    700000i32,
                );
                if wkc > 0i32 && SoEattribute.list() == 0 {
                    /* length : 0 = 8bit, 1 = 16bit .... */
                    *Osize = (*Osize).wrapping_add(
                        ((8i32) << SoEattribute.length() as libc::c_int) as libc::c_uint,
                    )
                }
                itemcount = itemcount.wrapping_add(1)
            }
        }
        psize = ::core::mem::size_of::<ec_SoEmappingt>() as libc::c_int;
        /* read input mapping via SoE */
        wkc = ecx_SoEread(
            context,
            slave,
            driveNr,
            0x40u8,
            16u16,
            &mut psize,
            &mut SoEmapping as *mut ec_SoEmappingt as *mut libc::c_void,
            700000i32,
        );
        if wkc > 0i32
            && psize >= 4i32
            && {
                entries = (SoEmapping.currentlength as libc::c_int / 2i32) as uint16;
                (entries as libc::c_int) > 0i32
            }
            && entries as libc::c_int <= 64i32
        {
            /* status word (uint16) is always mapped but not in list */
            *Isize = (*Isize).wrapping_add(16u32);
            itemcount = 0u16;
            while (itemcount as libc::c_int) < entries as libc::c_int {
                psize = ::core::mem::size_of::<ec_SoEattributet>() as libc::c_int;
                /* read attribute of each IDN in mapping list */
                wkc = ecx_SoEread(
                    context,
                    slave,
                    driveNr,
                    0x4u8,
                    SoEmapping.idn[itemcount as usize],
                    &mut psize,
                    &mut SoEattribute as *mut ec_SoEattributet as *mut libc::c_void,
                    700000i32,
                );
                if wkc > 0i32 && SoEattribute.list() == 0 {
                    /* length : 0 = 8bit, 1 = 16bit .... */
                    *Isize = (*Isize).wrapping_add(
                        ((8i32) << SoEattribute.length() as libc::c_int) as libc::c_uint,
                    )
                }
                itemcount = itemcount.wrapping_add(1)
            }
        }
        driveNr = driveNr.wrapping_add(1)
    }
    /* found some I/O bits ? */
    if *Isize > 0u32 || *Osize > 0u32 {
        retVal = 1i32
    }
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn ec_SoEread(
    mut slave: uint16,
    mut driveNo: uint8,
    mut elementflags: uint8,
    mut idn: uint16,
    mut psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_SoEread(
        &mut ecx_context,
        slave,
        driveNo,
        elementflags,
        idn,
        psize,
        p,
        timeout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ec_SoEwrite(
    mut slave: uint16,
    mut driveNo: uint8,
    mut elementflags: uint8,
    mut idn: uint16,
    mut psize: libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_SoEwrite(
        &mut ecx_context,
        slave,
        driveNo,
        elementflags,
        idn,
        psize,
        p,
        timeout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ec_readIDNmap(
    mut slave: uint16,
    mut Osize: *mut uint32,
    mut Isize: *mut uint32,
) -> libc::c_int {
    return ecx_readIDNmap(&mut ecx_context, slave, Osize, Isize);
}
