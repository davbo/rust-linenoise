pub type linenoiseCompletions = {
    len: libc::size_t,
    cvec: **libc::c_char,
};

// Build against the version of linenoise bundled with rust see rust/rt
// Hopefully in the future I can figure out how to include the updated version
extern mod linenoise {
    fn linenoise(prompt: *libc::c_char) -> *libc::c_char;
    fn linenoiseHistoryAdd(line: *libc::c_char) -> libc::c_int;
    fn linenoiseHistorySetMaxLen(length: libc::c_int) -> libc::c_int;
    fn linenoiseHistorySave(filename: *libc::c_char) -> libc::c_int;
    fn linenoiseHistoryLoad(filename: *libc::c_char) -> libc::c_int;
    fn linenoiseSetMultiLine(enabled: libc::c_int) -> libc::c_void;
    fn linenoiseClearScreen() -> libc::c_void;
}

pub fn init(prompt: &str) -> ~str {
    unsafe {
        let line = do str::as_c_str(prompt) |cstr| {
            linenoise::linenoise(cstr)
        };
    str::raw::from_c_str(line)
    }
}

pub fn history_add(line: &str) -> bool {
    let added = do str::as_c_str(line) |cstr| {
        unsafe {
            linenoise::linenoiseHistoryAdd(cstr)
        }
    };
    added as int == 1 // number of lines added perhaps?
}

pub fn history_save(filename: &str) -> bool {
    let saved = do str::as_c_str(filename) |cstr| {
        unsafe {
            linenoise::linenoiseHistorySave(cstr)
        }
    };
    saved as int == 0 // Seems to always be 0 if successful?
}

pub fn set_multiline(enable: bool) -> bool {
    if enable {
        unsafe {
            linenoise::linenoiseSetMultiLine(1);
            true
        }
    } else {
        unsafe {
            linenoise::linenoiseSetMultiLine(0);
            false
        }
    }
}

pub fn clear_screen() {
    unsafe {
        linenoise::linenoiseClearScreen();
    }
}
