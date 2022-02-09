use libc::strncpy;

pub type __uint16_t = libc::c_ushort;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_nameindex {
    pub if_index: libc::c_uint,
    pub if_name: *mut libc::c_char,
}
pub type uint16_t = __uint16_t;
pub type uint16 = uint16_t;

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
    i = 0i32;
    while (*ids.offset(i as isize)).if_index != 0u32 {
        adapter = malloc(core::mem::size_of::<ec_adaptert>()) as *mut ec_adaptert;
        /* If we got more than one adapter save link list pointer to previous
         * adapter.
         * Else save as pointer to return.
         */
        if i != 0 {
            (*prev_adapter).next = adapter
        } else {
            ret_adapter = adapter
        }
        /* fetch description and name, in Linux we use the same on both */
        (*adapter).next = 0 as *mut ec_adaptert;
        if !(*ids.offset(i as isize)).if_name.is_null() {
            strncpy(
                (*adapter).name.as_mut_ptr(),
                (*ids.offset(i as isize)).if_name,
                128u64,
            );
            (*adapter).name[(128i32 - 1i32) as usize] = '\u{0}' as libc::c_char;
            strncpy(
                (*adapter).desc.as_mut_ptr(),
                (*ids.offset(i as isize)).if_name,
                128u64,
            );
            (*adapter).desc[(128i32 - 1i32) as usize] = '\u{0}' as libc::c_char
        } else {
            (*adapter).name[0usize] = '\u{0}' as libc::c_char;
            (*adapter).desc[0usize] = '\u{0}' as libc::c_char
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
