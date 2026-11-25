#![cfg(all(target_os = "xbox360", target_env = "picolibc"))]

use core::ffi::c_void;
use core::ptr;

extern "C" fn thread_main(arg: *mut c_void) -> *mut c_void {
    arg
}

pub fn smoke() -> Result<(), &'static str> {
    unsafe {
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
