#[cfg(target_os = "macos")]
fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .shared_flag(true)
        .flag("-L/usr/local/lib -lopencv_core -std=c++17")
        .flag("-L/usr/local/Cellar/opencv/3.4.3_1/lib")
        .include("/usr/local/Cellar/opencv/3.4.3_1/include")
        .include("lib/include")
        .compile(output);
}

#[cfg(target_os = "linux")]
fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .shared_flag(true)
        .flag("-L/usr/local/lib -lopencv_core")
        .include("lib/include")
        .include("/usr/local/include/opencv4")
        .compile(output);
}

fn main() {
    let src_files = vec!["lib/src/vidstream.cpp"];
    build(src_files, "libvidstream.so");
}
