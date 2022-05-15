use super::Rule;
use crate::either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
}

impl Arguments {
    pub fn jvm_arguments(&self) -> Vec<String> {
        Self::collect_args(&self.jvm)
    }

    pub fn game_arguments(&self) -> Vec<String> {
        Self::collect_args(&self.game)
    }

    fn collect_args(args: &[Argument]) -> Vec<String> {
        let mut arguments = vec![];

        for argument in args {
            match argument {
                Argument::Simple(simple_argument) => {
                    if check_skip_argument(simple_argument) {
                        continue;
                    }

                    arguments.push(simple_argument.to_string());
                }
                Argument::Complex(complex_arg) => {
                    if !complex_arg.check_use() {
                        continue;
                    }

                    let values = complex_arg
                        .value()
                        .into_iter()
                        .filter(|arg| !check_skip_argument(arg));

                    arguments.extend(values);
                }
            }
        }

        arguments
    }
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

impl ComplexArgument {
    pub fn check_use(&self) -> bool {
        for rule in &self.rules {
            if !rule.allows() {
                return false;
            }
        }

        true
    }

    pub fn value(&self) -> Vec<String> {
        match &self.value {
            Either::Left(val) => vec![val.clone()],
            Either::Right(x) => x.clone(),
        }
    }
}

fn check_skip_argument(arg: &str) -> bool {
    matches!(
        arg,
        "--clientId" | "--xuid" | "${clientid}" | "${auth_xuid}"
    )
}
