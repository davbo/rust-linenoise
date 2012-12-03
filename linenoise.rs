pub type linenoiseCompletions = {
    len: libc::size_t,
    cvec: **libc::c_char,
};

#[nolink]
extern mod linenoise {
    fn linenoise(prompt: *libc::c_char) -> *libc::c_char;
    fn linenoiseHistoryAdd(line: *libc::c_char) -> libc::c_int;
    fn linenoiseHistorySetMaxLen(line: libc::c_int) -> libc::c_int;
    fn linenoiseHistorySave(line: *libc::c_char) -> libc::c_int;
    fn linenoiseHistoryLoad(line: *libc::c_char) -> libc::c_int;
}

pub fn init(prompt: &str) -> ~str unsafe {
    let line = do str::as_c_str(prompt) |cstr| {
        linenoise::linenoise(cstr)
    };
    str::raw::from_c_str(line)
}

pub fn history_add(line: &str) -> bool unsafe {
    let added = do str::as_c_str(line) |cstr| {
        linenoise::linenoiseHistoryAdd(cstr)
    };
    added as int == 1 // number of lines added perhaps?
}

pub fn history_save(filename: &str) -> bool unsafe {
    let saved = do str::as_c_str(filename) |cstr| {
        linenoise::linenoiseHistorySave(cstr)
    };
    saved as int == 0 // Seems to always be 0 if successful?
}
