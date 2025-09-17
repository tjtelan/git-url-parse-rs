use git_url_parse::types::provider::{
    AzureDevOpsProvider, GenericProvider, GitLabProvider, GitProvider,
};
use git_url_parse::{GitUrl, GitUrlParseError};
use log::debug;

#[test]
fn http_generic_git() {
    let _ = env_logger::try_init();
    let test_url = "https://github.com/tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let owner = "tjtelan";
    let repo = "git-url-parse-rs";
    let full = format!("{owner}/{repo}");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn ssh_generic_git() {
    let _ = env_logger::try_init();
    let test_url = "git@github.com:tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let owner = "tjtelan";
    let repo = "git-url-parse-rs";
    let full = format!("{owner}/{repo}");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
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

    let _ = env_logger::try_init();
    let test_url = "git@github.com:tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: TestProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let expected = TestProvider;
    assert_eq!(provider_info, expected)
}

#[test]
fn self_host() {
    let _ = env_logger::try_init();
    let test_url = "http://git.example.com:3000/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let owner = "user";
    let repo = "repo";
    let full = format!("{owner}/{repo}");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn http_azure_devops() {
    let _ = env_logger::try_init();
    let test_url = "https://CompanyName@dev.azure.com/CompanyName/ProjectName/_git/RepoName";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: AzureDevOpsProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let org = "CompanyName";
    let project = "ProjectName";
    let repo = "RepoName";
    let full = format!("{org}/{project}/{repo}");

    assert_eq!(provider_info.org(), org);
    assert_eq!(provider_info.project(), project);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn ssh_azure_devops() {
    let _ = env_logger::try_init();
    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: AzureDevOpsProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let org = "CompanyName";
    let project = "ProjectName";
    let repo = "RepoName";
    let full = format!("{org}/{project}/{repo}");

    assert_eq!(provider_info.org(), org);
    assert_eq!(provider_info.project(), project);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn http_gitlab() {
    let _ = env_logger::try_init();
    let test_url = "https://gitlab.com/gitlab-org/gitlab.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GitLabProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let owner = "gitlab-org";
    let subgroup = None;
    let repo = "gitlab";
    let full = format!("{owner}/{repo}");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.subgroup(), subgroup);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn ssh_gitlab() {
    let _ = env_logger::try_init();
    let test_url = "git@gitlab.com:gitlab-org/gitlab.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GitLabProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let owner = "gitlab-org";
    let subgroup = None;
    let repo = "gitlab";
    let full = format!("{owner}/{repo}");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.subgroup(), subgroup);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn http_gitlab_subgroups() {
    let _ = env_logger::try_init();
    let test_url = "https://gitlab.com/gitlab-org/sbom/systems/gitlab-core.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GitLabProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let owner = "gitlab-org";
    let subgroup = Some(vec!["sbom", "systems"]);
    let repo = "gitlab-core";
    let full = format!("{owner}/{}/{repo}", "sbom/systems");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.subgroup(), subgroup);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn ssh_gitlab_subgroups() {
    let _ = env_logger::try_init();
    let test_url = "git@gitlab.com:gitlab-org/sbom/systems/gitlab-core.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GitLabProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let owner = "gitlab-org";
    let subgroup = Some(vec!["sbom", "systems"]);
    let repo = "gitlab-core";
    let full = format!("{owner}/{}/{repo}", "sbom/systems");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.subgroup(), subgroup);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn url_without_git_suffix() {
    let _ = env_logger::try_init();
    let test_url = "http://git.example.com:3000/user/repo";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    debug!("{:#?}", provider_info);

    let owner = "user";
    let repo = "repo";
    let full = format!("{owner}/{repo}");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[test]
fn filepath() {
    let _ = env_logger::try_init();
    let test_url = "file:///home/user/Documents/";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: Result<GenericProvider, GitUrlParseError> = parsed.provider_info();
    debug!("{:#?}", provider_info);

    assert!(provider_info.is_err());
    if let Err(e) = provider_info {
        assert_eq!(e, GitUrlParseError::ProviderUnsupported)
    }
}
