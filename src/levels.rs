use std::fs;

pub fn load_level(level: u8) -> String {
    let level_code = format!("{:02}", level);
    let level_path = format!("./resources/levels/level_{}", level_code);
    let level_source = fs::read_to_string(&level_path)
        .expect(&format!("expected level loaded: {}", &level_path)[..]);

    String::from(level_source)
}