use git_url_parse::*;

#[test]
fn http_generic_git() {
    let test_url = "https://github.com/tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info = parsed.provider_info::<GenericProvider>().unwrap();
    let expected = GenericProvider {
        host: "github.com".to_string(),
        user: "tjtelan".to_string(),
        repo: "git-url-parse-rs.git".to_string(),
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_generic_git() {
    let test_url = "git@github.com:tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info = parsed.provider_info::<GenericProvider>().unwrap();
    let expected = GenericProvider {
        host: "github.com".to_string(),
        user: "tjtelan".to_string(),
        repo: "git-url-parse-rs.git".to_string(),
    };
    assert_eq!(provider_info, expected)
}
