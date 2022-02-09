use ::libc;
extern "C" {

    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> libc::c_int;

    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;

    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

    fn free(__ptr: *mut libc::c_void);

    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;

    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;

    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;

    fn pthread_attr_setstacksize(__attr: *mut pthread_attr_t, __stacksize: size_t) -> libc::c_int;

    fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: libc::c_int,
        __param: *const sched_param,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type boolean = uint8_t;
pub type uint32 = uint32_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_timet {
    pub sec: uint32,
    pub usec: uint32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct osal_timer {
    pub stop_time: ec_timet,
}
pub type osal_timert = osal_timer;
#[no_mangle]
pub unsafe extern "C" fn osal_usleep(mut usec: uint32) -> libc::c_int {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    ts.tv_sec = usec.wrapping_div(1000000u32) as __time_t;
    ts.tv_nsec = usec.wrapping_rem(1000000u32).wrapping_mul(1000u32) as __syscall_slong_t;
    /* usleep is deprecated, use nanosleep instead */
    return nanosleep(&mut ts, 0 as *mut timespec);
}
#[no_mangle]
pub unsafe extern "C" fn osal_current_time() -> ec_timet {
    let mut current_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut return_value: ec_timet = ec_timet { sec: 0, usec: 0 };
    clock_gettime(0i32, &mut current_time);
    return_value.sec = current_time.tv_sec as uint32;
    return_value.usec = (current_time.tv_nsec / 1000i64) as uint32;
    return return_value;
}
#[no_mangle]
pub unsafe extern "C" fn osal_time_diff(
    mut start: *mut ec_timet,
    mut end: *mut ec_timet,
    mut diff: *mut ec_timet,
) {
    if (*end).usec < (*start).usec {
        (*diff).sec = (*end).sec.wrapping_sub((*start).sec).wrapping_sub(1u32);
        (*diff).usec = (*end)
            .usec
            .wrapping_add(1000000u32)
            .wrapping_sub((*start).usec)
    } else {
        (*diff).sec = (*end).sec.wrapping_sub((*start).sec);
        (*diff).usec = (*end).usec.wrapping_sub((*start).usec)
    };
}
/* Returns time from some unspecified moment in past,
 * strictly increasing, used for time intervals measurement. */
unsafe extern "C" fn osal_getrelativetime(mut tv: *mut timeval) {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    /* Use clock_gettime to prevent possible live-lock.
     * Gettimeofday uses CLOCK_REALTIME that can get NTP timeadjust.
     * If this function preempts timeadjust and it uses vpage it live-locks.
     * Also when using XENOMAI, only clock_gettime is RT safe */
    clock_gettime(1i32, &mut ts);
    (*tv).tv_sec = ts.tv_sec;
    (*tv).tv_usec = ts.tv_nsec / 1000i64;
}
#[no_mangle]
pub unsafe extern "C" fn osal_timer_start(mut self_0: *mut osal_timert, mut timeout_usec: uint32) {
    let mut start_time: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut timeout: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut stop_time: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    osal_getrelativetime(&mut start_time);
    timeout.tv_sec = timeout_usec.wrapping_div(1000000u32) as __time_t;
    timeout.tv_usec = timeout_usec.wrapping_rem(1000000u32) as __suseconds_t;
    stop_time.tv_sec = start_time.tv_sec + timeout.tv_sec;
    stop_time.tv_usec = start_time.tv_usec + timeout.tv_usec;
    if stop_time.tv_usec >= 1000000i64 {
        stop_time.tv_sec += 1;
        stop_time.tv_usec -= 1000000i64
    }
    (*self_0).stop_time.sec = stop_time.tv_sec as uint32;
    (*self_0).stop_time.usec = stop_time.tv_usec as uint32;
}
#[no_mangle]
pub unsafe extern "C" fn osal_timer_is_expired(mut self_0: *mut osal_timert) -> boolean {
    let mut current_time: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut stop_time: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut is_not_yet_expired: libc::c_int = 0;
    osal_getrelativetime(&mut current_time);
    stop_time.tv_sec = (*self_0).stop_time.sec as __time_t;
    stop_time.tv_usec = (*self_0).stop_time.usec as __suseconds_t;
    is_not_yet_expired = if current_time.tv_sec == stop_time.tv_sec {
        (current_time.tv_usec < stop_time.tv_usec) as libc::c_int
    } else {
        (current_time.tv_sec < stop_time.tv_sec) as libc::c_int
    };
    return (is_not_yet_expired == 0i32) as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn osal_malloc(mut size: size_t) -> *mut libc::c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn osal_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn osal_thread_create(
    mut thandle: *mut libc::c_void,
    mut stacksize: libc::c_int,
    mut func: *mut libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut threadp: *mut pthread_t = 0 as *mut pthread_t;
    threadp = thandle as *mut pthread_t;
    pthread_attr_init(&mut attr);
    pthread_attr_setstacksize(&mut attr, stacksize as size_t);
    ret = pthread_create(
        threadp,
        &mut attr,
        ::core::mem::transmute::<
            *mut libc::c_void,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        >(func),
        param,
    );
    if ret < 0i32 {
        return 0i32;
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn osal_thread_create_rt(
    mut thandle: *mut libc::c_void,
    mut stacksize: libc::c_int,
    mut func: *mut libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut schparam: sched_param = sched_param { sched_priority: 0 };
    let mut threadp: *mut pthread_t = 0 as *mut pthread_t;
    threadp = thandle as *mut pthread_t;
    pthread_attr_init(&mut attr);
    pthread_attr_setstacksize(&mut attr, stacksize as size_t);
    ret = pthread_create(
        threadp,
        &mut attr,
        ::core::mem::transmute::<
            *mut libc::c_void,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        >(func),
        param,
    );
    pthread_attr_destroy(&mut attr);
    if ret < 0i32 {
        return 0i32;
    }
    memset(
        &mut schparam as *mut sched_param as *mut libc::c_void,
        0i32,
        ::core::mem::size_of::<sched_param>() as libc::c_ulong,
    );
    schparam.sched_priority = 40i32;
    ret = pthread_setschedparam(*threadp, 1i32, &mut schparam);
    if ret < 0i32 {
        return 0i32;
    }
    return 1i32;
}
