use ::libc;
extern "C" {
    
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    
    static mut ecx_context: ecx_contextt;
    
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ECT_FOE_BUSY: C2RustUnnamed_0 = 6;
pub const ECT_FOE_ERROR: C2RustUnnamed_0 = 5;
pub const ECT_FOE_ACK: C2RustUnnamed_0 = 4;
pub const ECT_FOE_DATA: C2RustUnnamed_0 = 3;
pub const ECT_FOE_WRITE: C2RustUnnamed_0 = 2;
pub const ECT_FOE_READ: C2RustUnnamed_0 = 1;
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
    pub c2rust_unnamed: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_1 {
    pub AbortCode: int32,
    pub c2rust_unnamed: C2RustUnnamed_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
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

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ec_FOEt {
    pub MbxHeader: ec_mbxheadert,
    pub OpCode: uint8,
    pub Reserved: uint8,
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_3 {
    pub FileName: [libc::c_char; 512],
    pub Data: [uint8; 512],
    pub ErrorText: [libc::c_char; 512],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
    pub Password: uint32,
    pub PacketNumber: uint32,
    pub ErrorCode: uint32,
}
/* * FoE progress hook.
 *
 * @param[in]  context        = context struct
 * @param[in]     hook       = Pointer to hook function.
 * @return 1
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_FOEdefinehook(
    mut context: *mut ecx_contextt,
    mut hook: *mut libc::c_void,
) -> libc::c_int {
    (*context).FOEhook = ::core::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: uint16, _: libc::c_int, _: libc::c_int) -> libc::c_int>,
    >(hook);
    return 1i32;
}
/* * FoE read, blocking.
 *
 * @param[in]  context        = context struct
 * @param[in]     slave      = Slave number.
 * @param[in]     filename   = Filename of file to read.
 * @param[in]     password   = password.
 * @param[in,out] psize      = Size in bytes of file buffer, returns bytes read from file.
 * @param[out]    p          = Pointer to file buffer
 * @param[in]     timeout    = Timeout per mailbox cycle in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_FOEread(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut filename: *mut libc::c_char,
    mut password: uint32,
    mut psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut FOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut aFOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut wkc: libc::c_int = 0;
    let mut dataread: int32 = 0i32;
    let mut buffersize: int32 = 0;
    let mut packetnumber: int32 = 0;
    let mut prevpacket: int32 = 0i32;
    let mut fnsize: uint16 = 0;
    let mut maxdata: uint16 = 0;
    let mut segmentdata: uint16 = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut worktodo: boolean = 0;
    buffersize = *psize;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0i32);
    ec_clearmbx(&mut MbxOut);
    aFOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_FOEt;
    FOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_FOEt;
    fnsize = strlen(filename) as uint16;
    maxdata =
        ((*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int - 12i32) as uint16;
    if fnsize as libc::c_int > maxdata as libc::c_int {
        fnsize = maxdata
    }
    (*FOEp).MbxHeader.length = (0x6i32 + fnsize as libc::c_int) as uint16;
    (*FOEp).MbxHeader.address = 0u16;
    (*FOEp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* FoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*FOEp).MbxHeader.mbxtype = (ECT_MBXT_FOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
        as uint8;
    (*FOEp).OpCode = ECT_FOE_READ as uint8;
    (*FOEp).c2rust_unnamed.Password = password;
    /* copy filename in mailbox */
    memcpy(
        &mut *(*FOEp)
            .c2rust_unnamed_0
            .FileName
            .as_mut_ptr()
            .offset(0isize) as *mut libc::c_char as *mut libc::c_void,
        filename as *const libc::c_void,
        fnsize as libc::c_ulong,
    );
    /* send FoE request to slave */
    wkc = ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft, 20000i32);
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        loop {
            worktodo = 0u8;
            /* clean mailboxbuffer */
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* succeeded to read slave response ? */
                /* slave response should be FoE */
                if (*aFOEp).MbxHeader.mbxtype as libc::c_int & 0xfi32 == ECT_MBXT_FOE as libc::c_int
                {
                    if (*aFOEp).OpCode as libc::c_int == ECT_FOE_DATA as libc::c_int {
                        segmentdata = ((*aFOEp).MbxHeader.length as libc::c_int - 0x6i32) as uint16;
                        packetnumber = (*aFOEp).c2rust_unnamed.PacketNumber as int32;
                        prevpacket += 1;
                        if packetnumber == prevpacket
                            && dataread + segmentdata as libc::c_int <= buffersize
                        {
                            memcpy(
                                p,
                                &mut *(*aFOEp).c2rust_unnamed_0.Data.as_mut_ptr().offset(0isize)
                                    as *mut uint8
                                    as *const libc::c_void,
                                segmentdata as libc::c_ulong,
                            );
                            dataread += segmentdata as libc::c_int;
                            p = (p as *mut uint8).offset(segmentdata as libc::c_int as isize)
                                as *mut libc::c_void;
                            if segmentdata as libc::c_int == maxdata as libc::c_int {
                                worktodo = 1u8
                            }
                            (*FOEp).MbxHeader.length = 0x6u16;
                            (*FOEp).MbxHeader.address = 0u16;
                            (*FOEp).MbxHeader.priority = 0u8;
                            /* get new mailbox count value */
                            cnt = ec_nextmbxcnt(
                                (*(*context).slavelist.offset(slave as isize)).mbx_cnt,
                            ); /* FoE */
                            (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
                            (*FOEp).MbxHeader.mbxtype = (ECT_MBXT_FOE as libc::c_int
                                + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
                                as uint8;
                            (*FOEp).OpCode = ECT_FOE_ACK as uint8;
                            (*FOEp).c2rust_unnamed.PacketNumber = packetnumber as uint32;
                            /* send FoE ack to slave */
                            wkc = ecx_mbxsend(
                                context,
                                slave,
                                &mut MbxOut as *mut ec_mbxbuft,
                                20000i32,
                            );
                            if wkc <= 0i32 {
                                worktodo = 0u8
                            }
                            if (*context).FOEhook.is_some() {
                                (*context).FOEhook.expect("non-null function pointer")(
                                    slave,
                                    packetnumber,
                                    dataread,
                                );
                            }
                        } else {
                            /* FoE error */
                            wkc = -(EC_ERR_TYPE_FOE_BUF2SMALL as libc::c_int)
                        }
                    } else if (*aFOEp).OpCode as libc::c_int == ECT_FOE_ERROR as libc::c_int {
                        /* FoE error */
                        wkc = -(EC_ERR_TYPE_FOE_ERROR as libc::c_int)
                    } else {
                        /* unexpected mailbox received */
                        wkc = -(EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
                    }
                } else {
                    /* unexpected mailbox received */
                    wkc = -(EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
                }
                *psize = dataread
            }
            if !(worktodo != 0) {
                break;
            }
        }
    }
    return wkc;
}
/* * FoE write, blocking.
 *
 * @param[in]  context        = context struct
 * @param[in]  slave      = Slave number.
 * @param[in]  filename   = Filename of file to write.
 * @param[in]  password   = password.
 * @param[in]  psize      = Size in bytes of file buffer.
 * @param[out] p          = Pointer to file buffer
 * @param[in]  timeout    = Timeout per mailbox cycle in us, standard is EC_TIMEOUTRXM
 * @return Workcounter from last slave response
 */
#[no_mangle]
pub unsafe extern "C" fn ecx_FOEwrite(
    mut context: *mut ecx_contextt,
    mut slave: uint16,
    mut filename: *mut libc::c_char,
    mut password: uint32,
    mut psize: libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut FOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut aFOEp: *mut ec_FOEt = 0 as *mut ec_FOEt;
    let mut wkc: libc::c_int = 0;
    let mut packetnumber: int32 = 0;
    let mut sendpacket: int32 = 0i32;
    let mut fnsize: uint16 = 0;
    let mut maxdata: uint16 = 0;
    let mut segmentdata: libc::c_int = 0;
    let mut MbxIn: ec_mbxbuft = [0; 1487];
    let mut MbxOut: ec_mbxbuft = [0; 1487];
    let mut cnt: uint8 = 0;
    let mut worktodo: boolean = 0;
    let mut dofinalzero: boolean = 0;
    let mut tsize: libc::c_int = 0;
    ec_clearmbx(&mut MbxIn);
    /* Empty slave out mailbox if something is in. Timeout set to 0 */
    wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, 0i32);
    ec_clearmbx(&mut MbxOut);
    aFOEp = &mut MbxIn as *mut ec_mbxbuft as *mut ec_FOEt;
    FOEp = &mut MbxOut as *mut ec_mbxbuft as *mut ec_FOEt;
    dofinalzero = 0u8;
    fnsize = strlen(filename) as uint16;
    maxdata =
        ((*(*context).slavelist.offset(slave as isize)).mbx_l as libc::c_int - 12i32) as uint16;
    if fnsize as libc::c_int > maxdata as libc::c_int {
        fnsize = maxdata
    }
    (*FOEp).MbxHeader.length = (0x6i32 + fnsize as libc::c_int) as uint16;
    (*FOEp).MbxHeader.address = 0u16;
    (*FOEp).MbxHeader.priority = 0u8;
    /* get new mailbox count value, used as session handle */
    cnt = ec_nextmbxcnt((*(*context).slavelist.offset(slave as isize)).mbx_cnt); /* FoE */
    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
    (*FOEp).MbxHeader.mbxtype = (ECT_MBXT_FOE as libc::c_int
        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
        as uint8;
    (*FOEp).OpCode = ECT_FOE_WRITE as uint8;
    (*FOEp).c2rust_unnamed.Password = password;
    /* copy filename in mailbox */
    memcpy(
        &mut *(*FOEp)
            .c2rust_unnamed_0
            .FileName
            .as_mut_ptr()
            .offset(0isize) as *mut libc::c_char as *mut libc::c_void,
        filename as *const libc::c_void,
        fnsize as libc::c_ulong,
    );
    /* send FoE request to slave */
    wkc = ecx_mbxsend(context, slave, &mut MbxOut as *mut ec_mbxbuft, 20000i32);
    if wkc > 0i32 {
        /* succeeded to place mailbox in slave ? */
        loop {
            worktodo = 0u8;
            /* clean mailboxbuffer */
            ec_clearmbx(&mut MbxIn);
            /* read slave response */
            wkc = ecx_mbxreceive(context, slave, &mut MbxIn as *mut ec_mbxbuft, timeout);
            if wkc > 0i32 {
                /* succeeded to read slave response ? */
                /* slave response should be FoE */
                if (*aFOEp).MbxHeader.mbxtype as libc::c_int & 0xfi32 == ECT_MBXT_FOE as libc::c_int
                {
                    match (*aFOEp).OpCode as libc::c_int {
                        4 => {
                            packetnumber = (*aFOEp).c2rust_unnamed.PacketNumber as int32;
                            if packetnumber == sendpacket {
                                if (*context).FOEhook.is_some() {
                                    (*context).FOEhook.expect("non-null function pointer")(
                                        slave,
                                        packetnumber,
                                        psize,
                                    );
                                }
                                tsize = psize;
                                if tsize > maxdata as libc::c_int {
                                    tsize = maxdata as libc::c_int
                                }
                                if tsize != 0 || dofinalzero as libc::c_int != 0 {
                                    worktodo = 1u8;
                                    dofinalzero = 0u8;
                                    segmentdata = tsize;
                                    psize -= segmentdata;
                                    /* if last packet was full size, add a zero size packet as final */
                                    /* EOF is defined as packetsize < full packetsize */
                                    if psize == 0 && segmentdata == maxdata as libc::c_int {
                                        dofinalzero = 1u8
                                    }
                                    (*FOEp).MbxHeader.length = (0x6i32 + segmentdata) as uint16;
                                    (*FOEp).MbxHeader.address = 0u16;
                                    (*FOEp).MbxHeader.priority = 0u8;
                                    /* get new mailbox count value */
                                    cnt = ec_nextmbxcnt(
                                        (*(*context).slavelist.offset(slave as isize)).mbx_cnt,
                                    ); /* FoE */
                                    (*(*context).slavelist.offset(slave as isize)).mbx_cnt = cnt;
                                    (*FOEp).MbxHeader.mbxtype = (ECT_MBXT_FOE as libc::c_int
                                        + ((cnt as libc::c_int) << 4i32) as uint8 as libc::c_int)
                                        as uint8;
                                    (*FOEp).OpCode = ECT_FOE_DATA as uint8;
                                    sendpacket += 1;
                                    (*FOEp).c2rust_unnamed.PacketNumber = sendpacket as uint32;
                                    memcpy(
                                        &mut *(*FOEp)
                                            .c2rust_unnamed_0
                                            .Data
                                            .as_mut_ptr()
                                            .offset(0isize)
                                            as *mut uint8
                                            as *mut libc::c_void,
                                        p,
                                        segmentdata as libc::c_ulong,
                                    );
                                    p = (p as *mut uint8).offset(segmentdata as isize)
                                        as *mut libc::c_void;
                                    /* send FoE data to slave */
                                    wkc = ecx_mbxsend(
                                        context,
                                        slave,
                                        &mut MbxOut as *mut ec_mbxbuft,
                                        20000i32,
                                    );
                                    if wkc <= 0i32 {
                                        worktodo = 0u8
                                    }
                                }
                            } else {
                                /* FoE error */
                                wkc = -(EC_ERR_TYPE_FOE_PACKETNUMBER as libc::c_int)
                            }
                        }
                        6 => {
                            /* resend if data has been send before */
                            /* otherwise ignore */
                            if sendpacket != 0 {
                                if psize == 0 {
                                    dofinalzero = 1u8
                                }
                                psize += segmentdata;
                                p = (p as *mut uint8).offset(-(segmentdata as isize))
                                    as *mut libc::c_void;
                                sendpacket -= 1
                            }
                        }
                        5 => {
                            /* FoE error */
                            if (*aFOEp).c2rust_unnamed.ErrorCode == 0x8001u32 {
                                wkc = -(EC_ERR_TYPE_FOE_FILE_NOTFOUND as libc::c_int)
                            } else {
                                wkc = -(EC_ERR_TYPE_FOE_ERROR as libc::c_int)
                            }
                        }
                        _ => {
                            /* unexpected mailbox received */
                            wkc = -(EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
                        }
                    }
                } else {
                    /* unexpected mailbox received */
                    wkc = -(EC_ERR_TYPE_PACKET_ERROR as libc::c_int)
                }
            }
            if !(worktodo != 0) {
                break;
            }
        }
    }
    return wkc;
}
#[no_mangle]
pub unsafe extern "C" fn ec_FOEdefinehook(mut hook: *mut libc::c_void) -> libc::c_int {
    return ecx_FOEdefinehook(&mut ecx_context, hook);
}
#[no_mangle]
pub unsafe extern "C" fn ec_FOEread(
    mut slave: uint16,
    mut filename: *mut libc::c_char,
    mut password: uint32,
    mut psize: *mut libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_FOEread(
        &mut ecx_context,
        slave,
        filename,
        password,
        psize,
        p,
        timeout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ec_FOEwrite(
    mut slave: uint16,
    mut filename: *mut libc::c_char,
    mut password: uint32,
    mut psize: libc::c_int,
    mut p: *mut libc::c_void,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return ecx_FOEwrite(
        &mut ecx_context,
        slave,
        filename,
        password,
        psize,
        p,
        timeout,
    );
}
