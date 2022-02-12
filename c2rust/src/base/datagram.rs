/*
 * Licensed under the GNU General Public License version 2 with exceptions. See
 * LICENSE file in the project root for full license information
 */

use crate::{
    oshw::linux::nicdrv::ecx_portt,
    types::{
        ec_bufT, Command, EthercatHeader, EthernetHeader, EC_ECATTYPE, EC_ELENGTHSIZE,
        EC_HEADERSIZE, EC_WKCSIZE, ETH_HEADERSIZE,
    },
};
use core::slice;
use libc::{c_void, memcpy, memset};

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
                memset(datagramdata as *mut c_void, 0, length as usize);
            }
            _ => {
                memcpy(datagramdata as *mut c_void, data, length as usize);
            }
        }
    }
}

unsafe fn ecx_writedatagramdata_new(
    datagram_buffer: &mut [u8],
    com: Command,
    length: usize,
    data: *const libc::c_void,
) {
    if length as libc::c_int > 0i32 {
        match com {
            Command::Nop | Command::Aprd | Command::Fprd | Command::Brd | Command::Lrd => {
                /* no data to write. initialise data so frame is in a known state */
                datagram_buffer.fill(0);
            }
            _ => {
                memcpy(
                    datagram_buffer.as_mut_ptr() as *mut c_void,
                    data,
                    length as usize,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::types::EC_BUFSIZE;

    use super::*;

    #[test]
    fn datagram_data_noop() {
        let mut buf_old = [0u8; EC_BUFSIZE];
        let mut buf_new = [0u8; EC_BUFSIZE];
        let command = Command::Aprd;
        let data = 0x1234u16;
        let length = size_of::<u16>();

        unsafe {
            ecx_writedatagramdata(
                buf_old.as_mut_ptr() as *mut c_void,
                command,
                length,
                data as *const c_void,
            )
        };
        unsafe {
            ecx_writedatagramdata_new(
                &mut buf_new,
                command,
                length,
                &data as *const _ as *const c_void,
            )
        };

        assert_eq!(buf_old, buf_new);
        assert_eq!(buf_old, [0u8; EC_BUFSIZE]);
    }

    #[test]
    fn datagram_data_simple() {
        let mut buf_old = [0u8; EC_BUFSIZE];
        let mut buf_new = [0u8; EC_BUFSIZE];
        let command = Command::Aprw;
        let data = 0x1234u16;
        let length = size_of::<u16>();

        unsafe {
            ecx_writedatagramdata_new(
                &mut buf_new,
                command,
                length,
                &data as *const _ as *const c_void,
            )
        };

        unsafe {
            ecx_writedatagramdata(
                buf_old.as_mut_ptr() as *mut c_void,
                command,
                length,
                &data as *const _ as *const c_void,
            )
        };

        assert_eq!(
            buf_old, buf_new,
            "old and new styles don't do the same thing"
        );
        assert_eq!(
            buf_old,
            {
                let mut buf = [0u8; EC_BUFSIZE];

                buf[0] = 0x34;
                buf[1] = 0x12;

                buf
            },
            "data not written correctly"
        );
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
    // let mut datagramP: *mut EthercatHeader = 0 as *mut EthercatHeader;
    let mut frameP: *mut u8 = 0 as *mut u8;
    frameP = frame as *mut u8;
    // Ethernet header is preset and fixed in frame buffers
    // EtherCAT header needs to be added after that
    let mut datagramP = EthercatHeader::default();
    datagramP.elength = EC_ECATTYPE + EC_HEADERSIZE as u16 + length as u16;
    datagramP.command = com as u8;
    datagramP.index = idx;
    datagramP.ADP = ADP;
    datagramP.ADO = ADO;
    datagramP.dlength = length as u16;
    ecx_writedatagramdata(
        &mut *frameP.offset(
            core::mem::size_of::<EthernetHeader>()
                .wrapping_add(core::mem::size_of::<EthercatHeader>()) as isize,
        ) as *mut u8 as *mut libc::c_void,
        // &mut *frameP.offset((ETH_HEADERSIZE + EC_HEADERSIZE) as isize) as *mut u8,
        com,
        length,
        data,
    );
    /* set WKC to zero */
    *frameP.offset((ETH_HEADERSIZE + EC_HEADERSIZE + length) as isize) = 0u8;
    *frameP.offset((ETH_HEADERSIZE + EC_HEADERSIZE + length + 1) as isize) = 0u8;
    /* set size of frame in buffer array */
    (*port).txbuflength[idx as usize] =
        (ETH_HEADERSIZE + EC_HEADERSIZE + EC_WKCSIZE + length) as i32;
    return 0i32;
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
pub unsafe fn ecx_setupdatagram_new(
    mut port: *mut ecx_portt,
    frame: &mut ec_bufT,
    com: Command,
    idx: u8,
    ADP: u16,
    ADO: u16,
    length: usize,
    data: *mut libc::c_void,
) -> i32 {
    // let mut datagramP: *mut EthercatHeader = 0 as *mut EthercatHeader;
    let mut frameP: *mut u8 = 0 as *mut u8;
    frameP = frame as *mut u8;
    // Ethernet header is preset and fixed in frame buffers
    // EtherCAT header needs to be added after that
    let mut datagramP = EthercatHeader::default();
    datagramP.elength = EC_ECATTYPE + EC_HEADERSIZE as u16 + length as u16;
    datagramP.command = com as u8;
    datagramP.index = idx;
    datagramP.ADP = ADP;
    datagramP.ADO = ADO;
    datagramP.dlength = length as u16;
    ecx_writedatagramdata(
        &mut *frameP.offset((ETH_HEADERSIZE + EC_HEADERSIZE) as isize) as *mut u8 as *mut c_void,
        com,
        length,
        data,
    );
    /* set WKC to zero */
    *frameP.offset((ETH_HEADERSIZE + EC_HEADERSIZE + length) as isize) = 0u8;
    *frameP.offset((ETH_HEADERSIZE + EC_HEADERSIZE + length + 1) as isize) = 0u8;
    /* set size of frame in buffer array */
    (*port).txbuflength[idx as usize] =
        (ETH_HEADERSIZE + EC_HEADERSIZE + EC_WKCSIZE + length) as i32;
    return 0;
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
    let mut datagramP: *mut EthercatHeader = 0 as *mut EthercatHeader;
    let mut frameP: *mut u8 = 0 as *mut u8;
    let mut prevlength: u16 = 0;
    frameP = frame as *mut u8;
    /* copy previous frame size */
    prevlength = (*port).txbuflength[idx as usize] as u16;
    datagramP = &mut *frameP.offset(::core::mem::size_of::<EthernetHeader>() as isize) as *mut u8
        as *mut EthercatHeader;
    /* add new datagram to ethernet frame size */
    (*datagramP).elength = ((*datagramP).elength as usize)
        .wrapping_add(core::mem::size_of::<EthercatHeader>())
        .wrapping_add(length as usize) as u16;
    /* add "datagram follows" flag to previous subframe dlength */
    (*datagramP).dlength = ((*datagramP).dlength as libc::c_int | (1i32) << 15i32) as u16;
    /* set new EtherCAT header position */
    datagramP = &mut *frameP
        .offset((prevlength as usize).wrapping_sub(core::mem::size_of::<u16>()) as isize)
        as *mut u8 as *mut EthercatHeader;
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
                .wrapping_add(core::mem::size_of::<EthercatHeader>())
                .wrapping_sub(core::mem::size_of::<u16>()) as isize,
        ) as *mut u8 as *mut libc::c_void,
        // &mut *frameP.offset((prevlength + (EC_HEADERSIZE - EC_ELENGTHSIZE) as u16) as isize)
        // as *mut u8,
        com,
        length,
        data,
    );
    /* set WKC to zero */
    *frameP.offset(
        (prevlength as usize)
            .wrapping_add(core::mem::size_of::<EthercatHeader>())
            .wrapping_sub(core::mem::size_of::<u16>())
            .wrapping_add(length as usize) as isize,
    ) = 0u8;
    *frameP.offset(
        (prevlength as usize)
            .wrapping_add(core::mem::size_of::<EthercatHeader>())
            .wrapping_sub(core::mem::size_of::<u16>())
            .wrapping_add(length as usize)
            .wrapping_add(1usize) as isize,
    ) = 0u8;
    /* set size of frame in buffer array */
    (*port).txbuflength[idx as usize] = (prevlength as usize)
        .wrapping_add(core::mem::size_of::<EthercatHeader>())
        .wrapping_sub(core::mem::size_of::<u16>())
        .wrapping_add(core::mem::size_of::<u16>())
        .wrapping_add(length as usize) as libc::c_int;
    /* return offset to data in rx frame
    14 bytes smaller than tx frame due to stripping of ethernet header */
    return prevlength + EC_HEADERSIZE as u16 - EC_ELENGTHSIZE as u16 - ETH_HEADERSIZE as u16;
}
