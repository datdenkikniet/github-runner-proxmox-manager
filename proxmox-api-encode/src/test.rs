use crate::{de::from_str, ser::to_string};

#[test]
pub fn test() {}

#[test]
fn test_struct() {
    use serde_derive::Serialize;
    #[derive(Serialize)]
    struct T {
        pub field1: String,
        pub filed2: String,
    }
    let t = T {
        field1: String::from("1"),
        filed2: String::from("2"),
    };

    let re = to_string(&t).unwrap();
    println!("re: {}", re);
}

#[test]
fn test_struct_inner() {
    use serde_derive::Serialize;
    #[derive(Serialize)]
    struct Inner {
        pub if1: String,
        pub if2: String,
    }
    #[derive(Serialize)]
    struct T {
        pub field1: String,
        pub filed2: String,
        #[serde(flatten)]
        pub inner: Inner,
    }

    let t = T {
        field1: String::from("1"),
        filed2: String::from("2"),
        inner: Inner {
            if1: String::from("i1"),
            if2: String::from("i2"),
        },
    };

    let re = to_string(&t).unwrap();
    println!("re: {}", re);
}

#[test]
fn test_struct_inners() {
    use serde_derive::Serialize;
    use std::collections::BTreeMap;
    #[derive(Serialize)]
    struct Inner {
        pub if1: String,
        pub if2: String,
    }
    #[derive(Serialize)]
    struct T {
        pub field1: String,
        pub filed2: String,
        #[serde(flatten)]
        pub inners: BTreeMap<String, Inner>,
    }

    let mut inners = BTreeMap::new();
    inners.insert(
        "list1".into(),
        Inner {
            if1: String::from("i1"),
            if2: String::from("i2"),
        },
    );
    let t = T {
        field1: String::from("1"),
        filed2: String::from("2"),
        inners,
    };

    let re = to_string(&t).unwrap();
    println!("re: {}", re);
}

use std::collections::{BTreeMap, HashMap};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Test1 {
    pub status: bool,
    pub key1: String,
    pub key2: String,
    pub sub1: SubTest1,
    pub seq: Vec<String>,
    #[serde(flatten)]
    pub map: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubTest1 {
    pub key1: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum EnumTest {
    I,
    II,
}
#[test]
fn test_serialise() {
    let mut t1 = Test1 {
        status: true,
        key1: "value1".into(),
        key2: "value2".into(),
        sub1: SubTest1 {
            key1: "subvalue1".into(),
        },
        seq: Vec::from(["1".into(), "2".into()]),
        map: BTreeMap::new(),
    };
    t1.map.insert("flattenmap1".into(), "mvalue1".into());
    let s = to_string(&t1);
    println!("t1: {}", s.unwrap())
}

#[test]
fn test_serialize_option() {
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub opt: Option<i32>,
        pub opt2: Option<i32>,
        pub opt3: Option<i32>,
        pub opt4: Option<i32>,
    }

    let s: Test1 = Test1 {
        opt: Some(1),
        opt2: None,
        opt3: Some(3),
        opt4: None,
    };
    let s = to_string(&s);
    println!("{:?}", s);
}

#[test]
fn test_serialize_option_map() {
    let mut map: HashMap<String, Option<i32>> = HashMap::new();

    map.insert("k".into(), Some(1));
    map.insert("k2".into(), None);
    let s = to_string(&map);
    println!("{:?}", s);
}

#[test]
fn test_serialize_nested_option() {
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub t: Test2,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Test2 {
        pub opt: Option<i32>,
        pub opt2: Option<i32>,
        pub opt3: Option<i32>,
        pub opt4: Option<i32>,
    }

    let s: Test1 = Test1 {
        t: Test2 {
            opt: Some(1),
            opt2: None,
            opt3: Some(3),
            opt4: None,
        },
    };
    let s = to_string(&s);
    println!("{:?}", s);
}

#[test]
fn test_serialize_nested_option_map() {
    let mut map: HashMap<String, HashMap<String, Option<i32>>> = HashMap::new();
    let mut map2 = HashMap::new();
    map2.insert("inner-key".into(), Some(1));
    map2.insert("innerkey".into(), None);
    map.insert("k".into(), map2);
    let s = to_string(&map);
    println!("{:?}", s);
}

#[test]
fn test_serialize_percent_encode() {
    #[derive(Serialize, Deserialize, Debug)]
    struct T {
        file: String,
        media: String,
    }
    let mut map: HashMap<String, T> = HashMap::new();
    map.insert(
        "ide2".into(),
        T {
            file: "local:iso/ubuntu-24.img".into(),
            media: "cdrom".into(),
        },
    );
    let s = to_string(&map);
    println!("{:?}", s);
}

#[test]
fn test_deserialize_percent_encode() {
    #[derive(Serialize, Deserialize, Debug)]
    struct T {
        #[serde(alias = "")]
        file: String,
        media: String,
    }
    let s = "ide2=local%3Aiso%2Fubuntu-24.img%2Cmedia%3Dcdrom";
    let map: HashMap<String, T> = from_str(&s).unwrap();

    println!("{:?}", map);
}

#[test]
fn test_deserialize() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}

#[test]
fn test_deserialize_enum() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
        pub e: EnumTest,
        pub arr: Vec<i32>,
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32&e=I&arr=1%3B2"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}

#[test]
fn test_deserialize_extra() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
        pub e: EnumTest,
        pub arr: Vec<i32>,
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32&e=I&arr=1%3B2&WTF=1"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}

#[test]
fn test_deserialize_option_none() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
        pub e: EnumTest,
        pub arr: Vec<i32>,
        pub option: Option<i32>,
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32&e=I&arr=1%3B2&WTF=1"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}

#[test]
fn test_deserialize_option_some() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
        pub e: EnumTest,
        pub arr: Vec<i32>,
        pub option: Option<i32>,
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32&e=I&arr=1%3B2&WTF=1&option=1"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}
