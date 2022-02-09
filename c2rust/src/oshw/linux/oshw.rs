use ::libc;
extern "C" {
    #[no_mangle]
    fn if_nameindex() -> *mut if_nameindex;
    #[no_mangle]
    fn if_freenameindex(__ptr: *mut if_nameindex);
    #[no_mangle]
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn htons(__hostshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
}
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_nameindex {
    pub if_index: libc::c_uint,
    pub if_name: *mut libc::c_char,
}
pub type uint16_t = __uint16_t;
pub type uint16 = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_adapter {
    pub name: [libc::c_char; 128],
    pub desc: [libc::c_char; 128],
    pub next: *mut ec_adaptert,
}
pub type ec_adaptert = ec_adapter;
/*
 * Licensed under the GNU General Public License version 2 with exceptions. See
 * LICENSE file in the project root for full license information
 */
/* *
 * Host to Network byte order (i.e. to big endian).
 *
 * Note that Ethercat uses little endian byte order, except for the Ethernet
 * header which is big endian as usual.
 */
#[no_mangle]
pub unsafe extern "C" fn oshw_htons(mut host: uint16) -> uint16 {
    let mut network: uint16 = htons(host);
    return network;
}
/* *
 * Network (i.e. big endian) to Host byte order.
 *
 * Note that Ethercat uses little endian byte order, except for the Ethernet
 * header which is big endian as usual.
 */
#[no_mangle]
pub unsafe extern "C" fn oshw_ntohs(mut network: uint16) -> uint16 {
    let mut host: uint16 = ntohs(network);
    return host;
}
/* * Create list over available network adapters.
 * @return First element in linked list of adapters
 */
#[no_mangle]
pub unsafe extern "C" fn oshw_find_adapters() -> *mut ec_adaptert {
    let mut i: libc::c_int = 0;
    let mut ids: *mut if_nameindex = 0 as *mut if_nameindex;
    let mut adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
    let mut prev_adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
    let mut ret_adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
    /* Iterate all devices and create a local copy holding the name and
    * description.
    */
    ids = if_nameindex();
    i = 0 as libc::c_int;
    while (*ids.offset(i as isize)).if_index !=
              0 as libc::c_int as libc::c_uint {
        adapter =
            malloc(::core::mem::size_of::<ec_adaptert>() as libc::c_ulong) as
                *mut ec_adaptert;
        /* If we got more than one adapter save link list pointer to previous
       * adapter.
       * Else save as pointer to return.
       */
        if i != 0 {
            (*prev_adapter).next = adapter
        } else { ret_adapter = adapter }
        /* fetch description and name, in Linux we use the same on both */
        (*adapter).next = 0 as *mut ec_adaptert;
        if !(*ids.offset(i as isize)).if_name.is_null() {
            strncpy((*adapter).name.as_mut_ptr(),
                    (*ids.offset(i as isize)).if_name,
                    128 as libc::c_int as libc::c_ulong);
            (*adapter).name[(128 as libc::c_int - 1 as libc::c_int) as usize]
                = '\u{0}' as i32 as libc::c_char;
            strncpy((*adapter).desc.as_mut_ptr(),
                    (*ids.offset(i as isize)).if_name,
                    128 as libc::c_int as libc::c_ulong);
            (*adapter).desc[(128 as libc::c_int - 1 as libc::c_int) as usize]
                = '\u{0}' as i32 as libc::c_char
        } else {
            (*adapter).name[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char;
            (*adapter).desc[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char
        }
        prev_adapter = adapter;
        i += 1
    }
    if_freenameindex(ids);
    return ret_adapter;
}
/* * Free memory allocated memory used by adapter collection.
 * @param[in] adapter = First element in linked list of adapters
 * EC_NOFRAME.
 */
#[no_mangle]
pub unsafe extern "C" fn oshw_free_adapters(mut adapter: *mut ec_adaptert) {
    let mut next_adapter: *mut ec_adaptert = 0 as *mut ec_adaptert;
    /* Iterate the linked list and free all elements holding
    * adapter information
    */
    if !adapter.is_null() {
        next_adapter = (*adapter).next;
        free(adapter as *mut libc::c_void);
        while !next_adapter.is_null() {
            adapter = next_adapter;
            next_adapter = (*adapter).next;
            free(adapter as *mut libc::c_void);
        }
    };
}
