use std::process;

use colored::Colorize;

pub trait Error {
    /// The errors name
    ///
    /// This indicates what type of error
    /// was thrown
    ///
    /// `>` `name: desc`
    ///  <br>
    /// `| additional context`
    /// <br>
    /// `tip`
    fn name(&self) -> &str;

    /// This is a short description
    /// of the error that was thrown
    ///
    /// `name: > desc`
    ///  <br>
    /// `| additional context`
    /// <br>
    /// `tip`
    fn desc(&self) -> String;

    // TODO: create helper for pointing to specific line in file

    /// This adds additional context to an error
    /// For example the line the error was thrown or
    /// an invalid part of a command
    ///
    /// In this vector, every entry represents a line.
    /// 
    /// `name: desc`
    ///  <br>
    /// `>` `| additional context`
    /// <br>
    /// `tip`
    fn additional_ctx(&self) -> Option<Vec<String>>;

    /// This gives the user an additional tip
    /// why the error was thrown.
    ///
    /// `name: desc`
    /// <br>
    /// `| additional context`
    /// <br>
    /// `>` `tip`
    fn tip(&self) -> Option<String>;
}

pub fn throw<E: Error>(err: E, stop_execution: bool) {
    let head = format!("{}: {}", err.name().red(), err.desc());

    let mut body = String::new();
    match err.additional_ctx() {
        Some(ctx) => ctx.iter().for_each(|str| {
            body.push_str("| ".bright_blue().to_string().as_str());
            body.push_str(&str);
            body.push_str("\n");
        }),
        None => (),
    }
    body.pop();

    let footer =
    match err.tip() {
        Some(tip) => Some(format!(
            "{} {}",
            "Tip:".blue(),
            tip
        )),
        None => None,
    };

    let mut msg = format!("{}\n{}\n", head, body);

    match footer {
        Some(tip) => msg.push_str(&tip),
        None => (),
    }

    println!("{}", msg);

    if stop_execution{
        process::exit(0)
    }
}
