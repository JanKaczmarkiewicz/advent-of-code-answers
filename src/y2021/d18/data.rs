use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Data {
    Pair(Box<(Data, Data)>),
    Integer(u8),
}
