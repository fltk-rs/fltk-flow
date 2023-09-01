use fltk_build::fltk_out_dir;

fn main() {
    let fltk_out_dir = fltk_out_dir().unwrap();

    cc::Build::new()
        .file("src/cfl_flow.cpp")
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-w")
        .include(&fltk_out_dir.join("include"))
        .compile("fltk_flow");
}
