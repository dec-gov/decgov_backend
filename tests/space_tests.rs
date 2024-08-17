use decgov_backend::{insert_space, types::space::Space, SPACES};

#[test]
pub fn test_insert_space() {
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
    let space: Space = serde_json::from_str(space).unwrap();

    insert_space(
        space.name,
        space.icon_link,
        space.website_link,
        space.vote_delay,
        space.vote_duration,
        space.min_vote_role,
        space.min_vote_power,
        space.quorum,
    );

    assert!(SPACES.with(|x| x.borrow().get(&1).is_some()));
    assert_eq!(SPACES.with(|x| x.borrow().get(&1).unwrap().name), "Space 1");
    assert_eq!(SPACES.with(|x| x.borrow().len()), 1);
}
