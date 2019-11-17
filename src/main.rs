fn main() {
    let url = rustygit::GitUrl::new(String::from("https://github.com/keirlawson/rusty-git.git")).unwrap();
    let repo = rustygit::Repository::clone(url, "/home/keir/Code/testclone");
    repo.unwrap();
    println!("Hello, world!");
}
