fn main() {
    cc::Build::new()
        .file("lib/src/vidstream.cpp")
        .cpp(true)
        .shared_flag(true)
        // .flag("")
        .include("lib/include")
        .compile("libvidstream.so");
}