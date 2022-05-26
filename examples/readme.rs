use git_url_parse::GitUrl;

fn main() {
    println!(
        "SSH: {:#?}",
        GitUrl::parse("git@github.com:tjtelan/git-url-parse-rs.git")
    );
    println!(
        "HTTPS: {:#?}",
        GitUrl::parse("https://github.com/tjtelan/git-url-parse-rs")
    );
}
