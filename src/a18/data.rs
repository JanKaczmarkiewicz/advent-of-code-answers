use serde::Deserialize;

#[serde(untagged)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Data {
    Pair(Box<(Data, Data)>),
    Integer(u8),
}
