use git_url_parse::*;

#[test]
fn generic_git() {
    let test_url = "ssh://git@host.tld:9999/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info = parsed.provider_info::<GenericProvider>().unwrap();
    let expected = GenericProvider::default();
    assert_eq!(provider_info, expected)
    //let provider = parsed
}
