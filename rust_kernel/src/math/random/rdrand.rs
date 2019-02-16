/// Ivybridge+ RDRAND feature.
/// large unsafe autocast with T, BE CAREFULL. Don't do nasty things with float types !
/// rdrand set the carry flag to 1 if the random is well done, else loop while it works
pub fn rdrand<T>() -> T {
    let result: T;

    unsafe {
        asm!("
            1:
            rdrand %eax
            jnc 1b" : "={eax}"(result) :::);
    }
    result
}
