use crate::{
    main::{
        ec_PDOassignt, ec_PDOdesct, ec_SMcommtypet, ec_clearmbx, ec_mbxbuft, ec_mbxheadert,
        ec_nextmbxcnt, ecx_context, ecx_contextt, ecx_mbxreceive, ecx_mbxsend, ecx_packeterror,
        ecx_pusherror, EC_MAXNAME, EC_MAXSM, EC_SMENABLEMASK,
    },
    osal::linux::osal::{ec_timet, osal_current_time},
    types::{
        ec_err_type, ec_errort, C2RustUnnamed_0, CoEMailboxType, CoEObjectDescription,
        CoESDOCommand, MailboxType, ECT_SDO_PDOASSIGN, ECT_SDO_SMCOMMTYPE, EC_TIMEOUTRXM,
        EC_TIMEOUTTXM,
    },
};
use libc::{memcpy, memset, strncpy};

/** max entries in Object Description list */
pub const EC_MAXODLIST: usize = 1024;

/** max entries in Object Entry list */
pub const EC_MAXOELIST: usize = 256;

#[derive(Copy, Clone)]
pub struct ec_ODlistt {
    pub Slave: u16,
    pub Entries: u16,
    pub Index: [u16; EC_MAXODLIST],
    pub DataType: [u16; EC_MAXODLIST],
    pub ObjectCode: [u8; EC_MAXODLIST],
    pub MaxSub: [u8; EC_MAXODLIST],
    pub Name: [[libc::c_char; EC_MAXNAME + 1]; EC_MAXODLIST],
}

#[derive(Copy, Clone)]
pub struct ec_OElistt {
    pub Entries: u16,
    pub ValueInfo: [u8; EC_MAXOELIST],
    pub DataType: [u16; EC_MAXOELIST],
    pub BitLength: [u16; EC_MAXOELIST],
    pub ObjAccess: [u16; EC_MAXOELIST],
    pub Name: [[libc::c_char; 41]; EC_MAXOELIST],
}

#[derive(Copy, Clone)]
pub union SdoData {
    pub bdata: [u8; 512],
    pub wdata: [u16; 256],
    pub ldata: [u32; 128],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_SDOt {
    pub MbxHeader: ec_mbxheadert,
    pub CANOpen: u16,
    pub Command: u8,
    pub Index: u16,
    pub SubIndex: u8,
    pub data: SdoData,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_SDOservicet {
    pub MbxHeader: ec_mbxheadert,
    pub CANOpen: u16,
    pub Opcode: u8,
    pub Reserved: u8,
    pub Fragments: u16,
    pub data: SdoData,
}
/* * Report SDO error.
 *
 * @param[in]  context    = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index that generated error
 * @param[in]  SubIdx     = Subindex that generated error
 * @param[in]  AbortCode  = Abortcode, see EtherCAT documentation for list
 */
#[no_mangle]
pub fn ecx_SDOerror(
    context: &mut ecx_contextt,
    Slave: u16,
    Index: u16,
    SubIdx: u8,
    AbortCode: i32,
) {
    let Ec: ec_errort = ec_errort {
        Time: osal_current_time(),
        Slave: Slave,
        Index: Index,
        SubIdx: SubIdx,
        Etype: ec_err_type::EC_ERR_TYPE_SDO_ERROR,
        c2rust_unnamed: C2RustUnnamed_0 { AbortCode },
        Signal: false,
    };

    ecx_pusherror(context, Ec);
}
/* * Report SDO info error
 *
 * @param[in]  context    = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index that generated error
 * @param[in]  SubIdx     = Subindex that generated error
 * @param[in]  AbortCode  = Abortcode, see EtherCAT documentation for list
 */
fn ecx_SDOinfoerror(
    context: &mut ecx_contextt,
    Slave: u16,
    Index: u16,
    SubIdx: u8,
    AbortCode: i32,
) {
    let Ec: ec_errort = ec_errort {
        Signal: false,
        Time: osal_current_time(),
        Slave: Slave,
        Index: Index,
        SubIdx: SubIdx,
        Etype: ec_err_type::EC_ERR_TYPE_SDOINFO_ERROR,
        c2rust_unnamed: C2RustUnnamed_0 { AbortCode },
    };

    context.ecaterror = true;

    ecx_pusherror(context, Ec);
}
/* * CoE SDO read, blocking. Single subindex or Complete Access.
 *
 * Only a "normal" upload request is issued. If the requested parameter is <= 4bytes
 * then a "expedited" response is returned, otherwise a "normal" response. If a "normal"
 * response is larger than the mailbox size then the response is segmented. The function
 * will combine all segments and copy them to the parameter buffer.
 *
 * @param[in]  context    = context struct
 * @param[in]  slave      = Slave number
 * @param[in]  index      = Index to read
 * @param[in]  subindex   = Subindex to read, must be 0 or 1 if CA is used.
 * @param[in]  CA         = FALSE = single subindex. TRUE = Complete Access, all subindexes read.
 * @param[in,out] psize   = Size in bytes of parameter buffer, returns bytes read from SDO.
 * @param[out] p          = Pointer to parameter buffer
 * @param[in]  timeout    = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe fn ecx_SDOread(
    context: *mut ecx_contextt,
    slave: u16,
    index: u16,
    mut subindex: u8,
    CA: bool,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut SDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut aSDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut bytesize: u16 = 0;
    let mut Framedatasize: u16 = 0;
    let mut wkc: libc::c_int = 0;
    let mut SDOlen: i32 = 0;
    let mut bp: *mut u8 = 0 as *mut u8;
    let mut hp: *mut u8 = 0 as *mut u8;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    let mut toggle: u8 = 0;
    let mut NotLast: bool = false;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOt;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
    (*SDOp).MbxHeader.length = 0xau16;
    (*SDOp).MbxHeader.address = 0u16;
    (*SDOp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* CoE */
    (*context).slavelist[slave as usize].mbx_cnt = cnt; /* number 9bits service upper 4 bits (SDO request) */
    (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8;
    (*SDOp).CANOpen = (0i32 + ((CoEMailboxType::SdoReq as libc::c_int) << 12i32)) as u16;
    if CA == true {
        (*SDOp).Command = CoESDOCommand::UpReqCa as u8
    /* upload request complete access */
    } else {
        (*SDOp).Command = CoESDOCommand::UpReq as u8
        /* upload request normal */
    }
    (*SDOp).Index = index;
    if CA == true && subindex as libc::c_int > 1i32 {
        subindex = 1u8
    }
    (*SDOp).SubIndex = subindex;
    (*SDOp).data.ldata[0usize] = 0u32;
    /* send CoE request to slave */
    wkc = ecx_mbxsend(
        context,
        slave,
        &mut MbxOut as *mut ec_mbxbuft,
        EC_TIMEOUTTXM,
    );
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        /* clean mailboxbuffer */
        ec_clearmbx(&mut MbxIn);
        wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
        if wkc > 0i32 {
            /* read slave response */
            /* succeeded to read slave response ? */
            /* slave response should be CoE, SDO response and the correct index */
            if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32 == MailboxType::Coe as libc::c_int
                && (*aSDOp).CANOpen as libc::c_int >> 12i32 == CoEMailboxType::SdoRes as libc::c_int
                && (*aSDOp).Index as libc::c_int == (*SDOp).Index as libc::c_int
            {
                if (*aSDOp).Command as libc::c_int & 0x2i32 > 0i32 {
                    /* expedited frame response */
                    bytesize = (4i32 - ((*aSDOp).Command as libc::c_int >> 2i32 & 0x3i32)) as u16;
                    if *psize >= bytesize as libc::c_int {
                        /* parameter buffer big enough ? */
                        /* copy parameter in parameter buffer */
                        memcpy(
                            p,
                            &mut *(*aSDOp).data.ldata.as_mut_ptr().offset(0isize) as *mut u32
                                as *const libc::c_void,
                            bytesize as usize,
                        );
                        *psize = bytesize as libc::c_int
                    } else {
                        wkc = 0i32;
                        ecx_packeterror(context.as_mut().unwrap(), slave, index, subindex, 3u16);
                        /* return the real parameter size */
                        /*  data container too small for type */
                    }
                } else {
                    /* normal frame response */
                    SDOlen = (*aSDOp).data.ldata[0usize] as i32;
                    /* Does parameter fit in parameter buffer ? */
                    if SDOlen <= *psize {
                        bp = p as *mut u8;
                        hp = p as *mut u8;
                        /* calculate mailbox transfer size */
                        Framedatasize = ((*aSDOp).MbxHeader.length as libc::c_int - 10i32) as u16;
                        if (Framedatasize as libc::c_int) < SDOlen {
                            /* transfer in segments? */
                            /* copy parameter data in parameter buffer */
                            memcpy(
                                hp as *mut libc::c_void,
                                &mut *(*aSDOp).data.ldata.as_mut_ptr().offset(1isize) as *mut u32
                                    as *const libc::c_void,
                                Framedatasize as usize,
                            );
                            hp = hp.offset(Framedatasize as libc::c_int as isize);
                            *psize = Framedatasize as libc::c_int;
                            NotLast = true;
                            toggle = 0u8;
                            while NotLast == false {
                                /* increment buffer pointer */
                                /* segmented transfer */
                                SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
                                (*SDOp).MbxHeader.length = 0xau16;
                                (*SDOp).MbxHeader.address = 0u16;
                                (*SDOp).MbxHeader.priority = 0u8;
                                cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt);
                                (*context).slavelist[slave as usize].mbx_cnt = cnt;
                                /* toggle bit for segment request */
                                (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
                                    + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
                                    as u8; /* CoE */
                                (*SDOp).CANOpen = (0i32
                                    + ((CoEMailboxType::SdoReq as libc::c_int) << 12i32))
                                    as u16; /* number 9bits service upper 4 bits (SDO request) */
                                (*SDOp).Command = (CoESDOCommand::SegUpReq as libc::c_int
                                    + toggle as libc::c_int)
                                    as u8; /* segment upload request */
                                (*SDOp).Index = index;
                                (*SDOp).SubIndex = subindex;
                                (*SDOp).data.ldata[0usize] = 0u32;
                                wkc = ecx_mbxsend(
                                    context,
                                    slave,
                                    &mut MbxOut as *mut ec_mbxbuft,
                                    EC_TIMEOUTTXM,
                                );
                                if wkc > 0i32 {
                                    ec_clearmbx(&mut MbxIn);
                                    /* send segmented upload request to slave */
                                    /* is mailbox transferred to slave ? */
                                    /* read slave response */
                                    wkc = ecx_mbxreceive(
                                        context,
                                        slave,
                                        &mut MbxIn as *mut ec_mbxbuft,
                                        timeout,
                                    );
                                    /* has slave responded ? */
                                    if wkc > 0i32 {
                                        /* slave response should be CoE, SDO response */
                                        if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                                            == MailboxType::Coe as libc::c_int
                                            && (*aSDOp).CANOpen as libc::c_int >> 12i32
                                                == CoEMailboxType::SdoRes as libc::c_int
                                            && (*aSDOp).Command as libc::c_int & 0xe0i32 == 0i32
                                        {
                                            /* calculate mailbox transfer size */
                                            Framedatasize =
                                                ((*aSDOp).MbxHeader.length as libc::c_int - 3i32)
                                                    as u16;
                                            if (*aSDOp).Command as libc::c_int & 0x1i32 > 0i32 {
                                                /* last segment */
                                                NotLast = false;
                                                if Framedatasize as libc::c_int == 7i32 {
                                                    /* subtract unused bytes from frame */
                                                    Framedatasize = (Framedatasize as libc::c_int
                                                        - (((*aSDOp).Command as libc::c_int
                                                            & 0xei32)
                                                            >> 1i32))
                                                        as u16
                                                }
                                                /* copy to parameter buffer */
                                                memcpy(
                                                    hp as *mut libc::c_void,
                                                    &mut (*aSDOp).Index as *mut u16
                                                        as *const libc::c_void,
                                                    Framedatasize as usize,
                                                );
                                            } else {
                                                /* segments follow */
                                                /* copy to parameter buffer */
                                                memcpy(
                                                    hp as *mut libc::c_void,
                                                    &mut (*aSDOp).Index as *mut u16
                                                        as *const libc::c_void,
                                                    Framedatasize as usize,
                                                );
                                                hp =
                                                    hp.offset(Framedatasize as libc::c_int as isize)
                                            }
                                            /* increment buffer pointer */
                                            /* update parameter size */
                                            *psize += Framedatasize as libc::c_int
                                        } else {
                                            /* unexpected frame returned from slave */
                                            NotLast = false; /* Unexpected frame returned */
                                            if (*aSDOp).Command as libc::c_int
                                                == CoESDOCommand::Abort as libc::c_int
                                            {
                                                /* SDO abort frame received */
                                                ecx_SDOerror(
                                                    context.as_mut().unwrap(),
                                                    slave,
                                                    index,
                                                    subindex,
                                                    (*aSDOp).data.ldata[0usize] as i32,
                                                );
                                            } else {
                                                ecx_packeterror(
                                                    context.as_mut().unwrap(),
                                                    slave,
                                                    index,
                                                    subindex,
                                                    1u16,
                                                );
                                            }
                                            wkc = 0i32
                                        }
                                    }
                                }
                                toggle = (toggle as libc::c_int ^ 0x10i32) as u8
                            }
                        } else {
                            /* non segmented transfer */
                            /* copy to parameter buffer */
                            memcpy(
                                bp as *mut libc::c_void,
                                &mut *(*aSDOp).data.ldata.as_mut_ptr().offset(1isize) as *mut u32
                                    as *const libc::c_void,
                                SDOlen as usize,
                            );
                            *psize = SDOlen
                        }
                    } else {
                        /* parameter buffer too small */
                        wkc = 0i32;
                        ecx_packeterror(context.as_mut().unwrap(), slave, index, subindex, 3u16);
                        /*  data container too small for type */
                    }
                }
            } else {
                /* other slave response */
                if (*aSDOp).Command as libc::c_int == CoESDOCommand::Abort as libc::c_int {
                    /* SDO abort frame received */
                    ecx_SDOerror(
                        context.as_mut().unwrap(),
                        slave,
                        index,
                        subindex,
                        (*aSDOp).data.ldata[0usize] as i32,
                    );
                } else {
                    ecx_packeterror(context.as_mut().unwrap(), slave, index, subindex, 1u16);
                    /* Unexpected frame returned */
                }
                wkc = 0i32
            }
        }
    }
    return wkc;
}
/* * CoE SDO write, blocking. Single subindex or Complete Access.
 *
 * A "normal" download request is issued, unless we have
 * small data, then a "expedited" transfer is used. If the parameter is larger than
 * the mailbox size then the download is segmented. The function will split the
 * parameter data in segments and send them to the slave one by one.
 *
 * @param[in]  context    = context struct
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index to write
 * @param[in]  SubIndex   = Subindex to write, must be 0 or 1 if CA is used.
 * @param[in]  CA         = FALSE = single subindex. TRUE = Complete Access, all subindexes written.
 * @param[in]  psize      = Size in bytes of parameter buffer.
 * @param[out] p          = Pointer to parameter buffer
 * @param[in]  Timeout    = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe fn ecx_SDOwrite(
    context: *mut ecx_contextt,
    slave: u16,
    Index: u16,
    SubIndex: u8,
    CA: bool,
    mut psize: libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut SDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut aSDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut wkc: libc::c_int = 0;
    let mut maxdata: libc::c_int = 0;
    let mut framedatasize: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    let mut toggle: u8 = 0;
    let mut NotLast: bool = false;
    let mut hp: *mut u8 = 0 as *mut u8;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0); /* data section=mailbox size - 6 mbx - 2 CoE - 8 sdo req */
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOt;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
    maxdata = (*context).slavelist[slave as usize].mbx_l as libc::c_int - 0x10i32;
    /* if small data use expedited transfer */
    if psize <= 4i32 && CA == false {
        (*SDOp).MbxHeader.length = 0xau16;
        (*SDOp).MbxHeader.address = 0u16;
        (*SDOp).MbxHeader.priority = 0u8;
        /* get new mailbox counter, used for session handle */
        cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* CoE */
        (*context).slavelist[slave as usize].mbx_cnt = cnt; /* number 9bits service upper 4 bits */
        (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
            + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
            as u8; /* expedited SDO download transfer */
        (*SDOp).CANOpen = (0i32 + ((CoEMailboxType::SdoReq as libc::c_int) << 12i32)) as u16;
        (*SDOp).Command =
            (CoESDOCommand::DownExp as libc::c_int | 4i32 - psize << 2i32 & 0xci32) as u8;
        (*SDOp).Index = Index;
        (*SDOp).SubIndex = SubIndex;
        hp = p as *mut u8;
        /* copy parameter data to mailbox */
        memcpy(
            &mut *(*SDOp).data.ldata.as_mut_ptr().offset(0isize) as *mut u32 as *mut libc::c_void,
            hp as *const libc::c_void,
            psize as usize,
        );
        /* send mailbox SDO download request to slave */
        wkc = ecx_mbxsend(
            context,
            slave,
            &mut MbxOut as *mut ec_mbxbuft,
            EC_TIMEOUTTXM,
        );
        if wkc > 0i32 {
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* response should be CoE, SDO response, correct index and subindex */
                if !((*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::Coe as libc::c_int
                    && (*aSDOp).CANOpen as libc::c_int >> 12i32
                        == CoEMailboxType::SdoRes as libc::c_int
                    && (*aSDOp).Index as libc::c_int == (*SDOp).Index as libc::c_int
                    && (*aSDOp).SubIndex as libc::c_int == (*SDOp).SubIndex as libc::c_int)
                {
                    /* unexpected response from slave */
                    if (*aSDOp).Command as libc::c_int == CoESDOCommand::Abort as libc::c_int {
                        /* SDO abort frame received */
                        ecx_SDOerror(
                            context.as_mut().unwrap(),
                            slave,
                            Index,
                            SubIndex,
                            (*aSDOp).data.ldata[0usize] as i32,
                        );
                    } else {
                        ecx_packeterror(context.as_mut().unwrap(), slave, Index, SubIndex, 1u16);
                        /* Unexpected frame returned */
                    } /*  segmented transfer needed  */
                    wkc = 0i32
                }
            }
        }
    } else {
        framedatasize = psize;
        NotLast = false;
        if framedatasize > maxdata {
            framedatasize = maxdata;
            NotLast = true
        }
        (*SDOp).MbxHeader.length = (0xai32 + framedatasize) as u16;
        (*SDOp).MbxHeader.address = 0u16;
        (*SDOp).MbxHeader.priority = 0u8;
        /* get new mailbox counter, used for session handle */
        cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* CoE */
        (*context).slavelist[slave as usize].mbx_cnt = cnt; /* number 9bits service upper 4 bits */
        (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
            + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
            as u8;
        (*SDOp).CANOpen = (0i32 + ((CoEMailboxType::SdoReq as libc::c_int) << 12i32)) as u16;
        if CA == true {
            (*SDOp).Command = CoESDOCommand::DownInitCa as u8
        /* Complete Access, normal SDO init download transfer */
        } else {
            (*SDOp).Command = CoESDOCommand::DownInit as u8
            /* normal SDO init download transfer */
        }
        (*SDOp).Index = Index;
        (*SDOp).SubIndex = SubIndex;
        if CA as libc::c_int != 0 && SubIndex as libc::c_int > 1i32 {
            (*SDOp).SubIndex = 1u8
        }
        (*SDOp).data.ldata[0usize] = psize as u32;
        hp = p as *mut u8;
        /* copy parameter data to mailbox */
        memcpy(
            &mut *(*SDOp).data.ldata.as_mut_ptr().offset(1isize) as *mut u32 as *mut libc::c_void,
            hp as *const libc::c_void,
            framedatasize as usize,
        );
        hp = hp.offset(framedatasize as isize);
        psize -= framedatasize;
        /* send mailbox SDO download request to slave */
        wkc = ecx_mbxsend(
            context,
            slave,
            &mut MbxOut as *mut ec_mbxbuft,
            EC_TIMEOUTTXM,
        );
        if wkc > 0i32 {
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* response should be CoE, SDO response, correct index and subindex */
                if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::Coe as libc::c_int
                    && (*aSDOp).CANOpen as libc::c_int >> 12i32
                        == CoEMailboxType::SdoRes as libc::c_int
                    && (*aSDOp).Index as libc::c_int == (*SDOp).Index as libc::c_int
                    && (*aSDOp).SubIndex as libc::c_int == (*SDOp).SubIndex as libc::c_int
                {
                    /* all ok */
                    maxdata += 7i32;
                    toggle = 0u8;
                    /* repeat while segments left */
                    while NotLast == true {
                        SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
                        framedatasize = psize;
                        NotLast = false;
                        /* toggle bit for segment request */
                        (*SDOp).Command = 0x1u8; /* last segment */
                        if framedatasize > maxdata {
                            framedatasize = maxdata; /*  more segments needed  */
                            NotLast = true;
                            (*SDOp).Command = 0u8; /* segments follow */
                        } /* minimum size */
                        if NotLast == false && framedatasize < 7i32 {
                            (*SDOp).MbxHeader.length = 0xau16;
                            (*SDOp).Command = (0x1i32 + (7i32 - framedatasize << 1i32)) as u8
                        /* last segment reduced octets */
                        } else {
                            (*SDOp).MbxHeader.length = (framedatasize + 3i32) as u16
                            /* data + 2 CoE + 1 SDO */
                        }
                        (*SDOp).MbxHeader.address = 0u16;
                        (*SDOp).MbxHeader.priority = 0u8;
                        cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt);
                        (*context).slavelist[slave as usize].mbx_cnt = cnt;
                        (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
                            + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
                            as u8;
                        (*SDOp).CANOpen =
                            (0i32 + ((CoEMailboxType::SdoReq as libc::c_int) << 12i32)) as u16;
                        (*SDOp).Command =
                            ((*SDOp).Command as libc::c_int + toggle as libc::c_int) as u8;
                        memcpy(
                            &mut (*SDOp).Index as *mut u16 as *mut libc::c_void,
                            hp as *const libc::c_void,
                            framedatasize as usize,
                        );
                        hp = hp.offset(framedatasize as isize);
                        psize -= framedatasize;
                        wkc = ecx_mbxsend(
                            context,
                            slave,
                            &mut MbxOut as *mut ec_mbxbuft,
                            EC_TIMEOUTTXM,
                        );
                        if wkc > 0i32 {
                            ec_clearmbx(&mut MbxIn);
                            /* get new mailbox counter value */
                            /* CoE */
                            /* number 9bits service upper 4 bits (SDO request) */
                            /* add toggle bit to command byte */
                            /* copy parameter data to mailbox */
                            /* update parameter buffer pointer */
                            /* send SDO download request */
                            /* read slave response */
                            wkc = ecx_mbxreceive(
                                context,
                                slave,
                                &mut MbxIn as *mut ec_mbxbuft,
                                timeout,
                            );
                            if wkc > 0i32 {
                                if !((*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                                    == MailboxType::Coe as libc::c_int
                                    && (*aSDOp).CANOpen as libc::c_int >> 12i32
                                        == CoEMailboxType::SdoRes as libc::c_int
                                    && (*aSDOp).Command as libc::c_int & 0xe0i32 == 0x20i32)
                                {
                                    if (*aSDOp).Command as libc::c_int
                                        == CoESDOCommand::Abort as libc::c_int
                                    {
                                        /* SDO abort frame received */
                                        ecx_SDOerror(
                                            context.as_mut().unwrap(),
                                            slave,
                                            Index,
                                            SubIndex,
                                            (*aSDOp).data.ldata[0usize] as i32,
                                        );
                                    } else {
                                        ecx_packeterror(
                                            context.as_mut().unwrap(),
                                            slave,
                                            Index,
                                            SubIndex,
                                            1u16,
                                        );
                                        /* Unexpected frame returned */
                                    }
                                    wkc = 0i32;
                                    NotLast = false
                                }
                            }
                        }
                        toggle = (toggle as libc::c_int ^ 0x10i32) as u8
                    }
                } else {
                    /* unexpected response from slave */
                    if (*aSDOp).Command as libc::c_int == CoESDOCommand::Abort as libc::c_int {
                        /* SDO abort frame received */
                        ecx_SDOerror(
                            context.as_mut().unwrap(),
                            slave,
                            Index,
                            SubIndex,
                            (*aSDOp).data.ldata[0usize] as i32,
                        );
                    } else {
                        ecx_packeterror(context.as_mut().unwrap(), slave, Index, SubIndex, 1u16);
                        /* Unexpected frame returned */
                    }
                    wkc = 0i32
                }
            }
        }
    }
    return wkc;
}
/* * CoE RxPDO write, blocking.
 *
 * A RxPDO download request is issued.
 *
 * @param[in]  context       = context struct
 * @param[in]  Slave         = Slave number
 * @param[in]  RxPDOnumber   = Related RxPDO number
 * @param[in]  psize         = Size in bytes of PDO buffer.
 * @param[out] p             = Pointer to PDO buffer
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe fn ecx_RxPDO(
    context: *mut ecx_contextt,
    slave: u16,
    RxPDOnumber: u16,
    psize: libc::c_int,
    p: *mut libc::c_void,
) -> libc::c_int {
    let mut SDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut wkc: libc::c_int = 0;
    let mut maxdata: libc::c_int = 0;
    let mut framedatasize: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0); /* data section=mailbox size - 6 mbx - 2 CoE */
    ec_clearmbx(&mut MbxOut);
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
    maxdata = (*context).slavelist[slave as usize].mbx_l as libc::c_int - 0x8i32;
    framedatasize = psize;
    if framedatasize > maxdata {
        framedatasize = maxdata
        /*  limit transfer */
    }
    (*SDOp).MbxHeader.length = (0x2i32 + framedatasize) as u16;
    (*SDOp).MbxHeader.address = 0u16;
    (*SDOp).MbxHeader.priority = 0u8;
    /* get new mailbox counter, used for session handle */
    cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* CoE */
    (*context).slavelist[slave as usize].mbx_cnt = cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8;
    (*SDOp).CANOpen = ((RxPDOnumber as libc::c_int & 0x1ffi32)
        + ((CoEMailboxType::RxPdo as libc::c_int) << 12i32)) as u16;
    /* copy PDO data to mailbox */
    memcpy(
        &mut (*SDOp).Command as *mut u8 as *mut libc::c_void,
        p,
        framedatasize as usize,
    );
    /* send mailbox RxPDO request to slave */
    wkc = ecx_mbxsend(
        context,
        slave,
        &mut MbxOut as *mut ec_mbxbuft,
        EC_TIMEOUTTXM,
    );
    return wkc;
}
/* * CoE TxPDO read remote request, blocking.
 *
 * A RxPDO download request is issued.
 *
 * @param[in]  context       = context struct
 * @param[in]  slave         = Slave number
 * @param[in]  TxPDOnumber   = Related TxPDO number
 * @param[in,out] psize      = Size in bytes of PDO buffer, returns bytes read from PDO.
 * @param[out] p             = Pointer to PDO buffer
 * @param[in]  timeout       = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe fn ecx_TxPDO(
    context: *mut ecx_contextt,
    slave: u16,
    TxPDOnumber: u16,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut SDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut aSDOp: *mut ec_SDOt = 0 as *mut ec_SDOt;
    let mut wkc: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    let mut framedatasize: u16 = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOt;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOt;
    (*SDOp).MbxHeader.length = 0x2u16;
    (*SDOp).MbxHeader.address = 0u16;
    (*SDOp).MbxHeader.priority = 0u8;
    /* get new mailbox counter, used for session handle */
    cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* CoE */
    (*context).slavelist[slave as usize].mbx_cnt = cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8;
    (*SDOp).CANOpen = ((TxPDOnumber as libc::c_int & 0x1ffi32)
        + ((CoEMailboxType::TxPdoRr as libc::c_int) << 12i32)) as u16;
    wkc = ecx_mbxsend(
        context,
        slave,
        &mut MbxOut as *mut ec_mbxbuft,
        EC_TIMEOUTTXM,
    );
    if wkc > 0i32 {
        /* clean mailboxbuffer */
        ec_clearmbx(&mut MbxIn);
        /* read slave response */
        wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
        if wkc > 0i32 {
            /* succeeded to read slave response ? */
            /* slave response should be CoE, TxPDO */
            if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32 == MailboxType::Coe as libc::c_int
                && (*aSDOp).CANOpen as libc::c_int >> 12i32 == CoEMailboxType::TxPdo as libc::c_int
            {
                /* TxPDO response */
                framedatasize = ((*aSDOp).MbxHeader.length as libc::c_int - 2i32) as u16;
                if *psize >= framedatasize as libc::c_int {
                    /* parameter buffer big enough ? */
                    /* copy parameter in parameter buffer */
                    memcpy(
                        p,
                        &mut (*aSDOp).Command as *mut u8 as *const libc::c_void,
                        framedatasize as usize,
                    );
                    *psize = framedatasize as libc::c_int
                } else {
                    /* return the real parameter size */
                    /* parameter buffer too small */
                    wkc = 0i32;
                    ecx_packeterror(context.as_mut().unwrap(), slave, 0u16, 0u8, 3u16);
                    /*  data container too small for type */
                }
            } else {
                /* other slave response */
                if (*aSDOp).Command as libc::c_int == CoESDOCommand::Abort as libc::c_int {
                    /* SDO abort frame received */
                    ecx_SDOerror(
                        context.as_mut().unwrap(),
                        slave,
                        0u16,
                        0u8,
                        (*aSDOp).data.ldata[0usize] as i32,
                    );
                } else {
                    ecx_packeterror(context.as_mut().unwrap(), slave, 0u16, 0u8, 1u16);
                    /* Unexpected frame returned */
                }
                wkc = 0i32
            }
        }
    }
    return wkc;
}
/* * Read PDO assign structure
 * @param[in]  context       = context struct
 * @param[in]  Slave         = Slave number
 * @param[in]  PDOassign     = PDO assign object
 * @return total bitlength of PDO assign
 */
#[no_mangle]
pub unsafe fn ecx_readPDOassign(context: *mut ecx_contextt, Slave: u16, PDOassign: u16) -> u32 {
    let mut idxloop: u16 = 0;
    let mut nidx: u16 = 0;
    let mut subidxloop: u16 = 0;
    let mut rdat: u16 = 0;
    let mut idx: u16 = 0;
    let mut subidx: u16 = 0;
    let mut subcnt: u8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut rdat2: i32 = 0;
    let mut bsize: u32 = 0u32;
    rdl = ::core::mem::size_of::<u16>() as libc::c_int;
    rdat = 0u16;
    /* read PDO assign subindex 0 ( = number of PDO's) */
    wkc = ecx_SDOread(
        context,
        Slave,
        PDOassign,
        0u8,
        false,
        &mut rdl,
        &mut rdat as *mut u16 as *mut libc::c_void,
        EC_TIMEOUTRXM,
    );
    rdat = rdat;
    /* positive result from slave ? */
    if wkc > 0i32 && rdat as libc::c_int > 0i32 {
        /* number of available sub indexes */
        nidx = rdat;
        bsize = 0u32;
        /* read all PDO's */
        idxloop = 1u16;
        while idxloop as libc::c_int <= nidx as libc::c_int {
            rdl = ::core::mem::size_of::<u16>() as libc::c_int;
            rdat = 0u16;
            /* read PDO assign */
            wkc = ecx_SDOread(
                context,
                Slave,
                PDOassign,
                idxloop as u8,
                false,
                &mut rdl,
                &mut rdat as *mut u16 as *mut libc::c_void,
                EC_TIMEOUTRXM,
            );
            /* result is index of PDO */
            idx = rdat;
            if idx as libc::c_int > 0i32 {
                rdl = ::core::mem::size_of::<u8>() as libc::c_int;
                subcnt = 0u8;
                /* read number of subindexes of PDO */
                wkc = ecx_SDOread(
                    context,
                    Slave,
                    idx,
                    0u8,
                    false,
                    &mut rdl,
                    &mut subcnt as *mut u8 as *mut libc::c_void,
                    EC_TIMEOUTRXM,
                );
                subidx = subcnt as u16;
                /* for each subindex */
                subidxloop = 1u16;
                while subidxloop as libc::c_int <= subidx as libc::c_int {
                    rdl = ::core::mem::size_of::<i32>() as libc::c_int;
                    rdat2 = 0i32;
                    /* read SDO that is mapped in PDO */
                    wkc = ecx_SDOread(
                        context,
                        Slave,
                        idx,
                        subidxloop as u8,
                        false,
                        &mut rdl,
                        &mut rdat2 as *mut i32 as *mut libc::c_void,
                        EC_TIMEOUTRXM,
                    );
                    rdat2 = rdat2;
                    /* extract bitlength of SDO */
                    if (rdat2 & 0xffi32) < 0xffi32 {
                        bsize = (bsize).wrapping_add((rdat2 & 0xffi32) as libc::c_uint)
                    } else {
                        rdl = ::core::mem::size_of::<u16>() as libc::c_int;
                        rdat = 0xffu16;
                        /* read Object Entry in Object database */
                        //                  wkc = ec_readOEsingle(idx, (u8)SubCount, pODlist, pOElist);
                        bsize = (bsize).wrapping_add(rdat as libc::c_uint)
                    }
                    subidxloop = subidxloop.wrapping_add(1)
                }
            }
            idxloop = idxloop.wrapping_add(1)
        }
    }
    /* return total found bitlength (PDO) */
    return bsize;
}
/* * Read PDO assign structure in Complete Access mode
 * @param[in]  context       = context struct
 * @param[in]  Slave         = Slave number
 * @param[in]  Thread_n      = Calling thread index
 * @param[in]  PDOassign     = PDO assign object
 * @return total bitlength of PDO assign
 */
#[no_mangle]
pub unsafe fn ecx_readPDOassignCA(
    context: *mut ecx_contextt,
    Slave: u16,
    Thread_n: libc::c_int,
    PDOassign: u16,
) -> u32 {
    let mut idxloop: u16 = 0;
    let mut nidx: u16 = 0;
    let mut subidxloop: u16 = 0;
    let mut idx: u16 = 0;
    let mut subidx: u16 = 0;
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut bsize: u32 = 0u32;
    /* find maximum size of PDOassign buffer */
    rdl = ::core::mem::size_of::<ec_PDOassignt>() as libc::c_int;
    (*(*context).PDOassign.offset(Thread_n as isize)).n = 0u8;
    /* read rxPDOassign in CA mode, all subindexes are read in one struct */
    wkc = ecx_SDOread(
        context,
        Slave,
        PDOassign,
        0u8,
        true,
        &mut rdl,
        &mut *(*context).PDOassign.offset(Thread_n as isize) as *mut ec_PDOassignt
            as *mut libc::c_void,
        EC_TIMEOUTRXM,
    );
    /* positive result from slave ? */
    if wkc > 0i32 && (*(*context).PDOassign.offset(Thread_n as isize)).n as libc::c_int > 0i32 {
        nidx = (*(*context).PDOassign.offset(Thread_n as isize)).n as u16;
        bsize = 0u32;
        /* for each PDO do */
        idxloop = 1u16;
        while idxloop as libc::c_int <= nidx as libc::c_int {
            /* get index from PDOassign struct */
            idx = (*(*context).PDOassign.offset(Thread_n as isize)).index
                [(idxloop as libc::c_int - 1i32) as usize];
            if idx as libc::c_int > 0i32 {
                rdl = ::core::mem::size_of::<ec_PDOdesct>() as libc::c_int;
                (*(*context).PDOdesc.offset(Thread_n as isize)).n = 0u8;
                /* read SDO's that are mapped in PDO, CA mode */
                wkc = ecx_SDOread(
                    context,
                    Slave,
                    idx,
                    0u8,
                    true,
                    &mut rdl,
                    &mut *(*context).PDOdesc.offset(Thread_n as isize) as *mut ec_PDOdesct
                        as *mut libc::c_void,
                    EC_TIMEOUTRXM,
                );
                subidx = (*(*context).PDOdesc.offset(Thread_n as isize)).n as u16;
                /* extract all bitlengths of SDO's */
                subidxloop = 1u16;
                while subidxloop as libc::c_int <= subidx as libc::c_int {
                    bsize = (bsize).wrapping_add(
                        (*(*context).PDOdesc.offset(Thread_n as isize)).PDO
                            [(subidxloop as libc::c_int - 1i32) as usize]
                            & 0xffu32,
                    );
                    subidxloop = subidxloop.wrapping_add(1)
                }
            }
            idxloop = idxloop.wrapping_add(1)
        }
    }
    /* return total found bitlength (PDO) */
    return bsize;
}
/* * CoE read PDO mapping.
 *
 * CANopen has standard indexes defined for PDO mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave.
 *
 * Principal structure in slave:\n
 * 1C00:00 is number of SM defined\n
 * 1C00:01 SM0 type -> 1C10\n
 * 1C00:02 SM1 type -> 1C11\n
 * 1C00:03 SM2 type -> 1C12\n
 * 1C00:04 SM3 type -> 1C13\n
 * Type 0 = unused, 1 = mailbox in, 2 = mailbox out,
 * 3 = outputs (RxPDO), 4 = inputs (TxPDO).
 *
 * 1C12:00 is number of PDO's defined for SM2\n
 * 1C12:01 PDO assign SDO #1 -> f.e. 1A00\n
 * 1C12:02 PDO assign SDO #2 -> f.e. 1A04\
 *
 * 1A00:00 is number of object defined for this PDO\n
 * 1A00:01 object mapping #1, f.e. 60100710 (SDO 6010 SI 07 bitlength 0x10)
 *
 * @param[in]  context = context struct
 * @param[in]  Slave   = Slave number
 * @param[out] Osize   = Size in bits of output mapping (rxPDO) found
 * @param[out] Isize   = Size in bits of input mapping (txPDO) found
 * @return >0 if mapping successful.
 */
#[no_mangle]
pub unsafe fn ecx_readPDOmap(
    context: *mut ecx_contextt,
    slave: u16,
    Osize: *mut u32,
    Isize: *mut u32,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut retVal: libc::c_int = 0i32;
    let mut nSM: u8 = 0;
    let mut iSM: u8 = 0;
    let mut tSM: u8 = 0;
    let mut Tsize: u32 = 0;
    let mut SMt_bug_add: u8 = 0;
    *Isize = 0u32;
    *Osize = 0u32;
    SMt_bug_add = 0u8;
    rdl = ::core::mem::size_of::<u8>() as libc::c_int;
    nSM = 0u8;
    /* read SyncManager Communication Type object count */
    wkc = ecx_SDOread(
        context,
        slave,
        ECT_SDO_SMCOMMTYPE,
        0u8,
        false,
        &mut rdl,
        &mut nSM as *mut u8 as *mut libc::c_void,
        EC_TIMEOUTRXM,
    );
    /* positive result from slave ? */
    if wkc > 0i32 && nSM as libc::c_int > 2i32 {
        /* limit to maximum number of SM defined, if true the slave can't be configured */
        if nSM as usize > EC_MAXSM {
            nSM = EC_MAXSM as u8
        }
        /* iterate for every SM type defined */
        iSM = 2u8;
        while (iSM as libc::c_int) < nSM as libc::c_int {
            rdl = ::core::mem::size_of::<u8>() as libc::c_int;
            tSM = 0u8;
            /* read SyncManager Communication Type */
            wkc = ecx_SDOread(
                context,
                slave,
                ECT_SDO_SMCOMMTYPE,
                (iSM as libc::c_int + 1i32) as u8,
                false,
                &mut rdl,
                &mut tSM as *mut u8 as *mut libc::c_void,
                EC_TIMEOUTRXM,
            );
            if wkc > 0i32 {
                // start slave bug prevention code, remove if possible
                if iSM as libc::c_int == 2i32 && tSM as libc::c_int == 2i32 {
                    // SM2 has type 2 == mailbox out, this is a bug in the slave!
                    SMt_bug_add = 1u8
                    // try to correct, this works if the types are 0 1 2 3 and should be 1 2 3 4
                }
                if tSM != 0 {
                    tSM = (tSM as libc::c_int + SMt_bug_add as libc::c_int) as u8
                    // only add if SMt > 0
                }
                if iSM as libc::c_int == 2i32 && tSM as libc::c_int == 0i32 {
                    // SM2 has type 0, this is a bug in the slave!
                    tSM = 3u8
                }
                if iSM as libc::c_int == 3i32 && tSM as libc::c_int == 0i32 {
                    // SM3 has type 0, this is a bug in the slave!
                    tSM = 4u8
                }
                // end slave bug prevention code
                (*context).slavelist[slave as usize].SMtype[iSM as usize] = tSM;
                /* check if SM is unused -> clear enable flag */
                if tSM as libc::c_int == 0i32 {
                    (*context).slavelist[slave as usize].SM[iSM as usize].SMflags =
                        (*context).slavelist[slave as usize].SM[iSM as usize].SMflags
                            & 0xfffeffffu32
                }
                if tSM as libc::c_int == 3i32 || tSM as libc::c_int == 4i32 {
                    /* read the assign PDO */
                    Tsize = ecx_readPDOassign(context, slave, ECT_SDO_PDOASSIGN + iSM as u16);
                    /* if a mapping is found */
                    if Tsize != 0 {
                        (*context).slavelist[slave as usize].SM[iSM as usize].SMlength =
                            Tsize.wrapping_add(7u32).wrapping_div(8u32) as u16;
                        if tSM as libc::c_int == 3i32 {
                            /* we are doing outputs */
                            *Osize = (*Osize).wrapping_add(Tsize)
                        } else {
                            /* we are doing inputs */
                            *Isize = (*Isize).wrapping_add(Tsize)
                        }
                    }
                }
            }
            iSM = iSM.wrapping_add(1)
        }
    }
    /* found some I/O bits ? */
    if *Isize > 0u32 || *Osize > 0u32 {
        retVal = 1i32
    }
    return retVal;
}
/* * CoE read PDO mapping in Complete Access mode (CA).
 *
 * CANopen has standard indexes defined for PDO mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave. Slave has to support CA, otherwise use ec_readPDOmap().
 *
 * @param[in]  context  = context struct
 * @param[in]  Slave    = Slave number
 * @param[in]  Thread_n = Calling thread index
 * @param[out] Osize    = Size in bits of output mapping (rxPDO) found
 * @param[out] Isize    = Size in bits of input mapping (txPDO) found
 * @return >0 if mapping successful.
 */
#[no_mangle]
pub unsafe fn ecx_readPDOmapCA(
    context: *mut ecx_contextt,
    slave: u16,
    Thread_n: libc::c_int,
    Osize: *mut u32,
    Isize: *mut u32,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut retVal: libc::c_int = 0i32;
    let mut nSM: u8 = 0;
    let mut iSM: u8 = 0;
    let mut tSM: u8 = 0;
    let mut Tsize: u32 = 0;
    let mut SMt_bug_add: u8 = 0;
    *Isize = 0u32;
    *Osize = 0u32;
    SMt_bug_add = 0u8;
    rdl = ::core::mem::size_of::<ec_SMcommtypet>() as libc::c_int;
    (*(*context).SMcommtype.offset(Thread_n as isize)).n = 0u8;
    /* read SyncManager Communication Type object count Complete Access*/
    wkc = ecx_SDOread(
        context,
        slave,
        ECT_SDO_SMCOMMTYPE,
        0u8,
        true,
        &mut rdl,
        &mut *(*context).SMcommtype.offset(Thread_n as isize) as *mut ec_SMcommtypet
            as *mut libc::c_void,
        EC_TIMEOUTRXM,
    );
    /* positive result from slave ? */
    if wkc > 0i32 && (*(*context).SMcommtype.offset(Thread_n as isize)).n as libc::c_int > 2i32 {
        nSM = (*(*context).SMcommtype.offset(Thread_n as isize)).n;
        /* limit to maximum number of SM defined, if true the slave can't be configured */
        if nSM as usize > EC_MAXSM {
            nSM = EC_MAXSM as u8;
            ecx_packeterror(context.as_mut().unwrap(), slave, 0u16, 0u8, 10u16);
            /* #SM larger than EC_MAXSM */
        }
        /* iterate for every SM type defined */
        iSM = 2u8;
        while (iSM as libc::c_int) < nSM as libc::c_int {
            tSM = (*(*context).SMcommtype.offset(Thread_n as isize)).SMtype[iSM as usize];
            // start slave bug prevention code, remove if possible
            if iSM as libc::c_int == 2i32 && tSM as libc::c_int == 2i32 {
                // SM2 has type 2 == mailbox out, this is a bug in the slave!
                SMt_bug_add = 1u8
                // try to correct, this works if the types are 0 1 2 3 and should be 1 2 3 4
            }
            if tSM != 0 {
                tSM = (tSM as libc::c_int + SMt_bug_add as libc::c_int) as u8
                // only add if SMt > 0
            }
            // end slave bug prevention code
            (*context).slavelist[slave as usize].SMtype[iSM as usize] = tSM;
            /* check if SM is unused -> clear enable flag */
            if tSM as libc::c_int == 0i32 {
                (*context).slavelist[slave as usize].SM[iSM as usize].SMflags =
                    (*context).slavelist[slave as usize].SM[iSM as usize].SMflags & EC_SMENABLEMASK
            }
            if tSM as libc::c_int == 3i32 || tSM as libc::c_int == 4i32 {
                /* read the assign PDO */
                Tsize =
                    ecx_readPDOassignCA(context, slave, Thread_n, ECT_SDO_PDOASSIGN + iSM as u16);
                /* if a mapping is found */
                if Tsize != 0 {
                    (*context).slavelist[slave as usize].SM[iSM as usize].SMlength =
                        Tsize.wrapping_add(7u32).wrapping_div(8u32) as u16;
                    if tSM as libc::c_int == 3i32 {
                        /* we are doing outputs */
                        *Osize = (*Osize).wrapping_add(Tsize)
                    } else {
                        /* we are doing inputs */
                        *Isize = (*Isize).wrapping_add(Tsize)
                    }
                }
            }
            iSM = iSM.wrapping_add(1)
        }
    }
    /* found some I/O bits ? */
    if *Isize > 0u32 || *Osize > 0u32 {
        retVal = 1i32
    }
    return retVal;
}
/* * CoE read Object Description List.
 *
 * @param[in]  context  = context struct
 * @param[in]  Slave    = Slave number.
 * @param[out] pODlist  = resulting Object Description list.
 * @return Workcounter of slave response.
 */
#[no_mangle]
pub unsafe fn ecx_readODlist(
    context: *mut ecx_contextt,
    slave: u16,
    mut pODlist: *mut ec_ODlistt,
) -> libc::c_int {
    let mut SDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut aSDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut wkc: libc::c_int = 0;
    let mut x: u16 = 0;
    let mut n: u16 = 0;
    let mut i: u16 = 0;
    let mut sp: u16 = 0;
    let mut offset: u16 = 0;
    let mut stop: bool = false;
    let mut cnt: u8 = 0;
    let mut First: bool = false;
    (*pODlist).Slave = slave;
    (*pODlist).Entries = 0u16;
    ec_clearmbx(&mut MbxIn);
    /* clear pending out mailbox in slave if available. Timeout is set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn, 0);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOservicet;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOservicet;
    (*SDOp).MbxHeader.length = 0x8u16;
    (*SDOp).MbxHeader.address = 0u16;
    (*SDOp).MbxHeader.priority = 0u8;
    /* Get new mailbox counter value */
    cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* CoE */
    (*context).slavelist[slave as usize].mbx_cnt = cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8; /* get object description list request */
    (*SDOp).CANOpen = (0i32 + ((CoEMailboxType::SdoInfo as libc::c_int) << 12i32)) as u16; /* fragments left */
    (*SDOp).Opcode = CoEObjectDescription::ECT_GET_ODLIST_REQ as u8; /* all objects */
    (*SDOp).Reserved = 0u8;
    (*SDOp).Fragments = 0u16;
    (*SDOp).data.wdata[0usize] = 0x1u16;
    /* send get object description list request to slave */
    wkc = ecx_mbxsend(context, slave, &mut MbxOut, EC_TIMEOUTTXM);
    /* mailbox placed in slave ? */
    if wkc > 0i32 {
        x = 0u16; /* offset to skip info header in first frame, otherwise set to 0 */
        sp = 0u16; /* assume this is last iteration */
        First = true;
        offset = 1u16;
        loop {
            stop = true;
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn, EC_TIMEOUTRXM);
            /* got response ? */
            if wkc > 0i32 {
                /* response should be CoE and "get object description list response" */
                if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32
                    == MailboxType::Coe as libc::c_int
                    && (*aSDOp).Opcode as libc::c_int & 0x7fi32
                        == CoEObjectDescription::ECT_GET_ODLIST_RES as libc::c_int
                {
                    if First == true {
                        /* extract number of indexes from mailbox data size */
                        n = (((*aSDOp).MbxHeader.length as libc::c_int - (6i32 + 2i32)) / 2i32)
                            as u16
                    } else {
                        /* extract number of indexes from mailbox data size */
                        n = (((*aSDOp).MbxHeader.length as libc::c_int - 6i32) / 2i32) as u16
                    }
                    /* check if indexes fit in buffer structure */
                    if sp + n > EC_MAXODLIST as u16 {
                        n = (EC_MAXODLIST + 1 - sp as usize) as u16; /* Too many entries for master buffer */
                        ecx_SDOinfoerror(context.as_mut().unwrap(), slave, 0u16, 0u8, 0xf000000i32);
                        stop = true
                    }
                    /* trim to maximum number of ODlist entries defined */
                    if (*pODlist).Entries + n > EC_MAXODLIST as u16 {
                        n = (EC_MAXODLIST - (*pODlist).Entries as usize) as u16
                    }
                    (*pODlist).Entries =
                        ((*pODlist).Entries as libc::c_int + n as libc::c_int) as u16;
                    /* extract indexes one by one */
                    i = 0u16;
                    while (i as libc::c_int) < n as libc::c_int {
                        (*pODlist).Index[(sp as libc::c_int + i as libc::c_int) as usize] =
                            (*aSDOp).data.wdata
                                [(i as libc::c_int + offset as libc::c_int) as usize];
                        i = i.wrapping_add(1)
                    }
                    sp = (sp as libc::c_int + n as libc::c_int) as u16;
                    /* check if more fragments will follow */
                    if (*aSDOp).Fragments as libc::c_int > 0i32 {
                        stop = false
                    }
                    First = false;
                    offset = 0u16
                } else {
                    /* got unexpected response from slave */
                    if (*aSDOp).Opcode as libc::c_int & 0x7fi32
                        == CoEObjectDescription::ECT_SDOINFO_ERROR as libc::c_int
                    {
                        /* SDO info error received */
                        ecx_SDOinfoerror(
                            context.as_mut().unwrap(),
                            slave,
                            0u16,
                            0u8,
                            (*aSDOp).data.ldata[0usize] as i32,
                        );
                        stop = true
                    } else {
                        ecx_packeterror(context.as_mut().unwrap(), slave, 0u16, 0u8, 1u16);
                        /* Unexpected frame returned */
                    }
                    wkc = 0i32;
                    x += 20;
                }
            }
            x = x.wrapping_add(1);
            if !(x <= 128 && stop == false) {
                break;
            }
        }
    }
    return wkc;
}
/* * CoE read Object Description. Adds textual description to object indexes.
 *
 * @param[in]  context       = context struct
 * @param[in] Item           = Item number in ODlist.
 * @param[in,out] pODlist    = referencing Object Description list.
 * @return Workcounter of slave response.
 */
#[no_mangle]
pub unsafe fn ecx_readODdescription(
    context: *mut ecx_contextt,
    Item: u16,
    mut pODlist: *mut ec_ODlistt,
) -> libc::c_int {
    let mut SDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut aSDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut wkc: libc::c_int = 0;
    let mut n: u16 = 0;
    let mut slave: u16 = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    slave = (*pODlist).Slave;
    (*pODlist).DataType[Item as usize] = 0u16;
    (*pODlist).ObjectCode[Item as usize] = 0u8;
    (*pODlist).MaxSub[Item as usize] = 0u8;
    (*pODlist).Name[Item as usize][0usize] = 0;
    ec_clearmbx(&mut MbxIn);
    /* clear pending out mailbox in slave if available. Timeout is set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn, 0);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOservicet;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOservicet;
    (*SDOp).MbxHeader.length = 0x8u16;
    (*SDOp).MbxHeader.address = 0u16;
    (*SDOp).MbxHeader.priority = 0u8;
    /* Get new mailbox counter value */
    cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* CoE */
    (*context).slavelist[slave as usize].mbx_cnt = cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8; /* get object description request */
    (*SDOp).CANOpen = (0i32 + ((CoEMailboxType::SdoInfo as libc::c_int) << 12i32)) as u16; /* fragments left */
    (*SDOp).Opcode = CoEObjectDescription::ECT_GET_OD_REQ as u8; /* Data of Index */
    (*SDOp).Reserved = 0u8;
    (*SDOp).Fragments = 0u16;
    (*SDOp).data.wdata[0usize] = (*pODlist).Index[Item as usize];
    /* send get object description request to slave */
    wkc = ecx_mbxsend(context, slave, &mut MbxOut, EC_TIMEOUTTXM);
    /* mailbox placed in slave ? */
    if wkc > 0i32 {
        ec_clearmbx(&mut MbxIn);
        /* read slave response */
        wkc = ecx_mbxreceive(context, slave, &mut MbxIn, EC_TIMEOUTRXM);
        /* got response ? */
        if wkc > 0i32 {
            if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32 == MailboxType::Coe as libc::c_int
                && (*aSDOp).Opcode as libc::c_int & 0x7fi32
                    == CoEObjectDescription::ECT_GET_OD_RES as libc::c_int
            {
                n = ((*aSDOp).MbxHeader.length as libc::c_int - 12i32) as u16;
                if n as libc::c_int > 40i32 {
                    n = 40u16 /* length of string(name of object) */
                    /* String terminator */
                    /* max chars */
                }
                (*pODlist).DataType[Item as usize] = (*aSDOp).data.wdata[1usize];
                (*pODlist).ObjectCode[Item as usize] = (*aSDOp).data.bdata[5usize];
                (*pODlist).MaxSub[Item as usize] = (*aSDOp).data.bdata[4usize];
                strncpy(
                    (*pODlist).Name[Item as usize].as_mut_ptr(),
                    &mut *(*aSDOp).data.bdata.as_mut_ptr().offset(6isize) as *mut u8
                        as *mut libc::c_char,
                    n as usize,
                );
                (*pODlist).Name[Item as usize][n as usize] = 0;
            } else {
                /* got unexpected response from slave */
                if (*aSDOp).Opcode as libc::c_int & 0x7fi32
                    == CoEObjectDescription::ECT_SDOINFO_ERROR as libc::c_int
                {
                    /* SDO info error received */
                    ecx_SDOinfoerror(
                        context.as_mut().unwrap(),
                        slave,
                        (*pODlist).Index[Item as usize],
                        0u8,
                        (*aSDOp).data.ldata[0usize] as i32,
                    );
                } else {
                    ecx_packeterror(
                        context.as_mut().unwrap(),
                        slave,
                        (*pODlist).Index[Item as usize],
                        0u8,
                        1u16,
                    );
                    /* Unexpected frame returned */
                }
                wkc = 0i32
            }
        }
    }
    return wkc;
}
/* * CoE read SDO service object entry, single subindex.
 * Used in ec_readOE().
 *
 * @param[in]  context       = context struct
 * @param[in] Item           = Item in ODlist.
 * @param[in] SubI           = Subindex of item in ODlist.
 * @param[in] pODlist        = Object description list for reference.
 * @param[out] pOElist       = resulting object entry structure.
 * @return Workcounter of slave response.
 */
#[no_mangle]
pub unsafe fn ecx_readOEsingle(
    context: *mut ecx_contextt,
    Item: u16,
    SubI: u8,
    pODlist: *mut ec_ODlistt,
    mut pOElist: *mut ec_OElistt,
) -> libc::c_int {
    let mut SDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut aSDOp: *mut ec_SDOservicet = 0 as *mut ec_SDOservicet;
    let mut wkc: libc::c_int = 0;
    let mut Index: u16 = 0;
    let mut slave: u16 = 0;
    let mut n: i16 = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: u8 = 0;
    wkc = 0i32;
    slave = (*pODlist).Slave;
    Index = (*pODlist).Index[Item as usize];
    ec_clearmbx(&mut MbxIn);
    /* clear pending out mailbox in slave if available. Timeout is set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn, 0);
    ec_clearmbx(&mut MbxOut);
    aSDOp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_SDOservicet;
    SDOp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_SDOservicet;
    (*SDOp).MbxHeader.length = 0xau16;
    (*SDOp).MbxHeader.address = 0u16;
    (*SDOp).MbxHeader.priority = 0u8;
    /* Get new mailbox counter value */
    cnt = ec_nextmbxcnt((*context).slavelist[slave as usize].mbx_cnt); /* CoE */
    (*context).slavelist[slave as usize].mbx_cnt = cnt; /* number 9bits service upper 4 bits */
    (*SDOp).MbxHeader.mbxtype = (MailboxType::Coe as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8; /* get object entry description request */
    (*SDOp).CANOpen = (0i32 + ((CoEMailboxType::SdoInfo as libc::c_int) << 12i32)) as u16; /* fragments left */
    (*SDOp).Opcode = CoEObjectDescription::ECT_GET_OE_REQ as u8; /* Index */
    (*SDOp).Reserved = 0u8; /* SubIndex */
    (*SDOp).Fragments = 0u16; /* get access rights, object category, PDO */
    (*SDOp).data.wdata[0usize] = Index;
    (*SDOp).data.bdata[2usize] = SubI;
    (*SDOp).data.bdata[3usize] = (1i32 + 2i32 + 4i32) as u8;
    /* send get object entry description request to slave */
    wkc = ecx_mbxsend(context, slave, &mut MbxOut, EC_TIMEOUTTXM);
    /* mailbox placed in slave ? */
    if wkc > 0i32 {
        ec_clearmbx(&mut MbxIn);
        /* read slave response */
        wkc = ecx_mbxreceive(context, slave, &mut MbxIn, EC_TIMEOUTRXM);
        /* got response ? */
        if wkc > 0i32 {
            if (*aSDOp).MbxHeader.mbxtype as libc::c_int & 0xfi32 == MailboxType::Coe as libc::c_int
                && (*aSDOp).Opcode as libc::c_int & 0x7fi32
                    == CoEObjectDescription::ECT_GET_OE_RES as libc::c_int
            {
                (*pOElist).Entries = (*pOElist).Entries.wrapping_add(1);
                /* string terminator */
                n = ((*aSDOp).MbxHeader.length as libc::c_int - 16i32) as i16; /* length of string(name of object) */
                if n as libc::c_int > 40i32 {
                    n = 40i16
                    /* max string length */
                }
                if (n as libc::c_int) < 0i32 {
                    n = 0i16
                }
                (*pOElist).ValueInfo[SubI as usize] = (*aSDOp).data.bdata[3usize];
                (*pOElist).DataType[SubI as usize] = (*aSDOp).data.wdata[2usize];
                (*pOElist).BitLength[SubI as usize] = (*aSDOp).data.wdata[3usize];
                (*pOElist).ObjAccess[SubI as usize] = (*aSDOp).data.wdata[4usize];
                strncpy(
                    (*pOElist).Name[SubI as usize].as_mut_ptr(),
                    &mut *(*aSDOp).data.wdata.as_mut_ptr().offset(5isize) as *mut u16
                        as *mut libc::c_char,
                    n as usize,
                );
                (*pOElist).Name[SubI as usize][n as usize] = 0;
            } else {
                /* got unexpected response from slave */
                if (*aSDOp).Opcode as libc::c_int & 0x7fi32
                    == CoEObjectDescription::ECT_SDOINFO_ERROR as libc::c_int
                {
                    /* SDO info error received */
                    ecx_SDOinfoerror(
                        context.as_mut().unwrap(),
                        slave,
                        Index,
                        SubI,
                        (*aSDOp).data.ldata[0usize] as i32,
                    );
                } else {
                    ecx_packeterror(context.as_mut().unwrap(), slave, Index, SubI, 1u16);
                    /* Unexpected frame returned */
                }
                wkc = 0i32
            }
        }
    }
    return wkc;
}
/* * CoE read SDO service object entry.
 *
 * @param[in] context        = context struct
 * @param[in] Item           = Item in ODlist.
 * @param[in] pODlist        = Object description list for reference.
 * @param[out] pOElist       = resulting object entry structure.
 * @return Workcounter of slave response.
 */
#[no_mangle]
pub unsafe fn ecx_readOE(
    context: *mut ecx_contextt,
    Item: u16,
    pODlist: *mut ec_ODlistt,
    mut pOElist: *mut ec_OElistt,
) -> libc::c_int {
    let mut SubCount: u16 = 0;
    let mut wkc: libc::c_int = 0;
    let mut SubI: u8 = 0;
    wkc = 0i32;
    (*pOElist).Entries = 0u16;
    SubI = (*pODlist).MaxSub[Item as usize];
    /* for each entry found in ODlist */
    SubCount = 0u16;
    while SubCount as libc::c_int <= SubI as libc::c_int {
        /* read subindex of entry */
        wkc = ecx_readOEsingle(context, Item, SubCount as u8, pODlist, pOElist);
        SubCount = SubCount.wrapping_add(1)
    }
    return wkc;
}
/* * Report SDO error.
 *
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index that generated error
 * @param[in]  SubIdx     = Subindex that generated error
 * @param[in]  AbortCode  = Abortcode, see EtherCAT documentation for list
 * @see ecx_SDOerror
 */
#[no_mangle]
pub unsafe fn ec_SDOerror(Slave: u16, Index: u16, SubIdx: u8, AbortCode: i32) {
    ecx_SDOerror(&mut ecx_context, Slave, Index, SubIdx, AbortCode);
}
/* * CoE SDO read, blocking. Single subindex or Complete Access.
 *
 * Only a "normal" upload request is issued. If the requested parameter is <= 4bytes
 * then a "expedited" response is returned, otherwise a "normal" response. If a "normal"
 * response is larger than the mailbox size then the response is segmented. The function
 * will combine all segments and copy them to the parameter buffer.
 *
 * @param[in]  slave      = Slave number
 * @param[in]  index      = Index to read
 * @param[in]  subindex   = Subindex to read, must be 0 or 1 if CA is used.
 * @param[in]  CA         = FALSE = single subindex. TRUE = Complete Access, all subindexes read.
 * @param[in,out] psize   = Size in bytes of parameter buffer, returns bytes read from SDO.
 * @param[out] p          = Pointer to parameter buffer
 * @param[in]  timeout    = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 * @see ecx_SDOread
 */
#[no_mangle]
pub unsafe fn ec_SDOread(
    slave: u16,
    index: u16,
    subindex: u8,
    CA: bool,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_SDOread(
        &mut ecx_context,
        slave,
        index,
        subindex,
        CA,
        psize,
        p,
        timeout,
    );
}
/* * CoE SDO write, blocking. Single subindex or Complete Access.
 *
 * A "normal" download request is issued, unless we have
 * small data, then a "expedited" transfer is used. If the parameter is larger than
 * the mailbox size then the download is segmented. The function will split the
 * parameter data in segments and send them to the slave one by one.
 *
 * @param[in]  Slave      = Slave number
 * @param[in]  Index      = Index to write
 * @param[in]  SubIndex   = Subindex to write, must be 0 or 1 if CA is used.
 * @param[in]  CA         = FALSE = single subindex. TRUE = Complete Access, all subindexes written.
 * @param[in]  psize      = Size in bytes of parameter buffer.
 * @param[out] p          = Pointer to parameter buffer
 * @param[in]  Timeout    = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 * @see ecx_SDOwrite
 */
#[no_mangle]
pub unsafe fn ec_SDOwrite(
    Slave: u16,
    Index: u16,
    SubIndex: u8,
    CA: bool,
    psize: libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_SDOwrite(
        &mut ecx_context,
        Slave,
        Index,
        SubIndex,
        CA,
        psize,
        p,
        timeout,
    );
}
/* * CoE RxPDO write, blocking.
 *
 * A RxPDO download request is issued.
 *
 * @param[in]  Slave         = Slave number
 * @param[in]  RxPDOnumber   = Related RxPDO number
 * @param[in]  psize         = Size in bytes of PDO buffer.
 * @param[out] p             = Pointer to PDO buffer
 * @return Workcounter from last slave response
 * @see ecx_RxPDO
 */
#[no_mangle]
pub unsafe fn ec_RxPDO(
    Slave: u16,
    RxPDOnumber: u16,
    psize: libc::c_int,
    p: *mut libc::c_void,
) -> libc::c_int {
    return ecx_RxPDO(&mut ecx_context, Slave, RxPDOnumber, psize, p);
}
/* * CoE TxPDO read remote request, blocking.
 *
 * A RxPDO download request is issued.
 *
 * @param[in]  slave         = Slave number
 * @param[in]  TxPDOnumber   = Related TxPDO number
 * @param[in,out] psize      = Size in bytes of PDO buffer, returns bytes read from PDO.
 * @param[out] p             = Pointer to PDO buffer
 * @param[in]  timeout       = Timeout in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 * @see ecx_TxPDO
 */
#[no_mangle]
pub unsafe fn ec_TxPDO(
    slave: u16,
    TxPDOnumber: u16,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_TxPDO(&mut ecx_context, slave, TxPDOnumber, psize, p, timeout);
}
/* * Read PDO assign structure
 * @param[in]  Slave         = Slave number
 * @param[in]  PDOassign     = PDO assign object
 * @return total bitlength of PDO assign
 */
#[no_mangle]
pub unsafe fn ec_readPDOassign(Slave: u16, PDOassign: u16) -> u32 {
    return ecx_readPDOassign(&mut ecx_context, Slave, PDOassign);
}
/* * Read PDO assign structure in Complete Access mode
 * @param[in]  Slave         = Slave number
 * @param[in]  PDOassign     = PDO assign object
 * @param[in]  Thread_n      = Calling thread index
 * @return total bitlength of PDO assign
 * @see ecx_readPDOmap
 */
#[no_mangle]
pub unsafe fn ec_readPDOassignCA(Slave: u16, PDOassign: u16, Thread_n: libc::c_int) -> u32 {
    return ecx_readPDOassignCA(&mut ecx_context, Slave, Thread_n, PDOassign);
}
/* * CoE read PDO mapping.
 *
 * CANopen has standard indexes defined for PDO mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave.
 *
 * For details, see #ecx_readPDOmap
 *
 * @param[in] Slave    = Slave number
 * @param[out] Osize   = Size in bits of output mapping (rxPDO) found
 * @param[out] Isize   = Size in bits of input mapping (txPDO) found
 * @return >0 if mapping succesful.
 */
#[no_mangle]
pub unsafe fn ec_readPDOmap(Slave: u16, Osize: *mut u32, Isize: *mut u32) -> libc::c_int {
    return ecx_readPDOmap(&mut ecx_context, Slave, Osize, Isize);
}
/* * CoE read PDO mapping in Complete Access mode (CA).
 *
 * CANopen has standard indexes defined for PDO mapping. This function
 * tries to read them and collect a full input and output mapping size
 * of designated slave. Slave has to support CA, otherwise use ec_readPDOmap().
 *
 * @param[in] Slave    = Slave number
 * @param[in] Thread_n = Calling thread index
 * @param[out] Osize   = Size in bits of output mapping (rxPDO) found
 * @param[out] Isize   = Size in bits of input mapping (txPDO) found
 * @return >0 if mapping succesful.
 * @see ecx_readPDOmap ec_readPDOmapCA
 */
#[no_mangle]
pub unsafe fn ec_readPDOmapCA(
    Slave: u16,
    Thread_n: libc::c_int,
    Osize: *mut u32,
    Isize: *mut u32,
) -> libc::c_int {
    return ecx_readPDOmapCA(&mut ecx_context, Slave, Thread_n, Osize, Isize);
}
/* * CoE read Object Description List.
 *
 * @param[in] Slave      = Slave number.
 * @param[out] pODlist  = resulting Object Description list.
 * @return Workcounter of slave response.
 * @see ecx_readODlist
 */
#[no_mangle]
pub unsafe fn ec_readODlist(Slave: u16, pODlist: *mut ec_ODlistt) -> libc::c_int {
    return ecx_readODlist(&mut ecx_context, Slave, pODlist);
}
/* * CoE read Object Description. Adds textual description to object indexes.
 *
 * @param[in] Item           = Item number in ODlist.
 * @param[in,out] pODlist    = referencing Object Description list.
 * @return Workcounter of slave response.
 * @see ecx_readODdescription
 */
#[no_mangle]
pub unsafe fn ec_readODdescription(Item: u16, pODlist: *mut ec_ODlistt) -> libc::c_int {
    return ecx_readODdescription(&mut ecx_context, Item, pODlist);
}
#[no_mangle]
pub unsafe fn ec_readOEsingle(
    Item: u16,
    SubI: u8,
    pODlist: *mut ec_ODlistt,
    pOElist: *mut ec_OElistt,
) -> libc::c_int {
    return ecx_readOEsingle(&mut ecx_context, Item, SubI, pODlist, pOElist);
}
/* * CoE read SDO service object entry.
 *
 * @param[in] Item           = Item in ODlist.
 * @param[in] pODlist        = Object description list for reference.
 * @param[out] pOElist       = resulting object entry structure.
 * @return Workcounter of slave response.
 * @see ecx_readOE
 */
#[no_mangle]
pub unsafe fn ec_readOE(
    Item: u16,
    pODlist: *mut ec_ODlistt,
    pOElist: *mut ec_OElistt,
) -> libc::c_int {
    return ecx_readOE(&mut ecx_context, Item, pODlist, pOElist);
}
