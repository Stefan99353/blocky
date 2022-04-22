use super::Rule;
use crate::either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Simple(String),
    Complex(ComplexArgument),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComplexArgument {
    #[serde(alias = "compatibilityRules")]
    pub rules: Vec<Rule>,
    pub value: Either<String, Vec<String>>,
}
