use serde;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Program {
    pub functions: Vec<Function>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Arg>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_type: Option<Type>,
    pub instrs: Vec<Instrs>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Arg {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: Type
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Int,
    Bool,
    Float,
    Char,
    None,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Value {
    Number(i64),
    Bool(bool),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Instruction {
    pub op: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_type: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funcs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Label {
    pub label: String
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Instrs {
    Instruction(Instruction),
    Label(Label)
}
