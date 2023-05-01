fn main() {
    // Tell Cargo that if the client.js file changes, to rerun this build script.
    // Needed to ensure that the client.js file is up to date when the example is run.
    println!("cargo:rerun-if-changed=src/client.js");
}