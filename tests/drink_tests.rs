use bac_journal::drink::{Drink, DrinkJSON};

#[test]
fn can_parse_json_drink() {
    let data = r#"
        {
            "name": "Bud",
            "percent": 5,
            "grams": 100,
            "datetime_rfc2822": "Wed, 18 Feb 2015 23:16:09 GMT"
        }
    "#;

    let drink_json: DrinkJSON = serde_json::from_str(data).unwrap();
    let drink = drink_json.as_drink();
    assert_eq!(&drink.name, "Bud");
}
