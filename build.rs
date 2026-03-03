use pkg_config::probe_library;

fn main() {
    // Link with GMP dynamic library. Requires following in Cargo.toml:
    // [build-dependencies]
    // pkg-config = "0.3"
    probe_library("gmp").unwrap();
}
