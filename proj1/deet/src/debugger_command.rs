pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Cont,
    Backtrace,
    Break(String),
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            }
            "c" | "cont" | "continue" => Some(DebuggerCommand::Cont),
            "bt" | "back" | "backtrace" => Some(DebuggerCommand::Backtrace),
            "break" | "b" => {
                let input = tokens[1].to_string();
                if input.starts_with("*") {
                    Some(DebuggerCommand::Break(input))
                } else {
                    None
                }
            }
            // Default case:
            _ => None,
        }
    }
}
