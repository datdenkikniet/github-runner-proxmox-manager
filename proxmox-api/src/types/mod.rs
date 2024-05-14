mod bool;
pub use bool::{
    deserialize_bool, deserialize_bool_optional, serialize_bool, serialize_bool_optional, Bool,
};

mod formatted_string;

mod integer;
pub use integer::{
    deserialize_int, deserialize_int_optional, serialize_int, serialize_int_optional, Integer,
};

pub mod multi;
pub use multi::{deserialize_additional_data, deserialize_multi, serialize_multi};

mod mac_addr;
pub use mac_addr::MacAddr;

mod number;
pub use number::{
    deserialize_number, deserialize_number_optional, serialize_number, serialize_number_optional,
    Number,
};

mod vmid;
pub use vmid::VmId;
