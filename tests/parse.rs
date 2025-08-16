use git_url_parse::*;
#[test]
fn ssh_user_ports() {
    let test_url = "ssh://git@host.tld:9999/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Ssh))
        .host(Some(String::from("host.tld")))
        .user(Some(String::from("git")))
        .port(Some(9999))
        .path(String::from("user/project-name.git"))
        .print_scheme(true)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn ssh_no_scheme_no_user() {
    let test_url = "host.tld:user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Ssh))
        .host(Some(String::from("host.tld")))
        .path(String::from("user/project-name.git"))
        .print_scheme(false)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

// Specific service support
#[test]
fn https_user_bitbucket() {
    let test_url = "https://user@bitbucket.org/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Https))
        .host(Some(String::from("bitbucket.org")))
        .user(Some(String::from("user")))
        .path(String::from("/user/repo.git"))
        .print_scheme(true)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn ssh_user_bitbucket() {
    let test_url = "git@bitbucket.org:user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .host(Some(String::from("bitbucket.org")))
        .scheme(Some(Scheme::Ssh))
        .user(Some(String::from("git")))
        .path(String::from("user/repo.git"))
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn https_user_auth_bitbucket() {
    let test_url = "https://x-token-auth:token@bitbucket.org/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Https))
        .host(Some("bitbucket.org".to_string()))
        .user(String::from("x-token-auth"))
        .token(String::from("token"))
        .path(String::from("/owner/name.git"))
        .print_scheme(true)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn https_user_github() {
    let test_url = "https://user@github.com/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Https))
        .user(Some(String::from("user")))
        .host(Some(String::from("github.com")))
        .path(String::from("/user/repo.git"))
        .print_scheme(true)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn ssh_user_github() {
    let test_url = "git@github.com:user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Ssh))
        .user(Some(String::from("git")))
        .host(Some(String::from("github.com")))
        .path(String::from("user/repo.git"))
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn https_user_auth_github() {
    let test_url = "https://token:x-oauth-basic@github.com/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Https))
        .user(Some(String::from("token")))
        .token(Some(String::from("x-oauth-basic")))
        .host(Some(String::from("github.com")))
        .path(String::from("/owner/name.git"))
        .print_scheme(true)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn ssh_user_azure_devops() {
    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Ssh))
        .user(Some(String::from("git")))
        .host(Some(String::from("ssh.dev.azure.com")))
        .path(String::from("v3/CompanyName/ProjectName/RepoName"))
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn https_user_azure_devops() {
    let test_url = "https://organization@dev.azure.com/organization/project/_git/repo";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Https))
        .user(Some(String::from("organization")))
        .host(Some(String::from("dev.azure.com")))
        .path(String::from("/organization/project/_git/repo"))
        .print_scheme(true)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn ftp_user() {
    let test_url = "ftp://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Ftp))
        .user(Some(String::from("git")))
        .host(Some(String::from("host.tld")))
        .path(String::from("/user/project-name.git"))
        .print_scheme(true)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn ftps_user() {
    let test_url = "ftps://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Ftps))
        .user(Some(String::from("git")))
        .host(Some(String::from("host.tld")))
        .path(String::from("/user/project-name.git"))
        .print_scheme(true)
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn relative_unix_path() {
    let test_url = "../project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::File))
        .path(String::from("../project-name.git"))
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn absolute_unix_path() {
    let test_url = "/path/to/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::File))
        .path(String::from("/path/to/project-name.git"))
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

// Issue #6 - Relative Windows paths will parse into Unix paths
#[test]
fn relative_windows_path() {
    let test_url = "..\\project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::File))
        .path(String::from("../project-name.git"))
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

// Can I use `typed-path` to deal with this?
// Issue #7 - Absolute Windows paths will not parse at all
#[should_panic(expected = "URL parse failed: UnexpectedFormat")]
#[test]
fn absolute_windows_path() {
    let test_url = "c:\\project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::File))
        .path(String::from("c:\\project-name.git"))
        .build()
        .unwrap();

    assert_eq!(parsed, expected);
}

// Move test
//#[test]
//fn ssh_user_path_not_acctname_reponame_format() {
//    let test_url = "git@test.com:repo";
//    let e = GitUrl::parse(test_url);
//
//    assert!(e.is_err());
//    assert_eq!(
//        format!("{}", e.err().unwrap()),
//        "Git Url not in expected format"
//    );
//}

// Move test
//#[test]
//fn ssh_without_organization() {
//    let test_url = "ssh://f589726c3611:29418/repo";
//    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
//    let expected = GitUrl {
//        host: Some("f589726c3611".to_string()),
//        //name: "repo".to_string(),
//        //owner: Some("repo".to_string()),
//        //organization: None,
//        //fullname: "repo/repo".to_string(),
//        scheme: Some(Scheme::Ssh),
//        user: None,
//        token: None,
//        port: Some(29418),
//        path: "repo".to_string(),
//        //git_suffix: false,
//        //scheme_prefix: true,
//        print_scheme: true,
//    };
//
//    assert_eq!(parsed, expected);
//}

//#[test]
//fn empty_path() {
//    assert_eq!(
//        GitUrlParseError::EmptyPath,
//        GitUrl::parse("file://").unwrap_err()
//    )
//}

#[test]
fn bad_port_number() {
    let test_url = "https://github.com:crypto-browserify/browserify-rsa.git";
    let e = GitUrl::parse(test_url);

    assert!(e.is_err());
    assert_eq!(
        format!("{}", e.err().unwrap()),
        "Error from Url crate: invalid port number"
    );
}

// This test might not have a use anymore if we're not expanding "git:" -> "git://"
#[test]
fn git() {
    let test_url = "git://github.com/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrlBuilder::default()
        .scheme(Some(Scheme::Git))
        .host(Some(String::from("github.com")))
        .path(String::from("/owner/name.git"))
        .print_scheme(true)
        .build()
        .unwrap();
    assert_eq!(parsed, expected);
}
