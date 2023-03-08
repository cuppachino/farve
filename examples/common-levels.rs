use farve::{efarve, farve};
use owo_colors::OwoColorize;

farve!(silly, "silly 😋".white().bold(), 2);
farve!(debug, '🐛', 0);
farve!(info, "info".blue(), 1);

efarve!(warn, "WARN".bright_yellow(), 2);
efarve!(error, "ERROR".bright_red().underline(), 0);

fn main() {
    silly("Hello, world!");
    debug("We're gonna need a bigger net...");
    info("The weather is nice today.");
    warn("I almost couldn't, but I did it!");
    error("Something went so wrong!");
}
