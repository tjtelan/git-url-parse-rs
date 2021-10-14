use git_url_parse::*;

// Url Normalization
#[test]
fn git() {
    let test_url = "git://host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "git://host.tld/user/project-name.git");
}

#[test]
fn http() {
    let test_url = "http://host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "http://host.tld/user/project-name.git");
}

#[test]
fn https() {
    let test_url = "https://host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(
        normalized.as_str(),
        "https://host.tld/user/project-name.git"
    );
}

#[test]
fn ssh_scheme() {
    let test_url = "ssh://git@host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(
        normalized.as_str(),
        "ssh://git@host.tld/user/project-name.git"
    );
}

#[test]
fn ssh_no_scheme() {
    let test_url = "git@host.tld:user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(
        normalized.as_str(),
        "ssh://git@host.tld/user/project-name.git"
    );
}

#[test]
fn ssh_no_scheme_no_user() {
    let test_url = "host.tld:user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "ssh://host.tld/user/project-name.git");
}

#[test]
fn unix_file_scheme_abs_path() {
    let test_url = "file:///user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "file:///user/project-name.git");
}

#[test]
fn unix_file_no_scheme_abs_path() {
    let test_url = "/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "file:///user/project-name.git");
}

#[test]
fn unix_file_scheme_rel_path() {
    let test_url = "file://../user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "file://../user/project-name.git");
}

#[test]
fn unix_file_no_scheme_rel_path() {
    let test_url = "../user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "file://../user/project-name.git");
}

#[should_panic(expected = "assertion failed: `(left == right)")]
#[test]
fn win_file_scheme_abs_path() {
    let test_url = "file://c:\\user\\project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    // I actually don't know how this should be normalized.
    assert_eq!(normalized.as_str(), "file://c:\\user\\project-name.git");
}

#[should_panic(expected = "assertion failed: `(left == right)")]
#[test]
fn win_file_no_scheme_abs_path() {
    let test_url = "c:\\user\\project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    // I actually don't know how this should be normalized.
    assert_eq!(normalized.as_str(), "file://c:\\user\\project-name.git");
}

#[test]
fn win_file_scheme_rel_path() {
    let test_url = "file://..\\user\\project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    // I actually don't know how this should be normalized.
    assert_eq!(normalized.as_str(), "file://../user/project-name.git");
}

#[test]
fn win_file_no_scheme_rel_path() {
    let test_url = "..\\user\\project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    // I actually don't know how this should be normalized.
    assert_eq!(normalized.as_str(), "file://../user/project-name.git");
}
#[test]
fn multi_git_ssh() {
    let test_url = "git+ssh://host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(
        normalized.as_str(),
        "git+ssh://host.tld/user/project-name.git"
    );
}

// From https://github.com/tjtelan/git-url-parse-rs/issues/16
#[test]
fn null_in_input1() {
    let test_url = "////////ws///////////*,\u{0}\u{0}^\u{0}\u{0}\u{0}\u{0}@2\u{1}\u{0}\u{1d})\u{0}\u{0}\u{0}:\u{0}\u{0}\u{0}";
    let normalized = normalize_url(test_url);

    assert!(normalized.is_err());
}

// From https://github.com/tjtelan/git-url-parse-rs/issues/16
#[test]
fn null_in_input2() {
    let test_url = "?\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1f}s\u{3}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{5}\u{1}@\u{0}\u{0}\u{4}!e\u{0}\u{0}2\u{1c}^3106://?<!41\u{0}\u{0}\u{0}?\u{0}\u{0}\u{0}\u{0}\u{4}?";
    let normalized = normalize_url(test_url);

    assert!(normalized.is_err());
}
