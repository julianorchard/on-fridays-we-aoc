use md5;

fn main() {
    let input: &str = "bgvyzdsv";
    let search_str: &str = "000000";

    for i in 1.. {
        if md5::convert(&format!("{}{}", input, i)).starts_with(search_str) {
            println!("Found SantaCoin: {}", i.to_string());
            return
        }
    }
}
