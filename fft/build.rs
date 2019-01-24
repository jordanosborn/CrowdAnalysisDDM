fn main() {
    cc::Build::new()
        .files(&["lib/src/vidstream.cpp"])
        .cpp(true)
        .shared_flag(true)
        .flag("-L/usr/local/lib -lopencv_core")
        .include("lib/include")
        .include("/usr/local/include/opencv4")
        .compile("libvidstream.so");
}
