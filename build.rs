extern crate cmake;
use cmake::Config;
/// This macro helps with debugging your buildfiles.
/// Especially handy if you try to find the build location of a third party cmake library.
/// Source: https://github.com/rust-lang/cargo/issues/985
/// 
/// Usage: `p!("{x}");
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}


fn main()
{
    let dst = Config::new("libfoo").build();       
    let d = dst.display();
    p!("{d}");
    
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");    
}
