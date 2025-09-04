use git_url_parse::*;

// GitHub
// https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository
// BitBucket
// https://confluence.atlassian.com/bitbucketserver/clone-a-repository-790632786.html
// Codeberg
// https://codeberg.org/explore/repos

#[test]
fn http_generic_git() {
    let test_url = "https://github.com/tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    let expected = GenericProvider {
        host: "github.com".to_string(),
        owner: "tjtelan".to_string(),
        repo: "git-url-parse-rs".to_string(),
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_generic_git() {
    let test_url = "git@github.com:tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    let expected = GenericProvider {
        host: "github.com".to_string(),
        owner: "tjtelan".to_string(),
        repo: "git-url-parse-rs".to_string(),
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn custom_provider() {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestProvider;
    impl GitProvider<GitUrl, GitUrlParseError> for TestProvider {
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
        host: "git.example.com:3000".to_string(),
        owner: "user".to_string(),
        repo: "repo".to_string(),
    };
    assert_eq!(provider_info, expected)
}

// Azure Devops
// https://learn.microsoft.com/en-us/azure/devops/repos/git/clone?view=azure-devops&tabs=visual-studio-2022
// https://learn.microsoft.com/en-us/azure/devops/release-notes/2018/sep-10-azure-devops-launch#administration
//vec!["dev.azure.com", "ssh.dev.azure.com", "visualstudio.com"];
#[test]
fn http_azure_devops() {
    let test_url = "https://CompanyName@dev.azure.com/CompanyName/ProjectName/_git/RepoName";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: types::AzureDevOpsProvider = parsed.provider_info().unwrap();
    let expected = types::AzureDevOpsProvider {
        host: "dev.azure.com".to_string(),
        org: "CompanyName".to_string(),
        project: "ProjectName".to_string(),
        repo: "RepoName".to_string(),
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_azure_devops() {
    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: types::AzureDevOpsProvider = parsed.provider_info().unwrap();
    let expected = types::AzureDevOpsProvider {
        host: "ssh.dev.azure.com".to_string(),
        org: "CompanyName".to_string(),
        project: "ProjectName".to_string(),
        repo: "RepoName".to_string(),
    };
    assert_eq!(provider_info, expected)
}

// GitLab
// https://docs.gitlab.com/topics/git/clone/#clone-with-ssh
// https://gitlab.com/explore/projects/trending?sort=latest_activity_desc
// https://gitlab.com/redhat/red-hat-ci-tools/kernel
#[test]
fn http_gitlab() {
    let test_url = "https://gitlab.com/gitlab-org/gitlab.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: types::GitLabProvider = parsed.provider_info().unwrap();
    let expected = types::GitLabProvider {
        host: "gitlab.com".to_string(),
        user: "gitlab-org".to_string(),
        subgroup: None,
        repo: "gitlab".to_string(),
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_gitlab() {
    let test_url = "git@gitlab.com:gitlab-org/gitlab.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: types::GitLabProvider = parsed.provider_info().unwrap();
    let expected = types::GitLabProvider {
        host: "gitlab.com".to_string(),
        user: "gitlab-org".to_string(),
        subgroup: None,
        repo: "gitlab".to_string(),
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn http_gitlab_subgroups() {
    let test_url = "https://gitlab.com/gitlab-org/sbom/systems/gitlab-core.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: types::GitLabProvider = parsed.provider_info().unwrap();
    let expected = types::GitLabProvider {
        host: "gitlab.com".to_string(),
        user: "gitlab-org".to_string(),
        subgroup: Some(vec!["sbom".to_string(), "systems".to_string()]),
        repo: "gitlab-core".to_string(),
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn ssh_gitlab_subgroups() {
    let test_url = "git@gitlab.com:gitlab-org/sbom/systems/gitlab-core.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: types::GitLabProvider = parsed.provider_info().unwrap();
    let expected = types::GitLabProvider {
        host: "gitlab.com".to_string(),
        user: "gitlab-org".to_string(),
        subgroup: Some(vec!["sbom".to_string(), "systems".to_string()]),
        repo: "gitlab-core".to_string(),
    };
    assert_eq!(provider_info, expected)
}

#[test]
fn filepath() {
    let test_url = "file:///home/user/Documents/";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    assert!(parsed.provider().is_none());

    let provider_info: Result<GenericProvider, GitUrlParseError> = parsed.provider_info();
    assert!(provider_info.is_err())
}
