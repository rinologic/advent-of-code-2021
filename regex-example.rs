fn regex_example() {
    let re = Regex::new(r"^(\d{4})-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));
    let cap = re.captures("2014-01-02").unwrap();
    println!("{}", &cap[1]);
}