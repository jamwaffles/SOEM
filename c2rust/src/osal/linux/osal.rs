use std::mem;

use libc::{
    clock_gettime, free, malloc, memset, nanosleep, pthread_attr_destroy, pthread_attr_init,
    pthread_attr_setstacksize, pthread_attr_t, pthread_create, pthread_setschedparam, pthread_t,
    sched_param, suseconds_t, time_t, timespec, timeval,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_timet {
    pub sec: u32,
    pub usec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct osal_timer {
    pub stop_time: ec_timet,
}
pub type osal_timert = osal_timer;

#[no_mangle]
pub unsafe fn osal_usleep(mut usec: u32) -> libc::c_int {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    ts.tv_sec = usec.wrapping_div(1000000u32) as time_t;
    ts.tv_nsec = usec.wrapping_rem(1000000u32).wrapping_mul(1000u32) as syscall_slong_t;
    /* usleep is deprecated, use nanosleep instead */
    return nanosleep(&mut ts, 0 as *mut timespec);
}
#[no_mangle]
pub unsafe fn osal_current_time() -> ec_timet {
    let mut current_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut return_value: ec_timet = ec_timet { sec: 0, usec: 0 };
    clock_gettime(0i32, &mut current_time);
    return_value.sec = current_time.tv_sec as u32;
    return_value.usec = (current_time.tv_nsec / 1000i64) as u32;
    return return_value;
}
#[no_mangle]
pub unsafe fn osal_time_diff(
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
unsafe fn osal_getrelativetime(mut tv: *mut timeval) {
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
pub unsafe fn osal_timer_start(mut self_0: *mut osal_timert, mut timeout_usec: u32) {
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
    timeout.tv_sec = timeout_usec.wrapping_div(1000000u32) as time_t;
    timeout.tv_usec = timeout_usec.wrapping_rem(1000000u32) as suseconds_t;
    stop_time.tv_sec = start_time.tv_sec + timeout.tv_sec;
    stop_time.tv_usec = start_time.tv_usec + timeout.tv_usec;
    if stop_time.tv_usec >= 1000000i64 {
        stop_time.tv_sec += 1;
        stop_time.tv_usec -= 1000000i64
    }
    (*self_0).stop_time.sec = stop_time.tv_sec as u32;
    (*self_0).stop_time.usec = stop_time.tv_usec as u32;
}
#[no_mangle]
pub unsafe fn osal_timer_is_expired(mut self_0: *mut osal_timert) -> bool {
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
    stop_time.tv_sec = (*self_0).stop_time.sec as time_t;
    stop_time.tv_usec = (*self_0).stop_time.usec as suseconds_t;
    is_not_yet_expired = if current_time.tv_sec == stop_time.tv_sec {
        (current_time.tv_usec < stop_time.tv_usec) as libc::c_int
    } else {
        (current_time.tv_sec < stop_time.tv_sec) as libc::c_int
    };
    return (is_not_yet_expired == 0i32) as bool;
}
#[no_mangle]
pub unsafe fn osal_malloc(mut size: size_t) -> *mut libc::c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe fn osal_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}
#[no_mangle]
pub unsafe fn osal_thread_create(
    mut thandle: *mut libc::c_void,
    mut stacksize: libc::c_int,
    mut func: *mut libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut attr: pthread_attr_t = mem::zeroed();
    let mut threadp: *mut pthread_t = 0 as *mut pthread_t;
    threadp = thandle as *mut pthread_t;
    pthread_attr_init(&mut attr);
    pthread_attr_setstacksize(&mut attr, stacksize as size_t);
    ret = pthread_create(
        threadp,
        &mut attr,
        ::core::mem::transmute::<
            *mut libc::c_void,
            extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void,
        >(func),
        param,
    );
    if ret < 0i32 {
        return 0i32;
    }
    return 1i32;
}
#[no_mangle]
pub unsafe fn osal_thread_create_rt(
    mut thandle: *mut libc::c_void,
    mut stacksize: libc::c_int,
    mut func: *mut libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut attr: pthread_attr_t = mem::zeroed();
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
            extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void,
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
        core::mem::size_of::<sched_param>(),
    );
    schparam.sched_priority = 40i32;
    ret = pthread_setschedparam(*threadp, 1i32, &mut schparam);
    if ret < 0i32 {
        return 0i32;
    }
    return 1i32;
}
