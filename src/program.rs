use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level program
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Program {
    pub functions: Vec<Function>,
}

/// A function (args default to empty vec, instrs default to empty vec)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function {
    pub name: String,

    // Bril: "Missing args is the same as an empty list."
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<Arg>,

    // Optional return type (None => no return type)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,

    // Sequence of instructions/labels. Missing == [].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instrs: Vec<Instr>,

    // Optional source-position metadata for whole function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<Pos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_end: Option<Pos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
}

/// Argument: name + type
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Arg {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: Type,
}

/// Two syntactic forms for types in Bril:
/// - primitive: "int", "bool", etc. (a single JSON string)
/// - parameterized: {"ptr": "int"} etc. (a JSON object with a single key)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Type {
    /// Primitive type expressed as a string, e.g. "int"
    Prim(String),

    /// Parameterized: map from param-name -> boxed Type
    /// E.g. {"ptr": "int"} deserializes to Param(map with key "ptr" -> Type::Prim("int"))
    Param(HashMap<String, Box<Type>>),
}

/// Literals: int, bool, maybe float
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    Number(i64),
    Bool(bool),
    Float(f64),
}

/// Source position struct used in many places
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

/// A label object
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Label {
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<Pos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_end: Option<Pos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum Instruction {
    // Arithmetic (int -> int)
    Add { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },
    Mul { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },
    Sub { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },
    Div { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },

    // Comparison (int -> bool)
    Eq { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },
    Lt { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },
    Gt { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },
    Le { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },
    Ge { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },

    // Logic (bool -> bool)
    Not { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 1] },
    And { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },
    Or  { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 2] },

    // Control flow
    Jmp { labels: [String; 1] },
    Br  { args: [String; 1], labels: [String; 2] },
    Call {
        funcs: [String; 1],
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        dest: Option<String>,
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        dest_type: Option<Type>,
    },
    Ret {
        #[serde(skip_serializing_if = "Option::is_none")]
        args: Option<[String; 1]>,
    },

    // Misc
    Id   { dest: String, #[serde(rename = "type")] dest_type: Type, args: [String; 1] },
    Print {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,
    },
    Nop {},

    // Constants
    Const {
        dest: String,
        #[serde(rename = "type")] dest_type: Type,
        value: Value,
    },
}

/// Top-level element in a function body: either an instruction (one of the kinds above) or a label
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Instr {
    Instruction(Instruction),
    Label(Label),
}
