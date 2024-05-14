use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap};

pub mod flattened;

mod format;
pub use format::{Format, KnownFormat};

mod info;
pub use info::Info;

mod output;
pub use output::Output;

mod parameters;
pub use parameters::Parameters;

mod permission;
pub use permission::Permission;

mod returns;

mod ty;
pub use ty::Type;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeNode<'a> {
    #[serde(borrow, flatten)]
    pub value: Value<'a>,
    #[serde(borrow, default)]
    pub children: Vec<TreeNode<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Value<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaf: Option<u32>,
    #[serde(borrow)]
    pub path: Cow<'a, str>,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
    #[serde(borrow, default)]
    pub info: BTreeMap<Cow<'a, str>, Info<'a>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Optional(Option<serde_json::Value>);

impl Optional {
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn get(&self) -> bool {
        use serde_json::Value::*;

        let val = self.0.as_ref();

        if let Some(String(str)) = val {
            str == "1"
        } else if let Some(Number(num)) = val {
            if let Some(v) = num.as_u64() {
                v == 1
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl PartialEq for Optional {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
