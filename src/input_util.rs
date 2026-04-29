pub fn get_input() -> usize {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    return word.trim().parse().ok().unwrap();
}
