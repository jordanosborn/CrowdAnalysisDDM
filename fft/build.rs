#[cfg(target_os = "macos")]
fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .static_flag(true)
        .compiler("g++-8")
        .include("lib/include")
        .include("/usr/local/include")
        .include("/opt/arrayfire/include")
        .include("/usr/local/include/opencv4")
        .cpp_link_stdlib("stdc++")
        .flag("-L/usr/local/lib -L/opt/arrayfire/lib -std=c++17 -lopencv_core")
        .compile(output);
}

#[cfg(target_os = "linux")]
fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .shared_flag(true)
        .flag("-L/usr/local/lib -L/opt/arrayfire/lib64 -lopencv_core")
        .cpp_link_stdlib("stdc++")
        .include("lib/include")
        .include("/opt/arrayfire/include")
        .include("/usr/local/include/opencv4")
        .compile(output);
}

fn main() {
    let src_files = vec!["lib/src/vidstream.cpp"];
    build(src_files, "vidstream");
}
