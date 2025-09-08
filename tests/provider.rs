use git_url_parse::types::provider::{
    AzureDevOpsProvider, GenericProvider, GitLabProvider, GitProvider,
};
use git_url_parse::{GitUrl, GitUrlParseError};

#[test]
fn http_generic_git() {
    let test_url = "https://github.com/tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    let expected = GenericProvider {
        owner: "tjtelan",
        repo: "git-url-parse-rs",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_generic_git() {
    let test_url = "git@github.com:tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    let expected = GenericProvider {
        owner: "tjtelan",
        repo: "git-url-parse-rs",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn custom_provider() {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestProvider;
    impl GitProvider<GitUrl<'_>, GitUrlParseError> for TestProvider {
        fn from_git_url(_url: &GitUrl) -> Result<Self, GitUrlParseError> {
            Ok(Self)
        }
    }

    let test_url = "git@github.com:tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: TestProvider = parsed.provider_info().unwrap();
    let expected = TestProvider;
    assert_eq!(provider_info, expected)
}

#[test]
fn self_host() {
    let test_url = "http://git.example.com:3000/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    let expected = GenericProvider {
        owner: "user",
        repo: "repo",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn http_azure_devops() {
    let test_url = "https://CompanyName@dev.azure.com/CompanyName/ProjectName/_git/RepoName";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: AzureDevOpsProvider = parsed.provider_info().unwrap();
    let expected = AzureDevOpsProvider {
        org: "CompanyName",
        project: "ProjectName",
        repo: "RepoName",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_azure_devops() {
    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: AzureDevOpsProvider = parsed.provider_info().unwrap();
    let expected = AzureDevOpsProvider {
        org: "CompanyName",
        project: "ProjectName",
        repo: "RepoName",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn http_gitlab() {
    let test_url = "https://gitlab.com/gitlab-org/gitlab.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GitLabProvider = parsed.provider_info().unwrap();
    let expected = GitLabProvider {
        owner: "gitlab-org",
        subgroup: None,
        repo: "gitlab",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_gitlab() {
    let test_url = "git@gitlab.com:gitlab-org/gitlab.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GitLabProvider = parsed.provider_info().unwrap();
    let expected = GitLabProvider {
        owner: "gitlab-org",
        subgroup: None,
        repo: "gitlab",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn http_gitlab_subgroups() {
    let test_url = "https://gitlab.com/gitlab-org/sbom/systems/gitlab-core.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GitLabProvider = parsed.provider_info().unwrap();
    let expected = GitLabProvider {
        owner: "gitlab-org",
        subgroup: Some(vec!["sbom", "systems"]),
        repo: "gitlab-core",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_gitlab_subgroups() {
    let test_url = "git@gitlab.com:gitlab-org/sbom/systems/gitlab-core.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GitLabProvider = parsed.provider_info().unwrap();
    let expected = GitLabProvider {
        owner: "gitlab-org",
        subgroup: Some(vec!["sbom", "systems"]),
        repo: "gitlab-core",
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn filepath() {
    let test_url = "file:///home/user/Documents/";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: Result<GenericProvider, GitUrlParseError> = parsed.provider_info();

    assert!(provider_info.is_err());
    if let Err(e) = provider_info {
        assert_eq!(e, GitUrlParseError::ProviderUnsupported)
    }
}
