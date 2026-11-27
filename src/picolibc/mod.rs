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
pub type mode_t = u32;
pub type fpos_t = off_t;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type sig_atomic_t = c_int;
pub type sighandler_t = size_t;
pub type sigset_t = c_ulong;

extern_ty! {
    pub type FILE;
}

pub const EOF: c_int = -1;
pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;
pub const SIG_DFL: sighandler_t = 0;
pub const SIG_IGN: sighandler_t = 1;
pub const SIG_ERR: sighandler_t = !0;
pub const SIGINT: c_int = 2;
pub const SIGILL: c_int = 4;
pub const SIGABRT: c_int = 6;
pub const SIGFPE: c_int = 8;
pub const SIGSEGV: c_int = 11;
pub const SIGPIPE: c_int = 13;
pub const SIGTERM: c_int = 15;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const _IOFBF: c_int = 0;
pub const _IOLBF: c_int = 1;
pub const _IONBF: c_int = 2;

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
    pub static stdin: *mut FILE;
    pub static stdout: *mut FILE;
    pub static stderr: *mut FILE;

    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;
    pub fn signal(sig: c_int, handler: sighandler_t) -> sighandler_t;
    pub fn raise(sig: c_int) -> c_int;
    pub fn sigaddset(set: *mut sigset_t, sig: c_int) -> c_int;
    pub fn sigemptyset(set: *mut sigset_t) -> c_int;

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
    pub fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    pub fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE;
    pub fn open_memstream(buf: *mut *mut c_char, size: *mut size_t) -> *mut FILE;
    pub fn fclose(stream: *mut FILE) -> c_int;
    pub fn fflush(stream: *mut FILE) -> c_int;
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn snprintf(buf: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    pub fn asprintf(buf: *mut *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn dprintf(fd: c_int, format: *const c_char, ...) -> c_int;
    pub fn scanf(format: *const c_char, ...) -> c_int;
    pub fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn sscanf(buf: *const c_char, format: *const c_char, ...) -> c_int;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn putc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn getc(stream: *mut FILE) -> c_int;
    pub fn getchar() -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn getc_unlocked(stream: *mut FILE) -> c_int;
    pub fn getchar_unlocked() -> c_int;
    pub fn putc_unlocked(c: c_int, stream: *mut FILE) -> c_int;
    pub fn putchar_unlocked(c: c_int) -> c_int;
    pub fn flockfile(stream: *mut FILE);
    pub fn ftrylockfile(stream: *mut FILE) -> c_int;
    pub fn funlockfile(stream: *mut FILE);
    pub fn getline(line: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    pub fn getdelim(
        line: *mut *mut c_char,
        n: *mut size_t,
        delim: c_int,
        stream: *mut FILE,
    ) -> ssize_t;
    pub fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fgetpos(stream: *mut FILE, pos: *mut fpos_t) -> c_int;
    pub fn fsetpos(stream: *mut FILE, pos: *const fpos_t) -> c_int;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn fseeko(stream: *mut FILE, offset: off_t, whence: c_int) -> c_int;
    pub fn ftello(stream: *mut FILE) -> off_t;
    pub fn rewind(stream: *mut FILE);
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn setvbuf(stream: *mut FILE, buf: *mut c_char, mode: c_int, size: size_t) -> c_int;
    pub fn fileno(stream: *mut FILE) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn clearerr(stream: *mut FILE);
    pub fn perror(s: *const c_char);
    pub fn remove(path: *const c_char) -> c_int;
    pub fn rename(old: *const c_char, new: *const c_char) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(s: *mut c_char) -> *mut c_char;
    pub fn popen(command: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn pclose(stream: *mut FILE) -> c_int;

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
