use crate::{
    command::{ArgumentSignature, Command, CommandFactory, Context, Error, ParsedArgument},
    configuration::Config,
};

pub struct HelpCommand {
    //
}

impl HelpCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for HelpCommand {
    fn name(&self) -> String {
        "help".into()
    }
    fn aliases(&self) -> Vec<String> {
        vec!["h".into(), "?".into()]
    }
    fn call(&self, ctx: &Context, args: Vec<ParsedArgument>) -> Result<(), Error> {
        Ok(())
    }
    fn arguments(&self) -> Vec<ArgumentSignature> {
        todo!()
    }
}

pub struct HelpCommandFactory {
    //
}

impl CommandFactory for HelpCommandFactory {
    fn make(config: &Config) -> Box<dyn Command> {
        Box::new(HelpCommand::new())
    }
}
