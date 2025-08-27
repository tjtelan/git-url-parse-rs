use git_url_parse::*;

#[test]
fn http_generic_git() {
    let test_url = "https://github.com/tjtelan/git-url-parse-rs.git";
    let parsed = GitUrl::parse(test_url).expect("URL parse failed");

    let provider_info: GenericProvider = parsed.provider_info().unwrap();
    let expected = GenericProvider {
        host: "github.com".to_string(),
        user: "tjtelan".to_string(),
        repo: "git-url-parse-rs.git".to_string(),
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
        user: "tjtelan".to_string(),
        repo: "git-url-parse-rs.git".to_string(),
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

// Azure Devops
// https://learn.microsoft.com/en-us/azure/devops/repos/git/clone?view=azure-devops&tabs=visual-studio-2022
// https://learn.microsoft.com/en-us/azure/devops/release-notes/2018/sep-10-azure-devops-launch#administration

// GitHub
// https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository

// GitLab
// https://docs.gitlab.com/topics/git/clone/#clone-with-ssh
// https://gitlab.com/explore/projects/trending?sort=latest_activity_desc
// https://gitlab.com/redhat/red-hat-ci-tools/kernel

// BitBucket
// https://confluence.atlassian.com/bitbucketserver/clone-a-repository-790632786.html

// SourceForge
// https://sourceforge.net/p/forge/documentation/Git/#h-how-to-clone-an-existing-repository

// Codeberg
// https://codeberg.org/explore/repos
