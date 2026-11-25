//! Common Picolibc bindings. OS-specific services and types are not included.

pub use crate::picolibc_unistd::{
    STDERR_FILENO,
    STDIN_FILENO,
    STDOUT_FILENO,
};
use crate::prelude::*;

#[cfg(target_os = "xbox360")]
mod xbox360;
#[cfg(target_os = "xbox360")]
pub use self::xbox360::*;

pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = i64;
pub type intptr_t = isize;
pub type uintptr_t = usize;

extern_ty! {
    pub type FILE;
}

pub const EOF: c_int = -1;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_ACCMODE: c_int = O_RDONLY | O_WRONLY | O_RDWR;
pub const O_CREAT: c_int = 0x0040;
pub const O_TRUNC: c_int = 0x0200;
pub const O_APPEND: c_int = 0x0400;
pub const O_EXCL: c_int = 0x0800;
pub const O_SYNC: c_int = 0x2000;
pub const O_NONBLOCK: c_int = 0x4000;
pub const O_NOCTTY: c_int = 0x8000;
pub const O_CLOEXEC: c_int = 0x40000;
pub const O_NOFOLLOW: c_int = 0x100000;
pub const O_DIRECTORY: c_int = 0x200000;
pub const AT_FDCWD: c_int = -2;

extern "C" {
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;

    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(dest: *mut c_void, value: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    pub fn strlen(s: *const c_char) -> size_t;
    pub fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    pub fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    pub fn strerror(errnum: c_int) -> *mut c_char;

    pub fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn freopen(path: *const c_char, mode: *const c_char, stream: *mut FILE) -> *mut FILE;
    pub fn fclose(stream: *mut FILE) -> c_int;
    pub fn fflush(stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn clearerr(stream: *mut FILE);
    pub fn remove(path: *const c_char) -> c_int;
    pub fn rename(old: *const c_char, new: *const c_char) -> c_int;

    pub fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    pub fn openat(dirfd: c_int, path: *const c_char, flags: c_int, ...) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn chdir(path: *const c_char) -> c_int;
    pub fn fchdir(fd: c_int) -> c_int;
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn unlink(path: *const c_char) -> c_int;
    pub fn rmdir(path: *const c_char) -> c_int;
}
