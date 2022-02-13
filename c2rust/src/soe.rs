use crate::{
    main::{
        ec_clearmbx, ec_mbxbuft, ec_mbxheadert, ec_nextmbxcnt, ecx_context, ecx_contextt,
        ecx_mbxempty, ecx_mbxreceive, ecx_mbxsend, ecx_packeterror, ecx_pusherror,
    },
    osal::linux::osal::osal_current_time,
    types::{
        ec_err_type, ec_errort, C2RustUnnamed_0, MailboxType, SoEOpCode, EC_TIMEOUTRXM,
        EC_TIMEOUTTXM,
    },
};
use libc::memcpy;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_SoEmappingt {
    pub currentlength: u16,
    pub maxlength: u16,
    pub idn: [u16; 64],
}

#[repr(C, packed)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct ec_SoEattributet {
    #[bitfield(name = "evafactor", ty = "u32", bits = "0..=15")]
    #[bitfield(name = "length", ty = "u32", bits = "16..=17")]
    #[bitfield(name = "list", ty = "u32", bits = "18..=18")]
    #[bitfield(name = "command", ty = "u32", bits = "19..=19")]
    #[bitfield(name = "datatype", ty = "u32", bits = "20..=22")]
    #[bitfield(name = "reserved1", ty = "u32", bits = "23..=23")]
    #[bitfield(name = "decimals", ty = "u32", bits = "24..=27")]
    #[bitfield(name = "wppreop", ty = "u32", bits = "28..=28")]
    #[bitfield(name = "wpsafeop", ty = "u32", bits = "29..=29")]
    #[bitfield(name = "wpop", ty = "u32", bits = "30..=30")]
    #[bitfield(name = "reserved2", ty = "u32", bits = "31..=31")]
    pub evafactor_length_list_command_datatype_reserved1_decimals_wppreop_wpsafeop_wpop_reserved2:
        [u8; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct ec_SoEt {
    pub MbxHeader: ec_mbxheadert,
    #[bitfield(name = "opCode", ty = "u8", bits = "0..=2")]
    #[bitfield(name = "incomplete", ty = "u8", bits = "3..=3")]
    #[bitfield(name = "error", ty = "u8", bits = "4..=4")]
    #[bitfield(name = "driveNo", ty = "u8", bits = "5..=7")]
    pub opCode_incomplete_error_driveNo: [u8; 1],
    pub elementflags: u8,
    pub c2rust_unnamed: C2RustUnnamed_3,
}

#[derive(Copy, Clone)]
pub union C2RustUnnamed_3 {
    pub idn: u16,
    pub fragmentsleft: u16,
}
/* * Report SoE error.
 *
 * @param[in]  context        = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  idn        = IDN that generated error
 * @param[in]  Error      = Error code, see EtherCAT documentation for list
 */
#[no_mangle]
pub fn ecx_SoEerror(context: &mut ecx_contextt, Slave: u16, idn: u16, Error: u16) {
    let Ec: ec_errort = ec_errort {
        Time: osal_current_time(),
        Slave: Slave,
        Index: idn,
        SubIdx: 0,
        Etype: ec_err_type::EC_ERR_TYPE_SOE_ERROR,
        Signal: false,
        c2rust_unnamed: C2RustUnnamed_0 {
            c2rust_unnamed: crate::types::C2RustUnnamed_1 {
                ErrorCode: Error,
                ..Default::default()
            },
        },
    };

    ecx_pusherror(context, Ec);
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
pub unsafe fn ecx_SoEread(
    context: *mut ecx_contextt,
    slave: u16,
    driveNo: u8,
    elementflags: u8,
    idn: u16,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut SoEp: *mut ec_SoEt = 0 as *mut ec_SoEt;
    let mut aSoEp: *mut ec_SoEt = 0 as *mut ec_SoEt;
    let mut totalsize: libc::c_int = 0;
    let mut framedatasize: libc::c_int = 0;
    let mut wkc: libc::c_int = 0;
    let mut bp: *mut u8 = 0 as *mut u8;
    let mut mp: *mut u8 = 0 as *mut u8;
    let mut errorcode: *mut u16 = 0 as *mut u16;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    let mut NotLast: bool = false;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0);
    ec_clearmbx(&mut MbxOut);
    aSoEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SoEt;
    SoEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SoEt;
    (*SoEp).MbxHeader.length =
        core::mem::size_of::<ec_SoEt>().wrapping_sub(core::mem::size_of::<ec_mbxheadert>()) as u16;
    (*SoEp).MbxHeader.address = 0u16;
    (*SoEp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* SoE */
    (*context).slavelist[slave as usize].mbx_cnt = cnt;
    (*SoEp).MbxHeader.mbxtype = (MailboxType::Soe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8;
    (*SoEp).set_opCode(SoEOpCode::ECT_SOE_READREQ as u8);
    (*SoEp).set_incomplete(0u8);
    (*SoEp).set_error(0u8);
    (*SoEp).set_driveNo(driveNo);
    (*SoEp).elementflags = elementflags;
    (*SoEp).c2rust_unnamed.idn = idn;
    totalsize = 0i32;
    bp = p as *mut u8;
    mp = (&mut MbxIn as *mut ec_mbxbuft as *mut u8)
        .offset(::core::mem::size_of::<ec_SoEt>() as isize);
    NotLast = true;
    /* send SoE request to slave */
    wkc = ecx_mbxsend(
        context,
        slave,
        &mut MbxOut as *mut ec_mbxbuft,
        EC_TIMEOUTTXM,
    );
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        while NotLast != false {
            /* clean mailboxbuffer */
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* succeeded to read slave response ? */
                /* slave response should be SoE, ReadRes */
                if (*aSoEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::Soe as libc::c_int
                    && (*aSoEp).opCode() as libc::c_int == SoEOpCode::ECT_SOE_READRES as libc::c_int
                    && (*aSoEp).error() as libc::c_int == 0i32
                    && (*aSoEp).driveNo() as libc::c_int == driveNo as libc::c_int
                    && (*aSoEp).elementflags as libc::c_int == elementflags as libc::c_int
                {
                    framedatasize = ((*aSoEp).MbxHeader.length as usize)
                        .wrapping_sub(core::mem::size_of::<ec_SoEt>() as usize)
                        .wrapping_add(core::mem::size_of::<ec_mbxheadert>() as usize)
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
                        NotLast = false;
                        *psize = totalsize
                    }
                } else {
                    /* other slave response */
                    NotLast = false;
                    if (*aSoEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                        == MailboxType::Soe as libc::c_int
                        && (*aSoEp).opCode() as libc::c_int
                            == SoEOpCode::ECT_SOE_READRES as libc::c_int
                        && (*aSoEp).error() as libc::c_int == 1i32
                    {
                        mp = (&mut MbxIn as *mut ec_mbxbuft as *mut u8).offset(
                            ((*aSoEp).MbxHeader.length as usize)
                                .wrapping_add(core::mem::size_of::<ec_mbxheadert>() as usize)
                                .wrapping_sub(core::mem::size_of::<u16>() as usize)
                                as isize,
                        );
                        errorcode = mp as *mut u16;
                        ecx_SoEerror(context.as_mut().unwrap(), slave, idn, *errorcode);
                    } else {
                        ecx_packeterror(context.as_mut().unwrap(), slave, idn, 0u8, 1u16);
                        /* Unexpected frame returned */
                    }
                    wkc = 0i32
                }
            } else {
                NotLast = false;
                ecx_packeterror(context.as_mut().unwrap(), slave, idn, 0u8, 4u16);
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
pub unsafe fn ecx_SoEwrite(
    context: *mut ecx_contextt,
    slave: u16,
    driveNo: u8,
    elementflags: u8,
    idn: u16,
    mut psize: libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut SoEp: *mut ec_SoEt = 0 as *mut ec_SoEt;
    let mut aSoEp: *mut ec_SoEt = 0 as *mut ec_SoEt;
    let mut framedatasize: libc::c_int = 0;
    let mut maxdata: libc::c_int = 0;
    let mut wkc: libc::c_int = 0;
    let mut mp: *mut u8 = 0 as *mut u8;
    let mut hp: *mut u8 = 0 as *mut u8;
    let mut errorcode: *mut u16 = 0 as *mut u16;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    let mut NotLast: bool = false;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0); /*  segmented transfer needed  */
    ec_clearmbx(&mut MbxOut);
    aSoEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SoEt;
    SoEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SoEt;
    (*SoEp).MbxHeader.address = 0u16;
    (*SoEp).MbxHeader.priority = 0u8;
    (*SoEp).set_opCode(SoEOpCode::ECT_SOE_WRITEREQ as u8);
    (*SoEp).set_error(0u8);
    (*SoEp).set_driveNo(driveNo);
    (*SoEp).elementflags = elementflags;
    hp = p as *mut u8;
    mp = (&mut MbxOut as *mut ec_mbxbuft as *mut u8)
        .offset(::core::mem::size_of::<ec_SoEt>() as isize);
    maxdata = ((*context).slavelist[slave as usize].mbx_l as libc::c_ulong)
        .wrapping_sub(core::mem::size_of::<ec_SoEt>() as u64) as libc::c_int;
    NotLast = true;
    while NotLast != false {
        framedatasize = psize;
        NotLast = false;
        (*SoEp).c2rust_unnamed.idn = idn;
        (*SoEp).set_incomplete(0u8);
        if framedatasize > maxdata {
            framedatasize = maxdata;
            NotLast = true;
            (*SoEp).set_incomplete(1u8);
            (*SoEp).c2rust_unnamed.fragmentsleft = (psize / maxdata) as u16
        }
        (*SoEp).MbxHeader.length = core::mem::size_of::<ec_SoEt>()
            .wrapping_sub(core::mem::size_of::<ec_mbxheadert>())
            .wrapping_add(framedatasize as usize) as u16;
        /* get new mailbox counter, used for session handle */
        cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* SoE */
        (*context).slavelist[slave as usize].mbx_cnt = cnt;
        (*SoEp).MbxHeader.mbxtype = (MailboxType::Soe as libc::c_int
            + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
            as u8;
        /* copy parameter data to mailbox */
        memcpy(
            mp as *mut libc::c_void,
            hp as *const libc::c_void,
            framedatasize as usize,
        );
        hp = hp.offset(framedatasize as isize);
        psize -= framedatasize;
        /* send SoE request to slave */
        wkc = ecx_mbxsend(
            context,
            slave,
            &mut MbxOut as *mut ec_mbxbuft,
            EC_TIMEOUTTXM,
        );
        if wkc > 0i32 {
            /* succeeded to place mailbox in slave ? */
            if NotLast == false || ecx_mbxempty(context, slave, timeout) == 0 {
                /* clean mailboxbuffer */
                ec_clearmbx(&mut MbxIn);
                /* read slave response */
                wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
                if wkc > 0i32 {
                    /* succeeded to read slave response ? */
                    NotLast = false;
                    /* slave response should be SoE, WriteRes */
                    if !((*aSoEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                        == MailboxType::Soe as libc::c_int
                        && (*aSoEp).opCode() as libc::c_int
                            == SoEOpCode::ECT_SOE_WRITERES as libc::c_int
                        && (*aSoEp).error() as libc::c_int == 0i32
                        && (*aSoEp).driveNo() as libc::c_int == driveNo as libc::c_int
                        && (*aSoEp).elementflags as libc::c_int == elementflags as libc::c_int)
                    {
                        /* other slave response */
                        if (*aSoEp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                            == MailboxType::Soe as libc::c_int
                            && (*aSoEp).opCode() as libc::c_int
                                == SoEOpCode::ECT_SOE_READRES as libc::c_int
                            && (*aSoEp).error() as libc::c_int == 1i32
                        {
                            mp = (&mut MbxIn as *mut ec_mbxbuft as *mut u8).offset(
                                ((*aSoEp).MbxHeader.length as usize)
                                    .wrapping_add(core::mem::size_of::<ec_mbxheadert>() as usize)
                                    .wrapping_sub(core::mem::size_of::<u16>() as usize)
                                    as isize,
                            );
                            errorcode = mp as *mut u16;
                            ecx_SoEerror(context.as_mut().unwrap(), slave, idn, *errorcode);
                        } else {
                            ecx_packeterror(context.as_mut().unwrap(), slave, idn, 0u8, 1u16);
                            /* Unexpected frame returned */
                        }
                        wkc = 0i32
                    }
                } else {
                    ecx_packeterror(context.as_mut().unwrap(), slave, idn, 0u8, 4u16);
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
pub unsafe fn ecx_readIDNmap(
    context: *mut ecx_contextt,
    slave: u16,
    Osize: *mut u32,
    Isize: *mut u32,
) -> libc::c_int {
    let mut retVal: libc::c_int = 0i32;
    let mut wkc: libc::c_int = 0;
    let mut psize: libc::c_int = 0;
    let mut driveNr: u8 = 0;
    let mut entries: u16 = 0;
    let mut itemcount: u16 = 0;
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
            EC_TIMEOUTRXM,
        );
        if wkc > 0i32
            && psize >= 4i32
            && {
                entries = (SoEmapping.currentlength as libc::c_int / 2i32) as u16;
                (entries as libc::c_int) > 0i32
            }
            && entries as libc::c_int <= 64i32
        {
            /* command word (u16) is always mapped but not in list */
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
                    EC_TIMEOUTRXM,
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
            EC_TIMEOUTRXM,
        );
        if wkc > 0i32
            && psize >= 4i32
            && {
                entries = (SoEmapping.currentlength as libc::c_int / 2i32) as u16;
                (entries as libc::c_int) > 0i32
            }
            && entries as libc::c_int <= 64i32
        {
            /* status word (u16) is always mapped but not in list */
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
                    EC_TIMEOUTRXM,
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
pub unsafe fn ec_SoEread(
    slave: u16,
    driveNo: u8,
    elementflags: u8,
    idn: u16,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
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
pub unsafe fn ec_SoEwrite(
    slave: u16,
    driveNo: u8,
    elementflags: u8,
    idn: u16,
    psize: libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
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
pub unsafe fn ec_readIDNmap(slave: u16, Osize: *mut u32, Isize: *mut u32) -> libc::c_int {
    return ecx_readIDNmap(&mut ecx_context, slave, Osize, Isize);
}
