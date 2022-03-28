use git_url_parse::*;
#[test]
fn ssh_user_ports() {
    let test_url = "ssh://git@host.tld:9999/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("host.tld".to_string()),
        name: "project-name".to_string(),
        owner: Some("user".to_string()),
        organization: None,
        fullname: "user/project-name".to_string(),
        scheme: Scheme::Ssh,
        user: Some("git".to_string()),
        token: None,
        port: Some(9999),
        path: "user/project-name.git".to_string(),
        git_suffix: true,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

// Specific service support
#[test]
fn https_user_bitbucket() {
    let test_url = "https://user@bitbucket.org/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("bitbucket.org".to_string()),
        name: "repo".to_string(),
        owner: Some("user".to_string()),
        organization: None,
        fullname: "user/repo".to_string(),
        scheme: Scheme::Https,
        user: Some("user".to_string()),
        token: None,
        port: None,
        path: "/user/repo.git".to_string(),
        git_suffix: true,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn ssh_user_bitbucket() {
    let test_url = "git@bitbucket.org:user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("bitbucket.org".to_string()),
        name: "repo".to_string(),
        owner: Some("user".to_string()),
        organization: None,
        fullname: "user/repo".to_string(),
        scheme: Scheme::Ssh,
        user: Some("git".to_string()),
        token: None,
        port: None,
        path: "user/repo.git".to_string(),
        git_suffix: true,
        scheme_prefix: false,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn https_user_auth_bitbucket() {
    let test_url = "https://x-token-auth:token@bitbucket.org/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("bitbucket.org".to_string()),
        name: "name".to_string(),
        owner: Some("owner".to_string()),
        organization: None,
        fullname: "owner/name".to_string(),
        scheme: Scheme::Https,
        user: Some("x-token-auth".to_string()),
        token: Some("token".to_string()),
        port: None,
        path: "/owner/name.git".to_string(),
        git_suffix: true,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn https_user_github() {
    let test_url = "https://user@github.com/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("github.com".to_string()),
        name: "repo".to_string(),
        owner: Some("user".to_string()),
        organization: None,
        fullname: "user/repo".to_string(),
        scheme: Scheme::Https,
        user: Some("user".to_string()),
        token: None,
        port: None,
        path: "/user/repo.git".to_string(),
        git_suffix: true,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn ssh_user_github() {
    let test_url = "git@github.com:user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("github.com".to_string()),
        name: "repo".to_string(),
        owner: Some("user".to_string()),
        organization: None,
        fullname: "user/repo".to_string(),
        scheme: Scheme::Ssh,
        user: Some("git".to_string()),
        token: None,
        port: None,
        path: "user/repo.git".to_string(),
        git_suffix: true,
        scheme_prefix: false,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn https_user_auth_github() {
    let test_url = "https://token:x-oauth-basic@github.com/owner/name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("github.com".to_string()),
        name: "name".to_string(),
        owner: Some("owner".to_string()),
        organization: None,
        fullname: "owner/name".to_string(),
        scheme: Scheme::Https,
        user: Some("token".to_string()),
        token: Some("x-oauth-basic".to_string()),
        port: None,
        path: "/owner/name.git".to_string(),
        git_suffix: true,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn ssh_user_azure_devops() {
    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("ssh.dev.azure.com".to_string()),
        name: "RepoName".to_string(),
        owner: Some("ProjectName".to_string()),
        organization: Some("CompanyName".to_string()),
        fullname: "CompanyName/ProjectName/RepoName".to_string(),
        scheme: Scheme::Ssh,
        user: Some("git".to_string()),
        token: None,
        port: None,
        path: "v3/CompanyName/ProjectName/RepoName".to_string(),
        git_suffix: false,
        scheme_prefix: false,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn https_user_azure_devops() {
    let test_url = "https://organization@dev.azure.com/organization/project/_git/repo";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("dev.azure.com".to_string()),
        name: "repo".to_string(),
        owner: Some("project".to_string()),
        organization: Some("organization".to_string()),
        fullname: "organization/project/repo".to_string(),
        scheme: Scheme::Https,
        user: Some("organization".to_string()),
        token: None,
        port: None,
        path: "/organization/project/_git/repo".to_string(),
        git_suffix: false,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn ftp_user() {
    let test_url = "ftp://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("host.tld".to_string()),
        name: "project-name".to_string(),
        owner: Some("user".to_string()),
        organization: None,
        fullname: "user/project-name".to_string(),
        scheme: Scheme::Ftp,
        user: Some("git".to_string()),
        token: None,
        port: None,
        path: "/user/project-name.git".to_string(),
        git_suffix: true,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn ftps_user() {
    let test_url = "ftps://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: Some("host.tld".to_string()),
        name: "project-name".to_string(),
        owner: Some("user".to_string()),
        organization: None,
        fullname: "user/project-name".to_string(),
        scheme: Scheme::Ftps,
        user: Some("git".to_string()),
        token: None,
        port: None,
        path: "/user/project-name.git".to_string(),
        git_suffix: true,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn relative_unix_path() {
    let test_url = "../project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: None,
        name: "project-name".to_string(),
        owner: None,
        organization: None,
        fullname: "project-name".to_string(),
        scheme: Scheme::File,
        user: None,
        token: None,
        port: None,
        path: "../project-name.git".to_string(),
        git_suffix: true,
        scheme_prefix: false,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn absolute_unix_path() {
    let test_url = "/path/to/project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: None,
        name: "project-name".to_string(),
        owner: None,
        organization: None,
        fullname: "project-name".to_string(),
        scheme: Scheme::File,
        user: None,
        token: None,
        port: None,
        path: "/path/to/project-name.git".to_string(),
        git_suffix: true,
        scheme_prefix: false,
    };

    assert_eq!(parsed, expected);
}

// Issue #6 - Relative Windows paths will parse into Unix paths
#[test]
fn relative_windows_path() {
    let test_url = "..\\project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: None,
        name: "project-name".to_string(),
        owner: None,
        organization: None,
        fullname: "project-name".to_string(),
        scheme: Scheme::File,
        user: None,
        token: None,
        port: None,
        path: "../project-name.git".to_string(),
        git_suffix: true,
        scheme_prefix: false,
    };

    assert_eq!(parsed, expected);
}

// Issue #7 - Absolute Windows paths will not parse at all
#[should_panic(expected = "git url is not of expected format")]
#[test]
fn absolute_windows_path() {
    let test_url = "c:\\project-name.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    let expected = GitUrl {
        host: None,
        name: "project-name".to_string(),
        owner: None,
        organization: None,
        fullname: "project-name".to_string(),
        scheme: Scheme::File,
        user: None,
        token: None,
        port: None,
        path: "c:\\project-name.git".to_string(),
        git_suffix: true,
        scheme_prefix: true,
    };

    assert_eq!(parsed, expected);
}

#[test]
fn do_not_panic_when_ssh_url_has_no_org() {
    let test_url = "git@test.com:repo";
    let e = GitUrl::parse(test_url);

    assert!(e.is_err());
    assert_eq!(
        format!("{}", e.err().unwrap()),
        "git url is not of expected format"
    );
}
