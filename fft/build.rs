#[cfg(unix)]
mod unix {
    pub fn opencv_include() -> &'static str {
        if cfg!(linux) {
            "/usr/local/include/opencv4"
        } else {
            "/usr/local/Cellar/opencv/4.0.1/include/opencv4"
        }
    }

    pub fn opencv_link() {
        if cfg!(linux) {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        } else {
            println!("cargo:rustc-link-search=native=/usr/local/Cellar/opencv/4.0.1/lib");
        }

        println!("cargo:rustc-link-lib=opencv_core");
        println!("cargo:rustc-link-lib=opencv_features2d");
        println!("cargo:rustc-link-lib=opencv_xfeatures2d");
        println!("cargo:rustc-link-lib=opencv_highgui");
        println!("cargo:rustc-link-lib=opencv_img_hash");
        println!("cargo:rustc-link-lib=opencv_imgcodecs");
        println!("cargo:rustc-link-lib=opencv_imgproc");
        println!("cargo:rustc-link-lib=opencv_objdetect");
        // println!("cargo:rustc-link-lib=opencv_text");
        println!("cargo:rustc-link-lib=opencv_videoio");
        println!("cargo:rustc-link-lib=opencv_video");
        if cfg!(feature = "cuda") {
            println!("cargo:rustc-link-lib=opencv_cudaobjdetect");
        }
    }
}

#[cfg(target_family = "unix")]
fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .shared_flag(true)
        .compiler("g++-8")
        .include("lib/include")
        .include("/usr/local/include")
        .include("/opt/arrayfire/include")
        .include(unix::opencv_include())
        .cpp_link_stdlib("stdc++")
        .flag("-L/usr/local/lib -L/opt/arrayfire/lib -L/opt/arrayfire/lib64 --std=c++17 -lopencv_core -lopencv_highgui -fopenmp -march=native")
        .compile(output);
    unix::opencv_link();
}

fn main() {
    let src_files = vec!["lib/src/vidstream.cpp"];
    build(src_files, "vidstream");
}
