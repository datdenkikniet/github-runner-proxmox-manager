pub enum Output<O, I> {
    Single(O),
    Iter(I),
}

pub fn get_keys_or_default_value<'str, O>(
    value: &'str str,
) -> Result<Output<O, impl Iterator<Item = Result<(&str, &str), String>>>, O::Error>
where
    O: TryFrom<&'str str>,
{
    // TODO: escape strings and stuff
    let is_multi_kvs = value.split_once(',').is_some() || value.split_once('=').is_some();

    let output = if !is_multi_kvs {
        Output::Single(TryFrom::try_from(value.trim())?)
    } else {
        let iter = value.split(',').map(|v| {
            let (k, v) = v.split_once('=').ok_or_else(|| "missing key or value")?;
            Ok((k.trim(), v.trim()))
        });

        Output::Iter(iter)
    };

    Ok(output)
}

#[cfg(test)]
mod test {

    use serde::{Deserialize, Serialize};

    use crate::types::MacAddr;

    use super::get_keys_or_default_value;

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(try_from = "&str", into = "String")]
    pub struct Test {
        name: String,
        mac_addr: Option<MacAddr<true>>,
    }

    impl TryFrom<&str> for Test {
        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let values = get_keys_or_default_value(value).unwrap();

            let iter = match values {
                super::Output::Single(name) => {
                    return Ok(Test {
                        name,
                        mac_addr: Default::default(),
                    })
                }
                super::Output::Iter(iter) => iter,
            };

            let mut name = None;
            let mut mac_addr = None;

            for value in iter {
                let (k, v) = value?;
                match k {
                    "name" if name.is_some() => {
                        return Err("Duplicate key 'name'".into());
                    }
                    "name" => name = Some(TryFrom::try_from(v).unwrap()),
                    "mac_addr" if mac_addr.is_some() => {
                        return Err("Duplicate key 'mac_addr'".into())
                    }
                    "mac_addr" => {
                        mac_addr = Some(TryFrom::try_from(v)?);
                    }
                    key => return Err(format!("Unknown key {key}")),
                }
            }

            let name = name.ok_or_else(|| "Missing required key 'name'".to_string())?;

            Ok(Test { name, mac_addr })
        }
    }

    impl From<Test> for String {
        fn from(value: Test) -> Self {
            use std::fmt::Write;

            let mut output = format!("name={}", value.name);

            if let Some(mac_addr) = value.mac_addr {
                write!(output, ", mac_addr={mac_addr}").unwrap();
            }

            output
        }
    }

    #[test]
    pub fn from_default() {
        let data = r#""name""#;
        let unwrapped: Test = serde_json::from_str(data).unwrap();

        assert_eq!(
            unwrapped,
            Test {
                name: "name".into(),
                mac_addr: None,
            }
        );
    }

    #[test]
    pub fn to_default() {
        let value = Test {
            name: "name".into(),
            mac_addr: None,
        };
        let data = "name=name";

        let output: String = String::from(value);
        assert_eq!(data, output);
    }

    #[test]
    pub fn from_kv() {
        let data = r#""name=name, mac_addr=AA:BB:CC:DD:EE:FF""#;
        let unwrapped: Test = serde_json::from_str(data).unwrap();
        assert_eq!(
            unwrapped,
            Test {
                name: "name".into(),
                mac_addr: Some(MacAddr::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]))
            }
        );
    }

    #[test]
    pub fn to_kv() {
        let value = Test {
            name: "name".into(),
            mac_addr: Some(MacAddr::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF])),
        };
        let data = "name=name, mac_addr=AA:BB:CC:DD:EE:FF";

        let output: String = String::from(value);
        assert_eq!(data, output);
    }
}
