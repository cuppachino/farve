use owo_colors::OwoColorize;
use owo_print::farve;

farve!(silly, "silly 😋".white().bold(), 2);
farve!(debug, '🐛', 1);

farve!(info, "info".blue(), 2);

farve!(warn, "WARN".bright_yellow(), 1);
farve!(error, "ERROR".bright_red().underline(), 0);

fn main() {
    silly("Hello, world!");
    debug("We're gonna need a bigger net...");
    info("The weather is nice today.");
    warn("I almost couldn't, but I did it!");
    error("Something went so wrong!");
}
