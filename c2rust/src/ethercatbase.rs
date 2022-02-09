use crate::{
    ethercatmain::ecx_port,
    ethercattype::{ec_bufT, ec_bufstate, ec_cmdtype, ec_comt, ec_etherheadert, EthercatRegister},
    oshw::linux::nicdrv::{ecx_getindex, ecx_portt, ecx_setbufstat, ecx_srconfirm},
};
use libc::{memcpy, memset};

pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;

pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type boolean = uint8_t;
pub type uint8 = uint8_t;
pub type uint16 = uint16_t;
pub type uint32 = uint32_t;
pub type int64 = int64_t;
pub type uint64 = uint64_t;

/*
 * Licensed under the GNU General Public License version 2 with exceptions. See
 * LICENSE file in the project root for full license information
 */
/* * \file
 * \brief
 * Base EtherCAT functions.
 *
 * Setting up a datagram in an ethernet frame.
 * EtherCAT datagram primitives, broadcast, auto increment, configured and
 * logical addressed data transfers. All base transfers are blocking, so
 * wait for the frame to be returned to the master or timeout. If this is
 * not acceptable build your own datagrams and use the functions from nicdrv.c.
 */
/* * Write data to EtherCAT datagram.
 *
 * @param[out] datagramdata   = data part of datagram
 * @param[in]  com            = command
 * @param[in]  length         = length of databuffer
 * @param[in]  data           = databuffer to be copied into datagram
 */
unsafe extern "C" fn ecx_writedatagramdata(
    mut datagramdata: *mut libc::c_void,
    mut com: ec_cmdtype,
    mut length: uint16,
    mut data: *const libc::c_void,
) {
    if length as libc::c_int > 0i32 {
        let mut current_block_1: u64;

        match com {
            ec_cmdtype::EC_CMD_NOP
            | ec_cmdtype::EC_CMD_APRD
            | ec_cmdtype::EC_CMD_FPRD
            | ec_cmdtype::EC_CMD_BRD
            | ec_cmdtype::EC_CMD_LRD => {
                /* no data to write. initialise data so frame is in a known state */
                memset(datagramdata, 0i32, length as usize);
            }
            _ => {
                memcpy(datagramdata, data, length as usize);
            }
        }
    }
}
/* * Generate and set EtherCAT datagram in a standard ethernet frame.
 *
 * @param[in] port        = port context struct
 * @param[out] frame       = framebuffer
 * @param[in]  com         = command
 * @param[in]  idx         = index used for TX and RX buffers
 * @param[in]  ADP         = Address Position
 * @param[in]  ADO         = Address Offset
 * @param[in]  length      = length of datagram excluding EtherCAT header
 * @param[in]  data        = databuffer to be copied in datagram
 * @return always 0
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_setupdatagram(
    mut port: *mut ecx_portt,
    mut frame: *mut libc::c_void,
    mut com: ec_cmdtype,
    mut idx: uint8,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut datagramP: *mut ec_comt = 0 as *mut ec_comt;
    let mut frameP: *mut uint8 = 0 as *mut uint8;
    frameP = frame as *mut uint8;
    /* Ethernet header is preset and fixed in frame buffers
    EtherCAT header needs to be added after that */
    datagramP = &mut *frameP.offset(::core::mem::size_of::<ec_etherheadert>() as isize)
        as *mut uint8 as *mut ec_comt;
    (*datagramP).elength = (0x1000u64)
        .wrapping_add(core::mem::size_of::<ec_comt>() as u64)
        .wrapping_add(length as u64) as uint16;
    (*datagramP).command = com as u8;
    (*datagramP).index = idx;
    (*datagramP).ADP = ADP;
    (*datagramP).ADO = ADO;
    (*datagramP).dlength = length;
    ecx_writedatagramdata(
        &mut *frameP.offset(
            core::mem::size_of::<ec_etherheadert>().wrapping_add(core::mem::size_of::<ec_comt>())
                as isize,
        ) as *mut uint8 as *mut libc::c_void,
        com,
        length,
        data,
    );
    /* set WKC to zero */
    *frameP.offset(
        core::mem::size_of::<ec_etherheadert>()
            .wrapping_add(core::mem::size_of::<ec_comt>())
            .wrapping_add(length as usize) as isize,
    ) = 0u8;
    *frameP.offset(
        core::mem::size_of::<ec_etherheadert>()
            .wrapping_add(core::mem::size_of::<ec_comt>())
            .wrapping_add(length as usize)
            .wrapping_add(1usize) as isize,
    ) = 0u8;
    /* set size of frame in buffer array */
    (*port).txbuflength[idx as usize] = core::mem::size_of::<ec_etherheadert>()
        .wrapping_add(core::mem::size_of::<ec_comt>())
        .wrapping_add(core::mem::size_of::<uint16>())
        .wrapping_add(length as usize) as libc::c_int;
    return 0i32;
}
/* * Add EtherCAT datagram to a standard ethernet frame with existing datagram(s).
 *
 * @param[in] port        = port context struct
 * @param[out] frame      = framebuffer
 * @param[in]  com        = command
 * @param[in]  idx        = index used for TX and RX buffers
 * @param[in]  more       = TRUE if still more datagrams to follow
 * @param[in]  ADP        = Address Position
 * @param[in]  ADO        = Address Offset
 * @param[in]  length     = length of datagram excluding EtherCAT header
 * @param[in]  data       = databuffer to be copied in datagram
 * @return Offset to data in rx frame, usefull to retrieve data after RX.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_adddatagram(
    mut port: *mut ecx_portt,
    mut frame: *mut libc::c_void,
    mut com: ec_cmdtype,
    mut idx: uint8,
    mut more: boolean,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
) -> uint16 {
    let mut datagramP: *mut ec_comt = 0 as *mut ec_comt;
    let mut frameP: *mut uint8 = 0 as *mut uint8;
    let mut prevlength: uint16 = 0;
    frameP = frame as *mut uint8;
    /* copy previous frame size */
    prevlength = (*port).txbuflength[idx as usize] as uint16;
    datagramP = &mut *frameP.offset(::core::mem::size_of::<ec_etherheadert>() as isize)
        as *mut uint8 as *mut ec_comt;
    /* add new datagram to ethernet frame size */
    (*datagramP).elength = ((*datagramP).elength as usize)
        .wrapping_add(core::mem::size_of::<ec_comt>())
        .wrapping_add(length as usize) as uint16;
    /* add "datagram follows" flag to previous subframe dlength */
    (*datagramP).dlength = ((*datagramP).dlength as libc::c_int | (1i32) << 15i32) as uint16;
    /* set new EtherCAT header position */
    datagramP = &mut *frameP
        .offset((prevlength as usize).wrapping_sub(core::mem::size_of::<uint16>()) as isize)
        as *mut uint8 as *mut ec_comt;
    (*datagramP).command = com as u8;
    (*datagramP).index = idx;
    (*datagramP).ADP = ADP;
    (*datagramP).ADO = ADO;
    if more != 0 {
        /* this is not the last datagram to add */
        (*datagramP).dlength = (length as libc::c_int | (1i32) << 15i32) as uint16
    } else {
        /* this is the last datagram in the frame */
        (*datagramP).dlength = length
    }
    ecx_writedatagramdata(
        &mut *frameP.offset(
            (prevlength as usize)
                .wrapping_add(core::mem::size_of::<ec_comt>())
                .wrapping_sub(core::mem::size_of::<uint16>()) as isize,
        ) as *mut uint8 as *mut libc::c_void,
        com,
        length,
        data,
    );
    /* set WKC to zero */
    *frameP.offset(
        (prevlength as usize)
            .wrapping_add(core::mem::size_of::<ec_comt>())
            .wrapping_sub(core::mem::size_of::<uint16>())
            .wrapping_add(length as usize) as isize,
    ) = 0u8;
    *frameP.offset(
        (prevlength as usize)
            .wrapping_add(core::mem::size_of::<ec_comt>())
            .wrapping_sub(core::mem::size_of::<uint16>())
            .wrapping_add(length as usize)
            .wrapping_add(1usize) as isize,
    ) = 0u8;
    /* set size of frame in buffer array */
    (*port).txbuflength[idx as usize] = (prevlength as usize)
        .wrapping_add(core::mem::size_of::<ec_comt>())
        .wrapping_sub(core::mem::size_of::<uint16>())
        .wrapping_add(core::mem::size_of::<uint16>())
        .wrapping_add(length as usize) as libc::c_int;
    /* return offset to data in rx frame
    14 bytes smaller than tx frame due to stripping of ethernet header */
    return (prevlength as usize)
        .wrapping_add(core::mem::size_of::<ec_comt>())
        .wrapping_sub(core::mem::size_of::<uint16>())
        .wrapping_sub(core::mem::size_of::<ec_etherheadert>()) as uint16;
}
/* * BRW "broadcast write" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in] ADP         = Address Position, normally 0
 * @param[in] ADO         = Address Offset, slave memory address
 * @param[in] length      = length of databuffer
 * @param[in] data        = databuffer to be written to slaves
 * @param[in] timeout     = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_BWR(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut idx: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    /* get fresh index */
    idx = ecx_getindex(port);
    /* setup datagram */
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_BWR,
        idx,
        ADP,
        ADO,
        length,
        data,
    );
    /* send data and wait for answer */
    wkc = ecx_srconfirm(port, idx, timeout);
    /* clear buffer status */
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * BRD "broadcast read" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in]  ADP        = Address Position, normally 0
 * @param[in]  ADO        = Address Offset, slave memory address
 * @param[in]  length     = length of databuffer
 * @param[out] data       = databuffer to put slave data in
 * @param[in]  timeout    = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_BRD(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut idx: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    /* get fresh index */
    idx = ecx_getindex(port);
    /* setup datagram */
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_BRD,
        idx,
        ADP,
        ADO,
        length,
        data,
    );
    /* send data and wait for answer */
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32 {
        /* copy datagram to data buffer */
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut uint8
                as *const libc::c_void,
            length as usize,
        );
    }
    /* clear buffer status */
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * APRD "auto increment address read" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in]  ADP        = Address Position, each slave ++, slave that has 0 executes
 * @param[in]  ADO        = Address Offset, slave memory address
 * @param[in]  length     = length of databuffer
 * @param[out] data       = databuffer to put slave data in
 * @param[in]  timeout    = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_APRD(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: uint8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_APRD,
        idx,
        ADP,
        ADO,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32 {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut uint8
                as *const libc::c_void,
            length as usize,
        );
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * APRMW "auto increment address read, multiple write" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in]  ADP        = Address Position, each slave ++, slave that has 0 reads,
 *                          following slaves write.
 * @param[in]  ADO        = Address Offset, slave memory address
 * @param[in]  length     = length of databuffer
 * @param[out] data       = databuffer to put slave data in
 * @param[in]  timeout    = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_ARMW(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: uint8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_ARMW,
        idx,
        ADP,
        ADO,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32 {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut uint8
                as *const libc::c_void,
            length as usize,
        );
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * FPRMW "configured address read, multiple write" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in]  ADP        = Address Position, slave that has address reads,
 *                          following slaves write.
 * @param[in]  ADO        = Address Offset, slave memory address
 * @param[in]  length     = length of databuffer
 * @param[out] data       = databuffer to put slave data in
 * @param[in]  timeout    = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_FRMW(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: uint8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_FRMW,
        idx,
        ADP,
        ADO,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32 {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut uint8
                as *const libc::c_void,
            length as usize,
        );
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * APRDw "auto increment address read" word return primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in] ADP         = Address Position, each slave ++, slave that has 0 reads.
 * @param[in] ADO         = Address Offset, slave memory address
 * @param[in] timeout     = timeout in us, standard is EC_TIMEOUTRET
 * @return word data from slave
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_APRDw(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    let mut w: uint16 = 0;
    w = 0u16;
    ecx_APRD(
        port,
        ADP,
        ADO,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut w as *mut uint16 as *mut libc::c_void,
        timeout,
    );
    return w;
}
/* * FPRD "configured address read" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in]  ADP        = Address Position, slave that has address reads.
 * @param[in]  ADO        = Address Offset, slave memory address
 * @param[in]  length     = length of databuffer
 * @param[out] data       = databuffer to put slave data in
 * @param[in]  timeout    = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_FPRD(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: uint8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_FPRD,
        idx,
        ADP,
        ADO,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32 {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut uint8
                as *const libc::c_void,
            length as usize,
        );
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * FPRDw "configured address read" word return primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in] ADP         = Address Position, slave that has address reads.
 * @param[in] ADO         = Address Offset, slave memory address
 * @param[in] timeout     = timeout in us, standard is EC_TIMEOUTRET
 * @return word data from slave
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_FPRDw(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    let mut w: uint16 = 0;
    w = 0u16;
    ecx_FPRD(
        port,
        ADP,
        ADO,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut w as *mut uint16 as *mut libc::c_void,
        timeout,
    );
    return w;
}
/* * APWR "auto increment address write" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in] ADP         = Address Position, each slave ++, slave that has 0 writes.
 * @param[in] ADO         = Address Offset, slave memory address
 * @param[in] length      = length of databuffer
 * @param[in] data        = databuffer to write to slave.
 * @param[in] timeout     = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_APWR(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut idx: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_APWR,
        idx,
        ADP,
        ADO,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * APWRw "auto increment address write" word primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in] ADP         = Address Position, each slave ++, slave that has 0 writes.
 * @param[in] ADO         = Address Offset, slave memory address
 * @param[in] data        = word data to write to slave.
 * @param[in] timeout     = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_APWRw(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_APWR(
        port,
        ADP,
        ADO,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut data as *mut uint16 as *mut libc::c_void,
        timeout,
    );
}
/* * FPWR "configured address write" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in] ADP         = Address Position, slave that has address writes.
 * @param[in] ADO         = Address Offset, slave memory address
 * @param[in] length      = length of databuffer
 * @param[in] data        = databuffer to write to slave.
 * @param[in] timeout     = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_FPWR(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: uint8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_FPWR,
        idx,
        ADP,
        ADO,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * FPWR "configured address write" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in] ADP         = Address Position, slave that has address writes.
 * @param[in] ADO         = Address Offset, slave memory address
 * @param[in] data        = word to write to slave.
 * @param[in] timeout     = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_FPWRw(
    mut port: *mut ecx_portt,
    mut ADP: uint16,
    mut ADO: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_FPWR(
        port,
        ADP,
        ADO,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut data as *mut uint16 as *mut libc::c_void,
        timeout,
    );
}
/* * LRW "logical memory read / write" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in]     LogAdr  = Logical memory address
 * @param[in]     length  = length of databuffer
 * @param[in,out] data    = databuffer to write to and read from slave.
 * @param[in]     timeout = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_LRW(
    mut port: *mut ecx_portt,
    mut LogAdr: uint32,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut idx: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_LRW,
        idx,
        (LogAdr & 0xffffu32) as uint16,
        (LogAdr >> 16i32) as uint16,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32
        && (*port).rxbuf[idx as usize][::core::mem::size_of::<uint16>()] as libc::c_int
            == ec_cmdtype::EC_CMD_LRW as libc::c_int
    {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut uint8
                as *const libc::c_void,
            length as usize,
        );
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * LRD "logical memory read" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in]  LogAdr     = Logical memory address
 * @param[in]  length     = length of bytes to read from slave.
 * @param[out] data       = databuffer to read from slave.
 * @param[in]  timeout    = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_LRD(
    mut port: *mut ecx_portt,
    mut LogAdr: uint32,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut idx: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_LRD,
        idx,
        (LogAdr & 0xffffu32) as uint16,
        (LogAdr >> 16i32) as uint16,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32
        && (*port).rxbuf[idx as usize][::core::mem::size_of::<uint16>()] as libc::c_int
            == ec_cmdtype::EC_CMD_LRD as libc::c_int
    {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut uint8
                as *const libc::c_void,
            length as usize,
        );
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * LWR "logical memory write" primitive. Blocking.
 *
 * @param[in] port        = port context struct
 * @param[in] LogAdr      = Logical memory address
 * @param[in] length      = length of databuffer
 * @param[in] data        = databuffer to write to slave.
 * @param[in] timeout     = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_LWR(
    mut port: *mut ecx_portt,
    mut LogAdr: uint32,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut idx: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_LWR,
        idx,
        (LogAdr & 0xffffu32) as uint16,
        (LogAdr >> 16i32) as uint16,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
/* * LRW "logical memory read / write" primitive plus Clock Distribution. Blocking.
 * Frame consists of two datagrams, one LRW and one FPRMW.
 *
 * @param[in] port        = port context struct
 * @param[in]     LogAdr  = Logical memory address
 * @param[in]     length  = length of databuffer
 * @param[in,out] data    = databuffer to write to and read from slave.
 * @param[in]     DCrs    = Distributed Clock reference slave address.
 * @param[out]    DCtime  = DC time read from reference slave.
 * @param[in]     timeout = timeout in us, standard is EC_TIMEOUTRET
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_LRWDC(
    mut port: *mut ecx_portt,
    mut LogAdr: uint32,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut DCrs: uint16,
    mut DCtime: *mut int64,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut DCtO: uint16 = 0;
    let mut idx: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut DCtE: uint64 = 0;
    idx = ecx_getindex(port);
    /* LRW in first datagram */
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_LRW,
        idx,
        (LogAdr & 0xffffu32) as uint16,
        (LogAdr >> 16i32) as uint16,
        length,
        data,
    );
    /* FPRMW in second datagram */
    DCtE = *DCtime as uint64;
    DCtO = ecx_adddatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        ec_cmdtype::EC_CMD_FRMW,
        idx,
        0u8,
        DCrs,
        EthercatRegister::ECT_REG_DCSYSTIME as uint16,
        ::core::mem::size_of::<*mut int64>() as uint16,
        &mut DCtE as *mut uint64 as *mut libc::c_void,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32
        && (*port).rxbuf[idx as usize][::core::mem::size_of::<uint16>()] as libc::c_int
            == ec_cmdtype::EC_CMD_LRW as libc::c_int
    {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut uint8
                as *const libc::c_void,
            length as usize,
        );
        memcpy(
            &mut wkc as *mut libc::c_int as *mut libc::c_void,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(core::mem::size_of::<ec_comt>().wrapping_add(length as usize) as isize)
                as *mut uint8 as *const libc::c_void,
            core::mem::size_of::<uint16>(),
        );
        memcpy(
            &mut DCtE as *mut uint64 as *mut libc::c_void,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(DCtO as isize) as *mut uint8 as *const libc::c_void,
            core::mem::size_of::<int64>(),
        );
        *DCtime = DCtE as int64
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
#[no_mangle]
pub unsafe extern "C" fn ec_setupdatagram(
    mut frame: *mut libc::c_void,
    mut com: ec_cmdtype,
    mut idx: uint8,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    return ecx_setupdatagram(&mut ecx_port, frame, com, idx, ADP, ADO, length, data);
}
#[no_mangle]
pub unsafe extern "C" fn ec_adddatagram(
    mut frame: *mut libc::c_void,
    mut com: ec_cmdtype,
    mut idx: uint8,
    mut more: boolean,
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
) -> uint16 {
    return ecx_adddatagram(&mut ecx_port, frame, com, idx, more, ADP, ADO, length, data);
}
#[no_mangle]
pub unsafe extern "C" fn ec_BWR(
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_BWR(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_BRD(
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_BRD(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_APRD(
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_APRD(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_ARMW(
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_ARMW(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_FRMW(
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_FRMW(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_APRDw(
    mut ADP: uint16,
    mut ADO: uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    let mut w: uint16 = 0;
    w = 0u16;
    ec_APRD(
        ADP,
        ADO,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut w as *mut uint16 as *mut libc::c_void,
        timeout,
    );
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn ec_FPRD(
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_FPRD(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_FPRDw(
    mut ADP: uint16,
    mut ADO: uint16,
    mut timeout: libc::c_int,
) -> uint16 {
    let mut w: uint16 = 0;
    w = 0u16;
    ec_FPRD(
        ADP,
        ADO,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut w as *mut uint16 as *mut libc::c_void,
        timeout,
    );
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn ec_APWR(
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_APWR(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_APWRw(
    mut ADP: uint16,
    mut ADO: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ec_APWR(
        ADP,
        ADO,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut data as *mut uint16 as *mut libc::c_void,
        timeout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ec_FPWR(
    mut ADP: uint16,
    mut ADO: uint16,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_FPWR(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_FPWRw(
    mut ADP: uint16,
    mut ADO: uint16,
    mut data: uint16,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ec_FPWR(
        ADP,
        ADO,
        ::core::mem::size_of::<uint16>() as uint16,
        &mut data as *mut uint16 as *mut libc::c_void,
        timeout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ec_LRW(
    mut LogAdr: uint32,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_LRW(&mut ecx_port, LogAdr, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_LRD(
    mut LogAdr: uint32,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_LRD(&mut ecx_port, LogAdr, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_LWR(
    mut LogAdr: uint32,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_LWR(&mut ecx_port, LogAdr, length, data, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_LRWDC(
    mut LogAdr: uint32,
    mut length: uint16,
    mut data: *mut libc::c_void,
    mut DCrs: uint16,
    mut DCtime: *mut int64,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_LRWDC(&mut ecx_port, LogAdr, length, data, DCrs, DCtime, timeout);
}
