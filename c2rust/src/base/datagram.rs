/*
 * Licensed under the GNU General Public License version 2 with exceptions. See
 * LICENSE file in the project root for full license information
 */

use core::slice;
use std::mem::size_of;

use crate::{
    oshw::linux::nicdrv::ecx_portt,
    types::{
        ec_bufT, Command, EthercatHeader, EthernetHeader, EC_DATAGRAMFOLLOWS, EC_ECATTYPE,
        EC_ELENGTHSIZE, EC_HEADERSIZE, EC_WKCSIZE, ETH_HEADERSIZE,
    },
};
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
    let mut datagramP: *mut EthercatHeader = 0 as *mut EthercatHeader;
    let mut frameP: *mut u8 = 0 as *mut u8;
    frameP = frame as *mut u8;
    /* Ethernet header is preset and fixed in frame buffers
    EtherCAT header needs to be added after that */
    datagramP = &mut *frameP
        .offset(::core::mem::size_of::<EthernetHeader>() as libc::c_ulong as isize)
        as *mut u8 as *mut EthercatHeader;
    (*datagramP).elength = (0x1000 as libc::c_int as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<EthercatHeader>() as libc::c_ulong)
        .wrapping_add(length as libc::c_ulong) as u16;
    (*datagramP).command = com as u8;
    (*datagramP).index = idx;
    (*datagramP).ADP = ADP;
    (*datagramP).ADO = ADO;
    (*datagramP).dlength = length as u16;
    ecx_writedatagramdata(
        &mut *frameP.offset(
            (::core::mem::size_of::<EthernetHeader>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<EthercatHeader>() as libc::c_ulong)
                as isize,
        ) as *mut u8 as *mut libc::c_void,
        com,
        length,
        data,
    );
    /* set WKC to zero */
    *frameP.offset(
        (::core::mem::size_of::<EthernetHeader>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<EthercatHeader>() as libc::c_ulong)
            .wrapping_add(length as libc::c_ulong) as isize,
    ) = 0 as libc::c_int as u8;
    *frameP.offset(
        (::core::mem::size_of::<EthernetHeader>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<EthercatHeader>() as libc::c_ulong)
            .wrapping_add(length as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
    ) = 0 as libc::c_int as u8;
    /* set size of frame in buffer array */
    (*port).txbuflength[idx as usize] = (::core::mem::size_of::<EthernetHeader>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<EthercatHeader>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<u16>() as libc::c_ulong)
        .wrapping_add(length as libc::c_ulong)
        as libc::c_int;
    return 0 as libc::c_int;
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
    address_position: u16,
    address_offset: u16,
    length: usize,
    data: *const libc::c_void,
) -> i32 {
    // Ethernet header is preset and fixed in frame buffers
    // EtherCAT header needs to be added after that
    let datagramP = EthercatHeader {
        elength: EC_ECATTYPE + EC_HEADERSIZE as u16 + length as u16,
        command: com as u8,
        index: idx,
        ADP: address_position,
        ADO: address_offset,
        dlength: length as u16,
        ..EthercatHeader::default()
    };

    // Add ethercat frame header after where ethernet header will go
    let header = slice::from_raw_parts(&datagramP as *const _ as *const u8, EC_HEADERSIZE);
    frame[ETH_HEADERSIZE..(ETH_HEADERSIZE + EC_HEADERSIZE)].copy_from_slice(header);

    let data_start = ETH_HEADERSIZE + EC_HEADERSIZE;

    ecx_writedatagramdata_new(&mut frame[data_start..], com, length, data);
    // set WKC to zero (16 bits, two bytes)
    frame[data_start + length] = 0u8;
    frame[data_start + length + 1] = 0u8;

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
    address_position: u16,
    address_offset: u16,
    length: usize,
    data: *mut libc::c_void,
) -> u16 {
    let mut datagramP: *mut EthercatHeader = 0 as *mut EthercatHeader;
    let mut frameP: *mut u8 = 0 as *mut u8;
    let mut prevlength: u16 = 0;
    frameP = frame as *mut u8;
    /* copy previous frame size */
    prevlength = (*port).txbuflength[idx as usize] as u16;
    // Load ethercat header that's after ethernet frame header
    datagramP = &mut *frameP.offset(ETH_HEADERSIZE as isize) as *mut u8 as *mut EthercatHeader;
    /* add new datagram to ethernet frame size */
    (*datagramP).elength = (*datagramP).elength + EC_HEADERSIZE as u16 + length as u16;
    /* add "datagram follows" flag to previous subframe dlength */
    (*datagramP).dlength = (*datagramP).dlength as u16 | EC_DATAGRAMFOLLOWS;

    /* set new EtherCAT header position */
    datagramP = &mut *frameP.offset((prevlength - EC_ELENGTHSIZE as u16) as isize) as *mut u8
        as *mut EthercatHeader;
    (*datagramP).command = com as u8;
    (*datagramP).index = idx;
    (*datagramP).ADP = address_position;
    (*datagramP).ADO = address_offset;
    if more == true {
        /* this is not the last datagram to add */
        (*datagramP).dlength = length as u16 | EC_DATAGRAMFOLLOWS;
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
    // FIXME: Saturating or checked sub
    return prevlength + EC_HEADERSIZE as u16 - EC_ELENGTHSIZE as u16 - ETH_HEADERSIZE as u16;
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
pub unsafe fn ecx_adddatagram_new(
    port: &mut ecx_portt,
    frame: &mut ec_bufT,
    com: Command,
    idx: u8,
    more: bool,
    address_position: u16,
    address_offset: u16,
    length: usize,
    data: *const libc::c_void,
) -> u16 {
    // copy previous frame size
    let prevlength = (*port).txbuflength[idx as usize] as u16;

    // Load ethercat header that's after ethernet frame header
    let mut first_header = &mut *(&mut frame[ETH_HEADERSIZE..(ETH_HEADERSIZE + EC_HEADERSIZE)]
        as *mut _ as *mut EthercatHeader);

    // add new datagram to ethernet frame size
    first_header.elength += (EC_HEADERSIZE + length) as u16;

    // add "datagram follows" flag to previous subframe dlength
    first_header.dlength |= EC_DATAGRAMFOLLOWS;

    let next_header = EthercatHeader {
        elength: 0,
        command: com as u8,
        index: idx,
        ADP: address_position,
        ADO: address_offset,
        dlength: if more {
            length as u16 | EC_DATAGRAMFOLLOWS
        } else {
            length as u16
        },
        ..EthercatHeader::default()
    };

    // Add new ethercat header to end of frame, overwriting the current work counter
    let next_header_start = prevlength as usize - size_of::<u16>();
    let next_header = slice::from_raw_parts(&next_header as *const _ as *const u8, EC_HEADERSIZE);
    frame[next_header_start..(next_header_start + EC_HEADERSIZE)].copy_from_slice(next_header);

    let data_start = prevlength as usize + EC_HEADERSIZE - EC_ELENGTHSIZE;

    ecx_writedatagramdata_new(&mut frame[data_start..], com, length, data);
    // set WKC to zero
    frame[data_start + length] = 0x00;
    frame[data_start + length + 1] = 0x00;
    // set size of frame in buffer array
    port.txbuflength[idx as usize] = (data_start + EC_WKCSIZE + length) as i32;
    // return offset to data in rx frame 14 bytes smaller than tx frame due to stripping of ethernet
    // header
    // FIXME: Saturating or checked sub
    return prevlength + EC_HEADERSIZE as u16 - EC_ELENGTHSIZE as u16 - ETH_HEADERSIZE as u16;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        oshw::linux::nicdrv::ec_stackT,
        types::{EC_BUFSIZE, EC_MAXBUF},
    };
    use libc::pthread_mutex_t;
    use std::mem::{self, size_of};

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

    #[test]
    fn datagram_data_struct() {
        #[repr(C, packed)]
        #[derive(PartialEq, Debug, Copy, Clone)]
        struct Data {
            foo: u16,
            bar: i32,
            baz: u8,
        }

        let mut buf_old = [0u8; EC_BUFSIZE];
        let mut buf_new = buf_old.clone();
        let command = Command::Aprw;
        let data = Data {
            foo: 0x1234u16,
            bar: -9999,
            baz: 255,
        };
        let length = size_of::<Data>();

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

                buf[0] = 52;
                buf[1] = 18;
                buf[2] = 241;
                buf[3] = 216;
                buf[4] = 255;
                buf[5] = 255;
                buf[6] = 255;

                buf
            },
            "data not written correctly"
        );

        unsafe { assert_eq!(*(buf_new[0..length].as_ptr() as *const Data), data) };
    }

    #[test]
    fn setup_datagram() {
        let fill = 0u8;

        let mut port_old: ecx_portt = ecx_portt {
            stack: ec_stackT {
                sock: 0 as *mut libc::c_int,
                txbuf: 0 as *mut [ec_bufT; EC_MAXBUF as usize],
                txbuflength: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
                tempbuf: 0 as *mut ec_bufT,
                rxbuf: 0 as *mut [ec_bufT; EC_MAXBUF as usize],
                rxbufstat: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
                rxsa: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
            },
            sockhandle: 0,
            rxbuf: [[0; EC_BUFSIZE]; EC_MAXBUF as usize],
            rxbufstat: [0; 16],
            rxsa: [0; EC_MAXBUF as usize],
            tempinbuf: [0; EC_BUFSIZE],
            tempinbufs: 0,
            txbuf: [[0; EC_BUFSIZE]; EC_MAXBUF as usize],
            txbuflength: [0; EC_MAXBUF as usize],
            txbuf2: [0; EC_BUFSIZE],
            txbuflength2: 0,
            lastidx: 0,
            redport: None,
            getindex_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
            tx_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
            rx_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
        };

        let mut port_new = port_old.clone();

        let mut buf_old = [fill; EC_BUFSIZE];
        let mut buf_new = [fill; EC_BUFSIZE];
        let command = Command::Aprw;
        let mut data_old = 0x1234u16;
        let data_new = data_old;
        let length = size_of::<u16>();

        // TODO: Figure out what these are
        let w1 = 0;
        let w2 = 0;

        unsafe {
            ecx_setupdatagram(
                &mut port_old,
                buf_old.as_mut_ptr() as *mut c_void,
                command,
                0,
                w1,
                w2,
                length,
                &mut data_old as *mut _ as *mut c_void,
            )
        };
        unsafe {
            ecx_setupdatagram_new(
                &mut port_new,
                &mut buf_new,
                command,
                0,
                w1,
                w2,
                length,
                &data_new as *const _ as *const c_void,
            )
        };

        assert_eq!(port_old.txbuflength, port_new.txbuflength);

        assert_eq!(
            buf_old[0..(port_new.txbuflength[0] as usize + 3)],
            buf_new[0..(port_new.txbuflength[0] as usize + 3)],
            "old, new do not match"
        );
    }

    #[test]
    fn add_datagram() {
        let fill = 0u8;

        let mut port_old: ecx_portt = ecx_portt {
            stack: ec_stackT {
                sock: 0 as *mut libc::c_int,
                txbuf: 0 as *mut [ec_bufT; EC_MAXBUF as usize],
                txbuflength: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
                tempbuf: 0 as *mut ec_bufT,
                rxbuf: 0 as *mut [ec_bufT; EC_MAXBUF as usize],
                rxbufstat: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
                rxsa: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
            },
            sockhandle: 0,
            rxbuf: [[0; EC_BUFSIZE]; EC_MAXBUF as usize],
            rxbufstat: [0; 16],
            rxsa: [0; EC_MAXBUF as usize],
            tempinbuf: [0; EC_BUFSIZE],
            tempinbufs: 0,
            txbuf: [[0; EC_BUFSIZE]; EC_MAXBUF as usize],
            txbuflength: [0; EC_MAXBUF as usize],
            txbuf2: [0; EC_BUFSIZE],
            txbuflength2: 0,
            lastidx: 0,
            redport: None,
            getindex_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
            tx_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
            rx_mutex: unsafe { mem::transmute::<[u8; 40], pthread_mutex_t>([0u8; 40]) },
        };

        let mut port_new = port_old.clone();

        let mut buf_old = [fill; EC_BUFSIZE];
        let mut buf_new = [fill; EC_BUFSIZE];
        let command = Command::Fprw;
        let mut data_old = [0x0102u16, 0x0304u16, 0x0506u16];
        let data_new = data_old.clone();
        let length = size_of::<u16>();

        // let mut port_orig = soem_sys::ecx_portt {
        //     _bindgen_opaque_blob: [0; 6502],
        // };
        // let mut buf_orig = [0u8; EC_BUFSIZE];
        // let mut data_orig = [0x0102u16, 0x0304u16, 0x0506u16];

        // TODO: Figure out what these are
        let w1 = 0;
        let w2 = 0;

        unsafe {
            ecx_setupdatagram(
                &mut port_old,
                buf_old.as_mut_ptr() as *mut c_void,
                command,
                0,
                w1,
                w2,
                length,
                &mut data_old[0] as *mut _ as *mut c_void,
            )
        };
        unsafe {
            ecx_setupdatagram_new(
                &mut port_new,
                &mut buf_new,
                command,
                0,
                w1,
                w2,
                length,
                &data_new[0] as *const _ as *const c_void,
            )
        };
        // unsafe {
        //     soem_sys::ecx_setupdatagram(
        //         &mut port_orig,
        //         buf_orig.as_mut_ptr() as *mut c_void,
        //         command as u8,
        //         0,
        //         w1,
        //         w2,
        //         length as u16,
        //         &mut data_orig[0] as *mut _ as *mut c_void,
        //     );
        // }

        assert_eq!(port_old.txbuflength, port_new.txbuflength);
        assert_eq!(port_old.txbuf, port_new.txbuf);
        assert_eq!(
            buf_old[0..(port_new.txbuflength[0] as usize + 3)],
            buf_new[0..(port_new.txbuflength[0] as usize + 3)],
            "old, new do not match, idx 0"
        );

        unsafe {
            ecx_adddatagram(
                &mut port_old,
                buf_old.as_mut_ptr() as *mut c_void,
                command,
                0,
                true,
                w1,
                w2,
                length,
                &mut data_old[1] as *mut _ as *mut c_void,
            )
        };
        unsafe {
            ecx_adddatagram_new(
                &mut port_new,
                &mut buf_new,
                command,
                0,
                true,
                w1,
                w2,
                length,
                &data_new[1] as *const _ as *const c_void,
            )
        };
        // unsafe {
        //     soem_sys::ecx_adddatagram(
        //         &mut port_orig,
        //         buf_orig.as_mut_ptr() as *mut c_void,
        //         command as u8,
        //         0,
        //         true as u8,
        //         w1,
        //         w2,
        //         length as u16,
        //         &mut data_orig[0] as *mut _ as *mut c_void,
        //     );
        // }

        // {
        //     #[derive(Debug, Copy, Clone)]
        //     #[repr(C, packed)]
        //     struct Frame {
        //         header: EthercatHeader,
        //         data: u16,
        //     }

        //     #[derive(Debug, Copy, Clone)]
        //     #[repr(C, packed)]
        //     struct Back {
        //         eth_header: EthernetHeader,
        //         frames: [Frame; 2],
        //     }

        //     let back_old = buf_old.as_ptr() as *const u8 as *const Back;
        //     unsafe {
        //         dbg!(*back_old);
        //     }

        //     let back_new = buf_new.as_ptr() as *const u8 as *const Back;
        //     unsafe {
        //         dbg!(*back_new);
        //     }

        //     let back_orig = buf_orig.as_ptr() as *const u8 as *const Back;
        //     unsafe {
        //         dbg!(*back_orig);
        //     }
        // }

        assert_eq!(port_old.txbuflength, port_new.txbuflength);
        assert_eq!(port_old.txbuf, port_new.txbuf);
        assert_eq!(
            buf_old[0..(port_new.txbuflength[0] as usize + 3)],
            buf_new[0..(port_new.txbuflength[0] as usize + 3)],
            "old, new do not match, idx 1"
        );

        unsafe {
            ecx_adddatagram(
                &mut port_old,
                buf_old.as_mut_ptr() as *mut c_void,
                command,
                0,
                false,
                w1,
                w2,
                length,
                &mut data_old[2] as *mut _ as *mut c_void,
            )
        };
        unsafe {
            ecx_adddatagram_new(
                &mut port_new,
                &mut buf_new,
                command,
                0,
                false,
                w1,
                w2,
                length,
                &data_new[2] as *const _ as *const c_void,
            )
        };

        assert_eq!(port_old.txbuflength, port_new.txbuflength);
        assert_eq!(port_old.txbuf, port_new.txbuf);
        assert_eq!(
            buf_old[0..(port_new.txbuflength[0] as usize + 3)],
            buf_new[0..(port_new.txbuflength[0] as usize + 3)],
            "old, new do not match, idx 2"
        );
    }
}
