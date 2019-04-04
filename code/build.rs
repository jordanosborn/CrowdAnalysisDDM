use std::process::Command;

#[cfg(unix)]
mod unix {
    pub fn opencv_include() -> &'static str {
        if cfg!(target_os = "linux") {
            "/usr/local/include/opencv4"
        } else {
            "/usr/local/Cellar/opencv/4.0.1/include/opencv4"
        }
    }

    pub fn opencv_link() {
        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        } else {
            println!("cargo:rustc-link-search=native=/usr/local/Cellar/opencv/4.0.1/lib");
        }

        println!("cargo:rustc-link-lib=opencv_core");
        println!("cargo:rustc-link-lib=opencv_features2d");
        //println!("cargo:rustc-link-lib=opencv_xfeatures2d");
        println!("cargo:rustc-link-lib=opencv_highgui");
        //println!("cargo:rustc-link-lib=opencv_img_hash");
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

#[cfg(target_os = "linux")]
fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .flag("-std=c++14")
        .flag("-L/opt/arrayfire/lib")
        .flag("-L/usr/local/lib")
        .flag(&get_opencv_flags())
        .include("./lib/include")
        .include("/usr/local/include")
        .include("/opt/arrayfire/include")
        .include(unix::opencv_include())
        .cpp_link_stdlib("stdc++")
        .cpp_link_stdlib("c++")
        .cpp_set_stdlib("c++")
        .compiler("clang++")
        .compile(output);
    unix::opencv_link();
}

fn get_opencv_flags() -> String {
    let opencv = Command::new("pkg-config")
        .args(&["--cflags", "--libs", "opencv4"])
        .output()
        .expect("failed to execute process");
    unsafe { String::from_utf8_unchecked(opencv.stdout) }
}

#[cfg(target_os = "macos")]
fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .flag(&get_opencv_flags())
        .include("lib/include")
        .include("/usr/local/include")
        .include("/opt/arrayfire/include")
        .include(unix::opencv_include())
        .cpp_link_stdlib("stdc++")
        .cpp_link_stdlib("c++")
        .cpp_set_stdlib("c++")
        .flag("-std=c++14")
        //.flag("-fopenmp")
        .compiler("clang++")
        .compile(output);
    unix::opencv_link();
}

fn main() {
    let src_files = vec!["lib/src/vidstream.cpp"];
    build(src_files, "vidstream");
}

#[allow(dead_code)]
fn get_files(path: &str) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|x| x.ok().map(|x| x.path()))
        .filter(|x| x.extension().map(|e| e == "cpp").unwrap_or(false))
        .collect::<Vec<_>>()
}
