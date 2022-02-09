use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn osal_current_time() -> ec_timet;
    #[no_mangle]
    fn osal_time_diff(start: *mut ec_timet, end: *mut ec_timet, diff: *mut ec_timet);
    #[no_mangle]
    static mut ec_slavecount: libc::c_int;
    #[no_mangle]
    fn ec_init(ifname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn ec_close();
    #[no_mangle]
    fn ec_readeepromAP(aiadr: uint16, eeproma: uint16, timeout: libc::c_int) -> uint64;
    #[no_mangle]
    fn ec_writeeepromAP(
        aiadr: uint16,
        eeproma: uint16,
        data: uint16,
        timeout: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ec_find_adapters() -> *mut ec_adaptert;
    #[no_mangle]
    fn ec_free_adapters(adapter: *mut ec_adaptert);
    #[no_mangle]
    fn ec_BRD(
        ADP: uint16,
        ADO: uint16,
        length: uint16,
        data: *mut libc::c_void,
        timeout: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ec_APRD(
        ADP: uint16,
        ADO: uint16,
        length: uint16,
        data: *mut libc::c_void,
        timeout: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ec_APWR(
        ADP: uint16,
        ADO: uint16,
        length: uint16,
        data: *mut libc::c_void,
        timeout: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uint8 = uint8_t;
pub type uint16 = uint16_t;
pub type uint32 = uint32_t;
pub type uint64 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_timet {
    pub sec: uint32,
    pub usec: uint32,
}
pub type C2RustUnnamed = libc::c_uint;
pub const ECT_REG_DCCYCLE1: C2RustUnnamed = 2468;
pub const ECT_REG_DCCYCLE0: C2RustUnnamed = 2464;
pub const ECT_REG_DCSTART0: C2RustUnnamed = 2448;
pub const ECT_REG_DCSYNCACT: C2RustUnnamed = 2433;
pub const ECT_REG_DCCUC: C2RustUnnamed = 2432;
pub const ECT_REG_DCTIMEFILT: C2RustUnnamed = 2356;
pub const ECT_REG_DCSPEEDCNT: C2RustUnnamed = 2352;
pub const ECT_REG_DCSYSDIFF: C2RustUnnamed = 2348;
pub const ECT_REG_DCSYSDELAY: C2RustUnnamed = 2344;
pub const ECT_REG_DCSYSOFFSET: C2RustUnnamed = 2336;
pub const ECT_REG_DCSOF: C2RustUnnamed = 2328;
pub const ECT_REG_DCSYSTIME: C2RustUnnamed = 2320;
pub const ECT_REG_DCTIME3: C2RustUnnamed = 2316;
pub const ECT_REG_DCTIME2: C2RustUnnamed = 2312;
pub const ECT_REG_DCTIME1: C2RustUnnamed = 2308;
pub const ECT_REG_DCTIME0: C2RustUnnamed = 2304;
pub const ECT_REG_SM1CONTR: C2RustUnnamed = 2063;
pub const ECT_REG_SM1ACT: C2RustUnnamed = 2062;
pub const ECT_REG_SM1STAT: C2RustUnnamed = 2061;
pub const ECT_REG_SM0STAT: C2RustUnnamed = 2053;
pub const ECT_REG_SM3: C2RustUnnamed = 2072;
pub const ECT_REG_SM2: C2RustUnnamed = 2064;
pub const ECT_REG_SM1: C2RustUnnamed = 2056;
pub const ECT_REG_SM0: C2RustUnnamed = 2048;
pub const ECT_REG_FMMU3: C2RustUnnamed = 1584;
pub const ECT_REG_FMMU2: C2RustUnnamed = 1568;
pub const ECT_REG_FMMU1: C2RustUnnamed = 1552;
pub const ECT_REG_FMMU0: C2RustUnnamed = 1536;
pub const ECT_REG_EEPDAT: C2RustUnnamed = 1288;
pub const ECT_REG_EEPADR: C2RustUnnamed = 1284;
pub const ECT_REG_EEPSTAT: C2RustUnnamed = 1282;
pub const ECT_REG_EEPCTL: C2RustUnnamed = 1282;
pub const ECT_REG_EEPCFG: C2RustUnnamed = 1280;
pub const ECT_REG_WDCNT: C2RustUnnamed = 1090;
pub const ECT_REG_LLCNT: C2RustUnnamed = 784;
pub const ECT_REG_PECODE: C2RustUnnamed = 782;
pub const ECT_REG_PECNT: C2RustUnnamed = 781;
pub const ECT_REG_EPUECNT: C2RustUnnamed = 780;
pub const ECT_REG_FRXERR: C2RustUnnamed = 776;
pub const ECT_REG_RXERR: C2RustUnnamed = 768;
pub const ECT_REG_IRQMASK: C2RustUnnamed = 512;
pub const ECT_REG_PDICTL: C2RustUnnamed = 320;
pub const ECT_REG_ALSTATCODE: C2RustUnnamed = 308;
pub const ECT_REG_ALSTAT: C2RustUnnamed = 304;
pub const ECT_REG_ALCTL: C2RustUnnamed = 288;
pub const ECT_REG_DLSTAT: C2RustUnnamed = 272;
pub const ECT_REG_DLALIAS: C2RustUnnamed = 259;
pub const ECT_REG_DLPORT: C2RustUnnamed = 257;
pub const ECT_REG_DLCTL: C2RustUnnamed = 256;
pub const ECT_REG_ALIAS: C2RustUnnamed = 18;
pub const ECT_REG_STADR: C2RustUnnamed = 16;
pub const ECT_REG_ESCSUP: C2RustUnnamed = 8;
pub const ECT_REG_PORTDES: C2RustUnnamed = 7;
pub const ECT_REG_TYPE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_adapter {
    pub name: [libc::c_char; 128],
    pub desc: [libc::c_char; 128],
    pub next: *mut ec_adaptert,
}
pub type ec_adaptert = ec_adapter;
#[no_mangle]
pub static mut ebuf: [uint8; 524288] = [0; 524288];
#[no_mangle]
pub static mut ob: uint8 = 0;
#[no_mangle]
pub static mut ow: uint16 = 0;
#[no_mangle]
pub static mut os: libc::c_int = 0;
#[no_mangle]
pub static mut slave: libc::c_int = 0;
#[no_mangle]
pub static mut alias: libc::c_int = 0;
#[no_mangle]
pub static mut tstart: ec_timet = ec_timet { sec: 0, usec: 0 };
#[no_mangle]
pub static mut tend: ec_timet = ec_timet { sec: 0, usec: 0 };
#[no_mangle]
pub static mut tdif: ec_timet = ec_timet { sec: 0, usec: 0 };

static mut wkc: libc::c_int = 0;
#[no_mangle]
pub static mut mode: libc::c_int = 0;
#[no_mangle]
pub static mut sline: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn calc_crc(mut crc: *mut uint8, mut b: uint8) {
    let mut j: libc::c_int = 0; /* force Eeprom from PDI */
    *crc = (*crc as libc::c_int ^ b as libc::c_int) as uint8; /* set Eeprom to master */
    j = 0 as libc::c_int; /* read eeprom status */
    while j <= 7 as libc::c_int {
        if *crc as libc::c_int & 0x80 as libc::c_int != 0 {
            *crc = ((*crc as libc::c_int) << 1 as libc::c_int ^ 0x7 as libc::c_int) as uint8
        } else {
            *crc = ((*crc as libc::c_int) << 1 as libc::c_int) as uint8
        } /* force Eeprom from PDI */
        j += 1
    } /* set Eeprom to master */
}
#[no_mangle]
pub unsafe extern "C" fn SIIcrc(mut buf: *mut uint8) -> uint16 {
    let mut i: libc::c_int = 0; /* force Eeprom from PDI */
    let mut crc: uint8 = 0; /* set Eeprom to master */
    crc = 0xff as libc::c_int as uint8;
    i = 0 as libc::c_int;
    while i <= 13 as libc::c_int {
        let fresh0 = buf;
        buf = buf.offset(1);
        calc_crc(&mut crc, *fresh0);
        i += 1
    }
    return crc as uint16;
}
#[no_mangle]
pub unsafe extern "C" fn input_bin(
    mut fname: *mut libc::c_char,
    mut length: *mut libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut cc: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    fp = fopen(fname, b"rb\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        c = fgetc(fp);
        if !(c != -(1 as libc::c_int) && cc < 524288 as libc::c_int) {
            break;
        }
        let fresh1 = cc;
        cc = cc + 1;
        ebuf[fresh1 as usize] = c as uint8
    }
    *length = cc;
    fclose(fp);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn input_intelhex(
    mut fname: *mut libc::c_char,
    mut start: *mut libc::c_int,
    mut length: *mut libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    let mut sc: libc::c_int = 0;
    let mut retval: libc::c_int = 1 as libc::c_int;
    let mut ll: libc::c_int = 0;
    let mut ladr: libc::c_int = 0;
    let mut lt: libc::c_int = 0;
    let mut sn: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lval: libc::c_int = 0;
    let mut hstart: libc::c_int = 0;
    let mut hlength: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    fp = fopen(fname, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    hstart = 524288 as libc::c_int;
    hlength = 0 as libc::c_int;
    sum = 0 as libc::c_int;
    loop {
        memset(
            sline.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            256 as libc::c_int as libc::c_ulong,
        );
        sc = 0 as libc::c_int;
        loop {
            c = fgetc(fp);
            if !(c != -(1 as libc::c_int)
                && c != 0xa as libc::c_int
                && sc < 256 as libc::c_int - 1 as libc::c_int)
            {
                break;
            }
            let fresh2 = sc;
            sc = sc + 1;
            sline[fresh2 as usize] = c as uint8 as libc::c_char
        }
        if c != -(1 as libc::c_int)
            && (sc < 11 as libc::c_int
                || sline[0 as libc::c_int as usize] as libc::c_int != ':' as i32)
        {
            c = -(1 as libc::c_int);
            retval = 0 as libc::c_int;
            printf(b"Invalid Intel Hex format.\n\x00" as *const u8 as *const libc::c_char);
        }
        if c != -(1 as libc::c_int) {
            sn = sscanf(
                sline.as_mut_ptr(),
                b":%2x%4x%2x\x00" as *const u8 as *const libc::c_char,
                &mut ll as *mut libc::c_int,
                &mut ladr as *mut libc::c_int,
                &mut lt as *mut libc::c_int,
            );
            if sn == 3 as libc::c_int
                && ladr + ll <= 524288 as libc::c_int
                && lt == 0 as libc::c_int
            {
                sum = ll + (ladr >> 8 as libc::c_int) + (ladr & 0xff as libc::c_int) + lt;
                if ladr < hstart {
                    hstart = ladr
                }
                i = 0 as libc::c_int;
                while i < ll {
                    sn = sscanf(
                        &mut *sline
                            .as_mut_ptr()
                            .offset((9 as libc::c_int + (i << 1 as libc::c_int)) as isize)
                            as *mut libc::c_char,
                        b"%2x\x00" as *const u8 as *const libc::c_char,
                        &mut lval as *mut libc::c_int,
                    );
                    ebuf[(ladr + i) as usize] = lval as uint8;
                    sum += lval as uint8 as libc::c_int;
                    i += 1
                }
                if ladr + ll - hstart > hlength {
                    hlength = ladr + ll - hstart
                }
                sum = 0x100 as libc::c_int - sum & 0xff as libc::c_int;
                sn = sscanf(
                    &mut *sline
                        .as_mut_ptr()
                        .offset((9 as libc::c_int + (i << 1 as libc::c_int)) as isize)
                        as *mut libc::c_char,
                    b"%2x\x00" as *const u8 as *const libc::c_char,
                    &mut lval as *mut libc::c_int,
                );
                if sn == 0 || sum - lval != 0 as libc::c_int {
                    c = -(1 as libc::c_int);
                    retval = 0 as libc::c_int;
                    printf(b"Invalid checksum.\n\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
        if !(c != -(1 as libc::c_int)) {
            break;
        }
    }
    if retval != 0 {
        *length = hlength;
        *start = hstart
    }
    fclose(fp);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn output_bin(
    mut fname: *mut libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut cc: libc::c_int = 0;
    fp = fopen(fname, b"wb\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    cc = 0 as libc::c_int;
    while cc < length {
        fputc(ebuf[cc as usize] as libc::c_int, fp);
        cc += 1
    }
    fclose(fp);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn output_intelhex(
    mut fname: *mut libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut cc: libc::c_int = 0 as libc::c_int;
    let mut ll: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    fp = fopen(fname, b"w\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    while cc < length {
        ll = length - cc;
        if ll > 0x20 as libc::c_int {
            ll = 0x20 as libc::c_int
        }
        sum = ll + (cc >> 8 as libc::c_int) + (cc & 0xff as libc::c_int);
        fprintf(
            fp,
            b":%2.2X%4.4X00\x00" as *const u8 as *const libc::c_char,
            ll,
            cc,
        );
        i = 0 as libc::c_int;
        while i < ll {
            fprintf(
                fp,
                b"%2.2X\x00" as *const u8 as *const libc::c_char,
                ebuf[(cc + i) as usize] as libc::c_int,
            );
            sum += ebuf[(cc + i) as usize] as libc::c_int;
            i += 1
        }
        fprintf(
            fp,
            b"%2.2X\n\x00" as *const u8 as *const libc::c_char,
            0x100 as libc::c_int - sum & 0xff as libc::c_int,
        );
        cc += ll
    }
    fprintf(fp, b":00000001FF\n\x00" as *const u8 as *const libc::c_char);
    fclose(fp);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eeprom_read(
    mut slave_0: libc::c_int,
    mut start: libc::c_int,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ainc: libc::c_int = 4 as libc::c_int;
    let mut estat: uint16 = 0;
    let mut aiadr: uint16 = 0;
    let mut b4: uint32 = 0;
    let mut b8: uint64 = 0;
    let mut eepctl: uint8 = 0;
    if ec_slavecount >= slave_0
        && slave_0 > 0 as libc::c_int
        && start + length <= 524288 as libc::c_int
    {
        aiadr = (1 as libc::c_int - slave_0) as uint16;
        eepctl = 2 as libc::c_int as uint8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int,
        );
        eepctl = 0 as libc::c_int as uint8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int,
        );
        estat = 0 as libc::c_int as uint16;
        aiadr = (1 as libc::c_int - slave_0) as uint16;
        ec_APRD(
            aiadr,
            ECT_REG_EEPSTAT as libc::c_int as uint16,
            ::core::mem::size_of::<uint16>() as libc::c_ulong as uint16,
            &mut estat as *mut uint16 as *mut libc::c_void,
            2000 as libc::c_int,
        );
        estat = estat;
        if estat as libc::c_int & 0x40 as libc::c_int != 0 {
            ainc = 8 as libc::c_int;
            i = start;
            while i < start + length {
                b8 = ec_readeepromAP(
                    aiadr,
                    (i >> 1 as libc::c_int) as uint16,
                    20000 as libc::c_int,
                );
                ebuf[i as usize] = (b8 & 0xff as libc::c_int as libc::c_ulong) as uint8;
                ebuf[(i + 1 as libc::c_int) as usize] =
                    (b8 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8;
                ebuf[(i + 2 as libc::c_int) as usize] =
                    (b8 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8;
                ebuf[(i + 3 as libc::c_int) as usize] =
                    (b8 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8;
                ebuf[(i + 4 as libc::c_int) as usize] =
                    (b8 >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8;
                ebuf[(i + 5 as libc::c_int) as usize] =
                    (b8 >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8;
                ebuf[(i + 6 as libc::c_int) as usize] =
                    (b8 >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8;
                ebuf[(i + 7 as libc::c_int) as usize] =
                    (b8 >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8;
                i += ainc
            }
        } else {
            i = start;
            while i < start + length {
                b4 = (ec_readeepromAP(
                    aiadr,
                    (i >> 1 as libc::c_int) as uint16,
                    20000 as libc::c_int,
                ) & 0xffffffff as libc::c_uint as libc::c_ulong) as uint32;
                ebuf[i as usize] = (b4 & 0xff as libc::c_int as libc::c_uint) as uint8;
                ebuf[(i + 1 as libc::c_int) as usize] =
                    (b4 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8;
                ebuf[(i + 2 as libc::c_int) as usize] =
                    (b4 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8;
                ebuf[(i + 3 as libc::c_int) as usize] =
                    (b4 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8;
                i += ainc
            }
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eeprom_write(
    mut slave_0: libc::c_int,
    mut start: libc::c_int,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dc: libc::c_int = 0 as libc::c_int;
    let mut aiadr: uint16 = 0;
    let mut wbuf: *mut uint16 = 0 as *mut uint16;
    let mut eepctl: uint8 = 0;
    if ec_slavecount >= slave_0
        && slave_0 > 0 as libc::c_int
        && start + length <= 524288 as libc::c_int
    {
        aiadr = (1 as libc::c_int - slave_0) as uint16;
        eepctl = 2 as libc::c_int as uint8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int,
        );
        eepctl = 0 as libc::c_int as uint8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int,
        );
        aiadr = (1 as libc::c_int - slave_0) as uint16;
        wbuf =
            &mut *ebuf.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut uint8 as *mut uint16;
        i = start;
        while i < start + length {
            ec_writeeepromAP(
                aiadr,
                (i >> 1 as libc::c_int) as uint16,
                *wbuf.offset((i >> 1 as libc::c_int) as isize),
                20000 as libc::c_int,
            );
            dc += 1;
            if dc >= 100 as libc::c_int {
                dc = 0 as libc::c_int;
                printf(b".\x00" as *const u8 as *const libc::c_char);
                fflush(stdout);
            }
            i += 2 as libc::c_int
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eeprom_writealias(
    mut slave_0: libc::c_int,
    mut alias_0: libc::c_int,
    mut crc: uint16,
) -> libc::c_int {
    let mut aiadr: uint16 = 0;
    let mut eepctl: uint8 = 0;
    let mut ret: libc::c_int = 0;
    if ec_slavecount >= slave_0 && slave_0 > 0 as libc::c_int && alias_0 <= 0xffff as libc::c_int {
        aiadr = (1 as libc::c_int - slave_0) as uint16;
        eepctl = 2 as libc::c_int as uint8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int,
        );
        eepctl = 0 as libc::c_int as uint8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as libc::c_int as uint16,
            ::core::mem::size_of::<uint8>() as libc::c_ulong as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000 as libc::c_int,
        );
        ret = ec_writeeepromAP(
            aiadr,
            0x4 as libc::c_int as uint16,
            alias_0 as uint16,
            20000 as libc::c_int,
        );
        if ret != 0 {
            ret = ec_writeeepromAP(
                aiadr,
                0x7 as libc::c_int as uint16,
                crc,
                20000 as libc::c_int,
            )
        }
        return ret;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eepromtool(
    mut ifname: *mut libc::c_char,
    mut slave_0: libc::c_int,
    mut mode_0: libc::c_int,
    mut fname: *mut libc::c_char,
) {
    let mut w: libc::c_int = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut estart: libc::c_int = 0;
    let mut esize: libc::c_int = 0;
    let mut wbuf: *mut uint16 = 0 as *mut uint16;
    /* initialise SOEM, bind socket to ifname */
    if ec_init(ifname) != 0 {
        printf(
            b"ec_init on %s succeeded.\n\x00" as *const u8 as *const libc::c_char,
            ifname,
        ); /* detect number of slaves */
        w = 0 as libc::c_int; // read first 128 bytes
        wkc = ec_BRD(
            0 as libc::c_int as uint16,
            ECT_REG_TYPE as libc::c_int as uint16,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as uint16,
            &mut w as *mut libc::c_int as *mut libc::c_void,
            20000 as libc::c_int,
        ); // read reminder
        if wkc > 0 as libc::c_int {
            ec_slavecount = wkc;
            printf(
                b"%d slaves found.\n\x00" as *const u8 as *const libc::c_char,
                ec_slavecount,
            );
            if ec_slavecount >= slave_0 && slave_0 > 0 as libc::c_int {
                if mode_0 == 6 as libc::c_int
                    || mode_0 == 1 as libc::c_int
                    || mode_0 == 2 as libc::c_int
                {
                    tstart = osal_current_time();
                    eeprom_read(slave_0, 0 as libc::c_int, 128 as libc::c_int);
                    wbuf = &mut *ebuf.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut uint8
                        as *mut uint16;
                    printf(
                        b"Slave %d data\n\x00" as *const u8 as *const libc::c_char,
                        slave_0,
                    );
                    printf(
                        b" PDI Control      : %4.4X\n\x00" as *const u8 as *const libc::c_char,
                        *wbuf.offset(0 as libc::c_int as isize) as libc::c_int,
                    );
                    printf(
                        b" PDI Config       : %4.4X\n\x00" as *const u8 as *const libc::c_char,
                        *wbuf.offset(0x1 as libc::c_int as isize) as libc::c_int,
                    );
                    printf(
                        b" Config Alias     : %4.4X\n\x00" as *const u8 as *const libc::c_char,
                        *wbuf.offset(0x4 as libc::c_int as isize) as libc::c_int,
                    );
                    printf(
                        b" Checksum         : %4.4X\n\x00" as *const u8 as *const libc::c_char,
                        *wbuf.offset(0x7 as libc::c_int as isize) as libc::c_int,
                    );
                    printf(
                        b"   calculated     : %4.4X\n\x00" as *const u8 as *const libc::c_char,
                        SIIcrc(&mut *ebuf.as_mut_ptr().offset(0 as libc::c_int as isize))
                            as libc::c_int,
                    );
                    printf(
                        b" Vendor ID        : %8.8X\n\x00" as *const u8 as *const libc::c_char,
                        *(wbuf.offset(0x8 as libc::c_int as isize) as *mut uint32),
                    );
                    printf(
                        b" Product Code     : %8.8X\n\x00" as *const u8 as *const libc::c_char,
                        *(wbuf.offset(0xa as libc::c_int as isize) as *mut uint32),
                    );
                    printf(
                        b" Revision Number  : %8.8X\n\x00" as *const u8 as *const libc::c_char,
                        *(wbuf.offset(0xc as libc::c_int as isize) as *mut uint32),
                    );
                    printf(
                        b" Serial Number    : %8.8X\n\x00" as *const u8 as *const libc::c_char,
                        *(wbuf.offset(0xe as libc::c_int as isize) as *mut uint32),
                    );
                    printf(
                        b" Mailbox Protocol : %4.4X\n\x00" as *const u8 as *const libc::c_char,
                        *wbuf.offset(0x1c as libc::c_int as isize) as libc::c_int,
                    );
                    esize = (*wbuf.offset(0x3e as libc::c_int as isize) as libc::c_int
                        + 1 as libc::c_int)
                        * 128 as libc::c_int;
                    if esize > 524288 as libc::c_int {
                        esize = 524288 as libc::c_int
                    }
                    printf(
                        b" Size             : %4.4X = %d bytes\n\x00" as *const u8
                            as *const libc::c_char,
                        *wbuf.offset(0x3e as libc::c_int as isize) as libc::c_int,
                        esize,
                    );
                    printf(
                        b" Version          : %4.4X\n\x00" as *const u8 as *const libc::c_char,
                        *wbuf.offset(0x3f as libc::c_int as isize) as libc::c_int,
                    );
                }
                if mode_0 == 1 as libc::c_int || mode_0 == 2 as libc::c_int {
                    if esize > 128 as libc::c_int {
                        eeprom_read(slave_0, 128 as libc::c_int, esize - 128 as libc::c_int);
                    }
                    tend = osal_current_time();
                    osal_time_diff(&mut tstart, &mut tend, &mut tdif);
                    if mode_0 == 2 as libc::c_int {
                        output_intelhex(fname, esize);
                    }
                    if mode_0 == 1 as libc::c_int {
                        output_bin(fname, esize);
                    }
                    printf(
                        b"\nTotal EEPROM read time :%ldms\n\x00" as *const u8
                            as *const libc::c_char,
                        (tdif.usec as libc::c_long
                            + tdif.sec as libc::c_long * 1000000 as libc::c_long)
                            / 1000 as libc::c_int as libc::c_long,
                    );
                }
                if mode_0 == 3 as libc::c_int || mode_0 == 4 as libc::c_int {
                    estart = 0 as libc::c_int;
                    if mode_0 == 4 as libc::c_int {
                        rc = input_intelhex(fname, &mut estart, &mut esize)
                    }
                    if mode_0 == 3 as libc::c_int {
                        rc = input_bin(fname, &mut esize)
                    }
                    if rc > 0 as libc::c_int {
                        wbuf = &mut *ebuf.as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut uint8 as *mut uint16;
                        printf(
                            b"Slave %d\n\x00" as *const u8 as *const libc::c_char,
                            slave_0,
                        );
                        printf(
                            b" Vendor ID        : %8.8X\n\x00" as *const u8 as *const libc::c_char,
                            *(wbuf.offset(0x8 as libc::c_int as isize) as *mut uint32),
                        );
                        printf(
                            b" Product Code     : %8.8X\n\x00" as *const u8 as *const libc::c_char,
                            *(wbuf.offset(0xa as libc::c_int as isize) as *mut uint32),
                        );
                        printf(
                            b" Revision Number  : %8.8X\n\x00" as *const u8 as *const libc::c_char,
                            *(wbuf.offset(0xc as libc::c_int as isize) as *mut uint32),
                        );
                        printf(
                            b" Serial Number    : %8.8X\n\x00" as *const u8 as *const libc::c_char,
                            *(wbuf.offset(0xe as libc::c_int as isize) as *mut uint32),
                        );
                        printf(b"Busy\x00" as *const u8 as *const libc::c_char);
                        fflush(stdout);
                        tstart = osal_current_time();
                        eeprom_write(slave_0, estart, esize);
                        tend = osal_current_time();
                        osal_time_diff(&mut tstart, &mut tend, &mut tdif);
                        printf(
                            b"\nTotal EEPROM write time :%ldms\n\x00" as *const u8
                                as *const libc::c_char,
                            (tdif.usec as libc::c_long
                                + tdif.sec as libc::c_long * 1000000 as libc::c_long)
                                / 1000 as libc::c_int as libc::c_long,
                        );
                    } else {
                        printf(
                            b"Error reading file, abort.\n\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                if mode_0 == 5 as libc::c_int {
                    if eeprom_read(slave_0, 0 as libc::c_int, 14 as libc::c_int) != 0 {
                        // read first 14 bytes
                        wbuf = &mut *ebuf.as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut uint8 as *mut uint16;
                        *wbuf.offset(0x4 as libc::c_int as isize) = alias as uint16;
                        if eeprom_writealias(
                            slave_0,
                            alias,
                            SIIcrc(&mut *ebuf.as_mut_ptr().offset(0 as libc::c_int as isize)),
                        ) != 0
                        {
                            printf(
                                b"Alias %4.4X written successfully to slave %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                alias,
                                slave_0,
                            );
                        } else {
                            printf(b"Alias not written\n\x00" as *const u8 as *const libc::c_char);
                        }
                    } else {
                        printf(
                            b"Could not read slave EEPROM\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            } else {
                printf(b"Slave number outside range.\n\x00" as *const u8 as *const libc::c_char);
            }
        } else {
            printf(b"No slaves found!\n\x00" as *const u8 as *const libc::c_char);
        }
        printf(b"End, close socket\n\x00" as *const u8 as *const libc::c_char);
        /* stop SOEM, close socket */
        ec_close();
    } else {
        printf(
            b"No socket connection on %s\nExcecute as root\n\x00" as *const u8
                as *const libc::c_char,
            ifname,
        );
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    printf(
        b"SOEM (Simple Open EtherCAT Master)\nEEPROM tool\n\x00" as *const u8
            as *const libc::c_char,
    );
    mode = 0 as libc::c_int;
    if argc > 3 as libc::c_int {
        slave = atoi(*argv.offset(2 as libc::c_int as isize));
        if strncmp(
            *argv.offset(3 as libc::c_int as isize),
            b"-i\x00" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            mode = 6 as libc::c_int
        }
        if argc > 4 as libc::c_int {
            if strncmp(
                *argv.offset(3 as libc::c_int as isize),
                b"-r\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                mode = 1 as libc::c_int
            }
            if strncmp(
                *argv.offset(3 as libc::c_int as isize),
                b"-ri\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                mode = 2 as libc::c_int
            }
            if strncmp(
                *argv.offset(3 as libc::c_int as isize),
                b"-w\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                mode = 3 as libc::c_int
            }
            if strncmp(
                *argv.offset(3 as libc::c_int as isize),
                b"-wi\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                mode = 4 as libc::c_int
            }
            if strncmp(
                *argv.offset(3 as libc::c_int as isize),
                b"-walias\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                mode = 5 as libc::c_int;
                alias = atoi(*argv.offset(4 as libc::c_int as isize))
            }
        }
        /* start tool */
        eepromtool(
            *argv.offset(1 as libc::c_int as isize),
            slave,
            mode,
            *argv.offset(4 as libc::c_int as isize),
        );
    } else {
        let mut adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
        printf(
            b"Usage: eepromtool ifname slave OPTION fname|alias\n\x00" as *const u8
                as *const libc::c_char,
        );
        printf(b"ifname = eth0 for example\n\x00" as *const u8 as *const libc::c_char);
        printf(
            b"slave = slave number in EtherCAT order 1..n\n\x00" as *const u8
                as *const libc::c_char,
        );
        printf(b"    -i      display EEPROM information\n\x00" as *const u8 as *const libc::c_char);
        printf(b"    -walias write slave alias\n\x00" as *const u8 as *const libc::c_char);
        printf(
            b"    -r      read EEPROM, output binary format\n\x00" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"    -ri     read EEPROM, output Intel Hex format\n\x00" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"    -w      write EEPROM, input binary format\n\x00" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"    -wi     write EEPROM, input Intel Hex format\n\x00" as *const u8
                as *const libc::c_char,
        );
        printf(b"\nAvailable adapters:\n\x00" as *const u8 as *const libc::c_char);
        adapter = ec_find_adapters();
        while !adapter.is_null() {
            printf(
                b"    - %s  (%s)\n\x00" as *const u8 as *const libc::c_char,
                (*adapter).name.as_mut_ptr(),
                (*adapter).desc.as_mut_ptr(),
            );
            adapter = (*adapter).next
        }
        ec_free_adapters(adapter);
    }
    printf(b"End program\n\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
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
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
