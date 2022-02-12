use crate::{
    ethercatmain::{
        ec_clearmbx, ec_mbxbuft, ec_mbxheadert, ec_nextmbxcnt, ecx_contextt, ecx_mbxreceive,
        ecx_mbxsend,
    },
    ethercattype::{ec_err_type, MailboxType, EC_TIMEOUTTXM},
};
use c2rust_bitfields;
use libc::memcpy;

pub const EOE_PARAM_OFFSET: u8 = 4;

#[derive(Copy, Clone)]
pub struct eoe_ip4_addr {
    pub addr: u32,
}
pub type eoe_ip4_addr_t = eoe_ip4_addr;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct eoe_ethaddr {
    pub addr: [u8; 6],
}
pub type eoe_ethaddr_t = eoe_ethaddr;

#[derive(Copy, Clone, BitfieldStruct)]
pub struct eoe_param {
    #[bitfield(name = "mac_set", ty = "u8", bits = "0..=0")]
    #[bitfield(name = "ip_set", ty = "u8", bits = "1..=1")]
    #[bitfield(name = "subnet_set", ty = "u8", bits = "2..=2")]
    #[bitfield(name = "default_gateway_set", ty = "u8", bits = "3..=3")]
    #[bitfield(name = "dns_ip_set", ty = "u8", bits = "4..=4")]
    #[bitfield(name = "dns_name_set", ty = "u8", bits = "5..=5")]
    pub mac_set_ip_set_subnet_set_default_gateway_set_dns_ip_set_dns_name_set: [u8; 1],
    pub mac: eoe_ethaddr_t,
    pub ip: eoe_ip4_addr_t,
    pub subnet: eoe_ip4_addr_t,
    pub default_gateway: eoe_ip4_addr_t,
    pub dns_ip: eoe_ip4_addr_t,
    pub dns_name: [libc::c_char; 32],
}
pub type eoe_param_t = eoe_param;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_EOEt {
    pub mbxheader: ec_mbxheadert,
    pub frameinfo1: u16,
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub data: [u8; 1476],
}

#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
    pub frameinfo2: u16,
    pub result: u16,
}
/*
 * Licensed under the GNU General Public License version 2 with exceptions. See
 * LICENSE file in the project root for full license information
 */
/* * \file
 * \brief
 * Ethernet over EtherCAT (EoE) module.
 *
 * Set / Get IP functions
 * Blocking send/receive Ethernet Frame
 * Read incoming EoE fragment to Ethernet Frame
 */
/* * EoE utility function to convert u32 to eoe ip bytes.
 * @param[in] ip       = ip in u32
 * @param[out] byte_ip = eoe ip 4th octet, 3ed octet, 2nd octet, 1st octet
 */
unsafe fn EOE_ip_uint32_to_byte(ip: *mut eoe_ip4_addr_t, byte_ip: *mut u8) {
    *byte_ip.offset(3isize) = *(&mut (*ip).addr as *mut u32 as *const u8).offset(0isize); /* 1st octet */
    *byte_ip.offset(2isize) = *(&mut (*ip).addr as *mut u32 as *const u8).offset(1isize); /* 2nd octet */
    *byte_ip.offset(1isize) = *(&mut (*ip).addr as *mut u32 as *const u8).offset(2isize); /* 3ed octet */
    *byte_ip.offset(0isize) = *(&mut (*ip).addr as *mut u32 as *const u8).offset(3isize);
    /* 4th octet */
}
/* * EoE utility function to convert eoe ip bytes to u32.
* @param[in] byte_ip = eoe ip 4th octet, 3ed octet, 2nd octet, 1st octet
* @param[out] ip     = ip in u32
*/
unsafe fn EOE_ip_byte_to_uint32(byte_ip: *mut u8, mut ip: *mut eoe_ip4_addr_t) {
    (*ip).addr = (((((*byte_ip.offset(3isize) as libc::c_int & 0xffi32) as u32) << 24i32
        | ((*byte_ip.offset(2isize) as libc::c_int & 0xffi32) as u32) << 16i32
        | ((*byte_ip.offset(1isize) as libc::c_int & 0xffi32) as u32) << 8i32
        | (*byte_ip.offset(0isize) as libc::c_int & 0xffi32) as u32)
        as libc::c_ulong
        & 0xffu64)
        << 24i32
        | ((((*byte_ip.offset(3isize) as libc::c_int & 0xffi32) as u32) << 24i32
            | ((*byte_ip.offset(2isize) as libc::c_int & 0xffi32) as u32) << 16i32
            | ((*byte_ip.offset(1isize) as libc::c_int & 0xffi32) as u32) << 8i32
            | (*byte_ip.offset(0isize) as libc::c_int & 0xffi32) as u32)
            as libc::c_ulong
            & 0xff00u64)
            << 8i32
        | ((((*byte_ip.offset(3isize) as libc::c_int & 0xffi32) as u32) << 24i32
            | ((*byte_ip.offset(2isize) as libc::c_int & 0xffi32) as u32) << 16i32
            | ((*byte_ip.offset(1isize) as libc::c_int & 0xffi32) as u32) << 8i32
            | (*byte_ip.offset(0isize) as libc::c_int & 0xffi32) as u32)
            as libc::c_ulong
            & 0xff0000u64)
            >> 8i32
        | ((((*byte_ip.offset(3isize) as libc::c_int & 0xffi32) as u32) << 24i32
            | ((*byte_ip.offset(2isize) as libc::c_int & 0xffi32) as u32) << 16i32
            | ((*byte_ip.offset(1isize) as libc::c_int & 0xffi32) as u32) << 8i32
            | (*byte_ip.offset(0isize) as libc::c_int & 0xffi32) as u32)
            as libc::c_ulong
            & 0xff000000u64)
            >> 24i32) as u32;
    /* 4th octet */
}
/* * EoE fragment data handler hook. Should not block.
*
* @param[in]  context = context struct
* @param[in]  hook    = Pointer to hook function.
* @return 1
*/
#[no_mangle]
pub unsafe fn ecx_EOEdefinehook(
    mut context: *mut ecx_contextt,
    hook: *mut libc::c_void,
) -> libc::c_int {
    (*context).EOEhook = ::core::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe fn(_: *mut ecx_contextt, _: u16, _: *mut libc::c_void) -> libc::c_int>,
    >(hook);
    return 1i32;
}
/* * EoE EOE set IP, blocking. Waits for response from the slave.
*
* @param[in]  context    = Context struct
* @param[in]  slave      = Slave number
* @param[in]  port       = Port number on slave if applicable
* @param[in]  ipparam    = IP parameter data to be sent
* @param[in]  timeout    = Timeout in us, standard is EC_TIMEOUTRXM
* @return Workcounter from last slave response or returned result code
*/
#[no_mangle]
pub unsafe fn ecx_EOEsetIp(
    context: *mut ecx_contextt,
    slave: u16,
    port: u8,
    ipparam: *mut eoe_param_t,
    timeout: u32,
) -> libc::c_int {
    let mut EOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut aEOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut frameinfo1: u16 = 0;
    let mut result: u16 = 0;
    let mut cnt: u8 = 0;
    let mut data_offset: u8 = 0;
    let mut flags: u8 = 0u8;
    let mut wkc: libc::c_int = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0);
    ec_clearmbx(&mut MbxOut);
    aEOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_EOEt;
    EOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_EOEt;
    (*EOEp).mbxheader.address = 0u16;
    (*EOEp).mbxheader.priority = 0u8;
    data_offset = EOE_PARAM_OFFSET;
    // FIXME: Lots of magic constants below
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* EoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*EOEp).mbxheader.mbxtype = (MailboxType::ECT_MBXT_EOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8;
    (*EOEp).frameinfo1 = ((2i32 & 0xfi32) << 0i32
        | ((port as libc::c_int & 0xfi32) << 4i32) as u16 as libc::c_int
        | (0x1i32) << 8i32) as u16;
    (*EOEp).c2rust_unnamed.frameinfo2 = 0u16;
    if (*ipparam).mac_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 0i32) as u8;
        memcpy(
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize) as *mut u8
                as *mut libc::c_void,
            (*ipparam).mac.addr.as_mut_ptr() as *const libc::c_void,
            6usize,
        );
        data_offset = (data_offset as libc::c_int + 6i32) as u8
    }
    if (*ipparam).ip_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 1i32) as u8;
        EOE_ip_uint32_to_byte(
            &mut (*ipparam).ip,
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize),
        );
        data_offset =
            (data_offset as usize).wrapping_add(core::mem::size_of::<u32>() as usize) as u8
    }
    if (*ipparam).subnet_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 2i32) as u8;
        EOE_ip_uint32_to_byte(
            &mut (*ipparam).subnet,
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize),
        );
        data_offset =
            (data_offset as usize).wrapping_add(core::mem::size_of::<u32>() as usize) as u8
    }
    if (*ipparam).default_gateway_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 3i32) as u8;
        EOE_ip_uint32_to_byte(
            &mut (*ipparam).default_gateway,
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize),
        );
        data_offset =
            (data_offset as usize).wrapping_add(core::mem::size_of::<u32>() as usize) as u8
    }
    if (*ipparam).dns_ip_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 4i32) as u8;
        EOE_ip_uint32_to_byte(
            &mut (*ipparam).dns_ip,
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize),
        );
        data_offset =
            (data_offset as usize).wrapping_add(core::mem::size_of::<u32>() as usize) as u8
    }
    if (*ipparam).dns_name_set() != 0 {
        /* TwinCAT include EOE_DNS_NAME_LENGTH chars even if name is shorter */
        flags = (flags as libc::c_int | (0x1i32) << 5i32) as u8;
        memcpy(
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize) as *mut u8
                as *mut libc::c_void,
            (*ipparam).dns_name.as_mut_ptr() as *mut libc::c_void,
            32usize,
        );
        data_offset = (data_offset as libc::c_int + 32i32) as u8
    }
    (*EOEp).mbxheader.length = (4i32 + data_offset as libc::c_int) as u16;
    (*EOEp).data[0usize] = flags;
    /* send EoE request to slave */
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
            /* slave response should be FoE */
            if (*aEOEp).mbxheader.mbxtype as libc::c_int & 0xfi32
                == MailboxType::ECT_MBXT_EOE as libc::c_int
            {
                frameinfo1 = (*aEOEp).frameinfo1;
                result = (*aEOEp).c2rust_unnamed.result;
                if frameinfo1 as libc::c_int >> 0i32 & 0xfi32 != 3i32
                    || result as libc::c_int != 0i32
                {
                    wkc = -(result as libc::c_int)
                }
            } else {
                /* unexpected mailbox received */
                wkc = -(ec_err_type::EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
            }
        }
    }
    return wkc;
}
/* * EoE EOE get IP, blocking. Waits for response from the slave.
*
* @param[in]  context    = Context struct
* @param[in]  slave      = Slave number
* @param[in]  port       = Port number on slave if applicable
* @param[out] ipparam    = IP parameter data retrived from slave
* @param[in]  timeout    = Timeout in us, standard is EC_TIMEOUTRXM
* @return Workcounter from last slave response or returned result code
*/
#[no_mangle]
pub unsafe fn ecx_EOEgetIp(
    context: *mut ecx_contextt,
    slave: u16,
    port: u8,
    ipparam: *mut eoe_param_t,
    timeout: u32,
) -> libc::c_int {
    let mut EOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut aEOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut frameinfo1: u16 = 0;
    let mut eoedatasize: u16 = 0;
    let mut cnt: u8 = 0;
    let mut data_offset: u8 = 0;
    let mut flags: u8 = 0u8;
    let mut wkc: libc::c_int = 0;
    /* Empty slave out mailbox if something is in. Timout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0);
    ec_clearmbx(&mut MbxOut);
    aEOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_EOEt;
    EOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_EOEt;
    (*EOEp).mbxheader.address = 0u16;
    (*EOEp).mbxheader.priority = 0u8;
    data_offset = 4u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* EoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*EOEp).mbxheader.mbxtype = (MailboxType::ECT_MBXT_EOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
        as u8;
    (*EOEp).frameinfo1 = ((6i32 & 0xfi32) << 0i32
        | ((port as libc::c_int & 0xfi32) << 4i32) as u16 as libc::c_int
        | (0x1i32) << 8i32) as u16;
    (*EOEp).c2rust_unnamed.frameinfo2 = 0u16;
    (*EOEp).mbxheader.length = 0x4u16;
    (*EOEp).data[0usize] = flags;
    /* send EoE request to slave */
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
            /* slave response should be FoE */
            if (*aEOEp).mbxheader.mbxtype as libc::c_int & 0xfi32
                == MailboxType::ECT_MBXT_EOE as libc::c_int
            {
                frameinfo1 = (*aEOEp).frameinfo1;
                eoedatasize = ((*aEOEp).mbxheader.length as libc::c_int - 0x4i32) as u16;
                if frameinfo1 as libc::c_int >> 0i32 & 0xfi32 != 7i32 {
                    wkc = -(0x2i32)
                } else {
                    flags = (*aEOEp).data[0usize];
                    if flags as libc::c_int & (0x1i32) << 0i32 != 0 {
                        memcpy(
                            (*ipparam).mac.addr.as_mut_ptr() as *mut libc::c_void,
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize) as *mut u8
                                as *const libc::c_void,
                            6usize,
                        );
                        (*ipparam).set_mac_set(1u8);
                        data_offset = (data_offset as libc::c_int + 6i32) as u8
                    }
                    if flags as libc::c_int & (0x1i32) << 1i32 != 0 {
                        EOE_ip_byte_to_uint32(
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize),
                            &mut (*ipparam).ip,
                        );
                        (*ipparam).set_ip_set(1u8);
                        data_offset = (data_offset as usize)
                            .wrapping_add(core::mem::size_of::<u32>() as usize)
                            as u8
                    }
                    if flags as libc::c_int & (0x1i32) << 2i32 != 0 {
                        EOE_ip_byte_to_uint32(
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize),
                            &mut (*ipparam).subnet,
                        );
                        (*ipparam).set_subnet_set(1u8);
                        data_offset = (data_offset as usize)
                            .wrapping_add(core::mem::size_of::<u32>() as usize)
                            as u8
                    }
                    if flags as libc::c_int & (0x1i32) << 3i32 != 0 {
                        EOE_ip_byte_to_uint32(
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize),
                            &mut (*ipparam).default_gateway,
                        );
                        (*ipparam).set_default_gateway_set(1u8);
                        data_offset = (data_offset as usize)
                            .wrapping_add(core::mem::size_of::<u32>() as usize)
                            as u8
                    }
                    if flags as libc::c_int & (0x1i32) << 4i32 != 0 {
                        EOE_ip_byte_to_uint32(
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize),
                            &mut (*ipparam).dns_ip,
                        );
                        (*ipparam).set_dns_ip_set(1u8);
                        data_offset = (data_offset as usize)
                            .wrapping_add(core::mem::size_of::<u32>() as usize)
                            as u8
                    }
                    if flags as libc::c_int & (0x1i32) << 5i32 != 0 {
                        let mut dns_len: u16 = 0;
                        if (eoedatasize as libc::c_int - data_offset as libc::c_int) < 32i32 {
                            dns_len =
                                (eoedatasize as libc::c_int - data_offset as libc::c_int) as u16
                        } else {
                            dns_len = 32u16
                        }
                        /* Assume ZERO terminated string */
                        memcpy(
                            (*ipparam).dns_name.as_mut_ptr() as *mut libc::c_void,
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize) as *mut u8
                                as *const libc::c_void,
                            dns_len as usize,
                        );
                        (*ipparam).set_dns_name_set(1u8);
                        data_offset = (data_offset as libc::c_int + 32i32) as u8
                    }
                    /* Something os not correct, flag the error */
                    if data_offset as libc::c_int > eoedatasize as libc::c_int {
                        wkc = -(ec_err_type::EC_ERR_TYPE_MBX_ERROR as libc::c_int)
                    }
                }
            } else {
                /* unexpected mailbox received */
                wkc = -(ec_err_type::EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
            }
        }
    }
    return wkc;
}
/* * EoE ethernet buffer write, blocking.
*
* If the buffer is larger than the mailbox size then the buffer is sent in
* several fragments. The function will split the buf data in fragments and
* send them to the slave one by one.
*
* @param[in]  context    = context struct
* @param[in]  slave      = Slave number
* @param[in]  port       = Port number on slave if applicable
* @param[in]  psize      = Size in bytes of parameter buffer.
* @param[in]  p          = Pointer to parameter buffer
* @param[in]  timeout    = Timeout in us, standard is EC_TIMEOUTRXM
* @return Workcounter from last slave transmission
*/
#[no_mangle]
pub unsafe fn ecx_EOEsend(
    context: *mut ecx_contextt,
    slave: u16,
    port: u8,
    psize: libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut EOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut frameinfo1: u16 = 0;
    let mut frameinfo2: u16 = 0;
    let mut cnt: u8 = 0;
    let mut txfragmentno: u8 = 0;
    let mut NotLast: bool = false;
    let mut wkc: libc::c_int = 0;
    let mut maxdata: libc::c_int = 0;
    let mut txframesize: libc::c_int = 0;
    let mut txframeoffset: libc::c_int = 0;
    let buf: *const u8 = p as *const u8;
    let mut txframeno: u8 = 0u8;
    ec_clearmbx(&mut MbxOut);
    EOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_EOEt;
    (*EOEp).mbxheader.address = 0u16;
    (*EOEp).mbxheader.priority = 0u8;
    /* data section=mailbox size - 6 mbx - 4 EoEh */
    maxdata = (*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int - 0xai32;
    txframesize = psize;
    txfragmentno = 0u8;
    txframeoffset = 0i32;
    NotLast = true;
    loop {
        txframesize = psize - txframeoffset;
        if txframesize > maxdata {
            /* Adjust to even 32-octect blocks */
            txframesize = (maxdata >> 5i32) << 5i32
        }
        if txframesize == psize - txframeoffset {
            frameinfo1 = ((1i32 & 0x1i32) << 8i32
                | ((port as libc::c_int & 0xfi32) << 4i32) as u16 as libc::c_int)
                as u16;
            NotLast = false
        } else {
            frameinfo1 = ((port as libc::c_int & 0xfi32) << 4i32) as u16
        }
        frameinfo2 = ((txfragmentno as libc::c_int & 0x3fi32) << 0i32) as u16;
        if txfragmentno as libc::c_int > 0i32 {
            frameinfo2 = (frameinfo2 as libc::c_int
                | ((txframeoffset >> 5i32 & 0x3fi32) << 6i32) as u16 as libc::c_int)
                as u16
        } else {
            frameinfo2 = (frameinfo2 as libc::c_int
                | ((psize + 31i32 >> 5i32 & 0x3fi32) << 6i32) as u16 as libc::c_int)
                as u16;
            txframeno = txframeno.wrapping_add(1)
        }
        frameinfo2 = (frameinfo2 as libc::c_int
            | ((txframeno as libc::c_int & 0xfi32) << 12i32) as u16 as libc::c_int)
            as u16;
        /* get new mailbox count value, used as session handle */
        cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* no timestamp */
        (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt; /* EoE */
        (*EOEp).mbxheader.length = (4i32 + txframesize) as u16;
        (*EOEp).mbxheader.mbxtype = (MailboxType::ECT_MBXT_EOE as libc::c_int
            + ((cnt as libc::c_int) << 4i32) as u8 as libc::c_int)
            as u8;
        (*EOEp).frameinfo1 = frameinfo1;
        (*EOEp).c2rust_unnamed.frameinfo2 = frameinfo2;
        memcpy(
            (*EOEp).data.as_mut_ptr() as *mut libc::c_void,
            &*buf.offset(txframeoffset as isize) as *const u8 as *const libc::c_void,
            txframesize as usize,
        );
        /* send EoE request to slave */
        wkc = ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft, timeout);
        if NotLast as libc::c_int == 1i32 && wkc > 0i32 {
            txframeoffset += txframesize;
            txfragmentno = txfragmentno.wrapping_add(1)
        }
        if !(NotLast as libc::c_int == 1i32 && wkc > 0i32) {
            break;
        }
    }
    return wkc;
}
/* * EoE ethernet buffer read, blocking.
*
* If the buffer is larger than the mailbox size then the buffer is received
* by several fragments. The function will assamble the fragments into
* a complete Ethernet buffer.
*
* @param[in]     context = context struct
* @param[in]     slave   = Slave number
* @param[in]     port    = Port number on slave if applicable
* @param[in,out] psize   = Size in bytes of parameter buffer.
* @param[in]     p       = Pointer to parameter buffer
* @param[in]     timeout = Timeout in us, standard is EC_TIMEOUTRXM
* @return Workcounter from last slave response or error code
*/
#[no_mangle]
pub unsafe fn ecx_EOErecv(
    context: *mut ecx_contextt,
    slave: u16,
    port: u8,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut aEOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut frameinfo1: u16 = 0;
    let mut frameinfo2: u16 = 0;
    let mut rxfragmentno: u8 = 0;
    let mut rxframeno: u8 = 0;
    let mut NotLast: bool = false;
    let mut wkc: libc::c_int = 0;
    let mut buffersize: libc::c_int = 0;
    let mut rxframesize: libc::c_int = 0;
    let mut rxframeoffset: libc::c_int = 0;
    let mut eoedatasize: libc::c_int = 0;
    let buf: *mut u8 = p as *mut u8;
    ec_clearmbx(&mut MbxIn);
    aEOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_EOEt;
    NotLast = true;
    buffersize = *psize;
    rxfragmentno = 0u8;
    /* Hang for a while if nothing is in */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
    while wkc > 0i32 && NotLast as libc::c_int == 1i32 {
        /* slave response should be FoE */
        if (*aEOEp).mbxheader.mbxtype as libc::c_int & 0xfi32
            == MailboxType::ECT_MBXT_EOE as libc::c_int
        {
            eoedatasize = (*aEOEp).mbxheader.length as libc::c_int - 0x4i32;
            frameinfo1 = (*aEOEp).frameinfo1;
            frameinfo2 = (*aEOEp).c2rust_unnamed.frameinfo2;
            if rxfragmentno as libc::c_int != frameinfo2 as libc::c_int >> 0i32 & 0x3fi32 {
                if frameinfo2 as libc::c_int >> 0i32 & 0x3fi32 > 0i32 {
                    wkc = -(ec_err_type::EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                    /* Exit here*/
                    break;
                }
            }
            if rxfragmentno as libc::c_int == 0i32 {
                rxframeoffset = 0i32;
                rxframeno = (frameinfo2 as libc::c_int >> 12i32 & 0xfi32) as u8;
                rxframesize = (frameinfo2 as libc::c_int >> 6i32 & 0x3fi32) << 5i32;
                if rxframesize > buffersize {
                    wkc = -(ec_err_type::EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                    /* Exit here*/
                    break;
                } else if port as libc::c_int != frameinfo1 as libc::c_int >> 4i32 & 0xfi32 {
                    wkc = -(ec_err_type::EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                    /* Exit here*/
                    break;
                }
            } else if rxframeno as libc::c_int != frameinfo2 as libc::c_int >> 12i32 & 0xfi32 {
                wkc = -(ec_err_type::EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                /* Exit here*/
                break;
            } else if rxframeoffset != (frameinfo2 as libc::c_int >> 6i32 & 0x3fi32) << 5i32 {
                wkc = -(ec_err_type::EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                break;
            }
            if rxframeoffset + eoedatasize <= buffersize {
                memcpy(
                    &mut *buf.offset(rxframeoffset as isize) as *mut u8 as *mut libc::c_void,
                    (*aEOEp).data.as_mut_ptr() as *const libc::c_void,
                    eoedatasize as usize,
                );
                rxframeoffset += eoedatasize;
                rxfragmentno = rxfragmentno.wrapping_add(1)
            }
            if frameinfo1 as libc::c_int >> 8i32 & 0x1i32 != 0 {
                /* Remove timestamp */
                if frameinfo1 as libc::c_int >> 9i32 & 0x1i32 != 0 {
                    rxframeoffset -= 4i32
                }
                NotLast = false;
                *psize = rxframeoffset
            } else {
                /* Hang for a while if nothing is in */
                wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout)
            }
        } else {
            /* unexpected mailbox received */
            wkc = -(ec_err_type::EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
        }
    }
    return wkc;
}
/* * EoE mailbox fragment read
*
* Will take the data in incoming mailbox buffer and copy to destination
* Ethernet frame buffer at given offset and update current fragment variables
*
* @param[in] MbxIn             = Received mailbox containing fragment data
* @param[in,out] rxfragmentno  = Fragment number
* @param[in,out] rxframesize   = Frame size
* @param[in,out] rxframeoffset = Frame offset
* @param[in,out] rxframeno     = Frame number
* @param[in,out] psize         = Size in bytes of frame buffer.
* @param[out] p                = Pointer to frame buffer
* @return 0= if fragment OK, >0 if last fragment, <0 on error
*/
#[no_mangle]
pub unsafe fn ecx_EOEreadfragment(
    MbxIn: *mut ec_mbxbuft,
    rxfragmentno: *mut u8,
    rxframesize: *mut u16,
    rxframeoffset: *mut u16,
    rxframeno: *mut u16,
    psize: *mut libc::c_int,
    p: *mut libc::c_void,
) -> libc::c_int {
    let mut frameinfo1: u16 = 0;
    let mut frameinfo2: u16 = 0;
    let mut eoedatasize: u16 = 0;
    let mut wkc: libc::c_int = 0;
    let mut aEOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut buf: *mut u8 = 0 as *mut u8;
    aEOEp = MbxIn as *mut ec_EOEt;
    buf = p as *mut u8;
    wkc = 0i32;
    /* slave response should be EoE */
    if (*aEOEp).mbxheader.mbxtype as libc::c_int & 0xfi32
        == MailboxType::ECT_MBXT_EOE as libc::c_int
    {
        eoedatasize = ((*aEOEp).mbxheader.length as libc::c_int - 0x4i32) as u16;
        frameinfo1 = (*aEOEp).frameinfo1;
        frameinfo2 = (*aEOEp).c2rust_unnamed.frameinfo2;
        /* Retrive fragment number, is it what we expect? */
        if *rxfragmentno as libc::c_int != frameinfo2 as libc::c_int >> 0i32 & 0x3fi32 {
            /* If expected fragment number is not 0, reset working variables */
            if *rxfragmentno as libc::c_int != 0i32 {
                *rxfragmentno = 0u8;
                *rxframesize = 0u16;
                *rxframeoffset = 0u16;
                *rxframeno = 0u16
            }
            /* If incoming fragment number is not 0 we can't recover, exit */
            if frameinfo2 as libc::c_int >> 0i32 & 0x3fi32 > 0i32 {
                wkc = -(ec_err_type::EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                return wkc;
            }
        }
        /* Is it a new frame?*/
        if *rxfragmentno as libc::c_int == 0i32 {
            *rxframesize = ((frameinfo2 as libc::c_int >> 6i32 & 0x3fi32) << 5i32) as u16;
            *rxframeoffset = 0u16;
            *rxframeno = (frameinfo2 as libc::c_int >> 12i32 & 0xfi32) as u16
        } else if *rxframeno as libc::c_int != frameinfo2 as libc::c_int >> 12i32 & 0xfi32 {
            *rxfragmentno = 0u8;
            *rxframesize = 0u16;
            *rxframeoffset = 0u16;
            *rxframeno = 0u16;
            wkc = -(ec_err_type::EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
            return wkc;
        } else {
            if *rxframeoffset as libc::c_int
                != (frameinfo2 as libc::c_int >> 6i32 & 0x3fi32) << 5i32
            {
                *rxfragmentno = 0u8;
                *rxframesize = 0u16;
                *rxframeoffset = 0u16;
                *rxframeno = 0u16;
                wkc = -(ec_err_type::EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                return wkc;
            }
        }
        /* If we're inside a frame, make sure it is the same */
        /* Make sure we're inside expected frame size */
        if *rxframeoffset as libc::c_int + eoedatasize as libc::c_int <= *rxframesize as libc::c_int
            && *rxframeoffset as libc::c_int + eoedatasize as libc::c_int <= *psize
        {
            memcpy(
                &mut *buf.offset(*rxframeoffset as isize) as *mut u8 as *mut libc::c_void,
                (*aEOEp).data.as_mut_ptr() as *const libc::c_void,
                eoedatasize as usize,
            );
            *rxframeoffset = (*rxframeoffset as libc::c_int + eoedatasize as libc::c_int) as u16;
            *rxfragmentno = (*rxfragmentno as libc::c_int + 1i32) as u8
        }
        /* Is it the last fragment */
        if frameinfo1 as libc::c_int >> 8i32 & 0x1i32 != 0 {
            /* Remove timestamp */
            if frameinfo1 as libc::c_int >> 9i32 & 0x1i32 != 0 {
                *rxframeoffset = (*rxframeoffset as libc::c_int - 4i32) as u16
            }
            *psize = *rxframeoffset as libc::c_int;
            *rxfragmentno = 0u8;
            *rxframesize = 0u16;
            *rxframeoffset = 0u16;
            *rxframeno = 0u16;
            wkc = 1i32
        }
    } else {
        /* unexpected mailbox received */
        wkc = -(ec_err_type::EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
    }
    return wkc;
}
