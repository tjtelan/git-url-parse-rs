use git_url_parse::*;
#[test]
fn ssh_user_ports() {
    let test_url = "ssh://git@host.tld:9999/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(
        parsed.url(),
        "ssh://git@host.tld:9999/user/project-name.git"
    );
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), Some("9999"));
    assert_eq!(parsed.path(), Some("user/project-name.git"));
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ssh_no_scheme_no_user() {
    let test_url = "host.tld:user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "host.tld:user/project-name.git");
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("user/project-name.git"));
    assert_eq!(parsed.print_scheme(), false);
}

// Specific service support
#[test]
fn https_user_bitbucket() {
    let test_url = "https://user@bitbucket.org/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "https://user@bitbucket.org/user/repo.git");
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("user"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("bitbucket.org"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/user/repo.git"));
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ssh_user_bitbucket() {
    let test_url = "git@bitbucket.org:user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "git@bitbucket.org:user/repo.git");
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("bitbucket.org"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("user/repo.git"));
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn https_user_auth_bitbucket() {
    let test_url = "https://x-token-auth:token@bitbucket.org/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(
        parsed.url(),
        "https://x-token-auth:token@bitbucket.org/owner/name.git"
    );
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("x-token-auth"));
    assert_eq!(parsed.token(), Some("token"));
    assert_eq!(parsed.host(), Some("bitbucket.org"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/owner/name.git"));
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn https_user_github() {
    let test_url = "https://user@github.com/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "https://user@github.com/user/repo.git");
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("user"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("github.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/user/repo.git"));
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ssh_user_github() {
    let test_url = "git@github.com:user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "git@github.com:user/repo.git");
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("github.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("user/repo.git"));
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn https_user_auth_github() {
    let test_url = "https://token:x-oauth-basic@github.com/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(
        parsed.url(),
        "https://token:x-oauth-basic@github.com/owner/name.git"
    );
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("token"));
    assert_eq!(parsed.token(), Some("x-oauth-basic"));
    assert_eq!(parsed.host(), Some("github.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/owner/name.git"));
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ssh_user_azure_devops() {
    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(
        parsed.url(),
        "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName"
    );
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("ssh.dev.azure.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("v3/CompanyName/ProjectName/RepoName"));
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn https_user_azure_devops() {
    let test_url = "https://organization@dev.azure.com/organization/project/_git/repo";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(
        parsed.url(),
        "https://organization@dev.azure.com/organization/project/_git/repo"
    );
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("organization"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("dev.azure.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/organization/project/_git/repo"));
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ftp_user() {
    let test_url = "ftp://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "ftp://git@host.tld/user/project-name.git");
    assert_eq!(parsed.scheme(), Some("ftp"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/user/project-name.git"));
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ftps_user() {
    let test_url = "ftps://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "ftps://git@host.tld/user/project-name.git");
    assert_eq!(parsed.scheme(), Some("ftps"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/user/project-name.git"));
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn relative_unix_path() {
    let test_url = "../project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "../project-name.git");
    assert_eq!(parsed.scheme(), Some("file"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), None);
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("../project-name.git"));
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn absolute_unix_path() {
    let test_url = "/path/to/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "/path/to/project-name.git");
    assert_eq!(parsed.scheme(), Some("file"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), None);
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/path/to/project-name.git"));
    assert_eq!(parsed.print_scheme(), false);
}

// Issue #6 - Relative Windows paths will parse into Unix paths
#[test]
fn relative_windows_path() {
    let test_url = r"..\project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), r"..\project-name.git");
    assert_eq!(parsed.scheme(), Some("file"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), None);
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("..\\project-name.git"));
    assert_eq!(parsed.print_scheme(), false);
}

// Can I use `typed-path` to deal with this?
// Issue #7 - Absolute Windows paths will not parse at all
#[should_panic(expected = "URL parse failed: UnexpectedFormat")]
#[test]
fn absolute_windows_path() {
    let test_url = r"c:\project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "ftps://git@host.tld/user/project-name.git");
    assert_eq!(parsed.scheme(), Some("ftp"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/user/project-name.git"));
    assert_eq!(parsed.print_scheme(), true);
}

//// Move test
////#[test]
////fn ssh_user_path_not_acctname_reponame_format() {
////    let test_url = "git@test.com:repo";
////    let e = GitUrl::parse(test_url);
////
////    assert!(e.is_err());
////    assert_eq!(
////        format!("{}", e.err().unwrap()),
////        "Git Url not in expected format"
////    );
////}
//
//// Move test
////#[test]
////fn ssh_without_organization() {
////    let test_url = "ssh://f589726c3611:29418/repo";
////    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
////    let expected = GitUrl {
////        host: Some("f589726c3611".to_string()),
////        //name: "repo".to_string(),
////        //owner: Some("repo".to_string()),
////        //organization: None,
////        //fullname: "repo/repo".to_string(),
////        scheme: Some(Scheme::Ssh),
////        user: None,
////        token: None,
////        port: Some(29418),
////        path: "repo".to_string(),
////        //git_suffix: false,
////        //scheme_prefix: true,
////        print_scheme: true,
////    };
////
////    assert_eq!(parsed, expected);
////}
//
////#[test]
////fn empty_path() {
////    assert_eq!(
////        GitUrlParseError::EmptyPath,
////        GitUrl::parse("file://").unwrap_err()
////    )
////}

#[test]
fn bad_port_number() {
    let test_url = "https://github.com:crypto-browserify/browserify-rsa.git";
    let e = GitUrl::parse(test_url);

    assert!(e.is_err());
    //assert_eq!(
    //    format!("{}", e.err().unwrap()),
    //    "Error from Url crate: invalid port number"
    //);
}

// This test might not have a use anymore if we're not expanding "git:" -> "git://"
#[test]
fn git() {
    let test_url = "git://github.com/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert_eq!(parsed.url(), "git://github.com/owner/name.git");
    assert_eq!(parsed.scheme(), Some("git"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.token(), None);
    assert_eq!(parsed.host(), Some("github.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), Some("/owner/name.git"));
    assert_eq!(parsed.print_scheme(), true);
}
