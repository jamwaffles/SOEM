use std::mem;

use crate::{
    main::ecx_port,
    osal::linux::osal::{ec_timet, osal_timer_is_expired, osal_timer_start, osal_timert},
    types::{
        ec_bufT, htons, ntohs, BufferState, EthercatHeader, EthernetHeader, EC_BUFSIZE, EC_MAXBUF,
        EC_NOFRAME, EC_OTHERFRAME, EC_TIMEOUTRET, ETH_HEADERSIZE,
    },
};
use libc::{
    bind, close, ioctl, memcpy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t,
    pthread_mutex_unlock, pthread_mutexattr_init, pthread_mutexattr_t, recv, send, setsockopt,
    sockaddr, sockaddr_ll, socket, socklen_t, strcpy, timeval, IFF_BROADCAST, IFF_PROMISC,
    SOCK_RAW,
};

#[derive(Copy, Clone)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}

#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_1,
    pub ifr_ifru: C2RustUnnamed_0,
}

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
    pub ifru_data: *mut libc::c_char,
}

#[derive(Copy, Clone)]
pub union C2RustUnnamed_1 {
    pub ifrn_name: [libc::c_char; 16],
}

#[derive(Clone)]
pub struct ec_stackT {
    pub sock: *mut libc::c_int,
    pub txbuf: *mut [ec_bufT; EC_MAXBUF as usize],
    pub txbuflength: *mut [libc::c_int; EC_MAXBUF as usize],
    pub tempbuf: *mut ec_bufT,
    pub rxbuf: *mut [ec_bufT; EC_MAXBUF as usize],
    pub rxbufstat: *mut [libc::c_int; EC_MAXBUF as usize],
    pub rxsa: *mut [libc::c_int; EC_MAXBUF as usize],
}

impl Default for ec_stackT {
    fn default() -> Self {
        Self {
            sock: 0 as *mut i32,
            txbuf: 0 as *mut [ec_bufT; EC_MAXBUF as usize],
            txbuflength: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
            tempbuf: 0 as *mut ec_bufT,
            rxbuf: 0 as *mut [ec_bufT; EC_MAXBUF as usize],
            rxbufstat: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
            rxsa: 0 as *mut [libc::c_int; EC_MAXBUF as usize],
        }
    }
}

#[derive(Clone)]
pub struct ecx_redportt {
    pub stack: ec_stackT,
    pub sockhandle: libc::c_int,
    pub rxbuf: [ec_bufT; EC_MAXBUF as usize],
    pub rxbufstat: [libc::c_int; EC_MAXBUF as usize],
    pub rxsa: [libc::c_int; EC_MAXBUF as usize],
    pub tempinbuf: ec_bufT,
}

#[derive(Clone)]
pub struct ecx_portt {
    pub stack: ec_stackT,
    pub sockhandle: libc::c_int,
    pub rxbuf: [ec_bufT; EC_MAXBUF as usize],
    pub rxbufstat: [libc::c_int; EC_MAXBUF as usize],
    pub rxsa: [libc::c_int; EC_MAXBUF as usize],
    pub tempinbuf: ec_bufT,
    pub tempinbufs: libc::c_int,
    pub txbuf: [ec_bufT; EC_MAXBUF as usize],
    pub txbuflength: [libc::c_int; EC_MAXBUF as usize],
    pub txbuf2: ec_bufT,
    pub txbuflength2: libc::c_int,
    pub lastidx: u8,
    pub redport: Option<ecx_redportt>,
    pub getindex_mutex: pthread_mutex_t,
    pub tx_mutex: pthread_mutex_t,
    pub rx_mutex: pthread_mutex_t,
}

impl Default for ecx_portt {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            sockhandle: Default::default(),
            rxbuf: [[0; EC_BUFSIZE as usize]; EC_MAXBUF as usize],
            rxbufstat: Default::default(),
            rxsa: Default::default(),
            tempinbuf: [0; EC_BUFSIZE as usize],
            tempinbufs: Default::default(),
            txbuf: [[0; EC_BUFSIZE as usize]; EC_MAXBUF as usize],
            txbuflength: Default::default(),
            txbuf2: [0; EC_BUFSIZE as usize],
            txbuflength2: Default::default(),
            lastidx: Default::default(),
            redport: Default::default(),
            getindex_mutex: unsafe { mem::zeroed() },
            tx_mutex: unsafe { mem::zeroed() },
            rx_mutex: unsafe { mem::zeroed() },
        }
    }
}

/// Redundancy mode.
enum NicMode {
    /// No redundancy, single NIC mode.
    None = 0,
    /// Double redundant NIC connection.
    Double = 1,
}

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

/* * Primary source MAC address used for EtherCAT.
 * This address is not the MAC address used from the NIC.
 * EtherCAT does not care about MAC addressing, but it is used here to
 * differentiate the route the packet traverses through the EtherCAT
 * segment. This is needed to find out the packet flow in redundant
 * configurations. */
#[no_mangle]
pub static mut priMAC: [u16; 3] = [0x101u16, 0x101u16, 0x101u16];
/* * Secondary source MAC address used for EtherCAT. */
#[no_mangle]
pub static mut secMAC: [u16; 3] = [0x404u16, 0x404u16, 0x404u16];
unsafe fn ecx_clear_rxbufstat(rxbufstat: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 16i32 {
        *rxbufstat.offset(i as isize) = BufferState::Empty as libc::c_int;
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
pub unsafe fn ecx_setupnic(
    port: &mut ecx_portt,
    ifname: *const libc::c_char,
    secondary: libc::c_int,
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
        if let Some(redport) = port.redport.as_mut() {
            /* when using secondary socket it is automatically a redundant setup */
            psock = &mut redport.sockhandle;
            *psock = -1;
            redport.stack.sock = &mut redport.sockhandle;
            redport.stack.txbuf = &mut (*port).txbuf;
            redport.stack.txbuflength = &mut (*port).txbuflength;
            redport.stack.tempbuf = &mut redport.tempinbuf;
            redport.stack.rxbuf = &mut redport.rxbuf;
            redport.stack.rxbufstat = &mut redport.rxbufstat;
            redport.stack.rxsa = &mut redport.rxsa;
            ecx_clear_rxbufstat(&mut *redport.rxbufstat.as_mut_ptr().offset(0isize));
        } else {
            /* fail */
            return 0i32;
        }
    } else {
        pthread_mutexattr_init(&mut mutexattr);
        // FIXME
        // pthread_mutexattr_setprotocol(&mut mutexattr, PTHREAD_PRIO_INHERIT as libc::c_int);
        pthread_mutex_init(&mut (*port).getindex_mutex, &mut mutexattr);
        pthread_mutex_init(&mut (*port).tx_mutex, &mut mutexattr);
        pthread_mutex_init(&mut (*port).rx_mutex, &mut mutexattr);
        (*port).sockhandle = -1;
        (*port).lastidx = 0;
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
        (*port).rxbufstat[i as usize] = BufferState::Empty as libc::c_int;
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
pub unsafe fn ecx_closenic(port: &mut ecx_portt) -> libc::c_int {
    if (*port).sockhandle >= 0i32 {
        close((*port).sockhandle);
    }

    if let Some(redport) = port.redport.as_mut() {
        if redport.sockhandle >= 0 {
            close(redport.sockhandle);
        }
    }

    return 0i32;
}
/* * Fill buffer with ethernet header structure.
 * Destination MAC is always broadcast.
 * Ethertype is always ETH_P_ECAT.
 * @param[out] p = buffer
 */
#[no_mangle]
pub unsafe fn ec_setupheader(p: *mut libc::c_void) {
    let mut bp: *mut EthernetHeader = 0 as *mut EthernetHeader;
    bp = p as *mut EthernetHeader;
    (*bp).da0 = htons(0xffffu16);
    (*bp).da1 = htons(0xffffu16);
    (*bp).da2 = htons(0xffffu16);
    (*bp).sa0 = htons(priMAC[0]);
    (*bp).sa1 = htons(priMAC[1]);
    (*bp).sa2 = htons(priMAC[2]);
    (*bp).etype = htons(0x88a4u16);
}
/* * Get new frame identifier index and allocate corresponding rx buffer.
 * @param[in] port        = port context struct
 * @return new index.
 */
#[no_mangle]
pub unsafe fn ecx_getindex(port: &mut ecx_portt) -> u8 {
    pthread_mutex_lock(&mut port.getindex_mutex);
    let mut idx = port.lastidx + 1;
    /* index can't be larger than buffer array */
    if idx >= EC_MAXBUF {
        idx = 0
    }
    let mut cnt = 0;
    /* try to find unused index */
    while port.rxbufstat[idx as usize] != BufferState::Empty as libc::c_int && cnt < EC_MAXBUF {
        idx += 1;
        cnt += 1;
        if idx >= EC_MAXBUF {
            idx = 0
        }
    }
    port.rxbufstat[idx as usize] = BufferState::Allocated as i32;

    if let Some(redport) = port.redport.as_mut() {
        redport.rxbufstat[idx as usize] = BufferState::Allocated as libc::c_int;
    }

    port.lastidx = idx;
    pthread_mutex_unlock(&mut port.getindex_mutex);
    return idx;
}
/* * Set rx buffer status.
 * @param[in] port        = port context struct
 * @param[in] idx      = index in buffer array
 * @param[in] bufstat  = status to set
 */
#[no_mangle]
pub fn ecx_setbufstat(port: &mut ecx_portt, idx: u8, bufstat: BufferState) {
    port.rxbufstat[idx as usize] = bufstat as i32;

    if let Some(redport) = port.redport.as_mut() {
        redport.rxbufstat[idx as usize] = bufstat as i32;
    }
}
/* * Transmit buffer over socket (non blocking).
 * @param[in] port        = port context struct
 * @param[in] idx         = index in tx buffer array
 * @param[in] stacknumber  = 0=Primary 1=Secondary stack
 * @return socket send result
 */
#[no_mangle]
pub unsafe fn ecx_outframe(port: &ecx_portt, idx: u8, stacknumber: libc::c_int) -> libc::c_int {
    let mut lp: libc::c_int = 0;
    let mut rval: libc::c_int = 0;
    // FIXME: This is actually mut
    let stack = if stacknumber == 0 {
        &port.stack
    } else {
        port.redport.as_ref().map(|redport| &redport.stack).unwrap()
    };
    lp = (*stack.txbuflength)[idx as usize];
    (*stack.rxbufstat)[idx as usize] = BufferState::Transmitted as libc::c_int;
    rval = send(
        *stack.sock,
        (*stack.txbuf)[idx as usize].as_mut_ptr() as *const libc::c_void,
        lp as usize,
        0i32,
    ) as libc::c_int;
    if rval == -1 {
        (*stack.rxbufstat)[idx as usize] = BufferState::Empty as libc::c_int
    }
    return rval;
}
/* * Transmit buffer over socket (non blocking).
 * @param[in] port        = port context struct
 * @param[in] idx = index in tx buffer array
 * @return socket send result
 */
#[no_mangle]
pub unsafe fn ecx_outframe_red(port: &mut ecx_portt, idx: u8) -> libc::c_int {
    let mut datagramP: *mut EthercatHeader = 0 as *mut EthercatHeader;
    let mut ehp: *mut EthernetHeader = 0 as *mut EthernetHeader;
    let mut rval: libc::c_int = 0;
    ehp = port.txbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT as *mut EthernetHeader;
    /* rewrite MAC source address 1 to primary */
    (*ehp).sa1 = htons(priMAC[1usize]);
    /* transmit over primary socket*/
    rval = ecx_outframe(port, idx, 0i32);

    if let Some(redport) = port.redport.as_mut() {
        pthread_mutex_lock(&mut (*port).tx_mutex);
        ehp = &mut (*port).txbuf2 as *mut ec_bufT as *mut EthernetHeader;
        /* use dummy frame for secondary socket transmit (BRD) */
        datagramP = &mut *(*port)
            .txbuf2
            .as_mut_ptr()
            .offset(::core::mem::size_of::<EthernetHeader>() as isize)
            as *mut u8 as *mut EthercatHeader;
        /* write index to frame */
        (*datagramP).index = idx;
        /* rewrite MAC source address 1 to secondary */
        (*ehp).sa1 = htons(secMAC[1usize]);
        /* transmit over secondary socket */
        redport.rxbufstat[idx as usize] = BufferState::Transmitted as libc::c_int;
        if send(
            redport.sockhandle,
            &mut (*port).txbuf2 as *mut ec_bufT as *const libc::c_void,
            (*port).txbuflength2 as usize,
            0i32,
        ) == -1isize
        {
            redport.rxbufstat[idx as usize] = BufferState::Empty as libc::c_int
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
unsafe fn ecx_recvpkt(
    port: &ecx_portt,
    tempinbufs: &mut i32,
    stacknumber: libc::c_int,
) -> libc::c_int {
    let stack = if stacknumber == 0 {
        &port.stack
    } else {
        port.redport.as_ref().map(|redport| &redport.stack).unwrap()
    };
    let lp = ::core::mem::size_of::<ec_bufT>() as libc::c_int;
    let bytesrx = recv(
        *(*stack).sock,
        (*(*stack).tempbuf).as_mut_ptr() as *mut libc::c_void,
        lp as usize,
        0i32,
    ) as libc::c_int;
    *tempinbufs = bytesrx;
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
unsafe fn ecx_inframe(port: &mut ecx_portt, idx: u8, stacknumber: libc::c_int) -> libc::c_int {
    let mut l: u16 = 0;
    let mut idxf: u8 = 0;
    let mut ehp: *mut EthernetHeader = 0 as *mut EthernetHeader;
    let mut ecp: *mut EthercatHeader = 0 as *mut EthercatHeader;
    let stack = if stacknumber == 0 {
        &port.stack
    } else {
        port.redport.as_ref().map(|redport| &redport.stack).unwrap()
    };
    let mut rval = EC_NOFRAME;
    let mut rxbuf = &mut *(*(*stack).rxbuf).as_mut_ptr().offset(idx as isize) as *mut ec_bufT;
    /* check if requested index is already in buffer ? */
    if idx < EC_MAXBUF
        && (*(*stack).rxbufstat)[idx as usize] == BufferState::Received as libc::c_int
    {
        l = ((*rxbuf)[0usize] as libc::c_int
            + ((((*rxbuf)[1usize] as libc::c_int & 0xfi32) as u16 as libc::c_int) << 8i32))
            as u16;
        /* return WKC */
        rval = (*rxbuf)[l as usize] as libc::c_int
            + (((*rxbuf)[(l as libc::c_int + 1i32) as usize] as libc::c_int) << 8i32);
        /* mark as completed */
        (*(*stack).rxbufstat)[idx as usize] = BufferState::Complete as libc::c_int
    } else {
        pthread_mutex_lock(&mut port.rx_mutex);
        /* non blocking call to retrieve frame from socket */
        if ecx_recvpkt(
            port,
            // FIXME: Just pass mutable reference when we sort out all this lifetime weirdness
            unsafe { port.tempinbufs as *mut i32 }.as_mut().unwrap(),
            stacknumber,
        ) != 0
        {
            rval = EC_OTHERFRAME;
            ehp = (*stack).tempbuf as *mut EthernetHeader;
            /* check if it is an EtherCAT frame */
            if (*ehp).etype as libc::c_int == htons(0x88a4u16) as libc::c_int {
                ecp = &mut *(*(*stack).tempbuf)
                    .as_mut_ptr()
                    .offset(::core::mem::size_of::<EthernetHeader>() as isize)
                    as *mut u8 as *mut EthercatHeader;
                l = ((*ecp).elength as libc::c_int & 0xfffi32) as u16;
                idxf = (*ecp).index;
                /* found index equals requested index ? */
                if idxf as libc::c_int == idx as libc::c_int {
                    /* yes, put it in the buffer array (strip ethernet header) */
                    memcpy(
                        rxbuf as *mut libc::c_void,
                        &mut *(*(*stack).tempbuf)
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<EthernetHeader>() as isize)
                            as *mut u8 as *const libc::c_void,
                        ((*(*stack).txbuflength)[idx as usize] as usize)
                            .wrapping_sub(core::mem::size_of::<EthernetHeader>()),
                    );
                    /* return WKC */
                    rval = (*rxbuf)[l as usize] as libc::c_int
                        + (((*rxbuf)[(l as libc::c_int + 1i32) as usize] as libc::c_int) << 8i32);
                    /* mark as completed */
                    (*(*stack).rxbufstat)[idx as usize] = BufferState::Complete as libc::c_int;
                    /* store MAC source word 1 for redundant routing info */
                    (*(*stack).rxsa)[idx as usize] = ntohs((*ehp).sa1) as libc::c_int
                } else if (idxf as libc::c_int) < 16i32
                    && (*(*stack).rxbufstat)[idxf as usize]
                        == BufferState::Transmitted as libc::c_int
                {
                    rxbuf =
                        &mut *(*(*stack).rxbuf).as_mut_ptr().offset(idxf as isize) as *mut ec_bufT;
                    /* check if index exist and someone is waiting for it */
                    /* put it in the buffer array (strip ethernet header) */
                    memcpy(
                        rxbuf as *mut libc::c_void,
                        &mut *(*(*stack).tempbuf)
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<EthernetHeader>() as isize)
                            as *mut u8 as *const libc::c_void,
                        ((*(*stack).txbuflength)[idxf as usize] as usize)
                            .wrapping_sub(core::mem::size_of::<EthernetHeader>()),
                    );
                    /* mark as received */
                    (*(*stack).rxbufstat)[idxf as usize] = BufferState::Received as libc::c_int;
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
unsafe fn ecx_waitinframe_red(
    port: &mut ecx_portt,
    idx: u8,
    timer: *mut osal_timert,
) -> libc::c_int {
    let mut timer2: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    let mut wkc: libc::c_int = EC_NOFRAME;
    let mut wkc2: libc::c_int = EC_NOFRAME;
    let mut primrx: libc::c_int = 0;
    let mut secrx: libc::c_int = 0;
    /* if not in redundant mode then always assume secondary is OK */
    if port.redport.is_none() {
        wkc2 = 0i32
    }
    loop {
        /* only read frame if not already in */
        if wkc <= EC_NOFRAME {
            wkc = ecx_inframe(port, idx, 0i32)
        }
        /* wait for both frames to arrive or timeout */
        if port.redport.is_some() {
            /* only try secondary if in redundant mode */
            /* only read frame if not already in */
            if wkc2 <= EC_NOFRAME {
                wkc2 = ecx_inframe(port, idx, 1i32)
            }
        }
        if !((wkc <= EC_NOFRAME || wkc2 <= EC_NOFRAME)
            && osal_timer_is_expired(timer.as_mut().unwrap()) == false)
        {
            break;
        }
    }

    /* only do redundant functions when in redundant mode */
    if let Some(redport) = port.redport.as_ref() {
        /* primrx if the received MAC source on primary socket */
        primrx = 0i32;
        if wkc > EC_NOFRAME {
            primrx = port.rxsa[idx as usize]
        }
        /* secrx if the received MAC source on psecondary socket */
        secrx = 0i32;
        if wkc2 > EC_NOFRAME {
            secrx = redport.rxsa[idx as usize]
        }
        /* primary socket got secondary frame and secondary socket got primary frame */
        /* normal situation in redundant mode */
        if primrx == secMAC[1usize] as libc::c_int && secrx == priMAC[1usize] as libc::c_int {
            port.rxbuf[idx as usize].copy_from_slice(
                &redport.rxbuf[idx as usize]
                    [0..(port.txbuflength[idx as usize] as usize - ETH_HEADERSIZE as usize)],
            );

            // /* copy secondary buffer to primary */
            // memcpy(
            //     &mut *port.rxbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
            //         as *mut libc::c_void,
            //     redport.rxbuf[idx as usize].as_ptr() as *const ec_bufT as *const libc::c_void,
            //     (port.txbuflength[idx as usize] as usize)
            //         .wrapping_sub(core::mem::size_of::<EthernetHeader>()),
            // );

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
                port.txbuf[idx as usize][ETH_HEADERSIZE..].copy_from_slice(
                    &redport.rxbuf[idx as usize]
                        [0..(port.txbuflength[idx as usize] as usize - ETH_HEADERSIZE as usize)],
                );

                // memcpy(
                //     &mut *(*port.txbuf.as_mut_ptr().offset(idx as isize))
                //         .as_mut_ptr()
                //         .offset(::core::mem::size_of::<EthernetHeader>() as isize)
                //         as *mut u8 as *mut libc::c_void,
                //     port.rxbuf[idx as usize].as_ptr() as *const ec_bufT as *const libc::c_void,
                //     (port.txbuflength[idx as usize] as usize)
                //         .wrapping_sub(core::mem::size_of::<EthernetHeader>()),
                // );
            }
            osal_timer_start(&mut timer2, EC_TIMEOUTRET);
            /* resend secondary tx */
            ecx_outframe(port, idx, 1i32);
            loop {
                /* retrieve frame */
                wkc2 = ecx_inframe(
                    // FIXME: Holy shit lmao
                    unsafe { port as *const ecx_portt as *mut ecx_portt }
                        .as_mut()
                        .unwrap(),
                    idx,
                    1i32,
                );
                if !(wkc2 <= -1 && osal_timer_is_expired(&mut timer2) == false) {
                    break;
                }
            }
            if wkc2 > -1 {
                /* copy secondary result to primary rx buffer */
                port.rxbuf[idx as usize].copy_from_slice(
                    &redport.rxbuf[idx as usize]
                        [0..(port.txbuflength[idx as usize] as usize - ETH_HEADERSIZE as usize)],
                );

                // memcpy(
                //     &mut *(*port).rxbuf.as_mut_ptr().offset(idx as isize) as *mut ec_bufT
                //         as *mut libc::c_void,
                //     redport.rxbuf[idx as usize].as_ptr() as *const ec_bufT as *const libc::c_void,
                //     ((*port).txbuflength[idx as usize] as usize)
                //         .wrapping_sub(core::mem::size_of::<EthernetHeader>()),
                // );
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
pub unsafe fn ecx_waitinframe(port: &mut ecx_portt, idx: u8, timeout: u32) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut timer: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer, timeout);
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
pub unsafe fn ecx_srconfirm(port: &mut ecx_portt, idx: u8, timeout: u32) -> libc::c_int {
    let mut wkc: libc::c_int = -1;
    let mut timer1: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    let mut timer2: osal_timert = osal_timert {
        stop_time: ec_timet { sec: 0, usec: 0 },
    };
    osal_timer_start(&mut timer1, timeout);
    loop {
        /* tx frame on primary and if in redundant mode a dummy on secondary */
        ecx_outframe_red(port, idx);
        if timeout < EC_TIMEOUTRET {
            osal_timer_start(&mut timer2, timeout);
        } else {
            /* wait for answer with WKC>=0 or otherwise retry until timeout */
            /* normally use partial timeout for rx */
            osal_timer_start(&mut timer2, EC_TIMEOUTRET);
        }
        wkc = ecx_waitinframe_red(port, idx, &mut timer2);
        if !(wkc <= -1 && osal_timer_is_expired(&mut timer1) == false) {
            break;
        }
    }
    return wkc;
}
#[no_mangle]
pub unsafe fn ec_setupnic(ifname: *const libc::c_char, secondary: libc::c_int) -> libc::c_int {
    return ecx_setupnic(&mut ecx_port, ifname, secondary);
}
#[no_mangle]
pub unsafe fn ec_closenic() -> libc::c_int {
    return ecx_closenic(&mut ecx_port);
}
#[no_mangle]
pub unsafe fn ec_getindex() -> u8 {
    return ecx_getindex(&mut ecx_port);
}
#[no_mangle]
pub unsafe fn ec_setbufstat(idx: u8, bufstat: BufferState) {
    ecx_setbufstat(&mut ecx_port, idx, bufstat);
}
#[no_mangle]
pub unsafe fn ec_outframe(idx: u8, stacknumber: libc::c_int) -> libc::c_int {
    return ecx_outframe(&mut ecx_port, idx, stacknumber);
}
#[no_mangle]
pub unsafe fn ec_outframe_red(idx: u8) -> libc::c_int {
    return ecx_outframe_red(&mut ecx_port, idx);
}
#[no_mangle]
pub unsafe fn ec_inframe(idx: u8, stacknumber: libc::c_int) -> libc::c_int {
    return ecx_inframe(&mut ecx_port, idx, stacknumber);
}
#[no_mangle]
pub unsafe fn ec_waitinframe(idx: u8, timeout: u32) -> libc::c_int {
    return ecx_waitinframe(&mut ecx_port, idx, timeout);
}
#[no_mangle]
pub unsafe fn ec_srconfirm(idx: u8, timeout: u32) -> libc::c_int {
    return ecx_srconfirm(&mut ecx_port, idx, timeout);
}
/* get frame from primary or if in redundant mode possibly from secondary */
