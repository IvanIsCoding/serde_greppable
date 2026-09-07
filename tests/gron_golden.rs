use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Person {
    name: String,
    age: u8,
}

#[test]
fn greppable_simple() {
    assert_eq!(
        serde_greppable::to_string(&json!({"name": "Alice", "age": 30})).unwrap(),
        "json = {};\njson.name = \"Alice\";\njson.age = 30;\n"
    );
}

#[test]
fn greppable_nested() {
    assert_eq!(
        serde_greppable::to_string(&json!({"person": {"name": "Bob", "city": "NYC"}, "id": 1}))
            .unwrap(),
        "json = {};\njson.person = {};\njson.person.name = \"Bob\";\njson.person.city = \"NYC\";\njson.id = 1;\n"
    );
}

#[test]
fn from_gron_simple() {
    let input = "json = {};\njson.age = 30;\njson.name = \"Alice\";\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"age": 30, "name": "Alice"})
    );
}

#[test]
fn from_gron_nested() {
    let input = "json = {};\njson.id = 1;\njson.person = {};\njson.person.city = \"NYC\";\njson.person.name = \"Bob\";\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"id": 1, "person": {"city": "NYC", "name": "Bob"}})
    );
}

#[test]
fn from_gron_arrays() {
    let input = "json = {};\njson.items = [];\njson.items[0] = \"first\";\njson.items[1] = \"second\";\njson.items[2] = \"third\";\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"items": ["first", "second", "third"]})
    );
}

#[test]
fn from_gron_sparse_array() {
    let input = "json.likes = [];\njson.likes[0] = \"code\";\njson.likes[2] = \"meat\";\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"likes": ["code", null, "meat"]})
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn from_gron_mixed_types() {
    let input = "json = {};\njson.integer = 42;\njson.float = 3.14;\njson.negative = -5;\njson.isTrue = true;\njson.isFalse = false;\njson.nothing = null;\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"float": 3.14, "integer": 42, "isFalse": false, "isTrue": true, "negative": -5, "nothing": null})
    );
}

#[test]
fn from_gron_special_keys() {
    let input = "json = {};\njson[\"key-with-dashes\"] = \"value1\";\njson[\"key.with.dots\"] = \"value2\";\njson[\"key with spaces\"] = \"value3\";\njson[\"123numeric\"] = \"value4\";\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"123numeric": "value4", "key with spaces": "value3", "key-with-dashes": "value1", "key.with.dots": "value2"})
    );
}

#[test]
fn from_gron_identifier_edge_cases() {
    let input =
        "json = {};\njson.$dollar = 1;\njson._underscore = 2;\njson.alpha$beta_gamma9 = 3;\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"$dollar": 1, "_underscore": 2, "alpha$beta_gamma9": 3})
    );
}

#[test]
fn from_gron_single_quotes() {
    let input = "json = {};\njson['User-Agent'] = 'gron/0.1';\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"User-Agent": "gron/0.1"})
    );
}

#[test]
fn from_gron_deeply_nested() {
    let input = "json = {};\njson.a = {};\njson.a.b = {};\njson.a.b.c = {};\njson.a.b.c.d = [];\njson.a.b.c.d[0] = {};\njson.a.b.c.d[0].e = \"deep\";\n";
    assert_eq!(
        serde_greppable::from_str::<Value>(input).unwrap(),
        json!({"a": {"b": {"c": {"d": [{"e": "deep"}]}}}})
    );
}

#[test]
fn serde_api_round_trip() {
    let person = Person {
        name: "Alice".to_owned(),
        age: 30,
    };
    let encoded = serde_greppable::to_string(&person).unwrap();
    assert_eq!(
        serde_greppable::from_str::<Person>(&encoded).unwrap(),
        person
    );

    let mut writer = Vec::new();
    serde_greppable::to_writer(&person, &mut writer).unwrap();
    assert_eq!(
        serde_greppable::from_slice::<Person>(&writer).unwrap(),
        person
    );
    assert_eq!(
        serde_greppable::from_reader::<Person, _>(writer.as_slice()).unwrap(),
        person
    );
}
