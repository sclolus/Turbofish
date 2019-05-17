use crate::syscall::{_user_exit, _user_fork};

/// Just a stupid kernel funnction which get the stack value
#[allow(dead_code)]
pub fn get_stack() {
    for _i in 0..1000000 {
        eprintln!("get stack: {:#X?}", unsafe { _get_stack() });
    }
    loop {}
}

extern "C" {
    fn _get_stack() -> u32;
}

/// stupid kernel space process zero
#[allow(dead_code)]
pub fn process_zero() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process A {}", i);
        }
    }
}

/// stupid kernel space process a
#[allow(dead_code)]
pub fn process_a() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process A {}", i);
        }
    }
}

/// stupid kernel space process b
#[allow(dead_code)]
pub fn process_b() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process B {}", i);
        }
    }
}

/// stupid kernel space process diying in pain
#[allow(dead_code)]
pub fn diyng_process() {
    unsafe {
        for i in 0..10 {
            user_eprintln!("process diying slowly {}", i);
        }
        _user_exit(0);
    }
}

/// stupid kernel space process doing a fork
#[allow(dead_code)]
pub fn fork_process() {
    unsafe {
        user_eprintln!("i am a the fork process");

        let fork_res = _user_fork();
        if fork_res == 0 {
            for i in 0..1000000 {
                user_eprintln!("i am a gentle child {}", i);
                asm!("hlt"::::"volatile");
            }
        } else {
            for i in 0..1000000 {
                user_eprintln!("i am a proud father of child with pid({}) {}", fork_res, i);
                asm!("hlt"::::"volatile");
            }
        }
        _user_exit(0);
    }
}

/// stupid kernel space process doing a fork
#[allow(dead_code)]
pub fn fork_test_different_stack() {
    let mut array: [u8; 128] = [42; 128];
    unsafe {
        user_eprintln!("i am a the fork process");

        let fork_res = _user_fork();
        if fork_res == 0 {
            user_eprintln!("i am a gentle child");
            user_eprintln!("i am a gentle child {:?}", &array[..]);
            array = [21; 128];
            asm!("hlt"::::"volatile");
            user_eprintln!("i am a gentle child {:?}", &array[..]);
        } else {
            user_eprintln!("i am a proud father of child with pid({})", fork_res);
            user_eprintln!("in the father {:?}", &array[..]);
            asm!("hlt"::::"volatile");
            array = [84; 128];
            user_eprintln!("in the father {:?}", &array[..]);
        }
        loop {}
    }
}

#[allow(dead_code)]
#[allow(unconditional_recursion)]
pub fn fork_bomb() {
    unsafe {
        user_eprintln!("i am a the fork process");

        let fork_res = _user_fork();
        if fork_res == 0 {
            user_eprintln!("i am a gentle child ");
            fork_bomb()
        } else {
            user_eprintln!("i am a proud father of child with pid({})", fork_res);
            fork_bomb()
        }
    }
}
