fn main() {
    cc::Build::new()
        .file("lib/src/vidstream.cpp")
        .cpp(true)
        .shared_flag(true)
        .flag("-I/usr/local/include/opencv4/ -lopencv_core")
        .include("lib/include")
        .compile("libvidstream.so");
}