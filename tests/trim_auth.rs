use git_url_parse::*;

//#[test]
//fn ssh_user_ports() {
//    let test_url = "ssh://git@host.tld:9999/user/project-name.git";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "ssh://host.tld:9999/user/project-name.git";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
//
//// Specific service support
//#[test]
//fn https_user_bitbucket() {
//    let test_url = "https://user@bitbucket.org/user/repo.git";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "https://bitbucket.org/user/repo.git";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
//
//#[test]
//fn ssh_user_bitbucket() {
//    let test_url = "git@bitbucket.org:user/repo.git";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "bitbucket.org:user/repo.git";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
//
//#[test]
//fn https_user_auth_bitbucket() {
//    let test_url = "https://x-token-auth:token@bitbucket.org/owner/name.git/";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "https://bitbucket.org/owner/name.git/";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
//
//#[test]
//fn https_user_github() {
//    let test_url = "https://user@github.com/user/repo.git/";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "https://github.com/user/repo.git/";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
//
//#[test]
//fn ssh_user_github() {
//    let test_url = "git@github.com:user/repo.git";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "github.com:user/repo.git";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
//
//#[test]
//fn https_user_auth_github() {
//    let test_url = "https://token:x-oauth-basic@github.com/owner/name.git/";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "https://github.com/owner/name.git/";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
//
//#[test]
//fn ssh_user_azure_devops() {
//    let test_url = "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
//
//#[test]
//fn https_user_azure_devops() {
//    let test_url = "https://organization@dev.azure.com/organization/project/_git/repo";
//    let parsed_and_trimmed = GitUrl::parse(test_url)
//        .expect("URL parse failed")
//        .trim_auth();
//    let expected = "https://dev.azure.com/organization/project/_git/repo";
//
//    assert_eq!(format!("{}", parsed_and_trimmed), expected);
//}
