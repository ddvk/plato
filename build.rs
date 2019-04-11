use std::env;
fn main() {
    let target = env::var("TARGET").expect("not set");

    if target.contains("arm") {
        println!("cargo:rustc-flags=-L libs -l jpeg -l png16 -l openjp2 -l jbig2dec -l bz2 -l z -l m");
        //      println!("cargo:rustc-flags=-L libs -l mupdfthird -l bz2");
    }
    else {
        println!("cargo:rustc-flags= -l jpeg -l png16 -l openjp2 -l jbig2dec -l bz2 -l z -l m");
    }
    println!("cargo:rustc-env=PKG_CONFIG_ALLOW_CROSS=1");
}
