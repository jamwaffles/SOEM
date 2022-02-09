use ::c2rust_bitfields;
use ::libc;
extern "C" {
    
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    
    fn ec_nextmbxcnt(cnt: uint8) -> uint8;
    
    fn ec_clearmbx(Mbx: *mut ec_mbxbuft);
    
    fn ecx_mbxsend(
        context: *mut ecx_contextt,
        slave: uint16,
        mbx: *mut ec_mbxbuft,
        timeout: libc::c_int,
    ) -> libc::c_int;
    
    fn ecx_mbxreceive(
        context: *mut ecx_contextt,
        slave: uint16,
        mbx: *mut ec_mbxbuft,
        timeout: libc::c_int,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_timet {
    pub sec: uint32,
    pub usec: uint32,
}
pub type ec_bufT = [uint8; 1518];
pub type C2RustUnnamed = libc::c_uint;
pub const ECT_MBXT_VOE: C2RustUnnamed = 15;
pub const ECT_MBXT_SOE: C2RustUnnamed = 5;
pub const ECT_MBXT_FOE: C2RustUnnamed = 4;
pub const ECT_MBXT_COE: C2RustUnnamed = 3;
pub const ECT_MBXT_EOE: C2RustUnnamed = 2;
pub const ECT_MBXT_AOE: C2RustUnnamed = 1;
pub const ECT_MBXT_ERR: C2RustUnnamed = 0;
pub type ec_err_type = libc::c_uint;
pub const EC_ERR_TYPE_EOE_INVALID_RX_DATA: ec_err_type = 11;
pub const EC_ERR_TYPE_FOE_FILE_NOTFOUND: ec_err_type = 10;
pub const EC_ERR_TYPE_MBX_ERROR: ec_err_type = 9;
pub const EC_ERR_TYPE_SOE_ERROR: ec_err_type = 8;
pub const EC_ERR_TYPE_FOE_PACKETNUMBER: ec_err_type = 7;
pub const EC_ERR_TYPE_FOE_BUF2SMALL: ec_err_type = 6;
pub const EC_ERR_TYPE_FOE_ERROR: ec_err_type = 5;
pub const EC_ERR_TYPE_SDOINFO_ERROR: ec_err_type = 4;
pub const EC_ERR_TYPE_PACKET_ERROR: ec_err_type = 3;
pub const EC_ERR_TYPE_EMERGENCY: ec_err_type = 1;
pub const EC_ERR_TYPE_SDO_ERROR: ec_err_type = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_errort {
    pub Time: ec_timet,
    pub Signal: boolean,
    pub Slave: uint16,
    pub Index: uint16,
    pub SubIdx: uint8,
    pub Etype: ec_err_type,
    pub c2rust_unnamed: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub AbortCode: int32,
    pub c2rust_unnamed: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub ErrorCode: uint16,
    pub ErrorReg: uint8,
    pub b1: uint8,
    pub w1: uint16,
    pub w2: uint16,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct ecx_redportt {
    pub stack: ec_stackT,
    pub sockhandle: libc::c_int,
    pub rxbuf: [ec_bufT; 16],
    pub rxbufstat: [libc::c_int; 16],
    pub rxsa: [libc::c_int; 16],
    pub tempinbuf: ec_bufT,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_fmmu {
    pub LogStart: uint32,
    pub LogLength: uint16,
    pub LogStartbit: uint8,
    pub LogEndbit: uint8,
    pub PhysStart: uint16,
    pub PhysStartBit: uint8,
    pub FMMUtype: uint8,
    pub FMMUactive: uint8,
    pub unused1: uint8,
    pub unused2: uint16,
}
pub type ec_fmmut = ec_fmmu;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_sm {
    pub StartAddr: uint16,
    pub SMlength: uint16,
    pub SMflags: uint32,
}
pub type ec_smt = ec_sm;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecx_context {
    pub port: *mut ecx_portt,
    pub slavelist: *mut ec_slavet,
    pub slavecount: *mut libc::c_int,
    pub maxslave: libc::c_int,
    pub grouplist: *mut ec_groupt,
    pub maxgroup: libc::c_int,
    pub esibuf: *mut uint8,
    pub esimap: *mut uint32,
    pub esislave: uint16,
    pub elist: *mut ec_eringt,
    pub idxstack: *mut ec_idxstackT,
    pub ecaterror: *mut boolean,
    pub DCtime: *mut int64,
    pub SMcommtype: *mut ec_SMcommtypet,
    pub PDOassign: *mut ec_PDOassignt,
    pub PDOdesc: *mut ec_PDOdesct,
    pub eepSM: *mut ec_eepromSMt,
    pub eepFMMU: *mut ec_eepromFMMUt,
    pub FOEhook:
        Option<unsafe extern "C" fn(_: uint16, _: libc::c_int, _: libc::c_int) -> libc::c_int>,
    pub EOEhook: Option<
        unsafe extern "C" fn(_: *mut ecx_contextt, _: uint16, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub manualstatechange: libc::c_int,
    pub userdata: *mut libc::c_void,
}
pub type ecx_contextt = ecx_context;
pub type ec_eepromFMMUt = ec_eepromFMMU;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_eepromFMMU {
    pub Startpos: uint16,
    pub nFMMU: uint8,
    pub FMMU0: uint8,
    pub FMMU1: uint8,
    pub FMMU2: uint8,
    pub FMMU3: uint8,
}
pub type ec_eepromSMt = ec_eepromSM;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_eepromSM {
    pub Startpos: uint16,
    pub nSM: uint8,
    pub PhStart: uint16,
    pub Plength: uint16,
    pub Creg: uint8,
    pub Sreg: uint8,
    pub Activate: uint8,
    pub PDIctrl: uint8,
}
pub type ec_PDOdesct = ec_PDOdesc;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_PDOdesc {
    pub n: uint8,
    pub nu1: uint8,
    pub PDO: [uint32; 256],
}
pub type ec_PDOassignt = ec_PDOassign;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_PDOassign {
    pub n: uint8,
    pub nu1: uint8,
    pub index: [uint16; 256],
}
pub type ec_SMcommtypet = ec_SMcommtype;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_SMcommtype {
    pub n: uint8,
    pub nu1: uint8,
    pub SMtype: [uint8; 8],
}
pub type ec_idxstackT = ec_idxstack;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_idxstack {
    pub pushed: uint8,
    pub pulled: uint8,
    pub idx: [uint8; 16],
    pub data: [*mut libc::c_void; 16],
    pub length: [uint16; 16],
    pub dcoffset: [uint16; 16],
}
pub type ec_eringt = ec_ering;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_ering {
    pub head: int16,
    pub tail: int16,
    pub Error: [ec_errort; 65],
}
pub type ec_groupt = ec_group;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_group {
    pub logstartaddr: uint32,
    pub Obytes: uint32,
    pub outputs: *mut uint8,
    pub Ibytes: uint32,
    pub inputs: *mut uint8,
    pub hasdc: boolean,
    pub DCnext: uint16,
    pub Ebuscurrent: int16,
    pub blockLRW: uint8,
    pub nsegments: uint16,
    pub Isegment: uint16,
    pub Ioffset: uint16,
    pub outputsWKC: uint16,
    pub inputsWKC: uint16,
    pub docheckstate: boolean,
    pub IOsegment: [uint32; 64],
}
pub type ec_slavet = ec_slave;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_slave {
    pub state: uint16,
    pub ALstatuscode: uint16,
    pub configadr: uint16,
    pub aliasadr: uint16,
    pub eep_man: uint32,
    pub eep_id: uint32,
    pub eep_rev: uint32,
    pub Itype: uint16,
    pub Dtype: uint16,
    pub Obits: uint16,
    pub Obytes: uint32,
    pub outputs: *mut uint8,
    pub Ostartbit: uint8,
    pub Ibits: uint16,
    pub Ibytes: uint32,
    pub inputs: *mut uint8,
    pub Istartbit: uint8,
    pub SM: [ec_smt; 8],
    pub SMtype: [uint8; 8],
    pub FMMU: [ec_fmmut; 4],
    pub FMMU0func: uint8,
    pub FMMU1func: uint8,
    pub FMMU2func: uint8,
    pub FMMU3func: uint8,
    pub mbx_l: uint16,
    pub mbx_wo: uint16,
    pub mbx_rl: uint16,
    pub mbx_ro: uint16,
    pub mbx_proto: uint16,
    pub mbx_cnt: uint8,
    pub hasdc: boolean,
    pub ptype: uint8,
    pub topology: uint8,
    pub activeports: uint8,
    pub consumedports: uint8,
    pub parent: uint16,
    pub parentport: uint8,
    pub entryport: uint8,
    pub DCrtA: int32,
    pub DCrtB: int32,
    pub DCrtC: int32,
    pub DCrtD: int32,
    pub pdelay: int32,
    pub DCnext: uint16,
    pub DCprevious: uint16,
    pub DCcycle: int32,
    pub DCshift: int32,
    pub DCactive: uint8,
    pub configindex: uint16,
    pub SIIindex: uint16,
    pub eep_8byte: uint8,
    pub eep_pdi: uint8,
    pub CoEdetails: uint8,
    pub FoEdetails: uint8,
    pub EoEdetails: uint8,
    pub SoEdetails: uint8,
    pub Ebuscurrent: int16,
    pub blockLRW: uint8,
    pub group: uint8,
    pub FMMUunused: uint8,
    pub islost: boolean,
    pub PO2SOconfig: Option<unsafe extern "C" fn(_: uint16) -> libc::c_int>,
    pub PO2SOconfigx: Option<unsafe extern "C" fn(_: *mut ecx_contextt, _: uint16) -> libc::c_int>,
    pub name: [libc::c_char; 41],
}
pub type ec_mbxbuft = [uint8; 1487];

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_mbxheader {
    pub length: uint16,
    pub address: uint16,
    pub priority: uint8,
    pub mbxtype: uint8,
}
pub type ec_mbxheadert = ec_mbxheader;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eoe_ip4_addr {
    pub addr: uint32_t,
}
pub type eoe_ip4_addr_t = eoe_ip4_addr;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct eoe_ethaddr {
    pub addr: [uint8_t; 6],
}
pub type eoe_ethaddr_t = eoe_ethaddr;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct eoe_param {
    #[bitfield(name = "mac_set", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "ip_set", ty = "uint8_t", bits = "1..=1")]
    #[bitfield(name = "subnet_set", ty = "uint8_t", bits = "2..=2")]
    #[bitfield(name = "default_gateway_set", ty = "uint8_t", bits = "3..=3")]
    #[bitfield(name = "dns_ip_set", ty = "uint8_t", bits = "4..=4")]
    #[bitfield(name = "dns_name_set", ty = "uint8_t", bits = "5..=5")]
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
    pub frameinfo1: uint16_t,
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub data: [uint8; 1476],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
    pub frameinfo2: uint16_t,
    pub result: uint16_t,
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
/* * EoE utility function to convert uint32 to eoe ip bytes.
 * @param[in] ip       = ip in uint32
 * @param[out] byte_ip = eoe ip 4th octet, 3ed octet, 2nd octet, 1st octet
 */
unsafe extern "C" fn EOE_ip_uint32_to_byte(mut ip: *mut eoe_ip4_addr_t, mut byte_ip: *mut uint8_t) {
    *byte_ip.offset(3isize) = *(&mut (*ip).addr as *mut uint32_t as *const uint8_t).offset(0isize); /* 1st octet */
    *byte_ip.offset(2isize) = *(&mut (*ip).addr as *mut uint32_t as *const uint8_t).offset(1isize); /* 2nd octet */
    *byte_ip.offset(1isize) = *(&mut (*ip).addr as *mut uint32_t as *const uint8_t).offset(2isize); /* 3ed octet */
    *byte_ip.offset(0isize) = *(&mut (*ip).addr as *mut uint32_t as *const uint8_t).offset(3isize);
    /* 4th octet */
}
/* * EoE utility function to convert eoe ip bytes to uint32.
* @param[in] byte_ip = eoe ip 4th octet, 3ed octet, 2nd octet, 1st octet
* @param[out] ip     = ip in uint32
*/
unsafe extern "C" fn EOE_ip_byte_to_uint32(mut byte_ip: *mut uint8_t, mut ip: *mut eoe_ip4_addr_t) {
    (*ip).addr = (((((*byte_ip.offset(3isize) as libc::c_int & 0xffi32) as uint32_t) << 24i32
        | ((*byte_ip.offset(2isize) as libc::c_int & 0xffi32) as uint32_t) << 16i32
        | ((*byte_ip.offset(1isize) as libc::c_int & 0xffi32) as uint32_t) << 8i32
        | (*byte_ip.offset(0isize) as libc::c_int & 0xffi32) as uint32_t)
        as libc::c_ulong
        & 0xffu64)
        << 24i32
        | ((((*byte_ip.offset(3isize) as libc::c_int & 0xffi32) as uint32_t) << 24i32
            | ((*byte_ip.offset(2isize) as libc::c_int & 0xffi32) as uint32_t) << 16i32
            | ((*byte_ip.offset(1isize) as libc::c_int & 0xffi32) as uint32_t) << 8i32
            | (*byte_ip.offset(0isize) as libc::c_int & 0xffi32) as uint32_t)
            as libc::c_ulong
            & 0xff00u64)
            << 8i32
        | ((((*byte_ip.offset(3isize) as libc::c_int & 0xffi32) as uint32_t) << 24i32
            | ((*byte_ip.offset(2isize) as libc::c_int & 0xffi32) as uint32_t) << 16i32
            | ((*byte_ip.offset(1isize) as libc::c_int & 0xffi32) as uint32_t) << 8i32
            | (*byte_ip.offset(0isize) as libc::c_int & 0xffi32) as uint32_t)
            as libc::c_ulong
            & 0xff0000u64)
            >> 8i32
        | ((((*byte_ip.offset(3isize) as libc::c_int & 0xffi32) as uint32_t) << 24i32
            | ((*byte_ip.offset(2isize) as libc::c_int & 0xffi32) as uint32_t) << 16i32
            | ((*byte_ip.offset(1isize) as libc::c_int & 0xffi32) as uint32_t) << 8i32
            | (*byte_ip.offset(0isize) as libc::c_int & 0xffi32) as uint32_t)
            as libc::c_ulong
            & 0xff000000u64)
            >> 24i32) as uint32_t;
    /* 4th octet */
}
/* * EoE fragment data handler hook. Should not block.
*
* @param[in]  context = context struct
* @param[in]  hook    = Pointer to hook function.
* @return 1
*/
#[no_mangle]
pub unsafe extern "C" fn ecx_EOEdefinehook(
    mut context: *mut ecx_contextt,
    mut hook: *mut libc::c_void,
) -> libc::c_int {
    (*context).EOEhook = ::core::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(
                _: *mut ecx_contextt,
                _: uint16,
                _: *mut libc::c_void,
            ) -> libc::c_int,
        >,
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
pub unsafe extern "C" fn ecx_EOEsetIp(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut port: uint8,
    mut ipparam: *mut eoe_param_t,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut EOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut aEOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut frameinfo1: uint16 = 0;
    let mut result: uint16 = 0;
    let mut cnt: uint8 = 0;
    let mut data_offset: uint8 = 0;
    let mut flags: uint8 = 0u8;
    let mut wkc: libc::c_int = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0i32);
    ec_clearmbx(&mut MbxOut);
    aEOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_EOEt;
    EOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_EOEt;
    (*EOEp).mbxheader.address = 0u16;
    (*EOEp).mbxheader.priority = 0u8;
    data_offset = 4u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* EoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*EOEp).mbxheader.mbxtype = (ECT_MBXT_EOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
        as uint8;
    (*EOEp).frameinfo1 = ((2i32 & 0xfi32) << 0i32
        | ((port as libc::c_int & 0xfi32) << 4i32) as uint16 as libc::c_int
        | (0x1i32) << 8i32) as uint16_t;
    (*EOEp).c2rust_unnamed.frameinfo2 = 0u16;
    if (*ipparam).mac_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 0i32) as uint8;
        memcpy(
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize) as *mut uint8
                as *mut libc::c_void,
            (*ipparam).mac.addr.as_mut_ptr() as *const libc::c_void,
            6u64,
        );
        data_offset = (data_offset as libc::c_int + 6i32) as uint8
    }
    if (*ipparam).ip_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 1i32) as uint8;
        EOE_ip_uint32_to_byte(
            &mut (*ipparam).ip,
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize),
        );
        data_offset = (data_offset as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as uint8
    }
    if (*ipparam).subnet_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 2i32) as uint8;
        EOE_ip_uint32_to_byte(
            &mut (*ipparam).subnet,
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize),
        );
        data_offset = (data_offset as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as uint8
    }
    if (*ipparam).default_gateway_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 3i32) as uint8;
        EOE_ip_uint32_to_byte(
            &mut (*ipparam).default_gateway,
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize),
        );
        data_offset = (data_offset as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as uint8
    }
    if (*ipparam).dns_ip_set() != 0 {
        flags = (flags as libc::c_int | (0x1i32) << 4i32) as uint8;
        EOE_ip_uint32_to_byte(
            &mut (*ipparam).dns_ip,
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize),
        );
        data_offset = (data_offset as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as uint8
    }
    if (*ipparam).dns_name_set() != 0 {
        /* TwinCAT include EOE_DNS_NAME_LENGTH chars even if name is shorter */
        flags = (flags as libc::c_int | (0x1i32) << 5i32) as uint8;
        memcpy(
            &mut *(*EOEp).data.as_mut_ptr().offset(data_offset as isize) as *mut uint8
                as *mut libc::c_void,
            (*ipparam).dns_name.as_mut_ptr() as *mut libc::c_void,
            32u64,
        );
        data_offset = (data_offset as libc::c_int + 32i32) as uint8
    }
    (*EOEp).mbxheader.length = (4i32 + data_offset as libc::c_int) as uint16;
    (*EOEp).data[0usize] = flags;
    /* send EoE request to slave */
    wkc = ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft, 20000i32);
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        /* clean mailboxbuffer */
        ec_clearmbx(&mut MbxIn);
        wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
        if wkc > 0i32 {
            /* read slave response */
            /* succeeded to read slave response ? */
            /* slave response should be FoE */
            if (*aEOEp).mbxheader.mbxtype as libc::c_int & 0xfi32 == ECT_MBXT_EOE as libc::c_int {
                frameinfo1 = (*aEOEp).frameinfo1;
                result = (*aEOEp).c2rust_unnamed.result;
                if frameinfo1 as libc::c_int >> 0i32 & 0xfi32 != 3i32
                    || result as libc::c_int != 0i32
                {
                    wkc = -(result as libc::c_int)
                }
            } else {
                /* unexpected mailbox received */
                wkc = -(EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
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
pub unsafe extern "C" fn ecx_EOEgetIp(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut port: uint8,
    mut ipparam: *mut eoe_param_t,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut EOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut aEOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut frameinfo1: uint16 = 0;
    let mut eoedatasize: uint16 = 0;
    let mut cnt: uint8 = 0;
    let mut data_offset: uint8 = 0;
    let mut flags: uint8 = 0u8;
    let mut wkc: libc::c_int = 0;
    /* Empty slave out mailbox if something is in. Timout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0i32);
    ec_clearmbx(&mut MbxOut);
    aEOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_EOEt;
    EOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_EOEt;
    (*EOEp).mbxheader.address = 0u16;
    (*EOEp).mbxheader.priority = 0u8;
    data_offset = 4u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* EoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*EOEp).mbxheader.mbxtype = (ECT_MBXT_EOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
        as uint8;
    (*EOEp).frameinfo1 = ((6i32 & 0xfi32) << 0i32
        | ((port as libc::c_int & 0xfi32) << 4i32) as uint16 as libc::c_int
        | (0x1i32) << 8i32) as uint16_t;
    (*EOEp).c2rust_unnamed.frameinfo2 = 0u16;
    (*EOEp).mbxheader.length = 0x4u16;
    (*EOEp).data[0usize] = flags;
    /* send EoE request to slave */
    wkc = ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft, 20000i32);
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        /* clean mailboxbuffer */
        ec_clearmbx(&mut MbxIn);
        wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
        if wkc > 0i32 {
            /* read slave response */
            /* succeeded to read slave response ? */
            /* slave response should be FoE */
            if (*aEOEp).mbxheader.mbxtype as libc::c_int & 0xfi32 == ECT_MBXT_EOE as libc::c_int {
                frameinfo1 = (*aEOEp).frameinfo1;
                eoedatasize = ((*aEOEp).mbxheader.length as libc::c_int - 0x4i32) as uint16;
                if frameinfo1 as libc::c_int >> 0i32 & 0xfi32 != 7i32 {
                    wkc = -(0x2i32)
                } else {
                    flags = (*aEOEp).data[0usize];
                    if flags as libc::c_int & (0x1i32) << 0i32 != 0 {
                        memcpy(
                            (*ipparam).mac.addr.as_mut_ptr() as *mut libc::c_void,
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize)
                                as *mut uint8 as *const libc::c_void,
                            6u64,
                        );
                        (*ipparam).set_mac_set(1u8);
                        data_offset = (data_offset as libc::c_int + 6i32) as uint8
                    }
                    if flags as libc::c_int & (0x1i32) << 1i32 != 0 {
                        EOE_ip_byte_to_uint32(
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize),
                            &mut (*ipparam).ip,
                        );
                        (*ipparam).set_ip_set(1u8);
                        data_offset = (data_offset as libc::c_ulong)
                            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                            as uint8
                    }
                    if flags as libc::c_int & (0x1i32) << 2i32 != 0 {
                        EOE_ip_byte_to_uint32(
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize),
                            &mut (*ipparam).subnet,
                        );
                        (*ipparam).set_subnet_set(1u8);
                        data_offset = (data_offset as libc::c_ulong)
                            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                            as uint8
                    }
                    if flags as libc::c_int & (0x1i32) << 3i32 != 0 {
                        EOE_ip_byte_to_uint32(
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize),
                            &mut (*ipparam).default_gateway,
                        );
                        (*ipparam).set_default_gateway_set(1u8);
                        data_offset = (data_offset as libc::c_ulong)
                            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                            as uint8
                    }
                    if flags as libc::c_int & (0x1i32) << 4i32 != 0 {
                        EOE_ip_byte_to_uint32(
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize),
                            &mut (*ipparam).dns_ip,
                        );
                        (*ipparam).set_dns_ip_set(1u8);
                        data_offset = (data_offset as libc::c_ulong)
                            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                            as uint8
                    }
                    if flags as libc::c_int & (0x1i32) << 5i32 != 0 {
                        let mut dns_len: uint16_t = 0;
                        if (eoedatasize as libc::c_int - data_offset as libc::c_int) < 32i32 {
                            dns_len = (eoedatasize as libc::c_int - data_offset as libc::c_int)
                                as uint16_t
                        } else {
                            dns_len = 32u16
                        }
                        /* Assume ZERO terminated string */
                        memcpy(
                            (*ipparam).dns_name.as_mut_ptr() as *mut libc::c_void,
                            &mut *(*aEOEp).data.as_mut_ptr().offset(data_offset as isize)
                                as *mut uint8 as *const libc::c_void,
                            dns_len as libc::c_ulong,
                        );
                        (*ipparam).set_dns_name_set(1u8);
                        data_offset = (data_offset as libc::c_int + 32i32) as uint8
                    }
                    /* Something os not correct, flag the error */
                    if data_offset as libc::c_int > eoedatasize as libc::c_int {
                        wkc = -(EC_ERR_TYPE_MBX_ERROR as libc::c_int)
                    }
                }
            } else {
                /* unexpected mailbox received */
                wkc = -(EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
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
pub unsafe extern "C" fn ecx_EOEsend(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut port: uint8,
    mut psize: libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut EOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut frameinfo1: uint16 = 0;
    let mut frameinfo2: uint16 = 0;
    let mut cnt: uint8 = 0;
    let mut txfragmentno: uint8 = 0;
    let mut NotLast: boolean = 0;
    let mut wkc: libc::c_int = 0;
    let mut maxdata: libc::c_int = 0;
    let mut txframesize: libc::c_int = 0;
    let mut txframeoffset: libc::c_int = 0;
    let mut buf: *const uint8 = p as *const uint8;
    static mut txframeno: uint8_t = 0u8;
    ec_clearmbx(&mut MbxOut);
    EOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_EOEt;
    (*EOEp).mbxheader.address = 0u16;
    (*EOEp).mbxheader.priority = 0u8;
    /* data section=mailbox size - 6 mbx - 4 EoEh */
    maxdata = (*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int - 0xai32;
    txframesize = psize;
    txfragmentno = 0u8;
    txframeoffset = 0i32;
    NotLast = 1u8;
    loop {
        txframesize = psize - txframeoffset;
        if txframesize > maxdata {
            /* Adjust to even 32-octect blocks */
            txframesize = (maxdata >> 5i32) << 5i32
        }
        if txframesize == psize - txframeoffset {
            frameinfo1 = ((1i32 & 0x1i32) << 8i32
                | ((port as libc::c_int & 0xfi32) << 4i32) as uint16 as libc::c_int)
                as uint16;
            NotLast = 0u8
        } else {
            frameinfo1 = ((port as libc::c_int & 0xfi32) << 4i32) as uint16
        }
        frameinfo2 = ((txfragmentno as libc::c_int & 0x3fi32) << 0i32) as uint16;
        if txfragmentno as libc::c_int > 0i32 {
            frameinfo2 = (frameinfo2 as libc::c_int
                | ((txframeoffset >> 5i32 & 0x3fi32) << 6i32) as uint16 as libc::c_int)
                as uint16
        } else {
            frameinfo2 = (frameinfo2 as libc::c_int
                | ((psize + 31i32 >> 5i32 & 0x3fi32) << 6i32) as uint16 as libc::c_int)
                as uint16;
            txframeno = txframeno.wrapping_add(1)
        }
        frameinfo2 = (frameinfo2 as libc::c_int
            | ((txframeno as libc::c_int & 0xfi32) << 12i32) as uint16 as libc::c_int)
            as uint16;
        /* get new mailbox count value, used as session handle */
        cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* no timestamp */
        (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt; /* EoE */
        (*EOEp).mbxheader.length = (4i32 + txframesize) as uint16;
        (*EOEp).mbxheader.mbxtype = (ECT_MBXT_EOE as libc::c_int
            + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
            as uint8;
        (*EOEp).frameinfo1 = frameinfo1;
        (*EOEp).c2rust_unnamed.frameinfo2 = frameinfo2;
        memcpy(
            (*EOEp).data.as_mut_ptr() as *mut libc::c_void,
            &*buf.offset(txframeoffset as isize) as *const uint8 as *const libc::c_void,
            txframesize as libc::c_ulong,
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
pub unsafe extern "C" fn ecx_EOErecv(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut port: uint8,
    mut psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut aEOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut frameinfo1: uint16 = 0;
    let mut frameinfo2: uint16 = 0;
    let mut rxfragmentno: uint8 = 0;
    let mut rxframeno: uint8 = 0;
    let mut NotLast: boolean = 0;
    let mut wkc: libc::c_int = 0;
    let mut buffersize: libc::c_int = 0;
    let mut rxframesize: libc::c_int = 0;
    let mut rxframeoffset: libc::c_int = 0;
    let mut eoedatasize: libc::c_int = 0;
    let mut buf: *mut uint8 = p as *mut uint8;
    ec_clearmbx(&mut MbxIn);
    aEOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_EOEt;
    NotLast = 1u8;
    buffersize = *psize;
    rxfragmentno = 0u8;
    /* Hang for a while if nothing is in */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
    while wkc > 0i32 && NotLast as libc::c_int == 1i32 {
        /* slave response should be FoE */
        if (*aEOEp).mbxheader.mbxtype as libc::c_int & 0xfi32 == ECT_MBXT_EOE as libc::c_int {
            eoedatasize = (*aEOEp).mbxheader.length as libc::c_int - 0x4i32;
            frameinfo1 = (*aEOEp).frameinfo1;
            frameinfo2 = (*aEOEp).c2rust_unnamed.frameinfo2;
            if rxfragmentno as libc::c_int != frameinfo2 as libc::c_int >> 0i32 & 0x3fi32 {
                if frameinfo2 as libc::c_int >> 0i32 & 0x3fi32 > 0i32 {
                    wkc = -(EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                    /* Exit here*/
                    break;
                }
            }
            if rxfragmentno as libc::c_int == 0i32 {
                rxframeoffset = 0i32;
                rxframeno = (frameinfo2 as libc::c_int >> 12i32 & 0xfi32) as uint8;
                rxframesize = (frameinfo2 as libc::c_int >> 6i32 & 0x3fi32) << 5i32;
                if rxframesize > buffersize {
                    wkc = -(EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                    /* Exit here*/
                    break;
                } else if port as libc::c_int != frameinfo1 as libc::c_int >> 4i32 & 0xfi32 {
                    wkc = -(EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                    /* Exit here*/
                    break;
                }
            } else if rxframeno as libc::c_int != frameinfo2 as libc::c_int >> 12i32 & 0xfi32 {
                wkc = -(EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                /* Exit here*/
                break;
            } else if rxframeoffset != (frameinfo2 as libc::c_int >> 6i32 & 0x3fi32) << 5i32 {
                wkc = -(EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                break;
            }
            if rxframeoffset + eoedatasize <= buffersize {
                memcpy(
                    &mut *buf.offset(rxframeoffset as isize) as *mut uint8 as *mut libc::c_void,
                    (*aEOEp).data.as_mut_ptr() as *const libc::c_void,
                    eoedatasize as libc::c_ulong,
                );
                rxframeoffset += eoedatasize;
                rxfragmentno = rxfragmentno.wrapping_add(1)
            }
            if frameinfo1 as libc::c_int >> 8i32 & 0x1i32 != 0 {
                /* Remove timestamp */
                if frameinfo1 as libc::c_int >> 9i32 & 0x1i32 != 0 {
                    rxframeoffset -= 4i32
                }
                NotLast = 0u8;
                *psize = rxframeoffset
            } else {
                /* Hang for a while if nothing is in */
                wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout)
            }
        } else {
            /* unexpected mailbox received */
            wkc = -(EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
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
pub unsafe extern "C" fn ecx_EOEreadfragment(
    mut MbxIn: *mut ec_mbxbuft,
    mut rxfragmentno: *mut uint8,
    mut rxframesize: *mut uint16,
    mut rxframeoffset: *mut uint16,
    mut rxframeno: *mut uint16,
    mut psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
) -> libc::c_int {
    let mut frameinfo1: uint16 = 0;
    let mut frameinfo2: uint16 = 0;
    let mut eoedatasize: uint16 = 0;
    let mut wkc: libc::c_int = 0;
    let mut aEOEp: *mut ec_EOEt = 0 as *mut ec_EOEt;
    let mut buf: *mut uint8 = 0 as *mut uint8;
    aEOEp = MbxIn as *mut ec_EOEt;
    buf = p as *mut uint8;
    wkc = 0i32;
    /* slave response should be EoE */
    if (*aEOEp).mbxheader.mbxtype as libc::c_int & 0xfi32 == ECT_MBXT_EOE as libc::c_int {
        eoedatasize = ((*aEOEp).mbxheader.length as libc::c_int - 0x4i32) as uint16;
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
                wkc = -(EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                return wkc;
            }
        }
        /* Is it a new frame?*/
        if *rxfragmentno as libc::c_int == 0i32 {
            *rxframesize = ((frameinfo2 as libc::c_int >> 6i32 & 0x3fi32) << 5i32) as uint16;
            *rxframeoffset = 0u16;
            *rxframeno = (frameinfo2 as libc::c_int >> 12i32 & 0xfi32) as uint16
        } else if *rxframeno as libc::c_int != frameinfo2 as libc::c_int >> 12i32 & 0xfi32 {
            *rxfragmentno = 0u8;
            *rxframesize = 0u16;
            *rxframeoffset = 0u16;
            *rxframeno = 0u16;
            wkc = -(EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
            return wkc;
        } else {
            if *rxframeoffset as libc::c_int
                != (frameinfo2 as libc::c_int >> 6i32 & 0x3fi32) << 5i32
            {
                *rxfragmentno = 0u8;
                *rxframesize = 0u16;
                *rxframeoffset = 0u16;
                *rxframeno = 0u16;
                wkc = -(EC_ERR_TYPE_EOE_INVALID_RX_DATA as libc::c_int);
                return wkc;
            }
        }
        /* If we're inside a frame, make sure it is the same */
        /* Make sure we're inside expected frame size */
        if *rxframeoffset as libc::c_int + eoedatasize as libc::c_int <= *rxframesize as libc::c_int
            && *rxframeoffset as libc::c_int + eoedatasize as libc::c_int <= *psize
        {
            memcpy(
                &mut *buf.offset(*rxframeoffset as isize) as *mut uint8 as *mut libc::c_void,
                (*aEOEp).data.as_mut_ptr() as *const libc::c_void,
                eoedatasize as libc::c_ulong,
            );
            *rxframeoffset = (*rxframeoffset as libc::c_int + eoedatasize as libc::c_int) as uint16;
            *rxfragmentno = (*rxfragmentno as libc::c_int + 1i32) as uint8
        }
        /* Is it the last fragment */
        if frameinfo1 as libc::c_int >> 8i32 & 0x1i32 != 0 {
            /* Remove timestamp */
            if frameinfo1 as libc::c_int >> 9i32 & 0x1i32 != 0 {
                *rxframeoffset = (*rxframeoffset as libc::c_int - 4i32) as uint16
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
        wkc = -(EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
    }
    return wkc;
}
