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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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
    j = 0i32; /* read eeprom status */
    while j <= 7i32 {
        if *crc as libc::c_int & 0x80i32 != 0 {
            *crc = ((*crc as libc::c_int) << 1i32 ^ 0x7i32) as uint8
        } else {
            *crc = ((*crc as libc::c_int) << 1i32) as uint8
        } /* force Eeprom from PDI */
        j += 1
    } /* set Eeprom to master */
}
#[no_mangle]
pub unsafe extern "C" fn SIIcrc(mut buf: *mut uint8) -> uint16 {
    let mut i: libc::c_int = 0; /* force Eeprom from PDI */
    let mut crc: uint8 = 0; /* set Eeprom to master */
    crc = 0xffu8;
    i = 0i32;
    while i <= 13i32 {
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
    let mut cc: libc::c_int = 0i32;
    let mut c: libc::c_int = 0;
    fp = fopen(fname, b"rb\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0i32;
    }
    loop {
        c = fgetc(fp);
        if !(c != -(1i32) && cc < 524288i32) {
            break;
        }
        let fresh1 = cc;
        cc = cc + 1;
        ebuf[fresh1 as usize] = c as uint8
    }
    *length = cc;
    fclose(fp);
    return 1i32;
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
    let mut retval: libc::c_int = 1i32;
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
        return 0i32;
    }
    hstart = 524288i32;
    hlength = 0i32;
    sum = 0i32;
    loop {
        memset(sline.as_mut_ptr() as *mut libc::c_void, 0i32, 256u64);
        sc = 0i32;
        loop {
            c = fgetc(fp);
            if !(c != -(1i32) && c != 0xai32 && sc < 256i32 - 1i32) {
                break;
            }
            let fresh2 = sc;
            sc = sc + 1;
            sline[fresh2 as usize] = c as libc::c_char
        }
        if c != -(1i32) && (sc < 11i32 || sline[0usize] as libc::c_int != ':' as i32) {
            c = -(1i32);
            retval = 0i32;
            println!("Invalid Intel Hex format.");
        }
        if c != -(1i32) {
            sn = sscanf(
                sline.as_mut_ptr(),
                b":%2x%4x%2x\x00" as *const u8 as *const libc::c_char,
                &mut ll as *mut libc::c_int,
                &mut ladr as *mut libc::c_int,
                &mut lt as *mut libc::c_int,
            );
            if sn == 3i32 && ladr + ll <= 524288i32 && lt == 0i32 {
                sum = ll + (ladr >> 8i32) + (ladr & 0xffi32) + lt;
                if ladr < hstart {
                    hstart = ladr
                }
                i = 0i32;
                while i < ll {
                    sn = sscanf(
                        &mut *sline.as_mut_ptr().offset((9i32 + (i << 1i32)) as isize)
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
                sum = 0x100i32 - sum & 0xffi32;
                sn = sscanf(
                    &mut *sline.as_mut_ptr().offset((9i32 + (i << 1i32)) as isize)
                        as *mut libc::c_char,
                    b"%2x\x00" as *const u8 as *const libc::c_char,
                    &mut lval as *mut libc::c_int,
                );
                if sn == 0 || sum - lval != 0i32 {
                    c = -(1i32);
                    retval = 0i32;
                    println!("Invalid checksum.");
                }
            }
        }
        if !(c != -(1i32)) {
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
        return 0i32;
    }
    cc = 0i32;
    while cc < length {
        fputc(ebuf[cc as usize] as libc::c_int, fp);
        cc += 1
    }
    fclose(fp);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn output_intelhex(
    mut fname: *mut libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut cc: libc::c_int = 0i32;
    let mut ll: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    fp = fopen(fname, b"w\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0i32;
    }
    while cc < length {
        ll = length - cc;
        if ll > 0x20i32 {
            ll = 0x20i32
        }
        sum = ll + (cc >> 8i32) + (cc & 0xffi32);
        fprintf(
            fp,
            b":%2.2X%4.4X00\x00" as *const u8 as *const libc::c_char,
            ll,
            cc,
        );
        i = 0i32;
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
            0x100i32 - sum & 0xffi32,
        );
        cc += ll
    }
    fprintf(fp, b":00000001FF\n\x00" as *const u8 as *const libc::c_char);
    fclose(fp);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn eeprom_read(
    mut slave_0: libc::c_int,
    mut start: libc::c_int,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ainc: libc::c_int = 4i32;
    let mut estat: uint16 = 0;
    let mut aiadr: uint16 = 0;
    let mut b4: uint32 = 0;
    let mut b8: uint64 = 0;
    let mut eepctl: uint8 = 0;
    if ec_slavecount >= slave_0 && slave_0 > 0i32 && start + length <= 524288i32 {
        aiadr = (1i32 - slave_0) as uint16;
        eepctl = 2u8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as uint16,
            ::core::mem::size_of::<uint8>() as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000i32,
        );
        eepctl = 0u8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as uint16,
            ::core::mem::size_of::<uint8>() as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000i32,
        );
        estat = 0u16;
        aiadr = (1i32 - slave_0) as uint16;
        ec_APRD(
            aiadr,
            ECT_REG_EEPSTAT as uint16,
            ::core::mem::size_of::<uint16>() as uint16,
            &mut estat as *mut uint16 as *mut libc::c_void,
            2000i32,
        );
        estat = estat;
        if estat as libc::c_int & 0x40i32 != 0 {
            ainc = 8i32;
            i = start;
            while i < start + length {
                b8 = ec_readeepromAP(aiadr, (i >> 1i32) as uint16, 20000i32);
                ebuf[i as usize] = (b8 & 0xffu64) as uint8;
                ebuf[(i + 1i32) as usize] = (b8 >> 8i32 & 0xffu64) as uint8;
                ebuf[(i + 2i32) as usize] = (b8 >> 16i32 & 0xffu64) as uint8;
                ebuf[(i + 3i32) as usize] = (b8 >> 24i32 & 0xffu64) as uint8;
                ebuf[(i + 4i32) as usize] = (b8 >> 32i32 & 0xffu64) as uint8;
                ebuf[(i + 5i32) as usize] = (b8 >> 40i32 & 0xffu64) as uint8;
                ebuf[(i + 6i32) as usize] = (b8 >> 48i32 & 0xffu64) as uint8;
                ebuf[(i + 7i32) as usize] = (b8 >> 56i32 & 0xffu64) as uint8;
                i += ainc
            }
        } else {
            i = start;
            while i < start + length {
                b4 = (ec_readeepromAP(aiadr, (i >> 1i32) as uint16, 20000i32) & 0xffffffffu64)
                    as uint32;
                ebuf[i as usize] = (b4 & 0xffu32) as uint8;
                ebuf[(i + 1i32) as usize] = (b4 >> 8i32 & 0xffu32) as uint8;
                ebuf[(i + 2i32) as usize] = (b4 >> 16i32 & 0xffu32) as uint8;
                ebuf[(i + 3i32) as usize] = (b4 >> 24i32 & 0xffu32) as uint8;
                i += ainc
            }
        }
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn eeprom_write(
    mut slave_0: libc::c_int,
    mut start: libc::c_int,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dc: libc::c_int = 0i32;
    let mut aiadr: uint16 = 0;
    let mut wbuf: *mut uint16 = 0 as *mut uint16;
    let mut eepctl: uint8 = 0;
    if ec_slavecount >= slave_0 && slave_0 > 0i32 && start + length <= 524288i32 {
        aiadr = (1i32 - slave_0) as uint16;
        eepctl = 2u8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as uint16,
            ::core::mem::size_of::<uint8>() as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000i32,
        );
        eepctl = 0u8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as uint16,
            ::core::mem::size_of::<uint8>() as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000i32,
        );
        aiadr = (1i32 - slave_0) as uint16;
        wbuf = &mut *ebuf.as_mut_ptr().offset(0isize) as *mut uint8 as *mut uint16;
        i = start;
        while i < start + length {
            ec_writeeepromAP(
                aiadr,
                (i >> 1i32) as uint16,
                *wbuf.offset((i >> 1i32) as isize),
                20000i32,
            );
            dc += 1;
            if dc >= 100i32 {
                dc = 0i32;
                print!(".");
                fflush(stdout);
            }
            i += 2i32
        }
        return 1i32;
    }
    return 0i32;
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
    if ec_slavecount >= slave_0 && slave_0 > 0i32 && alias_0 <= 0xffffi32 {
        aiadr = (1i32 - slave_0) as uint16;
        eepctl = 2u8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as uint16,
            ::core::mem::size_of::<uint8>() as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000i32,
        );
        eepctl = 0u8;
        ec_APWR(
            aiadr,
            ECT_REG_EEPCFG as uint16,
            ::core::mem::size_of::<uint8>() as uint16,
            &mut eepctl as *mut uint8 as *mut libc::c_void,
            2000i32,
        );
        ret = ec_writeeepromAP(aiadr, 0x4u16, alias_0 as uint16, 20000i32);
        if ret != 0 {
            ret = ec_writeeepromAP(aiadr, 0x7u16, crc, 20000i32)
        }
        return ret;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn eepromtool(
    mut ifname: *mut libc::c_char,
    mut slave_0: libc::c_int,
    mut mode_0: libc::c_int,
    mut fname: *mut libc::c_char,
) {
    let mut w: libc::c_int = 0;
    let mut rc: libc::c_int = 0i32;
    let mut estart: libc::c_int = 0;
    let mut esize: libc::c_int = 0;
    let mut wbuf: *mut uint16 = 0 as *mut uint16;
    /* initialise SOEM, bind socket to ifname */
    if ec_init(ifname) != 0 {
        println!("ec_init on {:} succeeded.", unsafe {
            std::ffi::CStr::from_ptr(ifname as *const libc::c_char)
                .to_str()
                .unwrap()
        }); /* detect number of slaves */
        w = 0i32; // read first 128 bytes
        wkc = ec_BRD(
            0u16,
            ECT_REG_TYPE as uint16,
            ::core::mem::size_of::<libc::c_int>() as uint16,
            &mut w as *mut libc::c_int as *mut libc::c_void,
            20000i32,
        ); // read reminder
        if wkc > 0i32 {
            ec_slavecount = wkc;
            println!("{:} slaves found.", ec_slavecount as libc::c_int);
            if ec_slavecount >= slave_0 && slave_0 > 0i32 {
                if mode_0 == 6i32 || mode_0 == 1i32 || mode_0 == 2i32 {
                    tstart = osal_current_time();
                    eeprom_read(slave_0, 0i32, 128i32);
                    wbuf = &mut *ebuf.as_mut_ptr().offset(0isize) as *mut uint8 as *mut uint16;

                    println!("Slave {:} data", slave_0 as libc::c_int);
                    println!(
                        " PDI Control      : {:4.4X}",
                        *wbuf.offset(0isize) as libc::c_int as libc::c_uint
                    );
                    println!(
                        " PDI Config       : {:4.4X}",
                        *wbuf.offset(0x1isize) as libc::c_int as libc::c_uint
                    );
                    println!(
                        " Config Alias     : {:4.4X}",
                        *wbuf.offset(0x4isize) as libc::c_int as libc::c_uint
                    );
                    println!(
                        " Checksum         : {:4.4X}",
                        *wbuf.offset(0x7isize) as libc::c_int as libc::c_uint
                    );
                    println!(
                        "   calculated     : {:4.4X}",
                        SIIcrc(&mut *ebuf.as_mut_ptr().offset(0isize)) as libc::c_int
                            as libc::c_uint
                    );
                    println!(
                        " Vendor ID        : {:8.8X}",
                        *(wbuf.offset(0x8isize) as *mut uint32) as libc::c_uint
                    );
                    println!(
                        " Product Code     : {:8.8X}",
                        *(wbuf.offset(0xaisize) as *mut uint32) as libc::c_uint
                    );
                    println!(
                        " Revision Number  : {:8.8X}",
                        *(wbuf.offset(0xcisize) as *mut uint32) as libc::c_uint
                    );
                    println!(
                        " Serial Number    : {:8.8X}",
                        *(wbuf.offset(0xeisize) as *mut uint32) as libc::c_uint
                    );
                    println!(
                        " Mailbox Protocol : {:4.4X}",
                        *wbuf.offset(0x1cisize) as libc::c_int as libc::c_uint
                    );
                    esize = (*wbuf.offset(0x3eisize) as libc::c_int + 1i32) * 128i32;
                    if esize > 524288i32 {
                        esize = 524288i32
                    }

                    println!(
                        " Size             : {:4.4X} = {:} bytes",
                        *wbuf.offset(0x3eisize) as libc::c_int as libc::c_uint,
                        esize as libc::c_int
                    );
                    println!(
                        " Version          : {:4.4X}",
                        *wbuf.offset(0x3fisize) as libc::c_int as libc::c_uint
                    );
                }
                if mode_0 == 1i32 || mode_0 == 2i32 {
                    if esize > 128i32 {
                        eeprom_read(slave_0, 128i32, esize - 128i32);
                    }
                    tend = osal_current_time();
                    osal_time_diff(&mut tstart, &mut tend, &mut tdif);
                    if mode_0 == 2i32 {
                        output_intelhex(fname, esize);
                    }
                    if mode_0 == 1i32 {
                        output_bin(fname, esize);
                    }
                    println!(
                        "\nTotal EEPROM read time :{:}ms",
                        ((tdif.usec as libc::c_long + tdif.sec as libc::c_long * 1000000i64)
                            / 1000i64) as libc::c_long
                    );
                }
                if mode_0 == 3i32 || mode_0 == 4i32 {
                    estart = 0i32;
                    if mode_0 == 4i32 {
                        rc = input_intelhex(fname, &mut estart, &mut esize)
                    }
                    if mode_0 == 3i32 {
                        rc = input_bin(fname, &mut esize)
                    }
                    if rc > 0i32 {
                        wbuf = &mut *ebuf.as_mut_ptr().offset(0isize) as *mut uint8 as *mut uint16;

                        println!("Slave {:}", slave_0 as libc::c_int);
                        println!(
                            " Vendor ID        : {:8.8X}",
                            *(wbuf.offset(0x8isize) as *mut uint32) as libc::c_uint
                        );
                        println!(
                            " Product Code     : {:8.8X}",
                            *(wbuf.offset(0xaisize) as *mut uint32) as libc::c_uint
                        );
                        println!(
                            " Revision Number  : {:8.8X}",
                            *(wbuf.offset(0xcisize) as *mut uint32) as libc::c_uint
                        );
                        println!(
                            " Serial Number    : {:8.8X}",
                            *(wbuf.offset(0xeisize) as *mut uint32) as libc::c_uint
                        );
                        print!("Busy");
                        fflush(stdout);
                        tstart = osal_current_time();
                        eeprom_write(slave_0, estart, esize);
                        tend = osal_current_time();
                        osal_time_diff(&mut tstart, &mut tend, &mut tdif);
                        println!(
                            "\nTotal EEPROM write time :{:}ms",
                            ((tdif.usec as libc::c_long + tdif.sec as libc::c_long * 1000000i64)
                                / 1000i64) as libc::c_long
                        );
                    } else {
                        println!("Error reading file, abort.");
                    }
                }
                if mode_0 == 5i32 {
                    if eeprom_read(slave_0, 0i32, 14i32) != 0 {
                        // read first 14 bytes
                        wbuf = &mut *ebuf.as_mut_ptr().offset(0isize) as *mut uint8 as *mut uint16;
                        *wbuf.offset(0x4isize) = alias as uint16;
                        if eeprom_writealias(
                            slave_0,
                            alias,
                            SIIcrc(&mut *ebuf.as_mut_ptr().offset(0isize)),
                        ) != 0
                        {
                            println!(
                                "Alias {:4.4X} written successfully to slave {:}",
                                alias as libc::c_uint, slave_0 as libc::c_int
                            );
                        } else {
                            println!("Alias not written");
                        }
                    } else {
                        print!("Could not read slave EEPROM");
                    }
                }
            } else {
                println!("Slave number outside range.");
            }
        } else {
            println!("No slaves found!");
        }
        println!("End, close socket");
        /* stop SOEM, close socket */
        ec_close();
    } else {
        println!("No socket connection on {:}\nExcecute as root", unsafe {
            std::ffi::CStr::from_ptr(ifname as *const libc::c_char)
                .to_str()
                .unwrap()
        });
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    println!("SOEM (Simple Open EtherCAT Master)\nEEPROM tool");
    mode = 0i32;
    if argc > 3i32 {
        slave = atoi(*argv.offset(2isize));
        if strncmp(
            *argv.offset(3isize),
            b"-i\x00" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
        ) == 0i32
        {
            mode = 6i32
        }
        if argc > 4i32 {
            if strncmp(
                *argv.offset(3isize),
                b"-r\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
            ) == 0i32
            {
                mode = 1i32
            }
            if strncmp(
                *argv.offset(3isize),
                b"-ri\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
            ) == 0i32
            {
                mode = 2i32
            }
            if strncmp(
                *argv.offset(3isize),
                b"-w\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
            ) == 0i32
            {
                mode = 3i32
            }
            if strncmp(
                *argv.offset(3isize),
                b"-wi\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
            ) == 0i32
            {
                mode = 4i32
            }
            if strncmp(
                *argv.offset(3isize),
                b"-walias\x00" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) == 0i32
            {
                mode = 5i32;
                alias = atoi(*argv.offset(4isize))
            }
        }
        /* start tool */
        eepromtool(*argv.offset(1isize), slave, mode, *argv.offset(4isize));
    } else {
        let mut adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;

        println!("Usage: eepromtool ifname slave OPTION fname|alias");
        println!("ifname = eth0 for example");
        println!("slave = slave number in EtherCAT order 1..n");
        println!("    -i      display EEPROM information");
        println!("    -walias write slave alias");
        println!("    -r      read EEPROM, output binary format");
        println!("    -ri     read EEPROM, output Intel Hex format");
        println!("    -w      write EEPROM, input binary format");
        println!("    -wi     write EEPROM, input Intel Hex format");
        println!("\nAvailable adapters:");
        adapter = ec_find_adapters();
        while !adapter.is_null() {
            println!(
                "    - {:}  ({:})",
                unsafe {
                    std::ffi::CStr::from_ptr((*adapter).name.as_mut_ptr() as *const libc::c_char)
                        .to_str()
                        .unwrap()
                },
                unsafe {
                    std::ffi::CStr::from_ptr((*adapter).desc.as_mut_ptr() as *const libc::c_char)
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
