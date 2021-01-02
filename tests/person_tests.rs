use bac_journal::person::{Person, PersonJSON, Gender};

#[test]
fn can_parse_person_json() {
    let data = r#"
        {
            "gender": "male",
            "grams": 200
        }
    "#;
    let person_json: PersonJSON = serde_json::from_str(data).unwrap();
    let person = person_json.as_person().unwrap();
    assert_eq!(person.gender, Gender::Male);
}