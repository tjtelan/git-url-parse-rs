use git_url_parse::{GitUrl, GitUrlParseError};

fn main() -> Result<(), GitUrlParseError> {
    env_logger::init();

    let test_vec = vec![
        "https://github.com/tjtelan/orbitalci.git",
        "git@github.com:tjtelan/orbitalci.git",
        "https://token:x-oauth-basic@host.xz/path/to/repo.git/",
        "https://x-token-auth:token@host.xz/path/to/repo.git/",
        "git+ssh://git@some-host.com/and-the-path/name",
        "git://some-host.com/and-the-path/name",
        "host.tld:user/project-name.git",
        "file:///path/to/repo.git/",
        "~/path/to/repo.git/",
        "./path/to/repo.git/",
        "./path/to/repo.git",
        "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName",
        "https://CompanyName@dev.azure.com/CompanyName/ProjectName/_git/RepoName",
    ];

    for test_url in test_vec {
        println!("Original:         {}", test_url);
        println!(
            "Parsed + Trimmed: {}\n",
            GitUrl::parse(test_url)?.trim_auth()
        );
    }
    Ok(())
}
