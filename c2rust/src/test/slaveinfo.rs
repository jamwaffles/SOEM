use crate::{
    ethercatcoe::{
        ec_ODlistt, ec_OElistt, ec_SDOread, ec_readODdescription, ec_readODlist, ec_readOE,
        ec_readOEsingle,
    },
    ethercatconfig::ec_config,
    ethercatdc::ec_configdc,
    ethercatmain::{
        ec_adaptert, ec_close, ec_eeprom2pdi, ec_eepromPDOt, ec_find_adapters, ec_free_adapters,
        ec_group, ec_init, ec_readstate, ec_siifind, ec_siigetbyte, ec_siistring, ec_slave,
        ec_slavecount, ec_statecheck, EcatError,
    },
    ethercatprint::{ec_ALstatuscode2string, ec_elist2string},
    ethercattype::{ec_state, SIICategory, ECT_SDO_PDOASSIGN, EC_TIMEOUTRXM, EC_TIMEOUTSTATE},
};
use libc::{memset, snprintf, sprintf, strcat, strcpy, strncmp};

pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

pub type boolean = uint8_t;
pub type int8 = int8_t;
pub type int16 = int16_t;
pub type int32 = int32_t;
pub type uint8 = uint8_t;
pub type uint16 = uint16_t;
pub type uint32 = uint32_t;
pub type int64 = int64_t;
pub type uint64 = uint64_t;

/* * \file
 * \brief Example code for Simple Open EtherCAT master
 *
 * Usage : slaveinfo [ifname] [-sdo] [-map]
 * Ifname is NIC interface, f.e. eth0.
 * Optional -sdo to display CoE object dictionary.
 * Optional -map to display slave PDO mapping
 *
 * This shows the configured slave data.
 *
 * (c)Arthur Ketels 2010 - 2011
 */

static mut IOmap: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut ODlist: ec_ODlistt = ec_ODlistt {
    Slave: 0,
    Entries: 0,
    Index: [0; 1024],
    DataType: [0; 1024],
    ObjectCode: [0; 1024],
    MaxSub: [0; 1024],
    Name: [[0; 41]; 1024],
};
#[no_mangle]
pub static mut OElist: ec_OElistt = ec_OElistt {
    Entries: 0,
    ValueInfo: [0; 256],
    DataType: [0; 256],
    BitLength: [0; 256],
    ObjAccess: [0; 256],
    Name: [[0; 41]; 256],
};
#[no_mangle]
pub static mut printSDO: boolean = 0u8;
#[no_mangle]
pub static mut printMAP: boolean = 0u8;
#[no_mangle]
pub static mut usdo: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub unsafe extern "C" fn dtype2string(mut dtype: uint16, mut bitlen: uint16) -> *mut libc::c_char {
    static mut str: [libc::c_char; 32] = [
        0i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    match dtype as libc::c_int {
        1 => {
            sprintf(
                str.as_mut_ptr(),
                b"BOOLEAN\x00" as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            sprintf(
                str.as_mut_ptr(),
                b"INTEGER8\x00" as *const u8 as *const libc::c_char,
            );
        }
        3 => {
            sprintf(
                str.as_mut_ptr(),
                b"INTEGER16\x00" as *const u8 as *const libc::c_char,
            );
        }
        4 => {
            sprintf(
                str.as_mut_ptr(),
                b"INTEGER32\x00" as *const u8 as *const libc::c_char,
            );
        }
        16 => {
            sprintf(
                str.as_mut_ptr(),
                b"INTEGER24\x00" as *const u8 as *const libc::c_char,
            );
        }
        21 => {
            sprintf(
                str.as_mut_ptr(),
                b"INTEGER64\x00" as *const u8 as *const libc::c_char,
            );
        }
        5 => {
            sprintf(
                str.as_mut_ptr(),
                b"UNSIGNED8\x00" as *const u8 as *const libc::c_char,
            );
        }
        6 => {
            sprintf(
                str.as_mut_ptr(),
                b"UNSIGNED16\x00" as *const u8 as *const libc::c_char,
            );
        }
        7 => {
            sprintf(
                str.as_mut_ptr(),
                b"UNSIGNED32\x00" as *const u8 as *const libc::c_char,
            );
        }
        22 => {
            sprintf(
                str.as_mut_ptr(),
                b"UNSIGNED24\x00" as *const u8 as *const libc::c_char,
            );
        }
        27 => {
            sprintf(
                str.as_mut_ptr(),
                b"UNSIGNED64\x00" as *const u8 as *const libc::c_char,
            );
        }
        8 => {
            sprintf(
                str.as_mut_ptr(),
                b"REAL32\x00" as *const u8 as *const libc::c_char,
            );
        }
        17 => {
            sprintf(
                str.as_mut_ptr(),
                b"REAL64\x00" as *const u8 as *const libc::c_char,
            );
        }
        48 => {
            sprintf(
                str.as_mut_ptr(),
                b"BIT1\x00" as *const u8 as *const libc::c_char,
            );
        }
        49 => {
            sprintf(
                str.as_mut_ptr(),
                b"BIT2\x00" as *const u8 as *const libc::c_char,
            );
        }
        50 => {
            sprintf(
                str.as_mut_ptr(),
                b"BIT3\x00" as *const u8 as *const libc::c_char,
            );
        }
        51 => {
            sprintf(
                str.as_mut_ptr(),
                b"BIT4\x00" as *const u8 as *const libc::c_char,
            );
        }
        52 => {
            sprintf(
                str.as_mut_ptr(),
                b"BIT5\x00" as *const u8 as *const libc::c_char,
            );
        }
        53 => {
            sprintf(
                str.as_mut_ptr(),
                b"BIT6\x00" as *const u8 as *const libc::c_char,
            );
        }
        54 => {
            sprintf(
                str.as_mut_ptr(),
                b"BIT7\x00" as *const u8 as *const libc::c_char,
            );
        }
        55 => {
            sprintf(
                str.as_mut_ptr(),
                b"BIT8\x00" as *const u8 as *const libc::c_char,
            );
        }
        9 => {
            sprintf(
                str.as_mut_ptr(),
                b"VISIBLE_STR(%d)\x00" as *const u8 as *const libc::c_char,
                bitlen as libc::c_int,
            );
        }
        10 => {
            sprintf(
                str.as_mut_ptr(),
                b"OCTET_STR(%d)\x00" as *const u8 as *const libc::c_char,
                bitlen as libc::c_int,
            );
        }
        _ => {
            sprintf(
                str.as_mut_ptr(),
                b"dt:0x%4.4X (%d)\x00" as *const u8 as *const libc::c_char,
                dtype as libc::c_int,
                bitlen as libc::c_int,
            );
        }
    }
    return str.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn otype2string(mut otype: uint16) -> *mut libc::c_char {
    static mut str: [libc::c_char; 32] = [
        0i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    match otype as libc::c_int {
        7 => {
            sprintf(
                str.as_mut_ptr(),
                b"VAR\x00" as *const u8 as *const libc::c_char,
            );
        }
        8 => {
            sprintf(
                str.as_mut_ptr(),
                b"ARRAY\x00" as *const u8 as *const libc::c_char,
            );
        }
        9 => {
            sprintf(
                str.as_mut_ptr(),
                b"RECORD\x00" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            sprintf(
                str.as_mut_ptr(),
                b"ot:0x%4.4X\x00" as *const u8 as *const libc::c_char,
                otype as libc::c_int,
            );
        }
    }
    return str.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn access2string(mut access: uint16) -> *mut libc::c_char {
    static mut str: [libc::c_char; 32] = [
        0i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    sprintf(
        str.as_mut_ptr(),
        b"%s%s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
        if access as libc::c_int & 0x1i32 != 0i32 {
            b"R\x00" as *const u8 as *const libc::c_char
        } else {
            b"_\x00" as *const u8 as *const libc::c_char
        },
        if access as libc::c_int & 0x8i32 != 0i32 {
            b"W\x00" as *const u8 as *const libc::c_char
        } else {
            b"_\x00" as *const u8 as *const libc::c_char
        },
        if access as libc::c_int & 0x2i32 != 0i32 {
            b"R\x00" as *const u8 as *const libc::c_char
        } else {
            b"_\x00" as *const u8 as *const libc::c_char
        },
        if access as libc::c_int & 0x10i32 != 0i32 {
            b"W\x00" as *const u8 as *const libc::c_char
        } else {
            b"_\x00" as *const u8 as *const libc::c_char
        },
        if access as libc::c_int & 0x4i32 != 0i32 {
            b"R\x00" as *const u8 as *const libc::c_char
        } else {
            b"_\x00" as *const u8 as *const libc::c_char
        },
        if access as libc::c_int & 0x20i32 != 0i32 {
            b"W\x00" as *const u8 as *const libc::c_char
        } else {
            b"_\x00" as *const u8 as *const libc::c_char
        },
    );
    return str.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn SDO2string(
    mut slave: uint16,
    mut index: uint16,
    mut subidx: uint8,
    mut dtype: uint16,
) -> *mut libc::c_char {
    let mut l: libc::c_int =
        core::mem::size_of::<[libc::c_char; 128]>().wrapping_sub(1usize) as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut u8: *mut uint8 = 0 as *mut uint8;
    let mut i8: *mut int8 = 0 as *mut int8;
    let mut u16: *mut uint16 = 0 as *mut uint16;
    let mut i16: *mut int16 = 0 as *mut int16;
    let mut u32: *mut uint32 = 0 as *mut uint32;
    let mut i32: *mut int32 = 0 as *mut int32;
    let mut u64: *mut uint64 = 0 as *mut uint64;
    let mut i64: *mut int64 = 0 as *mut int64;
    let mut sr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut dr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut es: [libc::c_char; 32] = [0; 32];
    memset(
        &mut usdo as *mut [libc::c_char; 128] as *mut libc::c_void,
        0i32,
        128usize,
    );
    ec_SDOread(
        slave,
        index,
        subidx,
        0u8,
        &mut l,
        &mut usdo as *mut [libc::c_char; 128] as *mut libc::c_void,
        EC_TIMEOUTRXM,
    );
    if EcatError != 0 {
        return ec_elist2string();
    } else {
        static mut str: [libc::c_char; 64] = [
            0i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        match dtype as libc::c_int {
            1 => {
                u8 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut uint8;
                if *u8 != 0 {
                    sprintf(
                        str.as_mut_ptr(),
                        b"TRUE\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                    sprintf(
                        str.as_mut_ptr(),
                        b"FALSE\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
            2 => {
                i8 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%2.2x / %d\x00" as *const u8 as *const libc::c_char,
                    *i8 as libc::c_int,
                    *i8 as libc::c_int,
                );
            }
            3 => {
                i16 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut int16;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%4.4x / %d\x00" as *const u8 as *const libc::c_char,
                    *i16 as libc::c_int,
                    *i16 as libc::c_int,
                );
            }
            4 | 16 => {
                i32 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut int32;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%8.8x / %d\x00" as *const u8 as *const libc::c_char,
                    *i32,
                    *i32,
                );
            }
            21 => {
                i64 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut int64;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%16.16lx / %ld\x00" as *const u8 as *const libc::c_char,
                    *i64,
                    *i64,
                );
            }
            5 => {
                u8 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut uint8;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%2.2x / %u\x00" as *const u8 as *const libc::c_char,
                    *u8 as libc::c_int,
                    *u8 as libc::c_int,
                );
            }
            6 => {
                u16 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut uint16;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%4.4x / %u\x00" as *const u8 as *const libc::c_char,
                    *u16 as libc::c_int,
                    *u16 as libc::c_int,
                );
            }
            7 | 22 => {
                u32 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut uint32;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%8.8x / %u\x00" as *const u8 as *const libc::c_char,
                    *u32,
                    *u32,
                );
            }
            27 => {
                u64 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut uint64;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%16.16lx / %lu\x00" as *const u8 as *const libc::c_char,
                    *u64,
                    *u64,
                );
            }
            8 => {
                sr = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char
                    as *mut libc::c_float;
                sprintf(
                    str.as_mut_ptr(),
                    b"%f\x00" as *const u8 as *const libc::c_char,
                    *sr as libc::c_double,
                );
            }
            17 => {
                dr = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char
                    as *mut libc::c_double;
                sprintf(
                    str.as_mut_ptr(),
                    b"%f\x00" as *const u8 as *const libc::c_char,
                    *dr,
                );
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                u8 = &mut *usdo.as_mut_ptr().offset(0isize) as *mut libc::c_char as *mut uint8;
                sprintf(
                    str.as_mut_ptr(),
                    b"0x%x / %u\x00" as *const u8 as *const libc::c_char,
                    *u8 as libc::c_int,
                    *u8 as libc::c_int,
                );
            }
            9 => {
                strcpy(
                    str.as_mut_ptr(),
                    b"\"\x00" as *const u8 as *const libc::c_char,
                );
                strcat(str.as_mut_ptr(), usdo.as_mut_ptr());
                strcat(
                    str.as_mut_ptr(),
                    b"\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            10 => {
                str[0usize] = 0i8;
                i = 0i32;
                while i < l {
                    sprintf(
                        es.as_mut_ptr(),
                        b"0x%2.2x \x00" as *const u8 as *const libc::c_char,
                        usdo[i as usize] as libc::c_int,
                    );
                    strcat(str.as_mut_ptr(), es.as_mut_ptr());
                    i += 1
                }
            }
            _ => {
                sprintf(
                    str.as_mut_ptr(),
                    b"Unknown type\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        return str.as_mut_ptr();
    };
}
/* * Read PDO assign structure */
#[no_mangle]
pub unsafe extern "C" fn si_PDOassign(
    mut slave: uint16,
    mut PDOassign: uint16,
    mut mapoffset: libc::c_int,
    mut bitoffset: libc::c_int,
) -> libc::c_int {
    let mut idxloop: uint16 = 0;
    let mut nidx: uint16 = 0;
    let mut subidxloop: uint16 = 0;
    let mut rdat: uint16 = 0;
    let mut idx: uint16 = 0;
    let mut subidx: uint16 = 0;
    let mut subcnt: uint8 = 0;
    let mut wkc: libc::c_int = 0;
    let mut bsize: libc::c_int = 0i32;
    let mut rdl: libc::c_int = 0;
    let mut rdat2: int32 = 0;
    let mut bitlen: uint8 = 0;
    let mut obj_subidx: uint8 = 0;
    let mut obj_idx: uint16 = 0;
    let mut abs_offset: libc::c_int = 0;
    let mut abs_bit: libc::c_int = 0;
    rdl = ::core::mem::size_of::<uint16>() as libc::c_int;
    rdat = 0u16;
    /* read PDO assign subindex 0 ( = number of PDO's) */
    wkc = ec_SDOread(
        slave,
        PDOassign,
        0u8,
        0u8,
        &mut rdl,
        &mut rdat as *mut uint16 as *mut libc::c_void,
        EC_TIMEOUTRXM,
    );
    rdat = rdat;
    /* positive result from slave ? */
    if wkc > 0i32 && rdat as libc::c_int > 0i32 {
        /* number of available sub indexes */
        nidx = rdat;
        bsize = 0i32;
        /* read all PDO's */
        idxloop = 1u16;
        while idxloop as libc::c_int <= nidx as libc::c_int {
            rdl = ::core::mem::size_of::<uint16>() as libc::c_int;
            rdat = 0u16;
            /* read PDO assign */
            wkc = ec_SDOread(
                slave,
                PDOassign,
                idxloop as uint8,
                0u8,
                &mut rdl,
                &mut rdat as *mut uint16 as *mut libc::c_void,
                EC_TIMEOUTRXM,
            );
            /* result is index of PDO */
            idx = rdat;
            if idx as libc::c_int > 0i32 {
                rdl = ::core::mem::size_of::<uint8>() as libc::c_int;
                subcnt = 0u8;
                /* read number of subindexes of PDO */
                wkc = ec_SDOread(
                    slave,
                    idx,
                    0u8,
                    0u8,
                    &mut rdl,
                    &mut subcnt as *mut uint8 as *mut libc::c_void,
                    EC_TIMEOUTRXM,
                );
                subidx = subcnt as uint16;
                /* for each subindex */
                subidxloop = 1u16;
                while subidxloop as libc::c_int <= subidx as libc::c_int {
                    rdl = ::core::mem::size_of::<int32>() as libc::c_int;
                    rdat2 = 0i32;
                    /* read SDO that is mapped in PDO */
                    wkc = ec_SDOread(
                        slave,
                        idx,
                        subidxloop as uint8,
                        0u8,
                        &mut rdl,
                        &mut rdat2 as *mut int32 as *mut libc::c_void,
                        EC_TIMEOUTRXM,
                    );
                    rdat2 = rdat2;
                    /* extract bitlength of SDO */
                    bitlen = (rdat2 & 0xffi32) as uint8;
                    bsize += bitlen as libc::c_int;
                    obj_idx = (rdat2 >> 16i32) as uint16;
                    obj_subidx = (rdat2 >> 8i32 & 0xffi32) as uint8;
                    abs_offset = mapoffset + bitoffset / 8i32;
                    abs_bit = bitoffset % 8i32;
                    ODlist.Slave = slave;
                    ODlist.Index[0usize] = obj_idx;
                    OElist.Entries = 0u16;
                    wkc = 0i32;
                    /* read object entry from dictionary if not a filler (0x0000:0x00) */
                    if obj_idx as libc::c_int != 0 || obj_subidx as libc::c_int != 0 {
                        wkc = ec_readOEsingle(0u16, obj_subidx, &mut ODlist, &mut OElist)
                    }
                    print!(
                        "  [0x{:4.4X}.{:1}] 0x{:4.4X}:0x{:2.2X} 0x{:2.2X}",
                        abs_offset as libc::c_uint,
                        abs_bit as libc::c_int,
                        obj_idx as libc::c_int as libc::c_uint,
                        obj_subidx as libc::c_int as libc::c_uint,
                        bitlen as libc::c_int as libc::c_uint
                    );
                    if wkc > 0i32 && OElist.Entries as libc::c_int != 0 {
                        println!(
                            " {:12} {:}",
                            {
                                std::ffi::CStr::from_ptr(dtype2string(
                                    OElist.DataType[obj_subidx as usize],
                                    bitlen as uint16,
                                )
                                    as *const libc::c_char)
                                .to_str()
                                .unwrap()
                            },
                            {
                                std::ffi::CStr::from_ptr(
                                    OElist.Name[obj_subidx as usize].as_mut_ptr()
                                        as *const libc::c_char,
                                )
                                .to_str()
                                .unwrap()
                            }
                        );
                    } else {
                        println!("");
                    }
                    bitoffset += bitlen as libc::c_int;
                    subidxloop = subidxloop.wrapping_add(1)
                }
            }
            idxloop = idxloop.wrapping_add(1)
        }
    }
    /* return total found bitlength (PDO) */
    return bsize;
}
#[no_mangle]
pub unsafe extern "C" fn si_map_sdo(mut slave: libc::c_int) -> libc::c_int {
    let mut wkc: libc::c_int = 0;
    let mut rdl: libc::c_int = 0;
    let mut retVal: libc::c_int = 0i32;
    let mut nSM: uint8 = 0;
    let mut iSM: uint8 = 0;
    let mut tSM: uint8 = 0;
    let mut Tsize: libc::c_int = 0;
    let mut outputs_bo: libc::c_int = 0;
    let mut inputs_bo: libc::c_int = 0;
    let mut SMt_bug_add: uint8 = 0;
    println!("PDO mapping according to CoE :");
    SMt_bug_add = 0u8;
    outputs_bo = 0i32;
    inputs_bo = 0i32;
    rdl = ::core::mem::size_of::<uint8>() as libc::c_int;
    nSM = 0u8;
    /* read SyncManager Communication Type object count */
    wkc = ec_SDOread(
        slave as uint16,
        0x1c00u16,
        0u8,
        0u8,
        &mut rdl,
        &mut nSM as *mut uint8 as *mut libc::c_void,
        EC_TIMEOUTRXM,
    );
    /* positive result from slave ? */
    if wkc > 0i32 && nSM as libc::c_int > 2i32 {
        /* make nSM equal to number of defined SM */
        nSM = nSM.wrapping_sub(1);
        /* limit to maximum number of SM defined, if true the slave can't be configured */
        if nSM as libc::c_int > 8i32 {
            nSM = 8u8
        }
        /* iterate for every SM type defined */
        iSM = 2u8;
        while iSM as libc::c_int <= nSM as libc::c_int {
            rdl = ::core::mem::size_of::<uint8>() as libc::c_int;
            tSM = 0u8;
            /* read SyncManager Communication Type */
            wkc = ec_SDOread(
                slave as uint16,
                0x1c00u16,
                (iSM as libc::c_int + 1i32) as uint8,
                0u8,
                &mut rdl,
                &mut tSM as *mut uint8 as *mut libc::c_void,
                EC_TIMEOUTRXM,
            );
            if wkc > 0i32 {
                if iSM as libc::c_int == 2i32 && tSM as libc::c_int == 2i32 {
                    // SM2 has type 2 == mailbox out, this is a bug in the slave!
                    SMt_bug_add = 1u8; // try to correct, this works if the types are 0 1 2 3 and should be 1 2 3 4
                    println!("Activated SM type workaround, possible incorrect mapping.");
                    // only add if SMt > 0
                }
                if tSM != 0 {
                    tSM = (tSM as libc::c_int + SMt_bug_add as libc::c_int) as uint8
                }
                if tSM as libc::c_int == 3i32 {
                    // outputs
                    /* read the assign RXPDO */
                    println!(
                        "  SM{:1} outputs\n     addr b   index: sub bitl data_type    name",
                        iSM as libc::c_int as libc::c_int
                    );
                    Tsize = si_PDOassign(
                        slave as uint16,
                        ECT_SDO_PDOASSIGN + iSM as u16,
                        ec_slave[slave as usize]
                            .outputs
                            .offset_from(&mut *IOmap.as_mut_ptr().offset(0isize)
                                as *mut libc::c_char
                                as *mut uint8) as libc::c_int,
                        outputs_bo,
                    );
                    outputs_bo += Tsize
                }
                if tSM as libc::c_int == 4i32 {
                    // inputs
                    /* read the assign TXPDO */
                    println!(
                        "  SM{:1} inputs\n     addr b   index: sub bitl data_type    name",
                        iSM as libc::c_int as libc::c_int
                    );
                    Tsize = si_PDOassign(
                        slave as uint16,
                        ECT_SDO_PDOASSIGN + iSM as u16,
                        ec_slave[slave as usize]
                            .inputs
                            .offset_from(&mut *IOmap.as_mut_ptr().offset(0isize)
                                as *mut libc::c_char
                                as *mut uint8) as libc::c_int,
                        inputs_bo,
                    );
                    inputs_bo += Tsize
                }
            }
            iSM = iSM.wrapping_add(1)
        }
    }
    /* found some I/O bits ? */
    if outputs_bo > 0i32 || inputs_bo > 0i32 {
        retVal = 1i32
    }
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn si_siiPDO(
    mut slave: uint16,
    mut t: uint8,
    mut mapoffset: libc::c_int,
    mut bitoffset: libc::c_int,
) -> libc::c_int {
    let mut a: uint16 = 0;
    let mut w: uint16 = 0;
    let mut c: uint16 = 0;
    let mut e: uint16 = 0;
    let mut er: uint16 = 0;
    let mut Size: uint16 = 0;
    let mut eectl: uint8 = 0;
    let mut obj_idx: uint16 = 0;
    let mut obj_subidx: uint8 = 0;
    let mut obj_name: uint8 = 0;
    let mut obj_datatype: uint8 = 0;
    let mut bitlen: uint8 = 0;
    let mut totalsize: libc::c_int = 0;
    let mut eepPDO: ec_eepromPDOt = ec_eepromPDOt {
        Startpos: 0,
        Length: 0,
        nPDO: 0,
        Index: [0; 512],
        SyncM: [0; 512],
        BitSize: [0; 512],
        SMbitsize: [0; 8],
    };
    let mut PDO: *mut ec_eepromPDOt = 0 as *mut ec_eepromPDOt;
    let mut abs_offset: libc::c_int = 0;
    let mut abs_bit: libc::c_int = 0;
    let mut str_name: [libc::c_char; 41] = [0; 41];
    eectl = ec_slave[slave as usize].eep_pdi;
    Size = 0u16;
    totalsize = 0i32;
    PDO = &mut eepPDO;
    (*PDO).nPDO = 0u16;
    (*PDO).Length = 0u16;
    (*PDO).Index[1usize] = 0u16;
    c = 0u16;
    while (c as libc::c_int) < 8i32 {
        (*PDO).SMbitsize[c as usize] = 0u16;
        c = c.wrapping_add(1)
    }
    if t as libc::c_int > 1i32 {
        t = 1u8
    }
    (*PDO).Startpos = ec_siifind(
        slave,
        (SIICategory::ECT_SII_PDO as libc::c_int + t as libc::c_int) as uint16,
    ) as uint16;
    if (*PDO).Startpos as libc::c_int > 0i32 {
        a = (*PDO).Startpos;
        let fresh0 = a;
        a = a.wrapping_add(1);
        w = ec_siigetbyte(slave, fresh0) as uint16;
        let fresh1 = a;
        a = a.wrapping_add(1);
        w = (w as libc::c_int + ((ec_siigetbyte(slave, fresh1) as libc::c_int) << 8i32)) as uint16;
        (*PDO).Length = w;
        c = 1u16;
        loop
        /* traverse through all PDOs */
        {
            (*PDO).nPDO = (*PDO).nPDO.wrapping_add(1);
            let fresh2 = a;
            a = a.wrapping_add(1);
            (*PDO).Index[(*PDO).nPDO as usize] = ec_siigetbyte(slave, fresh2) as uint16;
            let fresh3 = a;
            a = a.wrapping_add(1);
            (*PDO).Index[(*PDO).nPDO as usize] = ((*PDO).Index[(*PDO).nPDO as usize] as libc::c_int
                + ((ec_siigetbyte(slave, fresh3) as libc::c_int) << 8i32))
                as uint16;
            (*PDO).BitSize[(*PDO).nPDO as usize] = 0u16;
            c = c.wrapping_add(1);
            /* limit number of PDO entries in buffer */
            let fresh4 = a;
            a = a.wrapping_add(1);
            e = ec_siigetbyte(slave, fresh4) as uint16;
            let fresh5 = a;
            a = a.wrapping_add(1);
            (*PDO).SyncM[(*PDO).nPDO as usize] = ec_siigetbyte(slave, fresh5) as uint16;
            a = a.wrapping_add(1);
            let fresh6 = a;
            a = a.wrapping_add(1);
            obj_name = ec_siigetbyte(slave, fresh6);
            a = (a as libc::c_int + 2i32) as uint16;
            c = (c as libc::c_int + 2i32) as uint16;
            if ((*PDO).SyncM[(*PDO).nPDO as usize] as libc::c_int) < 8i32 {
                /* number of entries in PDO */
                /* active and in range SM? */
                str_name[0usize] = 0i8;
                if obj_name != 0 {
                    ec_siistring(str_name.as_mut_ptr(), slave, obj_name as uint16);
                }
                if t != 0 {
                    println!(
                        "  SM{:1} RXPDO 0x{:4.4X} {:}",
                        (*PDO).SyncM[(*PDO).nPDO as usize] as libc::c_int as libc::c_int,
                        (*PDO).Index[(*PDO).nPDO as usize] as libc::c_int as libc::c_uint,
                        {
                            std::ffi::CStr::from_ptr(str_name.as_mut_ptr() as *const libc::c_char)
                                .to_str()
                                .unwrap()
                        }
                    );
                } else {
                    println!(
                        "  SM{:1} TXPDO 0x{:4.4X} {:}",
                        (*PDO).SyncM[(*PDO).nPDO as usize] as libc::c_int as libc::c_int,
                        (*PDO).Index[(*PDO).nPDO as usize] as libc::c_int as libc::c_uint,
                        {
                            std::ffi::CStr::from_ptr(str_name.as_mut_ptr() as *const libc::c_char)
                                .to_str()
                                .unwrap()
                        }
                    );
                }
                println!("     addr b   index: sub bitl data_type    name");
                /* read all entries defined in PDO */
                er = 1u16;
                while er as libc::c_int <= e as libc::c_int {
                    c = (c as libc::c_int + 4i32) as uint16;
                    let fresh7 = a;
                    a = a.wrapping_add(1);
                    obj_idx = ec_siigetbyte(slave, fresh7) as uint16;
                    let fresh8 = a;
                    a = a.wrapping_add(1);
                    obj_idx = (obj_idx as libc::c_int
                        + ((ec_siigetbyte(slave, fresh8) as libc::c_int) << 8i32))
                        as uint16;
                    let fresh9 = a;
                    a = a.wrapping_add(1);
                    obj_subidx = ec_siigetbyte(slave, fresh9);
                    let fresh10 = a;
                    a = a.wrapping_add(1);
                    obj_name = ec_siigetbyte(slave, fresh10);
                    let fresh11 = a;
                    a = a.wrapping_add(1);
                    obj_datatype = ec_siigetbyte(slave, fresh11);
                    let fresh12 = a;
                    a = a.wrapping_add(1);
                    bitlen = ec_siigetbyte(slave, fresh12);
                    abs_offset = mapoffset + bitoffset / 8i32;
                    abs_bit = bitoffset % 8i32;
                    (*PDO).BitSize[(*PDO).nPDO as usize] =
                        ((*PDO).BitSize[(*PDO).nPDO as usize] as libc::c_int
                            + bitlen as libc::c_int) as uint16;
                    a = (a as libc::c_int + 2i32) as uint16;
                    /* skip entry if filler (0x0000:0x00) */
                    if obj_idx as libc::c_int != 0 || obj_subidx as libc::c_int != 0 {
                        str_name[0usize] = 0i8;
                        if obj_name != 0 {
                            ec_siistring(str_name.as_mut_ptr(), slave, obj_name as uint16);
                        }

                        print!(
                            "  [0x{:4.4X}.{:1}] 0x{:4.4X}:0x{:2.2X} 0x{:2.2X}",
                            abs_offset as libc::c_uint,
                            abs_bit as libc::c_int,
                            obj_idx as libc::c_int as libc::c_uint,
                            obj_subidx as libc::c_int as libc::c_uint,
                            bitlen as libc::c_int as libc::c_uint
                        );
                        println!(
                            " {:12} {:}",
                            {
                                std::ffi::CStr::from_ptr(dtype2string(
                                    obj_datatype as uint16,
                                    bitlen as uint16,
                                )
                                    as *const libc::c_char)
                                .to_str()
                                .unwrap()
                            },
                            {
                                std::ffi::CStr::from_ptr(
                                    str_name.as_mut_ptr() as *const libc::c_char
                                )
                                .to_str()
                                .unwrap()
                            }
                        );
                    }
                    bitoffset += bitlen as libc::c_int;
                    totalsize += bitlen as libc::c_int;
                    er = er.wrapping_add(1)
                }
                (*PDO).SMbitsize[(*PDO).SyncM[(*PDO).nPDO as usize] as usize] =
                    ((*PDO).SMbitsize[(*PDO).SyncM[(*PDO).nPDO as usize] as usize] as libc::c_int
                        + (*PDO).BitSize[(*PDO).nPDO as usize] as libc::c_int)
                        as uint16;
                Size = (Size as libc::c_int + (*PDO).BitSize[(*PDO).nPDO as usize] as libc::c_int)
                    as uint16;
                c = c.wrapping_add(1)
            } else {
                /* PDO deactivated because SM is 0xff or > EC_MAXSM */
                c = (c as libc::c_int + 4i32 * e as libc::c_int) as uint16; /* if eeprom control was previously pdi then restore */
                a = (a as libc::c_int + 8i32 * e as libc::c_int) as uint16;
                c = c.wrapping_add(1)
            }
            if (*PDO).nPDO as libc::c_int >= 0x200i32 - 1i32 {
                c = (*PDO).Length
            }
            if !((c as libc::c_int) < (*PDO).Length as libc::c_int) {
                break;
            }
        }
    }
    if eectl != 0 {
        ec_eeprom2pdi(slave);
    }
    return totalsize;
}
#[no_mangle]
pub unsafe extern "C" fn si_map_sii(mut slave: libc::c_int) -> libc::c_int {
    let mut retVal: libc::c_int = 0i32;
    let mut Tsize: libc::c_int = 0;
    let mut outputs_bo: libc::c_int = 0;
    let mut inputs_bo: libc::c_int = 0;
    println!("PDO mapping according to SII :");
    outputs_bo = 0i32;
    inputs_bo = 0i32;
    /* read the assign RXPDOs */
    Tsize = si_siiPDO(
        slave as uint16,
        1u8,
        ec_slave[slave as usize]
            .outputs
            .offset_from(&mut IOmap as *mut [libc::c_char; 4096] as *mut uint8)
            as libc::c_int,
        outputs_bo,
    );
    outputs_bo += Tsize;
    /* read the assign TXPDOs */
    Tsize = si_siiPDO(
        slave as uint16,
        0u8,
        ec_slave[slave as usize]
            .inputs
            .offset_from(&mut IOmap as *mut [libc::c_char; 4096] as *mut uint8)
            as libc::c_int,
        inputs_bo,
    );
    inputs_bo += Tsize;
    /* found some I/O bits ? */
    if outputs_bo > 0i32 || inputs_bo > 0i32 {
        retVal = 1i32
    }
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn si_sdo(mut cnt: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    ODlist.Entries = 0u16;
    memset(
        &mut ODlist as *mut ec_ODlistt as *mut libc::c_void,
        0i32,
        core::mem::size_of::<ec_ODlistt>(),
    );
    if ec_readODlist(cnt as uint16, &mut ODlist) != 0 {
        println!(
            " CoE Object Description found, {:} entries.",
            ODlist.Entries as libc::c_int as libc::c_int
        );
        i = 0i32;
        while i < ODlist.Entries as libc::c_int {
            let mut max_sub: uint8_t = 0;
            let mut name: [libc::c_char; 128] = [
                0i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            ec_readODdescription(i as uint16, &mut ODlist);
            while EcatError != 0 {
                println!(" - {:}", {
                    std::ffi::CStr::from_ptr(ec_elist2string() as *const libc::c_char)
                        .to_str()
                        .unwrap()
                });
            }
            snprintf(
                name.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 128]>().wrapping_sub(1usize),
                b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                ODlist.Name[i as usize].as_mut_ptr(),
            );
            if ODlist.ObjectCode[i as usize] as libc::c_int == 0x7i32 {
                println!(
                    "0x{:4x}      {:40}      [{:}]",
                    ODlist.Index[i as usize] as libc::c_int as libc::c_uint,
                    {
                        std::ffi::CStr::from_ptr(name.as_mut_ptr() as *const libc::c_char)
                            .to_str()
                            .unwrap()
                    },
                    {
                        std::ffi::CStr::from_ptr(otype2string(
                            ODlist.ObjectCode[i as usize] as uint16,
                        ) as *const libc::c_char)
                        .to_str()
                        .unwrap()
                    }
                );
            } else {
                println!(
                    "0x{:4x}      {:40}      [{:}  maxsub(0x{:2x} / {:})]",
                    ODlist.Index[i as usize] as libc::c_int as libc::c_uint,
                    {
                        std::ffi::CStr::from_ptr(name.as_mut_ptr() as *const libc::c_char)
                            .to_str()
                            .unwrap()
                    },
                    {
                        std::ffi::CStr::from_ptr(otype2string(
                            ODlist.ObjectCode[i as usize] as uint16,
                        ) as *const libc::c_char)
                        .to_str()
                        .unwrap()
                    },
                    ODlist.MaxSub[i as usize] as libc::c_int as libc::c_uint,
                    ODlist.MaxSub[i as usize] as libc::c_int as libc::c_int
                );
            }
            memset(
                &mut OElist as *mut ec_OElistt as *mut libc::c_void,
                0i32,
                core::mem::size_of::<ec_OElistt>(),
            );
            ec_readOE(i as uint16, &mut ODlist, &mut OElist);
            while EcatError != 0 {
                println!("- {:}", {
                    std::ffi::CStr::from_ptr(ec_elist2string() as *const libc::c_char)
                        .to_str()
                        .unwrap()
                });
            }
            if ODlist.ObjectCode[i as usize] as libc::c_int != 0x7i32 {
                let mut l: libc::c_int = ::core::mem::size_of::<uint8_t>() as libc::c_int;
                ec_SDOread(
                    cnt as uint16,
                    ODlist.Index[i as usize],
                    0u8,
                    0u8,
                    &mut l,
                    &mut max_sub as *mut uint8_t as *mut libc::c_void,
                    EC_TIMEOUTRXM,
                );
            } else {
                max_sub = ODlist.MaxSub[i as usize]
            }
            j = 0i32;
            while j < max_sub as libc::c_int + 1i32 {
                if OElist.DataType[j as usize] as libc::c_int > 0i32
                    && OElist.BitLength[j as usize] as libc::c_int > 0i32
                {
                    snprintf(
                        name.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 128]>().wrapping_sub(1usize),
                        b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                        OElist.Name[j as usize].as_mut_ptr(),
                    );
                    print!(
                        "    0x{:2x}      {:40}      [{:16} {:6}]      ",
                        j as libc::c_uint,
                        {
                            std::ffi::CStr::from_ptr(name.as_mut_ptr() as *const libc::c_char)
                                .to_str()
                                .unwrap()
                        },
                        {
                            std::ffi::CStr::from_ptr(dtype2string(
                                OElist.DataType[j as usize],
                                OElist.BitLength[j as usize],
                            )
                                as *const libc::c_char)
                            .to_str()
                            .unwrap()
                        },
                        {
                            std::ffi::CStr::from_ptr(
                                access2string(OElist.ObjAccess[j as usize]) as *const libc::c_char
                            )
                            .to_str()
                            .unwrap()
                        }
                    );
                    if OElist.ObjAccess[j as usize] as libc::c_int & 0x7i32 != 0 {
                        print!("{:}", {
                            std::ffi::CStr::from_ptr(SDO2string(
                                cnt as uint16,
                                ODlist.Index[i as usize],
                                j as uint8,
                                OElist.DataType[j as usize],
                            )
                                as *const libc::c_char)
                            .to_str()
                            .unwrap()
                        });
                    }
                    println!("");
                }
                j += 1
            }
            i += 1
        }
    } else {
        while EcatError != 0 {
            print!("{:}", {
                std::ffi::CStr::from_ptr(ec_elist2string() as *const libc::c_char)
                    .to_str()
                    .unwrap()
            });
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn slaveinfo(mut ifname: *mut libc::c_char) {
    let mut cnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nSM: libc::c_int = 0;
    let mut ssigen: uint16 = 0;
    let mut expectedWKC: libc::c_int = 0;
    println!("Starting slaveinfo");
    /* initialise SOEM, bind socket to ifname */
    if ec_init(ifname) != 0 {
        println!("ec_init on {:} succeeded.", {
            std::ffi::CStr::from_ptr(ifname as *const libc::c_char)
                .to_str()
                .unwrap()
        });
        /* find and auto-config slaves */
        if ec_config(
            0u8,
            &mut IOmap as *mut [libc::c_char; 4096] as *mut libc::c_void,
        ) > 0i32
        {
            ec_configdc();
            while EcatError != 0 {
                print!("{:}", {
                    std::ffi::CStr::from_ptr(ec_elist2string() as *const libc::c_char)
                        .to_str()
                        .unwrap()
                });
            }
            println!(
                "{:} slaves found and configured.",
                ec_slavecount as libc::c_int
            );
            expectedWKC = ec_group[0usize].outputsWKC as libc::c_int * 2i32
                + ec_group[0usize].inputsWKC as libc::c_int;
            println!("Calculated workcounter {:}", expectedWKC as libc::c_int);
            /* wait for all slaves to reach SAFE_OP state */
            ec_statecheck(
                0u16,
                ec_state::EC_STATE_SAFE_OP as uint16,
                EC_TIMEOUTSTATE * 3i32,
            );
            if ec_slave[0usize].state as libc::c_int != ec_state::EC_STATE_SAFE_OP as libc::c_int {
                println!("Not all slaves reached safe operational state.");
                ec_readstate();
                i = 1i32;
                while i <= ec_slavecount {
                    if ec_slave[i as usize].state as libc::c_int
                        != ec_state::EC_STATE_SAFE_OP as libc::c_int
                    {
                        println!(
                            "Slave {:} State={:2x} StatusCode={:4x} : {:}",
                            i as libc::c_int,
                            ec_slave[i as usize].state as libc::c_int as libc::c_uint,
                            ec_slave[i as usize].ALstatuscode as libc::c_int as libc::c_uint,
                            {
                                std::ffi::CStr::from_ptr(ec_ALstatuscode2string(
                                    ec_slave[i as usize].ALstatuscode,
                                )
                                    as *const libc::c_char)
                                .to_str()
                                .unwrap()
                            }
                        );
                    }
                    i += 1
                }
            }
            ec_readstate();
            cnt = 1i32;
            while cnt <= ec_slavecount {
                println!("\nSlave:{:}\n Name:{:}\n Output size: {:}bits\n Input size: {:}bits\n State: {:}\n Delay: {:}[ns]\n Has DC: {:}",
         cnt as libc::c_int,
          {
    std::ffi::CStr::from_ptr(ec_slave[cnt as usize].name.as_mut_ptr() as
                                 *const libc::c_char).to_str().unwrap()
},
         ec_slave[cnt as usize].Obits as libc::c_int as libc::c_int,
         ec_slave[cnt as usize].Ibits as libc::c_int as libc::c_int,
         ec_slave[cnt as usize].state as libc::c_int as libc::c_int,
         ec_slave[cnt as usize].pdelay as libc::c_int,
         ec_slave[cnt as usize].hasdc as libc::c_int as libc::c_int);
                if ec_slave[cnt as usize].hasdc != 0 {
                    println!(
                        " DCParentport:{:}",
                        ec_slave[cnt as usize].parentport as libc::c_int as libc::c_int
                    );
                }

                println!(
                    " Activeports:{:}.{:}.{:}.{:}",
                    (ec_slave[cnt as usize].activeports as libc::c_int & 0x1i32 > 0i32)
                        as libc::c_int as libc::c_int,
                    (ec_slave[cnt as usize].activeports as libc::c_int & 0x2i32 > 0i32)
                        as libc::c_int as libc::c_int,
                    (ec_slave[cnt as usize].activeports as libc::c_int & 0x4i32 > 0i32)
                        as libc::c_int as libc::c_int,
                    (ec_slave[cnt as usize].activeports as libc::c_int & 0x8i32 > 0i32)
                        as libc::c_int as libc::c_int
                );
                println!(
                    " Configured address: {:4.4x}",
                    ec_slave[cnt as usize].configadr as libc::c_int as libc::c_uint
                );
                println!(
                    " Man: {:8.8x} ID: {:8.8x} Rev: {:8.8x}",
                    ec_slave[cnt as usize].eep_man as libc::c_int as libc::c_uint,
                    ec_slave[cnt as usize].eep_id as libc::c_int as libc::c_uint,
                    ec_slave[cnt as usize].eep_rev as libc::c_int as libc::c_uint
                );
                nSM = 0i32;
                while nSM < 8i32 {
                    if ec_slave[cnt as usize].SM[nSM as usize].StartAddr as libc::c_int > 0i32 {
                        println!(
                            " SM{:1} A:{:4.4x} L:{:4} F:{:8.8x} Type:{:}",
                            nSM as libc::c_int,
                            ec_slave[cnt as usize].SM[nSM as usize].StartAddr as libc::c_int
                                as libc::c_uint,
                            ec_slave[cnt as usize].SM[nSM as usize].SMlength as libc::c_int
                                as libc::c_int,
                            ec_slave[cnt as usize].SM[nSM as usize].SMflags as libc::c_uint,
                            ec_slave[cnt as usize].SMtype[nSM as usize] as libc::c_int
                                as libc::c_int
                        );
                    }
                    nSM += 1
                }
                j = 0i32;
                while j < ec_slave[cnt as usize].FMMUunused as libc::c_int {
                    println!(" FMMU{:1} Ls:{:8.8x} Ll:{:4} Lsb:{:} Leb:{:} Ps:{:4.4x} Psb:{:} Ty:{:2.2x} Act:{:2.2x}",
         j as libc::c_int,
         ec_slave[cnt as usize].FMMU[j as usize].LogStart as libc::c_uint,
         ec_slave[cnt as usize].FMMU[j as usize].LogLength as libc::c_int as
    libc::c_int,
         ec_slave[cnt as usize].FMMU[j as usize].LogStartbit as libc::c_int as
    libc::c_int,
         ec_slave[cnt as usize].FMMU[j as usize].LogEndbit as libc::c_int as
    libc::c_int,
         ec_slave[cnt as usize].FMMU[j as usize].PhysStart as libc::c_int as
    libc::c_uint,
         ec_slave[cnt as usize].FMMU[j as usize].PhysStartBit as libc::c_int as
    libc::c_int,
         ec_slave[cnt as usize].FMMU[j as usize].FMMUtype as libc::c_int as
    libc::c_uint,
         ec_slave[cnt as usize].FMMU[j as usize].FMMUactive as libc::c_int as
    libc::c_uint);
                    j += 1
                }

                println!(
                    " FMMUfunc 0:{:} 1:{:} 2:{:} 3:{:}",
                    ec_slave[cnt as usize].FMMU0func as libc::c_int as libc::c_int,
                    ec_slave[cnt as usize].FMMU1func as libc::c_int as libc::c_int,
                    ec_slave[cnt as usize].FMMU2func as libc::c_int as libc::c_int,
                    ec_slave[cnt as usize].FMMU3func as libc::c_int as libc::c_int
                );
                println!(
                    " MBX length wr: {:} rd: {:} MBX protocols : {:2.2x}",
                    ec_slave[cnt as usize].mbx_l as libc::c_int as libc::c_int,
                    ec_slave[cnt as usize].mbx_rl as libc::c_int as libc::c_int,
                    ec_slave[cnt as usize].mbx_proto as libc::c_int as libc::c_uint
                );
                ssigen =
                    ec_siifind(cnt as uint16, SIICategory::ECT_SII_GENERAL as uint16) as uint16;
                /* SII general section */
                if ssigen != 0 {
                    ec_slave[cnt as usize].CoEdetails =
                        ec_siigetbyte(cnt as uint16, (ssigen as libc::c_int + 0x7i32) as uint16);
                    ec_slave[cnt as usize].FoEdetails =
                        ec_siigetbyte(cnt as uint16, (ssigen as libc::c_int + 0x8i32) as uint16);
                    ec_slave[cnt as usize].EoEdetails =
                        ec_siigetbyte(cnt as uint16, (ssigen as libc::c_int + 0x9i32) as uint16);
                    ec_slave[cnt as usize].SoEdetails =
                        ec_siigetbyte(cnt as uint16, (ssigen as libc::c_int + 0xai32) as uint16);
                    if ec_siigetbyte(cnt as uint16, (ssigen as libc::c_int + 0xdi32) as uint16)
                        as libc::c_int
                        & 0x2i32
                        > 0i32
                    {
                        ec_slave[cnt as usize].blockLRW = 1u8;
                        ec_slave[0usize].blockLRW = ec_slave[0usize].blockLRW.wrapping_add(1)
                    }
                    ec_slave[cnt as usize].Ebuscurrent =
                        ec_siigetbyte(cnt as uint16, (ssigen as libc::c_int + 0xei32) as uint16)
                            as int16;
                    ec_slave[cnt as usize].Ebuscurrent = (ec_slave[cnt as usize].Ebuscurrent
                        as libc::c_int
                        + ((ec_siigetbyte(cnt as uint16, (ssigen as libc::c_int + 0xfi32) as uint16)
                            as libc::c_int)
                            << 8i32))
                        as int16;
                    ec_slave[0usize].Ebuscurrent = (ec_slave[0usize].Ebuscurrent as libc::c_int
                        + ec_slave[cnt as usize].Ebuscurrent as libc::c_int)
                        as int16
                }

                println!(" CoE details: {:2.2x} FoE details: {:2.2x} EoE details: {:2.2x} SoE details: {:2.2x}",
         ec_slave[cnt as usize].CoEdetails as libc::c_int as libc::c_uint,
         ec_slave[cnt as usize].FoEdetails as libc::c_int as libc::c_uint,
         ec_slave[cnt as usize].EoEdetails as libc::c_int as libc::c_uint,
         ec_slave[cnt as usize].SoEdetails as libc::c_int as libc::c_uint);
                println!(
                    " Ebus current: {:}[mA]\n only LRD/LWR:{:}",
                    ec_slave[cnt as usize].Ebuscurrent as libc::c_int as libc::c_int,
                    ec_slave[cnt as usize].blockLRW as libc::c_int as libc::c_int
                );
                if ec_slave[cnt as usize].mbx_proto as libc::c_int & 0x4i32 != 0
                    && printSDO as libc::c_int != 0
                {
                    si_sdo(cnt);
                }
                if printMAP != 0 {
                    if ec_slave[cnt as usize].mbx_proto as libc::c_int & 0x4i32 != 0 {
                        si_map_sdo(cnt);
                    } else {
                        si_map_sii(cnt);
                    }
                }
                cnt += 1
            }
        } else {
            println!("No slaves found!");
        }
        println!("End slaveinfo, close socket");
        /* stop SOEM, close socket */
        ec_close();
    } else {
        println!("No socket connection on {:}\nExcecute as root", {
            std::ffi::CStr::from_ptr(ifname as *const libc::c_char)
                .to_str()
                .unwrap()
        });
    };
}
#[no_mangle]
pub static mut ifbuf: [libc::c_char; 1024] = [0; 1024];
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
    println!("SOEM (Simple Open EtherCAT Master)\nSlaveinfo");
    if argc > 1i32 {
        if argc > 2i32
            && strncmp(
                *argv.offset(2isize),
                b"-sdo\x00" as *const u8 as *const libc::c_char,
                core::mem::size_of::<[libc::c_char; 5]>(),
            ) == 0i32
        {
            printSDO = 1u8
        }
        if argc > 2i32
            && strncmp(
                *argv.offset(2isize),
                b"-map\x00" as *const u8 as *const libc::c_char,
                core::mem::size_of::<[libc::c_char; 5]>(),
            ) == 0i32
        {
            printMAP = 1u8
        }
        /* start slaveinfo */
        strcpy(ifbuf.as_mut_ptr(), *argv.offset(1isize));
        slaveinfo(ifbuf.as_mut_ptr());
    } else {
        println!("Usage: slaveinfo ifname [options]\nifname = eth0 for example\nOptions :\n -sdo : print SDO info\n -map : print mapping");
        println!("Available adapters");
        adapter = ec_find_adapters();
        while !adapter.is_null() {
            println!(
                "Description : {:}, Device to use for wpcap: {:}",
                {
                    std::ffi::CStr::from_ptr((*adapter).desc.as_mut_ptr() as *const libc::c_char)
                        .to_str()
                        .unwrap()
                },
                {
                    std::ffi::CStr::from_ptr((*adapter).name.as_mut_ptr() as *const libc::c_char)
                        .to_str()
                        .unwrap()
                }
            );
            adapter = (*adapter).next
        }
        ec_free_adapters(adapter);
    }
    println!("End program");
    return 0i32;
}
fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr())) }
}
