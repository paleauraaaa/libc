//! Xbox 360 interfaces provided by the NXEDK Picolibc port.

use crate::prelude::*;
use crate::{
    mode_t,
    off_t,
    size_t,
    ssize_t,
    FILE,
};

pub type pthread_t = c_int;
pub type pthread_key_t = c_uint;
pub type time_t = i64;
pub type clockid_t = c_ulong;
pub type va_list = *mut c_char;
pub type dev_t = u64;
pub type ino_t = u64;
pub type nlink_t = u64;
pub type uid_t = u32;
pub type gid_t = u32;
pub type blksize_t = i32;
pub type blkcnt_t = i64;
pub type pid_t = c_int;

pub const AT_SYMLINK_NOFOLLOW: c_int = 0x0002;
pub const AT_REMOVEDIR: c_int = 0x0008;
pub const AT_EMPTY_PATH: c_int = 0x0010;

pub const NSIG: c_int = 32;
pub const MINSIGSTKSZ: size_t = 2048;
pub const SIGSTKSZ: size_t = 8192;
pub const SIG2STR_MAX: c_int = 17;

extern_ty! {
    pub type DIR;
}

s! {
    pub struct dirent {
        pub d_ino: ino_t,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    pub struct pthread_attr_t {
        pub __stack_size: size_t,
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

    pub struct pthread_cond_t {
        pub __placeholder: c_int,
    }

    pub struct pthread_condattr_t {
        pub __placeholder: c_int,
    }

    #[derive(Default)]
    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
    }
}

pub const DT_UNKNOWN: c_uchar = 0;
pub const DT_BLK: c_uchar = 1;
pub const DT_CHR: c_uchar = 2;
pub const DT_DIR: c_uchar = 3;
pub const DT_FIFO: c_uchar = 4;
pub const DT_LNK: c_uchar = 5;
pub const DT_REG: c_uchar = 6;
pub const DT_SOCK: c_uchar = 7;

pub const S_IFMT: mode_t = 0o170000;
pub const S_IFDIR: mode_t = 0o040000;
pub const S_IFCHR: mode_t = 0o020000;
pub const S_IFBLK: mode_t = 0o060000;
pub const S_IFREG: mode_t = 0o100000;
pub const S_IFLNK: mode_t = 0o120000;
pub const S_IFSOCK: mode_t = 0o140000;
pub const S_IFIFO: mode_t = 0o010000;
pub const S_ISUID: mode_t = 0o004000;
pub const S_ISGID: mode_t = 0o002000;
pub const S_ISVTX: mode_t = 0o001000;
pub const S_IRWXU: mode_t = 0o000700;
pub const S_IRUSR: mode_t = 0o000400;
pub const S_IWUSR: mode_t = 0o000200;
pub const S_IXUSR: mode_t = 0o000100;
pub const S_IRWXG: mode_t = 0o000070;
pub const S_IRGRP: mode_t = 0o000040;
pub const S_IWGRP: mode_t = 0o000020;
pub const S_IXGRP: mode_t = 0o000010;
pub const S_IRWXO: mode_t = 0o000007;
pub const S_IROTH: mode_t = 0o000004;
pub const S_IWOTH: mode_t = 0o000002;
pub const S_IXOTH: mode_t = 0o000001;

pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;
pub const PTHREAD_STACK_MIN: size_t = 16384;
pub const PTHREAD_SCOPE_PROCESS: c_int = 0;
pub const PTHREAD_SCOPE_SYSTEM: c_int = 1;
pub const PTHREAD_MUTEX_STALLED: c_int = 0;
pub const PTHREAD_MUTEX_ROBUST: c_int = 1;
pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t { __placeholder: 0 };
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
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const EWOULDBLOCK: c_int = EAGAIN;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOTTY: c_int = 25;
pub const ETXTBSY: c_int = 26;
pub const EFBIG: c_int = 27;
pub const ENOSPC: c_int = 28;
pub const ESPIPE: c_int = 29;
pub const EROFS: c_int = 30;
pub const EMLINK: c_int = 31;
pub const EPIPE: c_int = 32;
pub const EDOM: c_int = 33;
pub const ERANGE: c_int = 34;
pub const ENOMSG: c_int = 35;
pub const EIDRM: c_int = 36;
pub const EDEADLK: c_int = 45;
pub const ENOLCK: c_int = 46;
pub const ENOSTR: c_int = 60;
pub const ENODATA: c_int = 61;
pub const ETIME: c_int = 62;
pub const ENOSR: c_int = 63;
pub const ENOLINK: c_int = 67;
pub const EPROTO: c_int = 71;
pub const EMULTIHOP: c_int = 74;
pub const EBADMSG: c_int = 77;
pub const ENOSYS: c_int = 88;
pub const ENOTEMPTY: c_int = 90;
pub const ENAMETOOLONG: c_int = 91;
pub const ELOOP: c_int = 92;
pub const EOPNOTSUPP: c_int = 95;
pub const ECONNRESET: c_int = 104;
pub const ENOBUFS: c_int = 105;
pub const EAFNOSUPPORT: c_int = 106;
pub const EPROTOTYPE: c_int = 107;
pub const ENOTSOCK: c_int = 108;
pub const ENOPROTOOPT: c_int = 109;
pub const ECONNREFUSED: c_int = 111;
pub const EADDRINUSE: c_int = 112;
pub const ECONNABORTED: c_int = 113;
pub const ENETUNREACH: c_int = 114;
pub const ENETDOWN: c_int = 115;
pub const ETIMEDOUT: c_int = 116;
pub const EHOSTUNREACH: c_int = 118;
pub const EINPROGRESS: c_int = 119;
pub const EALREADY: c_int = 120;
pub const EDESTADDRREQ: c_int = 121;
pub const EMSGSIZE: c_int = 122;
pub const EPROTONOSUPPORT: c_int = 123;
pub const EADDRNOTAVAIL: c_int = 125;
pub const ENETRESET: c_int = 126;
pub const EISCONN: c_int = 127;
pub const ENOTCONN: c_int = 128;
pub const EDQUOT: c_int = 132;
pub const ESTALE: c_int = 133;
pub const ENOTSUP: c_int = 134;
pub const EILSEQ: c_int = 138;
pub const EOVERFLOW: c_int = 139;
pub const ECANCELED: c_int = 140;
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
    pub fn fdopendir(fd: c_int) -> *mut DIR;
    pub fn opendir(name: *const c_char) -> *mut DIR;
    pub fn readdir(dir: *mut DIR) -> *mut dirent;
    pub fn dirfd(dir: *mut DIR) -> c_int;
    pub fn closedir(dir: *mut DIR) -> c_int;
    /// The Xbox port ignores POSIX mode bits.
    pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
    /// The Xbox port ignores POSIX mode bits.
    pub fn mkdirat(dirfd: c_int, path: *const c_char, mode: mode_t) -> c_int;
    pub fn chmod(path: *const c_char, mode: mode_t) -> c_int;
    pub fn fchmod(fd: c_int, mode: mode_t) -> c_int;
    pub fn fstat(fd: c_int, buf: *mut stat) -> c_int;
    pub fn fstatat(dirfd: c_int, path: *const c_char, buf: *mut stat, flags: c_int) -> c_int;
    pub fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    /// The current Xbox Picolibc implementation returns `ENOSYS`.
    pub fn lstat(path: *const c_char, buf: *mut stat) -> c_int;
    /// The Xbox port rejects locally open files and does not support `AT_REMOVEDIR`.
    pub fn unlinkat(dirfd: c_int, path: *const c_char, flags: c_int) -> c_int;
    /// The Xbox port rejects renames involving locally open files.
    pub fn renameat(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
    ) -> c_int;
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
    pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stack_size: size_t) -> c_int;
    pub fn pthread_attr_getstacksize(attr: *const pthread_attr_t, stack_size: *mut size_t)
        -> c_int;
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
    /// The Xbox implementation returns `ENOSYS`.
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, kind: c_int) -> c_int;
    pub fn pthread_mutex_init(
        mutex: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> c_int;
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_consistent(mutex: *mut pthread_mutex_t) -> c_int;

    // The integer-returning functions below return ENOSYS.
    pub fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> c_int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_cond_timedwait(
        cond: *mut pthread_cond_t,
        mutex: *mut pthread_mutex_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock_id: clockid_t) -> c_int;
    pub fn pthread_key_create(
        key: *mut pthread_key_t,
        destructor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> c_int;
    pub fn pthread_key_delete(key: pthread_key_t) -> c_int;
    /// Returns null and sets errno to ENOSYS.
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int;

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
