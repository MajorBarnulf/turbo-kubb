use crate::{configuration::Config, Id};

#[derive(Debug, Clone)]
pub struct ParsingArgError {
    name: String,
    type_: ArgumentType,
    received: String,
}

#[derive(Debug, Clone)]
pub enum Error {
    ParsingArgError(ParsingArgError),
    String(String),
}

impl Error {
    pub fn new_parsing_arg(name: String, type_: ArgumentType, received: String) -> Self {
        Self::ParsingArgError(ParsingArgError {
            name,
            type_,
            received,
        })
    }
}

pub struct Context {
    //
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumArgumentType {
    name: String,
    variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentType {
    String,
    Number,
    Enum(EnumArgumentType),
    User,
}

impl ArgumentType {
    pub fn string() -> Self {
        Self::String
    }
    pub fn number() -> Self {
        Self::Number
    }
    pub fn enum_<S: ToString>(name: impl ToString, variants: impl IntoIterator<Item = S>) -> Self {
        let name = name.to_string();
        let variants = variants.into_iter().map(|s| s.to_string()).collect();
        Self::Enum(EnumArgumentType { name, variants })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentSignature {
    pub name: String,
    pub type_: ArgumentType,
}

impl ArgumentSignature {
    pub fn new(name: impl ToString, type_: ArgumentType) -> Self {
        let name = name.to_string();
        Self { name, type_ }
    }

    pub fn parse(&self, input: impl ToString) -> Option<ParsedArgument> {
        let name = &self.name;
        let input = input.to_string();
        let value = match &self.type_ {
            ArgumentType::String => ArgumentValue::new_string(input),
            ArgumentType::Number => {
                let value = input.parse().ok()?;
                ArgumentValue::new_number(value)
            }
            ArgumentType::Enum(enum_type) => {
                let variant = enum_type.variants.iter().find(|&v| v == &input)?;
                ArgumentValue::new_enum(variant)
            }
            ArgumentType::User => {
                let id: u64 = input.parse().ok()?; // TODO: better detection, may not be a raw id (ex: nickname/mention)
                ArgumentValue::new_user(id)
            }
        };
        Some(ParsedArgument::new(name, value))
    }
}

pub enum ArgumentValue {
    String(String),
    Number(i64),
    Enum(String),
    User(Id),
}

impl ArgumentValue {
    pub fn new_string(value: impl ToString) -> Self {
        let value = value.to_string();
        Self::String(value)
    }
    pub fn new_number(value: i64) -> Self {
        Self::Number(value)
    }
    pub fn new_enum(value: impl ToString) -> Self {
        let value = value.to_string();
        Self::Enum(value)
    }
    pub fn new_user(value: Id) -> Self {
        Self::User(value)
    }
}

pub struct ParsedArgument {
    pub name: String,
    pub value: ArgumentValue,
}

impl ParsedArgument {
    pub fn new(name: impl ToString, value: ArgumentValue) -> Self {
        let name = name.to_string();
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct Example {
    pub arguments: Vec<String>,
    pub description: String,
}

impl Example {
    pub fn new<S: ToString>(
        arguments: impl IntoIterator<Item = S>,
        description: impl ToString,
    ) -> Self {
        let arguments = arguments.into_iter().map(|s| s.to_string()).collect();
        let description = description.to_string();
        Self {
            arguments,
            description,
        }
    }
}

pub trait Command {
    fn name(&self) -> String;
    fn call(&self, ctx: &Context, args: Vec<ParsedArgument>) -> Result<(), Error>;
    fn arguments(&self) -> Vec<ArgumentSignature>;

    fn aliases(&self) -> Vec<String> {
        vec![]
    }

    fn description(&self) -> String {
        "[no description provided]".into()
    }

    fn examples(&self) -> Vec<(String, String)> {
        Vec::new()
    }
}

pub trait CommandFactory {
    fn make(config: &Config) -> Box<dyn Command>;
}

pub struct CommandHolder {
    inner: Box<dyn Command>,
}

impl CommandHolder {
    pub fn try_run(&self, args: Vec<String>) -> Result<(), Error> {
        todo!()
    }

    /// validate arguments.
    fn parse_args(&self, args: Vec<String>) -> Result<Vec<ParsedArgument>, Error> {
        let expected = self.inner.arguments();
        let mapped: Vec<_> = args
            .into_iter()
            .zip(expected.into_iter())
            .map(|(provided, expected)| {
                expected
                    .parse(&provided)
                    .ok_or_else(|| Error::new_parsing_arg(expected.name, expected.type_, provided))
            })
            .collect();

        if let Some(err) = mapped.iter().find_map(|res| res.as_ref().err().cloned()) {
            return Err(err);
        }

        let parsed = mapped
            .into_iter()
            .map(|item| item.expect("we checked for errors above"))
            .collect();

        Ok(parsed)
    }
}

impl From<Box<dyn Command>> for CommandHolder {
    fn from(inner: Box<dyn Command>) -> Self {
        Self { inner }
    }
}

pub mod commands;

/*

TODOs:
 - maek command :s
 - maek context
 - maek runner
 - maek handles

*/
