fn main() {
    println!("cargo:rerun-if-changed=api/lib.js");
    println!("cargo:rerun-if-changed=api/lib.d.ts");
    println!("cargo:rerun-if-changed=api/lib.d.ts.map");
    println!("cargo:rerun-if-changed=loader.js");
}