use git_url_parse::types::provider::{
    AzureDevOpsProvider, GenericProvider, GitLabProvider, GitProvider,
};
use git_url_parse::{GitUrl, GitUrlParseError};

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

// Azure Devops
// https://learn.microsoft.com/en-us/azure/devops/repos/git/clone?view=azure-devops&tabs=visual-studio-2022
// https://learn.microsoft.com/en-us/azure/devops/release-notes/2018/sep-10-azure-devops-launch#administration
//vec!["dev.azure.com", "ssh.dev.azure.com", "visualstudio.com"];
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

// GitLab
// https://docs.gitlab.com/topics/git/clone/#clone-with-ssh
// https://gitlab.com/explore/projects/trending?sort=latest_activity_desc
// https://gitlab.com/redhat/red-hat-ci-tools/kernel
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

    //assert!(parsed.provider_info::<GenericProvider>().is_ok());

    let provider_info: Result<GenericProvider, GitUrlParseError> = parsed.provider_info();
    assert!(provider_info.is_err())
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
