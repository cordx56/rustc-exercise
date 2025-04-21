fn main() {
    let output = std::process::Command::new("rustc")
        .args(["--print=sysroot"])
        .output()
        .unwrap()
        .stdout;
    let stdout = String::from_utf8(output).unwrap();
    println!("cargo::rustc-link-arg=-Wl,-rpath,{}/lib", stdout.trim());
}
