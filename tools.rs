#[macro_export]
macro_rules! measuret_codeblock {
    ($name:literal, $($code_block:tt)+) => {
        let timer = std::time::Instant::now();
        $(
            $code_block
        )+
        println!("measuret_codeblock('{}'): {:?}", $name, timer.elapsed());
    };
}
/* `measuret_codeblock` macro: measure the execution time of some code.
 * usage: `measuret_codeblock!("tag", <code to be timed>);`
 * example: `measuret_codeblock!("calculate", let mut x = 1; for i in 0..10000 { x += 1; });`
 * output: `measuret_codeblock('calculate'): *`.
*/

pub fn measuret_function<T>(f: impl FnOnce() -> T) -> (T, std::time::Duration) {
    let timer = std::time::Instant::now();
    let result = f();
    (result, timer.elapsed())
}
/* `measuret_function<T>` function: measure the execution time of a closure.
 * usage: `measuret_function(<closure>);`
 * return value: (T, std::time::Duration)
 * example: `let (_, t) = measuret_function(|| {let mut x = 0; for i in 0..10000 {x += 1;}});`
*/
