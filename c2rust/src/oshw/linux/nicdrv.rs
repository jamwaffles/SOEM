use std::mem;

use crate::{
    ethercatmain::ecx_port,
    ethercattype::{ec_bufT, ec_bufstate, ec_comt, ec_etherheadert, htons, ntohs},
    osal::linux::osal::{ec_timet, osal_timer_is_expired, osal_timer_start, osal_timert},
};
use libc::{
    bind, close, ioctl, memcpy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t,
    pthread_mutex_unlock, pthread_mutexattr_init, pthread_mutexattr_t, recv, send, setsockopt,
    sockaddr, sockaddr_ll, socket, socklen_t, strcpy, timeval, IFF_BROADCAST, IFF_PROMISC,
    SOCK_RAW,
};

pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_1,
    pub ifr_ifru: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_1 {
    pub ifrn_name: [libc::c_char; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type boolean = uint8_t;
pub type uint8 = uint8_t;
pub type uint16 = uint16_t;
pub type uint32 = uint32_t;

#[repr(C)]
#[derive(Clone)]
pub struct ec_stackT {
    pub sock: *mut libc::c_int,
    pub txbuf: *mut [ec_bufT; 16],
    pub txbuflength: *mut [libc::c_int; 16],
    pub tempbuf: *mut ec_bufT,
    pub rxbuf: *mut [ec_bufT; 16],
    pub rxbufstat: *mut [libc::c_int; 16],
    pub rxsa: *mut [libc::c_int; 16],
}

#[repr(C)]
#[derive(Clone)]
pub struct ecx_redportt {
    pub stack: ec_stackT,
    pub sockhandle: libc::c_int,
    pub rxbuf: [ec_bufT; 16],
    pub rxbufstat: [libc::c_int; 16],
    pub rxsa: [libc::c_int; 16],
    pub tempinbuf: ec_bufT,
}

#[repr(C)]
#[derive(Clone)]
pub struct ecx_portt {
    pub stack: ec_stackT,
    pub sockhandle: libc::c_int,
    pub rxbuf: [ec_bufT; 16],
    pub rxbufstat: [libc::c_int; 16],
    pub rxsa: [libc::c_int; 16],
    pub tempinbuf: ec_bufT,
    pub tempinbufs: libc::c_int,
    pub txbuf: [ec_bufT; 16],
    pub txbuflength: [libc::c_int; 16],
    pub txbuf2: ec_bufT,
    pub txbuflength2: libc::c_int,
    pub lastidx: uint8,
    pub redstate: libc::c_int,
    pub redport: *mut ecx_redportt,
    pub getindex_mutex: pthread_mutex_t,
    pub tx_mutex: pthread_mutex_t,
    pub rx_mutex: pthread_mutex_t,
}

/* * No redundancy, single NIC mode */
pub const ECT_RED_NONE: C2RustUnnamed_4 = 0;
/* * Double redundant NIC connection */
pub const ECT_RED_DOUBLE: C2RustUnnamed_4 = 1;
/*
 * Licensed under the GNU General Public License version 2 with exceptions. See
 * LICENSE file in the project root for full license information
 */
/* * \file
 * \brief
 * EtherCAT RAW socket driver.
 *
 * Low level interface functions to send and receive EtherCAT packets.
 * EtherCAT has the property that packets are only send by the master,
 * and the send packets always return in the receive buffer.
 * There can be multiple packets "on the wire" before they return.
 * To combine the received packets with the original send packets a buffer
 * system is installed. The identifier is put in the index item of the
 * EtherCAT header. The index is stored and compared when a frame is received.
 * If there is a match the packet can be combined with the transmit packet
 * and returned to the higher level function.
 *
 * The socket layer can exhibit a reversal in the packet order (rare).
 * If the Tx order is A-B-C the return order could be A-C-B. The indexed buffer
 * will reorder the packets automatically.
 *
 * The "redundant" option will configure two sockets and two NIC interfaces.
 * Slaves are connected to both interfaces, one on the IN port and one on the
 * OUT port. Packets are send via both interfaces. Any one of the connections
 * (also an interconnect) can be removed and the slaves are still serviced with
 * packets. The software layer will detect the possible failure modes and
 * compensate. If needed the packets from interface A are resent through interface B.
 * This layer if fully transparent for the higher layers.
 */
/* * Redundancy modes */
pub type C2RustUnnamed_4 = libc::c_uint;
/* * Primary source MAC address used for EtherCAT.
 * This address is not the MAC address used from the NIC.
 * EtherCAT does not care about MAC addressing, but it is used here to
 * differentiate the route the packet traverses through the EtherCAT
 * segment. This is needed to find out the packet flow in redundant
 * configurations. */
#[no_mangle]
pub static mut priMAC: [uint16; 3] = [0x101u16, 0x101u16, 0x101u16];
/* * Secondary source MAC address used for EtherCAT. */
#[no_mangle]
pub static mut secMAC: [uint16; 3] = [0x404u16, 0x404u16, 0x404u16];
unsafe extern "C" fn ecx_clear_rxbufstat(mut rxbufstat: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 16i32 {
        *rxbufstat.offset(i as isize) = ec_bufstate::EC_BUF_EMPTY as libc::c_int;
        i += 1
    }
}
/* * Basic setup to connect NIC to socket.
 * @param[in] port        = port context struct
 * @param[in] ifname      = Name of NIC device, f.e. "eth0"
 * @param[in] secondary   = if >0 then use secondary stack instead of primary
 * @return >0 if succeeded
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_setupnic(
    mut port: *mut ecx_portt,
    mut ifname: *const libc::c_char,
    mut secondary: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut rval: libc::c_int = 0;
    let mut ifindex: libc::c_int = 0;
    let mut timeout: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ifr: ifreq = ifreq {
        ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
        ifr_ifru: C2RustUnnamed_0 {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    let mut sll: sockaddr_ll = sockaddr_ll {
        sll_family: 0,
        sll_protocol: 0,
        sll_ifindex: 0,
        sll_hatype: 0,
        sll_pkttype: 0,
        sll_halen: 0,
        sll_addr: [0; 8],
    };
    let mut psock: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mutexattr: pthread_mutexattr_t = mem::zeroed();
    rval = 0i32;
    if secondary != 0 {
        /* secondary port struct available? */
        if !(*port).redport.is_null() {
            /* when using secondary socket it is automatically a redundant setup */
            psock = &mut (*(*port).redport).sockhandle;
            *psock = -(1i32);
            (*port).redstate = ECT_RED_DOUBLE as libc::c_int;
            (*(*port).redport).stack.sock = &mut (*(*port).redport).sockhandle;
            (*(*port).redport).stack.txbuf = &mut (*port).txbuf;
            (*(*port).redport).stack.txbuflength = &mut (*port).txbuflength;
            (*(*port).redport).stack.tempbuf = &mut (*(*port).redport).tempinbuf;
            (*(*port).redport).stack.rxbuf = &mut (*(*port).redport).rxbuf;
            (*(*port).redport).stack.rxbufstat = &mut (*(*port).redport).rxbufstat;
            (*(*port).redport).stack.rxsa = &mut (*(*port).redport).rxsa;
            ecx_clear_rxbufstat(&mut *(*(*port).redport).rxbufstat.as_mut_ptr().offset(0isize));
        } else {
            /* fail */
            return 0i32;
        }
    } else {
        pthread_mutexattr_init(&mut mutexattr);
        pthread_mutexattr_setprotocol(&mut mutexattr, PTHREAD_PRIO_INHERIT as libc::c_int);
        pthread_mutex_init(&mut (*port).getindex_mutex, &mut mutexattr);
        pthread_mutex_init(&mut (*port).tx_mutex, &mut mutexattr);
        pthread_mutex_init(&mut (*port).rx_mutex, &mut mutexattr);
        (*port).sockhandle = -(1i32);
        (*port).lastidx = 0u8;
        (*port).redstate = ECT_RED_NONE as libc::c_int;
        (*port).stack.sock = &mut (*port).sockhandle;
        (*port).stack.txbuf = &mut (*port).txbuf;
        (*port).stack.txbuflength = &mut (*port).txbuflength;
        (*port).stack.tempbuf = &mut (*port).tempinbuf;
        (*port).stack.rxbuf = &mut (*port).rxbuf;
        (*port).stack.rxbufstat = &mut (*port).rxbufstat;
        (*port).stack.rxsa = &mut (*port).rxsa;
        ecx_clear_rxbufstat(&mut *(*port).rxbufstat.as_mut_ptr().offset(0isize));
        psock = &mut (*port).sockhandle
    }
    /* we use RAW packet socket, with packet type ETH_P_ECAT */
    *psock = socket(
        17i32,
        SOCK_RAW as libc::c_int,
        htons(0x88a4u16) as libc::c_int,
    );
    timeout.tv_sec = 0i64;
    timeout.tv_usec = 1i64;
    r = setsockopt(
        *psock,
        1i32,
        20i32,
        &mut timeout as *mut timeval as *const libc::c_void,
        ::core::mem::size_of::<timeval>() as socklen_t,
    );
    r = setsockopt(
        *psock,
        1i32,
        21i32,
        &mut timeout as *mut timeval as *const libc::c_void,
        ::core::mem::size_of::<timeval>() as socklen_t,
    );
    i = 1i32;
    r = setsockopt(
        *psock,
        1i32,
        5i32,
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as socklen_t,
    );
    /* connect socket to NIC by name */
    strcpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
    r = ioctl(*psock, 0x8933u64, &mut ifr as *mut ifreq);
    ifindex = ifr.ifr_ifru.ifru_ivalue;
    strcpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
    ifr.ifr_ifru.ifru_flags = 0i16;
    /* reset flags of NIC interface */
    r = ioctl(*psock, 0x8913u64, &mut ifr as *mut ifreq);
    /* set flags of NIC interface, here promiscuous and broadcast */
    ifr.ifr_ifru.ifru_flags = (ifr.ifr_ifru.ifru_flags as libc::c_int
        | IFF_PROMISC as libc::c_int
        | IFF_BROADCAST as libc::c_int) as libc::c_short;
    r = ioctl(*psock, 0x8914u64, &mut ifr as *mut ifreq);
    /* bind socket to protocol, in this case RAW EtherCAT */
    sll.sll_family = 17u16;
    sll.sll_ifindex = ifindex;
    sll.sll_protocol = htons(0x88a4u16);
    r = bind(
        *psock,
        &mut sll as *mut sockaddr_ll as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_ll>() as socklen_t,
    );
    /* setup ethernet headers in tx buffers so we don't have to repeat it */
    i = 0i32;
    while i < 16i32 {
        ec_setupheader(
            &mut *(*port).txbuf.as_mut_ptr().offset(i as isize) as *mut ec_bufT
                as *mut libc::c_void,
        );
        (*port).rxbufstat[i as usize] = ec_bufstate::EC_BUF_EMPTY as libc::c_int;
        i += 1
    }
    ec_setupheader(&mut (*port).txbuf2 as *mut ec_bufT as *mut libc::c_void);
    if r == 0i32 {
        rval = 1i32
    }
    return rval;
}
/* * Close sockets used
 * @param[in] port        = port context struct
 * @return 0
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_closenic(mut port: *mut ecx_portt) -> libc::c_int {
    if (*port).sockhandle >= 0i32 {
        close((*port).sockhandle);
    }
    if !(*port).redport.is_null() && (*(*port).redport).sockhandle >= 0i32 {
        close((*(*port).redport).sockhandle);
    }
    return 0i32;
}
/* * Fill buffer with ethernet header structure.
 * Destination MAC is always broadcast.
 * Ethertype is always ETH_P_ECAT.
 * @param[out] p = buffer
 */
#[no_mangle]
pub unsafe extern "C" fn ec_setupheader(mut p: *mut libc::c_void) {
    let mut bp: *mut ec_etherheadert = 0 as *mut ec_etherheadert;
    bp = p as *mut ec_etherheadert;
    (*bp).da0 = htons(0xffffu16);
    (*bp).da1 = htons(0xffffu16);
    (*bp).da2 = htons(0xffffu16);
    (*bp).sa0 = htons(priMAC[0usize]);
    (*bp).sa1 = htons(priMAC[1usize]);
    (*bp).sa2 = htons(priMAC[2usize]);
    (*bp).etype = htons(0x88a4u16);
}
/* * Get new frame identifier index and allocate corresponding rx buffer.
 * @param[in] port        = port context struct
 * @return new index.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_getindex(mut port: *mut ecx_portt) -> uint8 {
    let mut idx: uint8 = 0;
    let mut cnt: uint8 = 0;
    pthread_mutex_lock(&mut (*port).getindex_mutex);
    idx = ((*port).lastidx as libc::c_int + 1i32) as uint8;
    /* index can't be larger than buffer array */
    if idx as libc::c_int >= 16i32 {
        idx = 0u8
    }
    cnt = 0u8;
    /* try to find unused index */
    while (*port).rxbufstat[idx as usize] != ec_bufstate::EC_BUF_EMPTY as libc::c_int
        && (cnt as libc::c_int) < 16i32
    {
        idx = idx.wrapping_add(1);
        cnt = cnt.wrapping_add(1);
        if idx as libc::c_int >= 16i32 {
            idx = 0u8
        }
    }
    (*port).rxbufstat[idx as usize] = ec_bufstate::EC_BUF_ALLOC as libc::c_int;
    if (*port).redstate != ECT_RED_NONE as libc::c_int {
        (*(*port).redport).rxbufstat[idx as usize] = ec_bufstate::EC_BUF_ALLOC as libc::c_int
    }
    (*port).lastidx = idx;
    pthread_mutex_unlock(&mut (*port).getindex_mutex);
    return idx;
}
/* * Set rx buffer status.
 * @param[in] port        = port context struct
 * @param[in] idx      = index in buffer array
 * @param[in] bufstat  = status to set
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_setbufstat(
    mut port: *mut ecx_portt,
    mut idx: uint8,
    mut bufstat: libc::c_int,
) {
    (*port).rxbufstat[idx as usize] = bufstat;
    if (*port).redstate != ECT_RED_NONE as libc::c_int {
        (*(*port).redport).rxbufstat[idx as usize] = bufstat
    };
}
/* * Transmit buffer over socket (non blocking).
 * @param[in] port        = port context struct
 * @param[in] idx         = index in tx buffer array
 * @param[in] stacknumber  = 0=Primary 1=Secondary stack
 * @return socket send result
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_outframe(
    mut port: *mut ecx_portt,
    mut idx: uint8,
    mut stacknumber: libc::c_int,
) -> libc::c_int {
    let mut lp: libc::c_int = 0;
    let mut rval: libc::c_int = 0;
    let mut stack: *mut ec_stackT = 0 as *mut ec_stackT;
    if stacknumber == 0 {
        stack = &mut (*port).stack
    } else {
        stack = &mut (*(*port).redport).stack
    }
    lp = (*(*stack).txbuflength)[idx as usize];
    (*(*stack).rxbufstat)[idx as usize] = ec_bufstate::EC_BUF_TX as libc::c_int;
    rval = send(
        *(*stack).sock,
        (*(*stack).txbuf)[idx as usize].as_mut_ptr() as *const libc::c_void,
        lp as size_t,
        0i32,
    ) as libc::c_int;
    if rval == -(1i32) {
        (*(*stack).rxbufstat)[idx as usize] = ec_bufstate::EC_BUF_EMPTY as libc::c_int
    }
    return rval;
}
/* * Transmit buffer over socket (non blocking).
 * @param[in] port        = port context struct
 * @param[in] idx = index in tx buffer array
 * @return socket send result
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_outframe_red(mut port: *mut ecx_portt, mut idx: uint8) -> libc::c_int {
    let mut datagramP: *mut ec_comt = 0 as *mut ec_comt;
    let mut ehp: *mut ec_etherheadert = 0 as *mut ec_etherheadert;
    let mut rval: libc::c_int = 0;
    ehp = &mut *(*port).txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
        as *mut ec_etherheadert;
    /* rewrite MAC source address 1 to primary */
    (*ehp).sa1 = htons(priMAC[1usize]);
    /* transmit over primary socket*/
    rval = ecx_outframe(port, idx, 0i32);
    if (*port).redstate != ECT_RED_NONE as libc::c_int {
        pthread_mutex_lock(&mut (*port).tx_mutex);
        ehp = &mut (*port).txbuf2 as *mut ec_bufT as *mut ec_etherheadert;
        /* use dummy frame for secondary socket transmit (BRD) */
        datagramP = &mut *(*port)
            .txbuf2
            .as_mut_ptr()
            .offset(::core::mem::size_of::<ec_etherheadert>() as isize)
            as *mut uint8 as *mut ec_comt;
        /* write index to frame */
        (*datagramP).index = idx;
        /* rewrite MAC source address 1 to secondary */
        (*ehp).sa1 = htons(secMAC[1usize]);
        /* transmit over secondary socket */
        (*(*port).redport).rxbufstat[idx as usize] = ec_bufstate::EC_BUF_TX as libc::c_int;
        if send(
            (*(*port).redport).sockhandle,
            &mut (*port).txbuf2 as *mut ec_bufT as *const libc::c_void,
            (*port).txbuflength2 as size_t,
            0i32,
        ) == -1isize
        {
            (*(*port).redport).rxbufstat[idx as usize] = ec_bufstate::EC_BUF_EMPTY as libc::c_int
        }
        pthread_mutex_unlock(&mut (*port).tx_mutex);
    }
    return rval;
}
/* * Non blocking read of socket. Put frame in temporary buffer.
 * @param[in] port        = port context struct
 * @param[in] stacknumber = 0=primary 1=secondary stack
 * @return >0 if frame is available and read
 */
unsafe extern "C" fn ecx_recvpkt(
    mut port: *mut ecx_portt,
    mut stacknumber: libc::c_int,
) -> libc::c_int {
    let mut lp: libc::c_int = 0;
    let mut bytesrx: libc::c_int = 0;
    let mut stack: *mut ec_stackT = 0 as *mut ec_stackT;
    if stacknumber == 0 {
        stack = &mut (*port).stack
    } else {
        stack = &mut (*(*port).redport).stack
    }
    lp = ::core::mem::size_of::<ec_bufT>() as libc::c_int;
    bytesrx = recv(
        *(*stack).sock,
        (*(*stack).tempbuf).as_mut_ptr() as *mut libc::c_void,
        lp as size_t,
        0i32,
    ) as libc::c_int;
    (*port).tempinbufs = bytesrx;
    return (bytesrx > 0i32) as libc::c_int;
}
/* * Non blocking receive frame function. Uses RX buffer and index to combine
 * read frame with transmitted frame. To compensate for received frames that
 * are out-of-order all frames are stored in their respective indexed buffer.
 * If a frame was placed in the buffer previously, the function retrieves it
 * from that buffer index without calling ec_recvpkt. If the requested index
 * is not already in the buffer it calls ec_recvpkt to fetch it. There are
 * three options now, 1 no frame read, so exit. 2 frame read but other
 * than requested index, store in buffer and exit. 3 frame read with matching
 * index, store in buffer, set completed flag in buffer status and exit.
 *
 * @param[in] port        = port context struct
 * @param[in] idx         = requested index of frame
 * @param[in] stacknumber = 0=primary 1=secondary stack
 * @return Workcounter if a frame is found with corresponding index, otherwise
 * EC_NOFRAME or EC_OTHERFRAME.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_inframe(
    mut port: *mut ecx_portt,
    mut idx: uint8,
    mut stacknumber: libc::c_int,
) -> libc::c_int {
    let mut l: uint16 = 0;
    let mut rval: libc::c_int = 0;
    let mut idxf: uint8 = 0;
    let mut ehp: *mut ec_etherheadert = 0 as *mut ec_etherheadert;
    let mut ecp: *mut ec_comt = 0 as *mut ec_comt;
    let mut stack: *mut ec_stackT = 0 as *mut ec_stackT;
    let mut rxbuf: *mut ec_bufT = 0 as *mut ec_bufT;
    if stacknumber == 0 {
        stack = &mut (*port).stack
    } else {
        stack = &mut (*(*port).redport).stack
    }
    rval = -(1i32);
    rxbuf = &mut *(*(*stack).rxbuf).as_mut_ptr().offset(idx as isize) as *mut ec_bufT;
    /* check if requested index is already in buffer ? */
    if (idx as libc::c_int) < 16i32
        && (*(*stack).rxbufstat)[idx as usize] == ec_bufstate::EC_BUF_RCVD as libc::c_int
    {
        l = ((*rxbuf)[0usize] as libc::c_int
            + ((((*rxbuf)[1usize] as libc::c_int & 0xfi32) as uint16 as libc::c_int) << 8i32))
            as uint16;
        /* return WKC */
        rval = (*rxbuf)[l as usize] as libc::c_int
            + (((*rxbuf)[(l as libc::c_int + 1i32) as usize] as libc::c_int) << 8i32);
        /* mark as completed */
        (*(*stack).rxbufstat)[idx as usize] = ec_bufstate::EC_BUF_COMPLETE as libc::c_int
    } else {
        pthread_mutex_lock(&mut (*port).rx_mutex);
        /* non blocking call to retrieve frame from socket */
        if ecx_recvpkt(port, stacknumber) != 0 {
            rval = -(2i32);
            ehp = (*stack).tempbuf as *mut ec_etherheadert;
            /* check if it is an EtherCAT frame */
            if (*ehp).etype as libc::c_int == htons(0x88a4u16) as libc::c_int {
                ecp = &mut *(*(*stack).tempbuf)
                    .as_mut_ptr()
                    .offset(::core::mem::size_of::<ec_etherheadert>() as isize)
                    as *mut uint8 as *mut ec_comt;
                l = ((*ecp).elength as libc::c_int & 0xfffi32) as uint16;
                idxf = (*ecp).index;
                /* found index equals requested index ? */
                if idxf as libc::c_int == idx as libc::c_int {
                    /* yes, put it in the buffer array (strip ethernet header) */
                    memcpy(
                        rxbuf as *mut libc::c_void,
                        &mut *(*(*stack).tempbuf)
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<ec_etherheadert>() as isize)
                            as *mut uint8 as *const libc::c_void,
                        ((*(*stack).txbuflength)[idx as usize] as usize)
                            .wrapping_sub(core::mem::size_of::<ec_etherheadert>()),
                    );
                    /* return WKC */
                    rval = (*rxbuf)[l as usize] as libc::c_int
                        + (((*rxbuf)[(l as libc::c_int + 1i32) as usize] as libc::c_int) << 8i32);
                    /* mark as completed */
                    (*(*stack).rxbufstat)[idx as usize] =
                        ec_bufstate::EC_BUF_COMPLETE as libc::c_int;
                    /* store MAC source word 1 for redundant routing info */
                    (*(*stack).rxsa)[idx as usize] = ntohs((*ehp).sa1) as libc::c_int
                } else if (idxf as libc::c_int) < 16i32
                    && (*(*stack).rxbufstat)[idxf as usize] == ec_bufstate::EC_BUF_TX as libc::c_int
                {
                    rxbuf =
                        &mut *(*(*stack).rxbuf).as_mut_ptr().offset(idxf as isize) as *mut ec_bufT;
                    /* check if index exist and someone is waiting for it */
                    /* put it in the buffer array (strip ethernet header) */
                    memcpy(
                        rxbuf as *mut libc::c_void,
                        &mut *(*(*stack).tempbuf)
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<ec_etherheadert>() as isize)
                            as *mut uint8 as *const libc::c_void,
                        ((*(*stack).txbuflength)[idxf as usize] as usize)
                            .wrapping_sub(core::mem::size_of::<ec_etherheadert>()),
                    );
                    /* mark as received */
                    (*(*stack).rxbufstat)[idxf as usize] = ec_bufstate::EC_BUF_RCVD as libc::c_int;
                    (*(*stack).rxsa)[idxf as usize] = ntohs((*ehp).sa1) as libc::c_int
                }
            }
        }
        pthread_mutex_unlock(&mut (*port).rx_mutex);
    }
    /* WKC if matching frame found */
    return rval;
}
/* * Blocking redundant receive frame function. If redundant mode is not active then
 * it skips the secondary stack and redundancy functions. In redundant mode it waits
 * for both (primary and secondary) frames to come in. The result goes in an decision
 * tree that decides, depending on the route of the packet and its possible missing arrival,
 * how to reroute the original packet to get the data in an other try.
 *
 * @param[in] port        = port context struct
 * @param[in] idx = requested index of frame
 * @param[in] timer = absolute timeout time
 * @return Workcounter if a frame is found with corresponding index, otherwise
 * EC_NOFRAME.
 */
unsafe extern "C" fn ecx_waitinframe_red(
    mut port: *mut ecx_portt,
    mut idx: uint8,
    mut timer: *mut osal_timert,
) -> libc::c_int {
    let mut timer2: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    let mut wkc: libc::c_int = -(1i32);
    let mut wkc2: libc::c_int = -(1i32);
    let mut primrx: libc::c_int = 0;
    let mut secrx: libc::c_int = 0;
    /* if not in redundant mode then always assume secondary is OK */
    if (*port).redstate == ECT_RED_NONE as libc::c_int {
        wkc2 = 0i32
    }
    loop {
        /* only read frame if not already in */
        if wkc <= -(1i32) {
            wkc = ecx_inframe(port, idx, 0i32)
        }
        /* wait for both frames to arrive or timeout */
        if (*port).redstate != ECT_RED_NONE as libc::c_int {
            /* only try secondary if in redundant mode */
            /* only read frame if not already in */
            if wkc2 <= -(1i32) {
                wkc2 = ecx_inframe(port, idx, 1i32)
            }
        }
        if !((wkc <= -(1i32) || wkc2 <= -(1i32)) && osal_timer_is_expired(timer) == 0) {
            break;
        }
    }
    /* only do redundant functions when in redundant mode */
    if (*port).redstate != ECT_RED_NONE as libc::c_int {
        /* primrx if the received MAC source on primary socket */
        primrx = 0i32;
        if wkc > -(1i32) {
            primrx = (*port).rxsa[idx as usize]
        }
        /* secrx if the received MAC source on psecondary socket */
        secrx = 0i32;
        if wkc2 > -(1i32) {
            secrx = (*(*port).redport).rxsa[idx as usize]
        }
        /* primary socket got secondary frame and secondary socket got primary frame */
        /* normal situation in redundant mode */
        if primrx == secMAC[1usize] as libc::c_int && secrx == priMAC[1usize] as libc::c_int {
            /* copy secondary buffer to primary */
            memcpy(
                &mut *(*port).rxbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                    as *mut libc::c_void,
                &mut *(*(*port).redport).rxbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                    as *const libc::c_void,
                ((*port).txbuflength[idx as usize] as usize)
                    .wrapping_sub(core::mem::size_of::<ec_etherheadert>()),
            );
            wkc = wkc2
        }
        /* primary socket got nothing or primary frame, and secondary socket got secondary frame */
        /* we need to resend TX packet */
        if primrx == 0i32 && secrx == secMAC[1usize] as libc::c_int
            || primrx == priMAC[1usize] as libc::c_int && secrx == secMAC[1usize] as libc::c_int
        {
            /* If both primary and secondary have partial connection retransmit the primary received
             * frame over the secondary socket. The result from the secondary received frame is a combined
             * frame that traversed all slaves in standard order. */
            if primrx == priMAC[1usize] as libc::c_int && secrx == secMAC[1usize] as libc::c_int {
                /* copy primary rx to tx buffer */
                memcpy(
                    &mut *(*(*port).txbuf.as_mut_ptr().offset(idx as isize))
                        .as_mut_ptr()
                        .offset(::core::mem::size_of::<ec_etherheadert>() as isize)
                        as *mut uint8 as *mut libc::c_void,
                    &mut *(*port).rxbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                        as *const libc::c_void,
                    ((*port).txbuflength[idx as usize] as usize)
                        .wrapping_sub(core::mem::size_of::<ec_etherheadert>()),
                );
            }
            osal_timer_start(&mut timer2, 2000u32);
            /* resend secondary tx */
            ecx_outframe(port, idx, 1i32);
            loop {
                /* retrieve frame */
                wkc2 = ecx_inframe(port, idx, 1i32);
                if !(wkc2 <= -(1i32) && osal_timer_is_expired(&mut timer2) == 0) {
                    break;
                }
            }
            if wkc2 > -(1i32) {
                /* copy secondary result to primary rx buffer */
                memcpy(
                    &mut *(*port).rxbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                        as *mut libc::c_void,
                    &mut *(*(*port).redport).rxbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                        as *const libc::c_void,
                    ((*port).txbuflength[idx as usize] as usize)
                        .wrapping_sub(core::mem::size_of::<ec_etherheadert>()),
                );
                wkc = wkc2
            }
        }
    }
    /* return WKC or EC_NOFRAME */
    return wkc;
}
/* * Blocking receive frame function. Calls ec_waitinframe_red().
 * @param[in] port        = port context struct
 * @param[in] idx       = requested index of frame
 * @param[in] timeout   = timeout in us
 * @return Workcounter if a frame is found with corresponding index, otherwise
 * EC_NOFRAME.
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_waitinframe(
    mut port: *mut ecx_portt,
    mut idx: uint8,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer, timeout as uint32);
    wkc = ecx_waitinframe_red(port, idx, &mut timer);
    return wkc;
}
/* * Blocking send and receive frame function. Used for non processdata frames.
 * A datagram is build into a frame and transmitted via this function. It waits
 * for an answer and returns the workcounter. The function retries if time is
 * left and the result is WKC=0 or no frame received.
 *
 * The function calls ec_outframe_red() and ec_waitinframe_red().
 *
 * @param[in] port        = port context struct
 * @param[in] idx      = index of frame
 * @param[in] timeout  = timeout in us
 * @return Workcounter or EC_NOFRAME
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_srconfirm(
    mut port: *mut ecx_portt,
    mut idx: uint8,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut wkc: libc::c_int = -(1i32);
    let mut timer1: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    let mut timer2: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer1, timeout as uint32);
    loop {
        /* tx frame on primary and if in redundant mode a dummy on secondary */
        ecx_outframe_red(port, idx);
        if timeout < 2000i32 {
            osal_timer_start(&mut timer2, timeout as uint32);
        } else {
            /* wait for answer with WKC>=0 or otherwise retry until timeout */
            /* normally use partial timeout for rx */
            osal_timer_start(&mut timer2, 2000u32);
        }
        wkc = ecx_waitinframe_red(port, idx, &mut timer2);
        if !(wkc <= -(1i32) && osal_timer_is_expired(&mut timer1) == 0) {
            break;
        }
    }
    return wkc;
}
#[no_mangle]
pub unsafe extern "C" fn ec_setupnic(
    mut ifname: *const libc::c_char,
    mut secondary: libc::c_int,
) -> libc::c_int {
    return ecx_setupnic(&mut ecx_port, ifname, secondary);
}
#[no_mangle]
pub unsafe extern "C" fn ec_closenic() -> libc::c_int {
    return ecx_closenic(&mut ecx_port);
}
#[no_mangle]
pub unsafe extern "C" fn ec_getindex() -> uint8 {
    return ecx_getindex(&mut ecx_port);
}
#[no_mangle]
pub unsafe extern "C" fn ec_setbufstat(mut idx: uint8, mut bufstat: libc::c_int) {
    ecx_setbufstat(&mut ecx_port, idx, bufstat);
}
#[no_mangle]
pub unsafe extern "C" fn ec_outframe(mut idx: uint8, mut stacknumber: libc::c_int) -> libc::c_int {
    return ecx_outframe(&mut ecx_port, idx, stacknumber);
}
#[no_mangle]
pub unsafe extern "C" fn ec_outframe_red(mut idx: uint8) -> libc::c_int {
    return ecx_outframe_red(&mut ecx_port, idx);
}
#[no_mangle]
pub unsafe extern "C" fn ec_inframe(mut idx: uint8, mut stacknumber: libc::c_int) -> libc::c_int {
    return ecx_inframe(&mut ecx_port, idx, stacknumber);
}
#[no_mangle]
pub unsafe extern "C" fn ec_waitinframe(mut idx: uint8, mut timeout: libc::c_int) -> libc::c_int {
    return ecx_waitinframe(&mut ecx_port, idx, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn ec_srconfirm(mut idx: uint8, mut timeout: libc::c_int) -> libc::c_int {
    return ecx_srconfirm(&mut ecx_port, idx, timeout);
}
/* get frame from primary or if in redundant mode possibly from secondary */
