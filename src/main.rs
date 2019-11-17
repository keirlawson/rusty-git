use std::str::FromStr;

fn main() {
    let url =
        rustygit::GitUrl::from_str("https://github.com/keirlawson/rusty-git.git").unwrap();
    let repo = rustygit::Repository::clone(url, "/home/keir/Code/testclone");
    repo.unwrap();
    println!("Hello, world!");
}
