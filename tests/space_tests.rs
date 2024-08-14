#[test]
fn test_insert_space() {
    let space = r#"
        {
            "id": 1,
            "name": "Space 1",
            "icon_link": "https://space1.com/icon",
            "website_link": "https://space1.com",
            "vote_delay": 10,
            "vote_duration": 20,
            "min_vote_role": 1,
            "min_vote_power": 100,
            "quorum": 10,
            "proposals": []
        }
        "#;
    let space = serde_json::from_str(space).unwrap();
    insert_space(space);
    assert_eq!(SPACES.get(&1).is_some(), true);
    assert_eq!(SPACES.get(&1).unwrap().name, "Space 1");
    assert_eq!(SPACES.with(|spaces| spaces.len()), 1);
}
