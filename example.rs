extern mod linenoise;


fn main() {
    linenoise::clear_screen();
    linenoise::set_multiline(true);
    loop {
        let line = linenoise::init("prompt> ");
        linenoise::history_add(line);
        linenoise::history_save("history.txt");
        if line == ~"exit" { break; }
        io::println(line);
    }
}
