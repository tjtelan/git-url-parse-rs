use git_url_parse::GitUrl;
use git_url_parse::types::provider::{
    AzureDevOpsProvider, GenericProvider, GitLabProvider, GitProvider,
};

use log::debug;
#[cfg(feature = "url")]
use url::Url;

#[cfg(feature = "url")]
#[test]
fn try_from_url_ssh_git() {
    let _ = env_logger::try_init();
    let input = "git@host.tld:user/project-name.git";
    let expected = "ssh://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(input).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let convert = Url::try_from(parsed).unwrap();
    debug!("{:#?}", convert);
    assert_eq!(convert.as_str(), expected);
}

#[cfg(feature = "url")]
#[test]
fn parse_to_url_ssh_git() {
    let _ = env_logger::try_init();
    let input = "git@host.tld:user/project-name.git";
    let expected = "ssh://git@host.tld/user/project-name.git";
    let parsed = GitUrl::parse(input).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let direct = GitUrl::parse_to_url(input).unwrap();
    debug!("{:#?}", direct);
    assert_eq!(direct.as_str(), expected);
}

#[cfg(feature = "url")]
#[test]
fn https_user_github() {
    let _ = env_logger::try_init();
    let test_url = "https://user@github.com/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let convert = Url::try_from(parsed).unwrap();
    debug!("{:#?}", convert);
    assert_eq!(convert.as_str(), test_url);

    let direct = GitUrl::parse_to_url(test_url).unwrap();
    debug!("{:#?}", direct);
    assert_eq!(direct.as_str(), test_url);
}

#[cfg(feature = "url")]
#[test]
fn ssh_user_github() {
    let _ = env_logger::try_init();
    let test_url = "git@github.com:user/repo.git";
    let expected_url = "ssh://git@github.com/user/repo.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let convert = Url::try_from(parsed).unwrap();
    debug!("{:#?}", convert);
    assert_eq!(convert.as_str(), expected_url);

    let direct = GitUrl::parse_to_url(test_url).unwrap();
    debug!("{:#?}", direct);
    assert_eq!(direct.as_str(), expected_url);
}

#[cfg(feature = "url")]
#[test]
fn url_relative_unix_path() {
    let _ = env_logger::try_init();
    let test_url = "../project-name.git";
    let expected = "file://../project-name.git";
    let parsed = GitUrl::parse_to_url(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    assert_eq!(parsed.as_str(), expected)
}

#[cfg(feature = "url")]
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

// Providers
#[cfg(feature = "url")]
#[test]
fn url_http_generic_git() {
    let _ = env_logger::try_init();
    let test_url = "https://github.com/tjtelan/git-url-parse-rs.git";
    let parsed = Url::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GenericProvider = GenericProvider::from_git_url(&parsed).unwrap();
    debug!("{:#?}", provider_info);

    let owner = "tjtelan";
    let repo = "git-url-parse-rs";
    let full = format!("{owner}/{repo}");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[cfg(feature = "url")]
#[test]
fn url_self_host() {
    let _ = env_logger::try_init();
    let test_url = "http://git.example.com:3000/user/repo.git";
    let parsed = Url::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info = GenericProvider::from_git_url(&parsed).unwrap();
    debug!("{:#?}", provider_info);

    let owner = "user";
    let repo = "repo";
    let full = format!("{owner}/{repo}");

    assert_eq!(provider_info.owner(), owner);
    assert_eq!(provider_info.repo(), repo);
    assert_eq!(provider_info.fullname(), full);
}

#[cfg(feature = "url")]
#[test]
fn url_http_azure_devops() {
    let _ = env_logger::try_init();
    let test_url = "https://CompanyName@dev.azure.com/CompanyName/ProjectName/_git/RepoName";
    let parsed = Url::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info = AzureDevOpsProvider::from_git_url(&parsed).unwrap();
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

#[cfg(feature = "url")]
#[test]
fn url_ssh_azure_devops() {
    let _ = env_logger::try_init();
    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName.git";
    let parsed = GitUrl::parse_to_url(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info = AzureDevOpsProvider::from_git_url(&parsed).unwrap();
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

#[cfg(feature = "url")]
#[test]
fn url_http_gitlab() {
    let _ = env_logger::try_init();
    let test_url = "https://gitlab.com/gitlab-org/gitlab.git";
    let parsed = Url::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info = GitLabProvider::from_git_url(&parsed).unwrap();
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

#[cfg(feature = "url")]
#[test]
fn url_ssh_gitlab() {
    let _ = env_logger::try_init();
    let test_url = "git@gitlab.com:gitlab-org/gitlab.git";
    let parsed = GitUrl::parse_to_url(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info = GitLabProvider::from_git_url(&parsed).unwrap();
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

#[cfg(feature = "url")]
#[test]
fn url_http_gitlab_subgroups() {
    let _ = env_logger::try_init();
    let test_url = "https://gitlab.com/gitlab-org/sbom/systems/gitlab-core.git";
    let parsed = Url::parse(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GitLabProvider = GitLabProvider::from_git_url(&parsed).unwrap();
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

#[cfg(feature = "url")]
#[test]
fn url_ssh_gitlab_subgroups() {
    let _ = env_logger::try_init();
    let test_url = "git@gitlab.com:gitlab-org/sbom/systems/gitlab-core.git";
    let parsed = GitUrl::parse_to_url(test_url).expect("URL parse failed");
    debug!("{:#?}", parsed);

    let provider_info: GitLabProvider = GitLabProvider::from_git_url(&parsed).unwrap();
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
