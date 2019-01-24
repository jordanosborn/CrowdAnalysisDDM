fn main() {
    cc::Build::new()
        .file("lib/src/vidstream.cpp")
        .cpp(true)
        .flag("-lopencv_core -lopencv_highgui -lopencv_imgproc")
        .include("lib/include")
        .compile("libvidstream");
}