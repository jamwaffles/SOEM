use crate::{
    ethercatmain::ecx_port,
    ethercattype::{ec_bufT, ec_bufstate, ec_comt, ec_etherheadert, Command, EthercatRegister},
    oshw::linux::nicdrv::{ecx_getindex, ecx_portt, ecx_setbufstat, ecx_srconfirm},
};
use libc::{memcpy, memset};

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
unsafe fn ecx_writedatagramdata(
    datagramdata: *mut libc::c_void,
    com: Command,
    length: usize,
    data: *const libc::c_void,
) {
    if length as libc::c_int > 0i32 {
        match com {
            Command::Nop | Command::Aprd | Command::Fprd | Command::Brd | Command::Lrd => {
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
pub unsafe fn ecx_setupdatagram(
    mut port: *mut ecx_portt,
    frame: *mut libc::c_void,
    com: Command,
    idx: u8,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
) -> libc::c_int {
    let mut datagramP: *mut ec_comt = 0 as *mut ec_comt;
    let mut frameP: *mut u8 = 0 as *mut u8;
    frameP = frame as *mut u8;
    /* Ethernet header is preset and fixed in frame buffers
    EtherCAT header needs to be added after that */
    datagramP = &mut *frameP.offset(::core::mem::size_of::<ec_etherheadert>() as isize) as *mut u8
        as *mut ec_comt;
    (*datagramP).elength = (0x1000u64)
        .wrapping_add(core::mem::size_of::<ec_comt>() as u64)
        .wrapping_add(length as u64) as u16;
    (*datagramP).command = com as u8;
    (*datagramP).index = idx;
    (*datagramP).ADP = ADP;
    (*datagramP).ADO = ADO;
    (*datagramP).dlength = length as u16;
    ecx_writedatagramdata(
        &mut *frameP.offset(
            core::mem::size_of::<ec_etherheadert>().wrapping_add(core::mem::size_of::<ec_comt>())
                as isize,
        ) as *mut u8 as *mut libc::c_void,
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
        .wrapping_add(core::mem::size_of::<u16>())
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
pub unsafe fn ecx_adddatagram(
    mut port: *mut ecx_portt,
    frame: *mut libc::c_void,
    com: Command,
    idx: u8,
    more: bool,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
) -> u16 {
    let mut datagramP: *mut ec_comt = 0 as *mut ec_comt;
    let mut frameP: *mut u8 = 0 as *mut u8;
    let mut prevlength: u16 = 0;
    frameP = frame as *mut u8;
    /* copy previous frame size */
    prevlength = (*port).txbuflength[idx as usize] as u16;
    datagramP = &mut *frameP.offset(::core::mem::size_of::<ec_etherheadert>() as isize) as *mut u8
        as *mut ec_comt;
    /* add new datagram to ethernet frame size */
    (*datagramP).elength = ((*datagramP).elength as usize)
        .wrapping_add(core::mem::size_of::<ec_comt>())
        .wrapping_add(length as usize) as u16;
    /* add "datagram follows" flag to previous subframe dlength */
    (*datagramP).dlength = ((*datagramP).dlength as libc::c_int | (1i32) << 15i32) as u16;
    /* set new EtherCAT header position */
    datagramP = &mut *frameP
        .offset((prevlength as usize).wrapping_sub(core::mem::size_of::<u16>()) as isize)
        as *mut u8 as *mut ec_comt;
    (*datagramP).command = com as u8;
    (*datagramP).index = idx;
    (*datagramP).ADP = ADP;
    (*datagramP).ADO = ADO;
    if more == true {
        /* this is not the last datagram to add */
        (*datagramP).dlength = (length as libc::c_int | (1i32) << 15i32) as u16
    } else {
        /* this is the last datagram in the frame */
        (*datagramP).dlength = length as u16
    }
    ecx_writedatagramdata(
        &mut *frameP.offset(
            (prevlength as usize)
                .wrapping_add(core::mem::size_of::<ec_comt>())
                .wrapping_sub(core::mem::size_of::<u16>()) as isize,
        ) as *mut u8 as *mut libc::c_void,
        com,
        length,
        data,
    );
    /* set WKC to zero */
    *frameP.offset(
        (prevlength as usize)
            .wrapping_add(core::mem::size_of::<ec_comt>())
            .wrapping_sub(core::mem::size_of::<u16>())
            .wrapping_add(length as usize) as isize,
    ) = 0u8;
    *frameP.offset(
        (prevlength as usize)
            .wrapping_add(core::mem::size_of::<ec_comt>())
            .wrapping_sub(core::mem::size_of::<u16>())
            .wrapping_add(length as usize)
            .wrapping_add(1usize) as isize,
    ) = 0u8;
    /* set size of frame in buffer array */
    (*port).txbuflength[idx as usize] = (prevlength as usize)
        .wrapping_add(core::mem::size_of::<ec_comt>())
        .wrapping_sub(core::mem::size_of::<u16>())
        .wrapping_add(core::mem::size_of::<u16>())
        .wrapping_add(length as usize) as libc::c_int;
    /* return offset to data in rx frame
    14 bytes smaller than tx frame due to stripping of ethernet header */
    return (prevlength as usize)
        .wrapping_add(core::mem::size_of::<ec_comt>())
        .wrapping_sub(core::mem::size_of::<u16>())
        .wrapping_sub(core::mem::size_of::<ec_etherheadert>()) as u16;
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
pub unsafe fn ecx_BWR(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut idx: u8 = 0;
    let mut wkc: libc::c_int = 0;
    /* get fresh index */
    idx = ecx_getindex(port);
    /* setup datagram */
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Bwr,
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
pub unsafe fn ecx_BRD(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut idx: u8 = 0;
    let mut wkc: libc::c_int = 0;
    /* get fresh index */
    idx = ecx_getindex(port);
    /* setup datagram */
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Brd,
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
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut u8
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
pub unsafe fn ecx_APRD(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: u8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Aprd,
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
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut u8
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
pub unsafe fn ecx_ARMW(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: u8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Armw,
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
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut u8
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
pub unsafe fn ecx_FRMW(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: u8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Frmw,
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
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut u8
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
pub unsafe fn ecx_APRDw(port: *mut ecx_portt, ADP: u16, ADO: u16, timeout: u32) -> u16 {
    let mut w: u16 = 0;
    w = 0u16;
    ecx_APRD(
        port,
        ADP,
        ADO,
        ::core::mem::size_of::<u16>(),
        &mut w as *mut u16 as *mut libc::c_void,
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
pub unsafe fn ecx_FPRD(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: u8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Fprd,
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
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut u8
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
pub unsafe fn ecx_FPRDw(port: *mut ecx_portt, ADP: u16, ADO: u16, timeout: u32) -> u16 {
    let mut w: u16 = 0;
    w = 0u16;
    ecx_FPRD(
        port,
        ADP,
        ADO,
        ::core::mem::size_of::<u16>(),
        &mut w as *mut u16 as *mut libc::c_void,
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
pub unsafe fn ecx_APWR(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut idx: u8 = 0;
    let mut wkc: libc::c_int = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Apwr,
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
pub unsafe fn ecx_APWRw(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    mut data: u16,
    timeout: u32,
) -> libc::c_int {
    return ecx_APWR(
        port,
        ADP,
        ADO,
        ::core::mem::size_of::<u16>(),
        &mut data as *mut u16 as *mut libc::c_void,
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
pub unsafe fn ecx_FPWR(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut idx: u8 = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Fpwr,
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
pub unsafe fn ecx_FPWRw(
    port: *mut ecx_portt,
    ADP: u16,
    ADO: u16,
    mut data: u16,
    timeout: u32,
) -> libc::c_int {
    return ecx_FPWR(
        port,
        ADP,
        ADO,
        ::core::mem::size_of::<u16>(),
        &mut data as *mut u16 as *mut libc::c_void,
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
pub unsafe fn ecx_LRW(
    port: *mut ecx_portt,
    LogAdr: u32,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut idx: u8 = 0;
    let mut wkc: libc::c_int = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Lrw,
        idx,
        (LogAdr & 0xffffu32) as u16,
        (LogAdr >> 16i32) as u16,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32
        && (*port).rxbuf[idx as usize][::core::mem::size_of::<u16>()] as libc::c_int
            == Command::Lrw as libc::c_int
    {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut u8
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
pub unsafe fn ecx_LRD(
    port: *mut ecx_portt,
    LogAdr: u32,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut idx: u8 = 0;
    let mut wkc: libc::c_int = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Lrd,
        idx,
        (LogAdr & 0xffffu32) as u16,
        (LogAdr >> 16i32) as u16,
        length,
        data,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32
        && (*port).rxbuf[idx as usize][::core::mem::size_of::<u16>()] as libc::c_int
            == Command::Lrd as libc::c_int
    {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut u8
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
pub unsafe fn ecx_LWR(
    port: *mut ecx_portt,
    LogAdr: u32,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    let mut idx: u8 = 0;
    let mut wkc: libc::c_int = 0;
    idx = ecx_getindex(port);
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Lwr,
        idx,
        (LogAdr & 0xffffu32) as u16,
        (LogAdr >> 16i32) as u16,
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
pub unsafe fn ecx_LRWDC(
    port: *mut ecx_portt,
    LogAdr: u32,
    length: usize,
    data: *mut libc::c_void,
    DCrs: u16,
    DCtime: *mut i64,
    timeout: u32,
) -> libc::c_int {
    let mut DCtO: u16 = 0;
    let mut idx: u8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut DCtE: u64 = 0;
    idx = ecx_getindex(port);
    /* LRW in first datagram */
    ecx_setupdatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Lrw,
        idx,
        (LogAdr & 0xffffu32) as u16,
        (LogAdr >> 16i32) as u16,
        length,
        data,
    );
    /* FPRMW in second datagram */
    DCtE = *DCtime as u64;
    DCtO = ecx_adddatagram(
        port,
        &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut libc::c_void,
        Command::Frmw,
        idx,
        false,
        DCrs,
        EthercatRegister::ECT_REG_DCSYSTIME as u16,
        ::core::mem::size_of::<*mut i64>(),
        &mut DCtE as *mut u64 as *mut libc::c_void,
    );
    wkc = ecx_srconfirm(port, idx, timeout);
    if wkc > 0i32
        && (*port).rxbuf[idx as usize][::core::mem::size_of::<u16>()] as libc::c_int
            == Command::Lrw as libc::c_int
    {
        memcpy(
            data,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(::core::mem::size_of::<ec_comt>() as isize) as *mut u8
                as *const libc::c_void,
            length as usize,
        );
        memcpy(
            &mut wkc as *mut libc::c_int as *mut libc::c_void,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(core::mem::size_of::<ec_comt>().wrapping_add(length as usize) as isize)
                as *mut u8 as *const libc::c_void,
            core::mem::size_of::<u16>(),
        );
        memcpy(
            &mut DCtE as *mut u64 as *mut libc::c_void,
            &mut *(*(*port).rxbuf.as_mut_ptr().offset(idx as isize))
                .as_mut_ptr()
                .offset(DCtO as isize) as *mut u8 as *const libc::c_void,
            core::mem::size_of::<i64>(),
        );
        *DCtime = DCtE as i64
    }
    ecx_setbufstat(port, idx, ec_bufstate::EC_BUF_EMPTY as libc::c_int);
    return wkc;
}
#[no_mangle]
pub unsafe fn ec_setupdatagram(
    frame: *mut libc::c_void,
    com: Command,
    idx: u8,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
) -> libc::c_int {
    return ecx_setupdatagram(&mut ecx_port, frame, com, idx, ADP, ADO, length, data);
}
#[no_mangle]
pub unsafe fn ec_adddatagram(
    frame: *mut libc::c_void,
    com: Command,
    idx: u8,
    more: bool,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
) -> u16 {
    return ecx_adddatagram(&mut ecx_port, frame, com, idx, more, ADP, ADO, length, data);
}
#[no_mangle]
pub unsafe fn ec_BWR(
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_BWR(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_BRD(
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_BRD(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_APRD(
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_APRD(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_ARMW(
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_ARMW(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_FRMW(
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_FRMW(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_APRDw(ADP: u16, ADO: u16, timeout: u32) -> u16 {
    let mut w: u16 = 0;
    w = 0u16;
    ec_APRD(
        ADP,
        ADO,
        ::core::mem::size_of::<u16>(),
        &mut w as *mut u16 as *mut libc::c_void,
        timeout,
    );
    return w;
}
#[no_mangle]
pub unsafe fn ec_FPRD(
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_FPRD(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_FPRDw(ADP: u16, ADO: u16, timeout: u32) -> u16 {
    let mut w: u16 = 0;
    w = 0u16;
    ec_FPRD(
        ADP,
        ADO,
        ::core::mem::size_of::<u16>(),
        &mut w as *mut u16 as *mut libc::c_void,
        timeout,
    );
    return w;
}
#[no_mangle]
pub unsafe fn ec_APWR(
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_APWR(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_APWRw(ADP: u16, ADO: u16, mut data: u16, timeout: u32) -> libc::c_int {
    return ec_APWR(
        ADP,
        ADO,
        ::core::mem::size_of::<u16>(),
        &mut data as *mut u16 as *mut libc::c_void,
        timeout,
    );
}
#[no_mangle]
pub unsafe fn ec_FPWR(
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_FPWR(&mut ecx_port, ADP, ADO, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_FPWRw(ADP: u16, ADO: u16, mut data: u16, timeout: u32) -> libc::c_int {
    return ec_FPWR(
        ADP,
        ADO,
        ::core::mem::size_of::<u16>(),
        &mut data as *mut u16 as *mut libc::c_void,
        timeout,
    );
}
#[no_mangle]
pub unsafe fn ec_LRW(
    LogAdr: u32,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_LRW(&mut ecx_port, LogAdr, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_LRD(
    LogAdr: u32,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_LRD(&mut ecx_port, LogAdr, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_LWR(
    LogAdr: u32,
    length: usize,
    data: *mut libc::c_void,
    timeout: u32,
) -> libc::c_int {
    return ecx_LWR(&mut ecx_port, LogAdr, length, data, timeout);
}
#[no_mangle]
pub unsafe fn ec_LRWDC(
    LogAdr: u32,
    length: usize,
    data: *mut libc::c_void,
    DCrs: u16,
    DCtime: *mut i64,
    timeout: u32,
) -> libc::c_int {
    return ecx_LRWDC(&mut ecx_port, LogAdr, length, data, DCrs, DCtime, timeout);
}
