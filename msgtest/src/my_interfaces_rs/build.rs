fn main() {
    println!("cargo:rustc-link-lib=my_interfaces__rosidl_generator_c");
    println!("cargo:rustc-link-lib=my_interfaces__rosidl_typesupport_c");

    if let Some(e) = std::env::var_os("AMENT_PREFIX_PATH") {
        let env = e.to_str().unwrap();
        for path in env.split(':') {
            println!("cargo:rustc-link-search={path}/lib");
        }
    }
}
