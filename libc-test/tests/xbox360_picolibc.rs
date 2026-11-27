#![cfg(all(target_os = "xbox360", target_env = "picolibc"))]

use core::ffi::c_void;
use core::ptr;

extern "C" fn thread_main(arg: *mut c_void) -> *mut c_void {
    arg
}

pub fn smoke() -> Result<(), &'static str> {
    unsafe {
        let mut buf = [0u8; 32];
        let format = b"%d:%08x\0";
        if libc::snprintf(
            buf.as_mut_ptr().cast(),
            buf.len(),
            format.as_ptr().cast(),
            -42 as libc::c_int,
            0x1234 as libc::c_uint,
        ) != 12
            || &buf[..13] != b"-42:00001234\0"
        {
            return Err("snprintf");
        }
        let mut decimal: libc::c_int = 0;
        let mut hex: libc::c_uint = 0;
        if libc::sscanf(
            buf.as_ptr().cast(),
            b"%d:%x\0".as_ptr().cast(),
            &mut decimal,
            &mut hex,
        ) != 2
            || decimal != -42
            || hex != 0x1234
        {
            return Err("sscanf");
        }
        if libc::snprintf(
            buf.as_mut_ptr().cast(),
            5,
            format.as_ptr().cast(),
            -42 as libc::c_int,
            0x1234 as libc::c_uint,
        ) != 12
            || &buf[..5] != b"-42:\0"
        {
            return Err("snprintf truncation");
        }
        let mut allocated: *mut libc::c_char = ptr::null_mut();
        let result = libc::asprintf(&mut allocated, b"%d\0".as_ptr().cast(), 73 as libc::c_int);
        if allocated.is_null() {
            return Err("asprintf allocation");
        }
        let allocated_ok = result == 2 && libc::strcmp(allocated, b"73\0".as_ptr().cast()) == 0;
        libc::free(allocated.cast());
        if !allocated_ok {
            return Err("asprintf");
        }

        let mut metadata = core::mem::MaybeUninit::<libc::stat>::uninit();
        if libc::fstat(1, metadata.as_mut_ptr()) != 0
            || metadata.assume_init_ref().st_mode & libc::S_IFMT != libc::S_IFCHR
        {
            return Err("fstat stdout");
        }
        let game_root = b"game:\\\0";
        if libc::stat(game_root.as_ptr().cast(), metadata.as_mut_ptr()) != 0
            || metadata.assume_init_ref().st_mode & libc::S_IFMT != libc::S_IFDIR
        {
            return Err("stat game directory");
        }
        let fd = libc::open(
            game_root.as_ptr().cast(),
            libc::O_RDONLY | libc::O_DIRECTORY,
        );
        if fd < 0 {
            return Err("open game directory");
        }
        let mut fd_metadata = core::mem::MaybeUninit::<libc::stat>::uninit();
        let fd_ok = libc::fstat(fd, fd_metadata.as_mut_ptr()) == 0
            && fd_metadata.assume_init_ref().st_mode & libc::S_IFMT == libc::S_IFDIR
            && fd_metadata.assume_init_ref().st_dev == metadata.assume_init_ref().st_dev
            && fd_metadata.assume_init_ref().st_ino == metadata.assume_init_ref().st_ino;
        if libc::close(fd) != 0 || !fd_ok {
            return Err("fstat game directory");
        }
        if libc::fstatat(-1, game_root.as_ptr().cast(), metadata.as_mut_ptr(), 0) != 0
            || metadata.assume_init_ref().st_mode & libc::S_IFMT != libc::S_IFDIR
            || libc::fstatat(
                1,
                b"\0".as_ptr().cast(),
                metadata.as_mut_ptr(),
                libc::AT_EMPTY_PATH,
            ) != 0
            || metadata.assume_init_ref().st_mode & libc::S_IFMT != libc::S_IFCHR
            || libc::fstatat(
                -1,
                game_root.as_ptr().cast(),
                metadata.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            ) != -1
        {
            return Err("fstatat");
        }
        if libc::lstat(game_root.as_ptr().cast(), metadata.as_mut_ptr()) != -1 {
            return Err("lstat unsupported");
        }

        let mut contents = [0u8; 32];
        let stream = libc::fmemopen(
            contents.as_mut_ptr().cast(),
            contents.len(),
            b"w+\0".as_ptr().cast(),
        );
        if stream.is_null() {
            return Err("fmemopen");
        }
        let mut pos: libc::fpos_t = 0;
        let mut read_buf = [0u8; 8];
        let stream_ok = libc::fprintf(stream, b"%d\0".as_ptr().cast(), 73 as libc::c_int) == 2
            && libc::fflush(stream) == 0
            && libc::fseek(stream, 0, libc::SEEK_SET) == 0
            && libc::fgetpos(stream, &mut pos) == 0
            && libc::fgetc(stream) == b'7' as libc::c_int
            && libc::fsetpos(stream, &pos) == 0
            && !libc::fgets(
                read_buf.as_mut_ptr().cast(),
                read_buf.len() as libc::c_int,
                stream,
            )
            .is_null()
            && &read_buf[..3] == b"73\0";
        if libc::fclose(stream) != 0 || !stream_ok {
            return Err("memory stream");
        }

        let mut attr = libc::pthread_attr_t {
            __stack_size: 0,
            __detachstate: 0,
        };
        if libc::pthread_attr_init(&mut attr) != 0 {
            return Err("pthread_attr_init");
        }
        let mut stack_size = 0;
        if libc::pthread_attr_setstacksize(&mut attr, 65536) != 0
            || libc::pthread_attr_getstacksize(&mut attr, &mut stack_size) != 0
            || stack_size != 65536
            || libc::pthread_attr_destroy(&mut attr) != 0
        {
            return Err("pthread attributes");
        }

        let mut mutex_attr = libc::pthread_mutexattr_t { __robustness: 0 };
        if libc::pthread_mutexattr_init(&mut mutex_attr) != 0 {
            return Err("pthread_mutexattr_init");
        }
        let mut robustness = -1;
        if libc::pthread_mutexattr_setrobust(&mut mutex_attr, libc::PTHREAD_MUTEX_ROBUST) != 0
            || libc::pthread_mutexattr_getrobust(&mutex_attr, &mut robustness) != 0
            || robustness != libc::PTHREAD_MUTEX_ROBUST
            || libc::pthread_mutexattr_destroy(&mut mutex_attr) != 0
        {
            return Err("mutex attributes");
        }

        let mut mutex = libc::PTHREAD_MUTEX_INITIALIZER;
        if libc::pthread_mutex_lock(&mut mutex) != 0
            || libc::pthread_mutex_trylock(&mut mutex) != libc::EBUSY
            || libc::pthread_mutex_unlock(&mut mutex) != 0
            || libc::pthread_mutex_destroy(&mut mutex) != 0
        {
            return Err("mutex");
        }

        let mut thread = 0;
        let arg = &mut thread as *mut libc::pthread_t as *mut c_void;
        if libc::pthread_create(&mut thread, ptr::null(), thread_main, arg) != 0 {
            return Err("pthread_create");
        }
        let mut result = ptr::null_mut();
        if libc::pthread_join(thread, &mut result) != 0 || result != arg {
            return Err("pthread_join");
        }

        let address = libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        if address == libc::MAP_FAILED {
            return Err("mmap");
        }
        address.cast::<u8>().write(0x5a);
        if address.cast::<u8>().read() != 0x5a
            || libc::mprotect(address, 4096, libc::PROT_READ) != 0
            || libc::munmap(address, 4096) != 0
        {
            return Err("mmap access or release");
        }
        if libc::mmap(
            ptr::null_mut(),
            0,
            libc::PROT_READ,
            libc::MAP_PRIVATE,
            -1,
            0,
        ) != libc::MAP_FAILED
        {
            return Err("invalid mmap");
        }

        let mut now = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now) != 0
            || now.tv_sec < 0
            || !(0..1_000_000_000).contains(&now.tv_nsec)
            || libc::sched_yield() != 0
        {
            return Err("clock or scheduler");
        }
        let zero = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if libc::nanosleep(&zero, ptr::null_mut()) != 0 {
            return Err("nanosleep");
        }
    }
    Ok(())
}

#[test]
fn xbox360_picolibc_smoke() {
    smoke().unwrap();
}

pub fn unlink_smoke() -> Result<(), &'static str> {
    unsafe {
        let path = b"game:\\libc-unlink-smoke.bin\0";
        let path_ptr = path.as_ptr().cast();
        let fd = libc::open(
            path_ptr,
            libc::O_CREAT | libc::O_EXCL | libc::O_RDWR,
            0 as libc::mode_t,
        );
        if fd < 0 {
            return Err("create unlinkat test file");
        }
        if libc::close(fd) != 0 {
            return Err("close unlinkat test file");
        }
        if libc::unlinkat(-1, path_ptr, 0) != 0 {
            return Err("unlinkat file");
        }
        let mut metadata = core::mem::MaybeUninit::<libc::stat>::uninit();
        if libc::stat(path_ptr, metadata.as_mut_ptr()) != -1 {
            return Err("unlinkat file still exists");
        }
        let fd = libc::open(
            path_ptr,
            libc::O_CREAT | libc::O_EXCL | libc::O_RDWR,
            0 as libc::mode_t,
        );
        if fd < 0 || libc::close(fd) != 0 {
            return Err("create unlink test file");
        }
        if libc::unlink(path_ptr) != 0 || libc::stat(path_ptr, metadata.as_mut_ptr()) != -1 {
            return Err("unlink file");
        }
        if libc::unlinkat(-1, b"game:\\\0".as_ptr().cast(), libc::AT_REMOVEDIR) != -1 {
            return Err("unlinkat directory deletion");
        }
    }
    Ok(())
}

#[test]
fn xbox360_picolibc_unlink() {
    unlink_smoke().unwrap();
}
