use libc;
use rule;
use list;
use streams;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ashuffle_options {
    pub ruleset: list::list,
    pub queue_only: libc::c_uint,
    pub file_in: *mut libc::FILE,
    pub check_uris: bool,
    pub queue_buffer: libc::c_uint,
}

pub type rule_type = libc::c_uint;
pub const RULE_EXCLUDE: rule_type = 0;
/* Enum representing the various state of the parser */
pub type parse_state = libc::c_uint;
// expecting queue buffer value
pub const QUEUE_BUFFER: parse_state = 5;
// expecting song list input file
pub const IFILE: parse_state = 4;
// expecting "queue_only" int value
pub const QUEUE: parse_state = 3;
// expecting rule value (like "modest mouse")
pub const RULE_VALUE: parse_state = 2;
// expecting a rule matcher (like "artist")
pub const RULE: parse_state = 1;
// Ready for anything!
pub const NO_STATE: parse_state = 0;

pub static mut ARGS_QUEUE_BUFFER_NONE: libc::c_uint = 0i32 as libc::c_uint;

pub unsafe fn ashuffle_init(opts: *mut ashuffle_options) -> libc::c_int {
    (*opts).queue_only = 0i32 as libc::c_uint;
    (*opts).file_in = 0 as *mut libc::FILE;
    (*opts).check_uris = 0 != 1i32;
    list::list_init(&mut (*opts).ruleset);
    (*opts).queue_buffer = ARGS_QUEUE_BUFFER_NONE;
    return 0i32;
}
/* parse the options in to the 'ashuffle options' structure.
 * Returns 0 on success, -1 for failure. */

pub unsafe fn ashuffle_options(
    opts: *mut ashuffle_options,
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
) -> libc::c_int {
    /* State for the state machine */
    let mut state: parse_state = NO_STATE;
    let mut match_field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule = rule::song_rule {
        type_0: RULE_EXCLUDE,
        matchers: list::list {
            length: 0,
            list: 0 as *mut list::node,
        },
    };
    let mut type_flag: libc::c_int = -1i32;
    let mut i: libc::c_int = 1i32;
    while i < argc {
        let transable = state_can_trans(state);
        if transable {
            type_flag = rule_type_from_flag(*argv.offset(i as isize))
        }
        /* check we should print the help text */
        let mut help: [*const libc::c_char; 3] = [
            b"--help\x00" as *const u8 as *const libc::c_char,
            b"-h\x00" as *const u8 as *const libc::c_char,
            b"-?\x00" as *const u8 as *const libc::c_char,
        ];
        let mut nocheck: [*const libc::c_char; 2] = [
            b"--nocheck\x00" as *const u8 as *const libc::c_char,
            b"-n\x00" as *const u8 as *const libc::c_char,
        ];
        let mut queue: [*const libc::c_char; 1] =
            [b"--queue_buffer\x00" as *const u8 as *const libc::c_char];
        let mut only: [*const libc::c_char; 2] = [
            b"--only\x00" as *const u8 as *const libc::c_char,
            b"-o\x00" as *const u8 as *const libc::c_char,
        ];
        let mut file: [*const libc::c_char; 2] = [
            b"--file\x00" as *const u8 as *const libc::c_char,
            b"-f\x00" as *const u8 as *const libc::c_char,
        ];
        if check_flags(
            *argv.offset(i as isize),
            3i32 as libc::c_uint,
            help.as_mut_ptr(),
        ) {
            return -1i32;
        } else {
            if type_flag != -1i32 {
                flush_rule(state, opts, &mut rule);
                rule::rule_init(&mut rule, type_flag as rule_type);
                type_flag = -1i32;
                state = RULE
            } else if 0 != transable as libc::c_int && 0 != check_flags(
                *argv.offset(i as isize),
                2i32 as libc::c_uint,
                nocheck.as_mut_ptr(),
            ) as libc::c_int
            {
                flush_rule(state, opts, &mut rule);
                (*opts).check_uris = 0 != 0i32;
                state = NO_STATE
            } else if 0 != transable as libc::c_int && 0 != check_flags(
                *argv.offset(i as isize),
                1i32 as libc::c_uint,
                queue.as_mut_ptr(),
            ) as libc::c_int
            {
                flush_rule(state, opts, &mut rule);
                state = QUEUE_BUFFER
            } else if 0 != transable as libc::c_int
                && (*opts).queue_only == 0i32 as libc::c_uint
                && 0 != check_flags(
                    *argv.offset(i as isize),
                    2i32 as libc::c_uint,
                    only.as_mut_ptr(),
                ) as libc::c_int
            {
                flush_rule(state, opts, &mut rule);
                state = QUEUE
            } else if 0 != transable as libc::c_int
                && (*opts).file_in.is_null()
                && 0 != check_flags(
                    *argv.offset(i as isize),
                    2i32 as libc::c_uint,
                    file.as_mut_ptr(),
                ) as libc::c_int
            {
                flush_rule(state, opts, &mut rule);
                state = IFILE
            } else if state as libc::c_uint == RULE as libc::c_int as libc::c_uint {
                match_field = *argv.offset(i as isize);
                state = RULE_VALUE
            } else if state as libc::c_uint == RULE_VALUE as libc::c_int as libc::c_uint {
                rule::rule_add_criteria(&mut rule, match_field, *argv.offset(i as isize));
                match_field = 0 as *mut libc::c_char;
                state = RULE
            } else if state as libc::c_uint == QUEUE as libc::c_int as libc::c_uint {
                (*opts).queue_only =
                    libc::strtoul(*argv.offset(i as isize), 0 as *mut *mut libc::c_char, 10i32)
                        as libc::c_uint;
                /* Make sure we got a valid queue number */
                if streams::errno_val() == libc::EINVAL || streams::errno_val() == libc::ERANGE {
                    eprintln!(
                        "Error converting queue length to integer."
                    );
                    return -1i32;
                } else {
                    state = NO_STATE
                }
            } else if state as libc::c_uint == IFILE as libc::c_int as libc::c_uint {
                if check_flags(
                    *argv.offset(i as isize),
                    1i32 as libc::c_uint,
                    b"-\x00" as *const u8 as *const libc::c_char as *mut *const libc::c_char,
                ) {
                    (*opts).file_in = streams::stdin_file();
                } else {
                    (*opts).file_in = libc::fopen(
                        *argv.offset(i as isize),
                        b"r\x00" as *const u8 as *const libc::c_char,
                    )
                }
                state = NO_STATE
            } else if state as libc::c_uint == QUEUE_BUFFER as libc::c_int as libc::c_uint {
                (*opts).queue_buffer =
                    libc::strtoul(*argv.offset(i as isize), 0 as *mut *mut libc::c_char, 10i32)
                        as libc::c_uint;
                if streams::errno_val() == libc::EINVAL || streams::errno_val() == libc::ERANGE {
                    eprintln!(
                        "Error converting queue buffer length to integer."
                    );
                    return -1i32;
                } else {
                    state = NO_STATE
                }
            } else {
                eprintln!(
                    "Invalid option: {:?}.",
                    *argv.offset(i as isize),
                );
                return -1i32;
            }
            i += 1
        }
    }
    if state as libc::c_uint == RULE_VALUE as libc::c_int as libc::c_uint {
        eprintln!(
            "No value supplied for match \'{:?}\'.",
            match_field,
        );
        return -1i32;
    } else {
        /* if we're provisioning a rule right now, flush it */
        flush_rule(state, opts, &mut rule);
        return 0i32;
    };
}

/* if we're in a correct state, then add the rule to the
 * ruleset in the list of options */
pub unsafe fn flush_rule(
    state: parse_state,
    opts: *mut ashuffle_options,
    rule: *mut rule::song_rule,
) -> libc::c_int {
    if state as libc::c_uint == RULE as libc::c_int as libc::c_uint
        && (*rule).matchers.length > 0i32 as libc::c_uint
    {
        /* add the rule to the ruleset */
        list::list_push(
            &mut (*opts).ruleset,
            list::node_from(
                rule as *const libc::c_void,
                ::std::mem::size_of::<rule::song_rule>(),
            ),
        );
    }
    return 0i32;
}
/* check and see if 'to_check' matches any of 'count' given
 * arguments */

pub unsafe fn check_flags(
    to_check: *const libc::c_char,
    count: libc::c_uint,
    items: *mut *const libc::c_char,
) -> bool {
    let mut out: bool = 0 != 0i32;
    let mut i: libc::c_int = 0i32;
    while (i as libc::c_uint) < count {
        if libc::strcasecmp(to_check, *items.offset(i as isize)) == 0i32 {
            /* don't break, we need to process the arguments */
            out = 0 != 1i32
        }
        i += 1
    }
    return out;
}
/* get the enum rule_type type from the option if possible.
 * Otherwise, return -1 */

pub unsafe fn rule_type_from_flag(option: *mut libc::c_char) -> libc::c_int {
    let mut items: [*const libc::c_char; 2] = [
        b"--exclude\x00" as *const u8 as *const libc::c_char,
        b"-e\x00" as *const u8 as *const libc::c_char,
    ];
    if check_flags(option, 2i32 as libc::c_uint, items.as_mut_ptr()) {
        return RULE_EXCLUDE as libc::c_int;
    } else {
        return -1i32;
    };
}
/* check and see if we can transition to a new top-level
 * state from our current state */

pub unsafe fn state_can_trans(state: parse_state) -> bool {
    if state as libc::c_uint == NO_STATE as libc::c_int as libc::c_uint
        || state as libc::c_uint == RULE as libc::c_int as libc::c_uint
    {
        return 0 != 1i32;
    } else {
        return 0 != 0i32;
    };
}

pub unsafe fn ashuffle_help() {
    eprintln!("usage: ashuffle -h -n {{ ..opts..}} [-e PATTERN ...] [-o NUMBER] [-f FILENAME]
    
    Optional Arguments:
       -e,--exclude   Specify things to remove from shuffle (think blacklist).
       -o,--only      Instead of continuously adding songs, just add \'NUMBER\'
                      songs and then exit.
       -h,-?,--help   Display this help message.
       -f,--file      Use MPD URI\'s found in \'file\' instead of using the entire MPD
                      library. You can supply `-` instead of a filename to retrive
                      URI\'s from standard in. This can be used to pipe song URI\'s
                      from another program into ashuffle.
       -n,--nocheck   When reading URIs from a file, don\'t check to ensure that
                      the URIs match the given exclude rules. This option is most
                      helpful when shuffling songs with -f, that aren\'t in the
                      MPD library.
       --queue_buffer Specify to keep a buffer of `n` songs queued after the
                      currently playing song. This is to support MPD features
                      like crossfade that don\'t work if there are no more
                      songs in the queue.
      
      See included `readme.md` file for PATTERN syntax.");
}
