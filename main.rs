#![feature(asm)]
fn main() {
    PrintHello();
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn PrintHello(){
    println!("Hello World!");
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn foo() {
    println!("Not arch");
}
