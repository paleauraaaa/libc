//! Xbox 360 interfaces provided by the NXEDK Picolibc port.

use crate::prelude::*;
use crate::{
    off_t,
    size_t,
    ssize_t,
    FILE,
};

pub type pthread_t = c_int;
pub type time_t = i64;
pub type clockid_t = c_ulong;
pub type va_list = *mut c_char;

s! {
    pub struct pthread_attr_t {
        pub __stack_size: c_int,
        pub __detachstate: c_int,
    }

    pub struct pthread_mutexattr_t {
        pub __robustness: c_int,
    }

    pub struct pthread_mutex_t {
        pub __robustness: c_int,
        pub __lock: c_int,
        pub __owner: c_uint,
        pub __next_owned: *mut pthread_mutex_t,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }
}

pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;
pub const PTHREAD_SCOPE_PROCESS: c_int = 0;
pub const PTHREAD_SCOPE_SYSTEM: c_int = 1;
pub const PTHREAD_MUTEX_STALLED: c_int = 0;
pub const PTHREAD_MUTEX_ROBUST: c_int = 1;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __robustness: PTHREAD_MUTEX_STALLED,
    __lock: 0,
    __owner: 0,
    __next_owned: core::ptr::null_mut(),
};

pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const EBADF: c_int = 9;
pub const EAGAIN: c_int = 11;
pub const EWOULDBLOCK: c_int = EAGAIN;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENOSPC: c_int = 28;
pub const EDEADLK: c_int = 45;
pub const ENOSYS: c_int = 88;
pub const ENAMETOOLONG: c_int = 91;
pub const ENOTSUP: c_int = 134;
pub const EOVERFLOW: c_int = 139;
pub const ENOTRECOVERABLE: c_int = 141;
pub const EOWNERDEAD: c_int = 142;

pub const PROT_NONE: c_int = 0;
pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const PROT_EXEC: c_int = 4;
pub const MAP_SHARED: c_int = 0x001;
pub const MAP_PRIVATE: c_int = 0x002;
pub const MAP_FIXED: c_int = 0x010;
pub const MAP_ANONYMOUS: c_int = 0x020;
pub const MAP_ANON: c_int = MAP_ANONYMOUS;
pub const MAP_LARGE_PAGES: c_int = 0x040;
pub const MAP_FAILED: *mut c_void = !0 as *mut c_void;

pub const MADV_NORMAL: c_int = 0;
pub const MADV_RANDOM: c_int = 1;
pub const MADV_SEQUENTIAL: c_int = 2;
pub const MADV_WILLNEED: c_int = 3;
pub const MADV_DONTNEED: c_int = 4;
pub const MADV_FREE: c_int = 8;
pub const MADV_REMOVE: c_int = 9;
pub const MADV_DONTFORK: c_int = 10;
pub const MADV_DOFORK: c_int = 11;
pub const MADV_MERGEABLE: c_int = 12;
pub const MADV_UNMERGEABLE: c_int = 13;
pub const MADV_HUGEPAGE: c_int = 14;
pub const MADV_NOHUGEPAGE: c_int = 15;
pub const MADV_DONTDUMP: c_int = 16;
pub const MADV_DODUMP: c_int = 17;
pub const MADV_WIPEONFORK: c_int = 18;
pub const MADV_KEEPONFORK: c_int = 19;
pub const MADV_COLD: c_int = 20;
pub const MADV_PAGEOUT: c_int = 21;
pub const MADV_POPULATE_READ: c_int = 22;
pub const MADV_POPULATE_WRITE: c_int = 23;

pub const CLOCK_REALTIME: clockid_t = 1;
pub const CLOCK_MONOTONIC: clockid_t = 4;

extern "C" {
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;
    pub fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;

    pub fn vprintf(format: *const c_char, ap: va_list) -> c_int;
    pub fn vfprintf(stream: *mut FILE, format: *const c_char, ap: va_list) -> c_int;
    pub fn vsprintf(buf: *mut c_char, format: *const c_char, ap: va_list) -> c_int;
    pub fn vsnprintf(buf: *mut c_char, size: size_t, format: *const c_char, ap: va_list) -> c_int;
    pub fn vasprintf(buf: *mut *mut c_char, format: *const c_char, ap: va_list) -> c_int;
    pub fn vdprintf(fd: c_int, format: *const c_char, ap: va_list) -> c_int;
    pub fn vscanf(format: *const c_char, ap: va_list) -> c_int;
    pub fn vfscanf(stream: *mut FILE, format: *const c_char, ap: va_list) -> c_int;
    pub fn vsscanf(buf: *const c_char, format: *const c_char, ap: va_list) -> c_int;

    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stack_size: c_int) -> c_int;
    pub fn pthread_attr_getstacksize(attr: *mut pthread_attr_t, stack_size: *mut c_int) -> c_int;
    pub fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: c_int) -> c_int;
    pub fn pthread_attr_getdetachstate(attr: *mut pthread_attr_t, detachstate: *mut c_int)
        -> c_int;
    pub fn pthread_self() -> pthread_t;
    pub fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    pub fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    pub fn pthread_detach(thread: pthread_t) -> c_int;
    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_getrobust(
        attr: *const pthread_mutexattr_t,
        robustness: *mut c_int,
    ) -> c_int;
    pub fn pthread_mutexattr_setrobust(attr: *mut pthread_mutexattr_t, robustness: c_int) -> c_int;
    pub fn pthread_mutex_init(
        mutex: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> c_int;
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_consistent(mutex: *mut pthread_mutex_t) -> c_int;

    pub fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    pub fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    pub fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int;

    pub fn clock_gettime(clock_id: clockid_t, tp: *mut timespec) -> c_int;
    pub fn nanosleep(duration: *const timespec, rem: *mut timespec) -> c_int;
    pub fn sched_yield() -> c_int;
}
