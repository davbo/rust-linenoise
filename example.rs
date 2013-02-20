mod linenoise;


fn main() {
    linenoise::set_multiline(true);
    linenoise::clear_screen();
    loop {
        let line = linenoise::init("prompt> ");
        linenoise::history_add(line);
        linenoise::history_save("history.txt");
        if line == ~"exit" { break; }
        io::println(line);
    }
}
