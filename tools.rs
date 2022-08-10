use std::{fmt::Debug, io::Write, vec};

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

pub fn measuret_closure<T>(f: impl FnOnce() -> T) -> (T, std::time::Duration) {
    let timer = std::time::Instant::now();
    let result = f();
    (result, timer.elapsed())
}
/* `measuret_closure<T>` function: measure the execution time of a closure.
 * usage: `measuret_closure(<closure>);`
 * return value: (T, std::time::Duration)
 * example: `let (_, t) = measuret_closure(|| {let mut x = 0; for i in 0..10000 {x += 1;}});`
*/

pub struct Dumper {
    pub path: &'static str,
    pub file: std::fs::File,
}

impl Dumper {
    pub fn new(path: &'static str) -> Dumper {
        Dumper {
            path,
            file: std::fs::File::options()
                .write(true)
                .create(true)
                .append(true)
                .open(path)
                .unwrap(),
        }
    }
    pub fn dump_line(&mut self, content: &str) {
        self.file.write_fmt(format_args!("{}\n", content)).unwrap();
    }
    pub fn dump_debug_obj<T: Debug>(&mut self, obj: T) {
        self.file.write_fmt(format_args!("{:?}\n", obj)).unwrap();
    }
    pub fn dump_closure_exetime<T>(&mut self, name: &str, f: impl FnOnce() -> T) -> T {
        let (res, exetime) = measuret_closure(f);
        self.dump_line(format!("+ '{}' exec time: {:?}", name, exetime).as_str());
        res
    }
    pub fn dump_closure_result<T: Debug>(&mut self, name: &str, f: impl FnOnce() -> T) -> T {
        let res = f();
        self.dump_line(format!("+ '{}' result: {:?}", name, res).as_str());
        res
    }
    pub fn dump_closure_exetime_result<T: Debug>(
        &mut self,
        name: &str,
        f: impl FnOnce() -> T,
    ) -> T {
        let (res, exetime) = measuret_closure(f);
        self.dump_line(format!("+ '{}' result: {:?}; exetime: {:?}", name, res, exetime).as_str());
        res
    }
}
