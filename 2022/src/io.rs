#[macro_export]
macro_rules! get_input_file {
    ($filename:expr) => {{
        let module = std::module_path!();
        let root = env!("CARGO_MANIFEST_DIR");
        let path = std::path::Path::new(root)
            .join("inputs")
            .join(module)
            .join($filename);
        println!("Using input file {:?}", path);
        std::fs::File::open(path)
    }};
}
