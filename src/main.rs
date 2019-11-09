fn main() {
    let url = rustygit::GitUrl::new(String::from("https://github.com/keirlawson/rusty-git.git"));
    let repo = rustygit::Repository::clone(url, "/home/keir/Code/testclone");
    println!("Hello, world!");
}
