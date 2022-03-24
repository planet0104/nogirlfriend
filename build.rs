fn main() {
    if cfg!(target_os = "windows") {
        winres::WindowsResource::new()
            .set_icon("favicon.ico")
            .set("InternalName", "找不到对象.EXE")
            .compile()
            .unwrap();
    }
}
