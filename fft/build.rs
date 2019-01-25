#[cfg(target_os = "macos")]
fn build(src_files: Vec<&str>, output: &str) {
    cc::Build::new()
        .files(src_files)
        .cpp(true)
        .static_flag(true)
        .compiler("g++-8")
        .include("lib/include")
        .include("/usr/local/include/opencv4")
        .flag("-L/usr/local/lib -std=c++17 -lopencv_stitching -lopencv_superres -lopencv_videostab -lopencv_aruco -lopencv_bgsegm -lopencv_bioinspired -lopencv_ccalib -lopencv_dnn_objdetect -lopencv_dpm -lopencv_face -lopencv_fuzzy -lopencv_hfs -lopencv_img_hash -lopencv_line_descriptor -lopencv_optflow -lopencv_reg -lopencv_rgbd -lopencv_saliency -lopencv_stereo -lopencv_structured_light -lopencv_phase_unwrapping -lopencv_surface_matching -lopencv_tracking -lopencv_datasets -lopencv_dnn -lopencv_plot -lopencv_xfeatures2d -lopencv_shape -lopencv_video -lopencv_ml -lopencv_ximgproc -lopencv_xobjdetect -lopencv_objdetect -lopencv_calib3d -lopencv_features2d -lopencv_highgui -lopencv_videoio -lopencv_imgcodecs -lopencv_flann -lopencv_xphoto -lopencv_photo -lopencv_imgproc -lopencv_core")
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
    build(src_files, "vidstream");
}
