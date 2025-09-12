use git_url_parse::*;
use log::debug;

#[test]
fn ssh_user_ports() {
    let _ = env_logger::try_init();
    let test_url = "ssh://git@host.tld:9999/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), Some(9999));
    assert_eq!(parsed.path(), "user/project-name.git");
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ssh_no_scheme_no_user() {
    let _ = env_logger::try_init();
    let test_url = "host.tld:user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "user/project-name.git");
    assert_eq!(parsed.print_scheme(), false);
}

// Specific service support
#[test]
fn https_user_bitbucket() {
    let _ = env_logger::try_init();
    let test_url = "https://user@bitbucket.org/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("user"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("bitbucket.org"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/user/repo.git");
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ssh_user_bitbucket() {
    let _ = env_logger::try_init();
    let test_url = "git@bitbucket.org:user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("bitbucket.org"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "user/repo.git");
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn https_user_auth_bitbucket() {
    let _ = env_logger::try_init();
    let test_url = "https://x-password-auth:token@bitbucket.org/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("x-password-auth"));
    assert_eq!(parsed.password(), Some("token"));
    assert_eq!(parsed.host(), Some("bitbucket.org"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/owner/name.git");
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn https_user_github() {
    let _ = env_logger::try_init();
    let test_url = "https://user@github.com/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("user"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("github.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/user/repo.git");
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ssh_user_github() {
    let _ = env_logger::try_init();
    let test_url = "git@github.com:user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("github.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "user/repo.git");
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn https_user_auth_github() {
    let _ = env_logger::try_init();
    let test_url = "https://password:x-oauth-basic@github.com/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("password"));
    assert_eq!(parsed.password(), Some("x-oauth-basic"));
    assert_eq!(parsed.host(), Some("github.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/owner/name.git");
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ssh_user_azure_devops() {
    let _ = env_logger::try_init();
    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("ssh"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("ssh.dev.azure.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "v3/CompanyName/ProjectName/RepoName");
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn https_user_azure_devops() {
    let _ = env_logger::try_init();
    let test_url = "https://organization@dev.azure.com/organization/project/_git/repo";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("https"));
    assert_eq!(parsed.user(), Some("organization"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("dev.azure.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/organization/project/_git/repo");
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ftp_user() {
    let _ = env_logger::try_init();
    let test_url = "ftp://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("ftp"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/user/project-name.git");
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn ftps_user() {
    let _ = env_logger::try_init();
    let test_url = "ftps://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("ftps"));
    assert_eq!(parsed.user(), Some("git"));
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("host.tld"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/user/project-name.git");
    assert_eq!(parsed.print_scheme(), true);
}

#[test]
fn relative_unix_path() {
    let _ = env_logger::try_init();
    let test_url = "../project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("file"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), None);
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "../project-name.git");
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn absolute_unix_path() {
    let _ = env_logger::try_init();
    let test_url = "/path/to/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("file"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), None);
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/path/to/project-name.git");
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn relative_windows_path() {
    let _ = env_logger::try_init();
    let test_url = r"..\project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("file"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), None);
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "..\\project-name.git");
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn absolute_windows_path() {
    let _ = env_logger::try_init();
    let test_url = r"c:\project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), test_url);
    assert_eq!(parsed.scheme(), Some("file"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), None);
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), r"c:\project-name.git");
    assert_eq!(parsed.print_scheme(), false);
}

#[test]
fn bad_port_1() {
    let _ = env_logger::try_init();
    let test_url = "https://github.com:crypto-browserify/browserify-rsa.git";
    let e = GitUrl::parse(test_url);
    debug!("{:#?}", e);

    assert!(e.is_err());
    if let Err(err) = e {
        assert_eq!(err, GitUrlParseError::InvalidPortNumber)
    }
}

#[test]
fn bad_port_2() {
    let _ = env_logger::try_init();
    let test_url = "https://example.org:7z";
    let e = GitUrl::parse(test_url);
    debug!("{:#?}", e);

    assert!(e.is_err());
    if let Err(err) = e {
        assert_eq!(err, GitUrlParseError::InvalidPortNumber)
    }
}

#[test]
fn port_out_of_range() {
    let _ = env_logger::try_init();
    let test_url = "https://example.org:70000";
    let e = GitUrl::parse(test_url);
    debug!("{:#?}", e);

    assert!(e.is_err());
    if let Err(err) = e {
        assert_eq!(err, GitUrlParseError::InvalidPortNumber)
    }
}

#[test]
fn host_missing_1() {
    let _ = env_logger::try_init();
    let test_url = "https://:443";
    let e = GitUrl::parse(test_url);
    debug!("{:#?}", e);

    assert!(e.is_err());
    if let Err(err) = e {
        assert_eq!(err, GitUrlParseError::InvalidPathEmpty)
    }
}

#[test]
fn host_missing_2() {
    let _ = env_logger::try_init();
    let test_url = "https://user:pass@";
    let e = GitUrl::parse(test_url);
    debug!("{:#?}", e);

    assert!(e.is_err());
    if let Err(err) = e {
        assert_eq!(err, GitUrlParseError::InvalidPathEmpty)
    }
}

// FIXME: This test does not throw the correct error
#[test]
fn host_invalid() {
    let _ = env_logger::try_init();
    let test_url = "foo://exa[mple.org/owner/repo.git";
    let e = GitUrl::parse(test_url);
    debug!("{:#?}", e);

    assert!(e.is_err());
}

#[test]
fn short_git() {
    let _ = env_logger::try_init();
    let test_url = "git:github.com/owner/name.git";
    let expected_url = "git://github.com/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.to_string(), expected_url);
    assert_eq!(parsed.scheme(), Some("git"));
    assert_eq!(parsed.user(), None);
    assert_eq!(parsed.password(), None);
    assert_eq!(parsed.host(), Some("github.com"));
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.path(), "/owner/name.git");
    assert_eq!(parsed.print_scheme(), true);
}
