
pub fn f_c(i :f64) {
    let f_to_k = (((i - 32.0) * 5.0) / 9.0) + 273.15;

    println!("{f_to_k:.2}°K");
}