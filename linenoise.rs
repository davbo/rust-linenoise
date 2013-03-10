// Crate linkage metadata
#[link(name = "linenoise", vers = "0.1", author = "davbo")];

// Make a library ("bin" is the default)
#[crate_type = "lib"];

struct linenoiseCompletions {
    len: libc::size_t,
    cvec: **libc::c_char,
}

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

pub fn set_multiline(enable: bool) {
    if enable {
        unsafe {
            linenoise::linenoiseSetMultiLine(1);
        }
    } else {
        unsafe {
            linenoise::linenoiseSetMultiLine(0);
        }
    }
}

pub fn clear_screen() {
    unsafe {
        linenoise::linenoiseClearScreen();
    }
}
