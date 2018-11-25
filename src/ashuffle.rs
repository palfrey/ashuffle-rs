use libc;
use args;
use mpd;
use list;
use streams;
use std::process;
use shuffle;
use rule;
use getpass;
use std::ffi::CStr;

pub unsafe fn mpd_perror(mpd: *mut mpd::mpd_connection) {
    assert!(mpd::mpd_connection_get_error(mpd) != mpd::MPD_ERROR_SUCCESS,
        "must be an error present");
    let err_msg = CStr::from_ptr(mpd::mpd_connection_get_error_message(mpd));
    eprintln!(
        "MPD error: {}",
        err_msg.to_str().unwrap()
    );
    process::exit(1);
}

pub unsafe fn mpd_perror_if_error(mpd: *mut mpd::mpd_connection) {
    if mpd::mpd_connection_get_error(mpd) as libc::c_uint
        != mpd::MPD_ERROR_SUCCESS as libc::c_int as libc::c_uint
    {
        mpd_perror(mpd);
    };
}

/* check wheter a song is allowed by the given ruleset */
pub unsafe fn ruleset_accepts_song(
    ruleset: *mut list::list,
    song: *mut mpd::mpd_song,
) -> bool {
    let mut i = 0;
    while i < (*ruleset).length {
        let rule = list::list_at(ruleset, i) as *mut rule::song_rule;
        if !rule::rule_match(rule, song) {
            return true;
        } else {
            i += 1;
        }
    }
    return false;
}

pub unsafe fn ruleset_accepts_uri(
    mpd: *mut mpd::mpd_connection,
    ruleset: *mut list::list,
    uri: *mut libc::c_char,
) -> bool {
    let mut accepted: bool = false;
    /* search for the song URI in MPD */
    mpd::mpd_search_db_songs(mpd, true);
    mpd::mpd_search_add_uri_constraint(mpd, mpd::MPD_OPERATOR_DEFAULT, uri);
    if mpd::mpd_search_commit(mpd) as libc::c_int != 1i32 {
        mpd_perror(mpd);
    }
    let song = mpd::mpd_recv_song(mpd);
    mpd_perror_if_error(mpd);
    if !song.is_null() {
        if ruleset_accepts_song(ruleset, song) {
            accepted = true;
        }
        /* free the song we got from MPD */
        mpd::mpd_song_free(song);
        /* even though we're searching for a single song, libmpdclient
         * still acts like we're reading a song list. We read an aditional
         * element to convince MPD this is the end of the song list. */
        mpd::mpd_recv_song(mpd);
    } else {
        eprintln!(
            "Song uri \'{:?}\' not found.",
            uri,
        );
    }
    return accepted;
}
/* build the list of songs to shuffle from using
 * the supplied file. */

pub unsafe fn build_songs_file(
    mpd: *mut mpd::mpd_connection,
    ruleset: *mut list::list,
    input: *mut libc::FILE,
    songs: *mut shuffle::shuffle_chain,
    check: bool,
) -> libc::c_int {
    let mut uri: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ignored: usize = 0;;
    let mut length = libc::getline(&mut uri, &mut ignored, input);
    while 0 == libc::feof(input) && 0 == libc::ferror(input) {
        if length < 1 {
            eprintln!(
                "invalid URI in input stream",
            );
            process::exit(1);
        } else {
            /* if this line has terminating newline attached, set it
             * to null and decrement the length (effectively removing
             * the newline). */
            if *uri.offset((length - 1) as isize) as libc::c_int == '\n' as i32 {
                *uri.offset((length - 1) as isize) =
                    '\u{0}' as i32 as libc::c_char;
                length -= 1
            }
            if 0 != check as libc::c_int
                && 0 != ruleset_accepts_uri(mpd, ruleset, uri) as libc::c_int
                || !check
            {
                shuffle::shuffle_add(
                    songs,
                    uri as *const libc::c_void,
                    (length + 1) as usize,
                );
            }
            /* free the temporary memory */
            libc::free(uri as *mut libc::c_void);
            uri = 0 as *mut libc::c_char;
            /* get the next uri */
            length = libc::getline(&mut uri, &mut ignored, input)
        }
    }
    libc::fclose(input);
    return 0i32;
}
/* build the list of songs to shuffle from using MPD */

pub unsafe fn build_songs_mpd(
    mpd: *mut mpd::mpd_connection,
    ruleset: *mut list::list,
    songs: *mut shuffle::shuffle_chain,
) -> libc::c_int {
    /* ask for a list of songs */
    if mpd::mpd_send_list_all_meta(mpd, 0 as *const libc::c_char) as libc::c_int != 1i32 {
        mpd_perror(mpd);
    }
    /* parse out the pairs */
    let mut song = mpd::mpd_recv_song(mpd);
    mpd_perror_if_error(mpd);
    while !song.is_null() {
        /* if this song is allowed, add it to the list */
        if ruleset_accepts_song(ruleset, song) {
            shuffle::shuffle_add(
                songs,
                mpd::mpd_song_get_uri(song) as *const libc::c_void,
                libc::strlen(mpd::mpd_song_get_uri(song)).wrapping_add(1),
            );
        }
        /* free the current song */
        mpd::mpd_song_free(song);
        /* get the next song from the list */
        song = mpd::mpd_recv_song(mpd)
    }
    return 0i32;
}
/* Append a random song from the given list of
 * songs to the queue */

pub unsafe fn queue_random_song(
    mpd: *mut mpd::mpd_connection,
    songs: *mut shuffle::shuffle_chain,
) {
    if mpd::mpd_run_add(mpd, shuffle::shuffle_pick(songs) as *const libc::c_char) as libc::c_int != 1i32 {
        mpd_perror(mpd);
    };
}

pub unsafe fn try_first(
    mpd: *mut mpd::mpd_connection,
    songs: *mut shuffle::shuffle_chain,
) -> libc::c_int {
    let status = mpd::mpd_run_status(mpd);
    if status.is_null() {
        libc::puts(mpd::mpd_connection_get_error_message(mpd));
        return -1;
    } else {
        if mpd::mpd_status_get_state(status) as libc::c_uint
            != mpd::MPD_STATE_PLAY as libc::c_int as libc::c_uint
        {
            queue_random_song(mpd, songs);
            if mpd::mpd_run_play_pos(mpd, mpd::mpd_status_get_queue_length(status)) as libc::c_int != 1i32 {
                mpd_perror(mpd);
            }
        }
        mpd::mpd_status_free(status);
        return 0;
    };
}

pub unsafe fn try_enqueue(
    mpd: *mut mpd::mpd_connection,
    songs: *mut shuffle::shuffle_chain,
    options: *mut args::ashuffle_options,
) -> libc::c_int {
    let status = mpd::mpd_run_status(mpd);
    /* Check for error while fetching the status */
    if status.is_null() {
        /* print the error message from the server */
        libc::puts(mpd::mpd_connection_get_error_message(mpd));
        return -1i32;
    } else {
        let past_last: bool = mpd::mpd_status_get_song_pos(status) == -1i32;
        let queue_empty: bool = mpd::mpd_status_get_queue_length(status) == 0i32 as libc::c_uint;
        let mut queue_songs_remaining: libc::c_uint = 0i32 as libc::c_uint;
        if !past_last {
            /* +1 on song_pos because it is zero-indexed */
            queue_songs_remaining = mpd::mpd_status_get_queue_length(status)
                .wrapping_sub((mpd::mpd_status_get_song_pos(status) + 1i32) as libc::c_uint)
        }
        let mut should_add: bool = false;
        /* Always add if we've progressed past the last song. Even if
         * --queue_buffer, we should have already enqueued a song by now. */
        if past_last {
            should_add = true
        } else if (*options).queue_buffer != args::ARGS_QUEUE_BUFFER_NONE
            && queue_songs_remaining < (*options).queue_buffer
        {
            should_add = true
        } else if queue_empty {
            should_add = true
        }
        /* Add another song to the list and restart the player */
        if should_add {
            if (*options).queue_buffer != args::ARGS_QUEUE_BUFFER_NONE {
                let mut i: libc::c_uint = queue_songs_remaining;
                while i < (*options).queue_buffer {
                    queue_random_song(mpd, songs);
                    i = i.wrapping_add(1)
                }
            } else {
                queue_random_song(mpd, songs);
            }
        }
        /* If we added a song, and the player was not already playing, we need
         * to re-start it. */
        if 0 != should_add as libc::c_int
            && (0 != past_last as libc::c_int || 0 != queue_empty as libc::c_int)
        {
            /* Since the 'status' was before we added our song, and the queue
             * is zero-indexed, the length will be the position of the song we
             * just added. Play that song */
            if mpd::mpd_run_play_pos(mpd, mpd::mpd_status_get_queue_length(status)) as libc::c_int != 1i32 {
                mpd_perror(mpd);
            }
            /* Immediately pause playback if mpd single mode is on */
            if mpd::mpd_status_get_single(status) {
                if mpd::mpd_run_pause(mpd, true) as libc::c_int != 1i32 {
                    mpd_perror(mpd);
                }
            }
        }
        /* free the status we retrieved */
        mpd::mpd_status_free(status);
        return 0i32;
    };
}
/* Keep adding songs when the queue runs out */

pub unsafe fn shuffle_idle(
    mpd: *mut mpd::mpd_connection,
    songs: *mut shuffle::shuffle_chain,
    options: *mut args::ashuffle_options,
) -> libc::c_int {
    assert!(mpd::MPD_IDLE_QUEUE as libc::c_int == mpd::MPD_IDLE_PLAYLIST as libc::c_int,
        "QUEUE Now different signal.");
    let idle_mask: libc::c_int = mpd::MPD_IDLE_DATABASE as libc::c_int
        | mpd::MPD_IDLE_QUEUE as libc::c_int
        | mpd::MPD_IDLE_PLAYER as libc::c_int;
    if try_first(mpd, songs) != 0i32 {
        return -1i32;
    } else if try_enqueue(mpd, songs, options) != 0i32 {
        return -1i32;
    } else {
        loop {
            /* wait till the player state changes */
            let event = mpd::mpd_run_idle_mask(mpd, idle_mask as mpd::mpd_idle);
            mpd_perror_if_error(mpd);
            let idle_db: bool =
                0 != event as libc::c_uint & mpd::MPD_IDLE_DATABASE as libc::c_int as libc::c_uint;
            let idle_queue: bool =
                0 != event as libc::c_uint & mpd::MPD_IDLE_QUEUE as libc::c_int as libc::c_uint;
            let idle_player: bool =
                0 != event as libc::c_uint & mpd::MPD_IDLE_PLAYER as libc::c_int as libc::c_uint;
            if idle_db {
                shuffle::shuffle_free(songs);
                build_songs_mpd(mpd, &mut (*options).ruleset, songs);
                libc::printf(
                    b"Picking random songs out of a pool of %u.\n\x00" as *const u8
                        as *const libc::c_char,
                    shuffle::shuffle_length(songs),
                );
            } else {
                if !(0 != idle_queue as libc::c_int || 0 != idle_player as libc::c_int) {
                    continue;
                }
                if !(try_enqueue(mpd, songs, options) != 0i32) {
                    continue;
                }
                return -1i32;
            }
        }
    };
}

pub unsafe fn get_mpd_password(mpd: *mut mpd::mpd_connection) {
    /* keep looping till we get a bad error, or we get a good password. */
    loop {
        let pass: *mut libc::c_char = getpass::as_getpass(
            streams::stdin_file(),
            streams::stdout_file(),
            b"mpd password: \x00" as *const u8 as *const libc::c_char,
        );
        mpd::mpd_run_password(mpd, pass);
        let err = mpd::mpd_connection_get_error(mpd);
        if err as libc::c_uint == mpd::MPD_ERROR_SUCCESS as libc::c_int as libc::c_uint {
            return;
        } else if err as libc::c_uint == mpd::MPD_ERROR_SERVER as libc::c_int as libc::c_uint {
            let server_err = mpd::mpd_connection_get_server_error(mpd);
            if server_err as libc::c_int == mpd::MPD_SERVER_ERROR_PASSWORD as libc::c_int {
                mpd::mpd_connection_clear_error(mpd);
                eprintln!(
                    "incorrect password."
                );
            } else {
                mpd_perror(mpd);
            }
        } else {
            mpd_perror(mpd);
        }
    }
}
/* If a password is required, "password" is used if not null, otherwise
 * a password is obtained from stdin. */

pub unsafe fn check_mpd_password(
    mpd: *mut mpd::mpd_connection,
    password: *mut libc::c_char,
) {
    let stats: *mut mpd::mpd_stats = mpd::mpd_run_stats(mpd);
    let err = mpd::mpd_connection_get_error(mpd);
    if err as libc::c_uint == mpd::MPD_ERROR_SUCCESS as libc::c_int as libc::c_uint {
        mpd::mpd_stats_free(stats);
        return;
    } else {
        if err as libc::c_uint == mpd::MPD_ERROR_SERVER as libc::c_int as libc::c_uint {
            let server_err = mpd::mpd_connection_get_server_error(mpd);
            if server_err as libc::c_int == mpd::MPD_SERVER_ERROR_PERMISSION as libc::c_int {
                mpd::mpd_connection_clear_error(mpd);
                if !password.is_null() {
                    mpd::mpd_run_password(mpd, password);
                    mpd_perror_if_error(mpd);
                } else {
                    get_mpd_password(mpd);
                }
                return;
            }
        }
        /* if the problem wasn't a simple password issue abort */
        mpd_perror(mpd);
        return;
    };
}

pub unsafe fn parse_mpd_host(
    mpd_host: *mut libc::c_char,
    o_mpd_host: *mut mpd::mpd_host,
) {
    let at: *mut libc::c_char = libc::strrchr(mpd_host, '@' as i32);
    if !at.is_null() {
        (*o_mpd_host).host = &mut *at.offset(1isize) as *mut libc::c_char;
        (*o_mpd_host).password = mpd_host;
        *at = '\u{0}' as i32 as libc::c_char
    } else {
        (*o_mpd_host).host = mpd_host;
        (*o_mpd_host).password = 0 as *mut libc::c_char
    };
}

pub unsafe fn main_0(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    /* attempt to parse out options given on the command line */
    let mut options = args::ashuffle_options {
        ruleset: list::list {
            length: 0,
            list: 0 as *mut list::node,
        },
        queue_only: 0,
        file_in: 0 as *mut libc::FILE,
        check_uris: false,
        queue_buffer: 0,
    };
    args::ashuffle_init(&mut options);
    let status: libc::c_int = args::ashuffle_options(&mut options, argc, argv);
    if status != 0 {
        args::ashuffle_help();
        return status;
    } else {
        /* attempt to connect to MPD */
        /* Attempt to use MPD_HOST variable if available.
         * Otherwise use 'localhost'. */
        let mpd_host_raw: *mut libc::c_char =
            (if !libc::getenv(b"MPD_HOST\x00" as *const u8 as *const libc::c_char).is_null() {
                libc::getenv(b"MPD_HOST\x00" as *const u8 as *const libc::c_char)
            } else {
                b"localhost\x00" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        let mut mpd_host = mpd::mpd_host {
            host: 0 as *mut libc::c_char,
            password: 0 as *mut libc::c_char,
        };
        parse_mpd_host(mpd_host_raw, &mut mpd_host);
        /* Same thing for the port, use the environment defined port
         * or the default port */
        let mpd_port: libc::c_uint =
            (if !libc::getenv(b"MPD_PORT\x00" as *const u8 as *const libc::c_char).is_null() {
                libc::atoi(libc::getenv(b"MPD_PORT\x00" as *const u8 as *const libc::c_char))
            } else {
                6600i32
            }) as libc::c_uint;
        /* Create a new connection to mpd */
        let mpd = mpd::mpd_connection_new(mpd_host.host, mpd_port, 25000i32 as libc::c_uint);
        if mpd.is_null() {
            eprintln!(
                "Could not connect due to lack of memory."
            );
            return 1;
        } else if mpd::mpd_connection_get_error(mpd) as libc::c_uint
            != mpd::MPD_ERROR_SUCCESS as libc::c_int as libc::c_uint
        {
            eprintln!("Could not connect to {}:{}",
                mpd_host.hostname().unwrap(),
                mpd_port,
            );
            return 1;
        } else {
            check_mpd_password(mpd, mpd_host.password);
            let mut songs = shuffle::shuffle_chain {
                max_window: 0,
                window: list::list {
                    length: 0,
                    list: 0 as *mut list::node,
                },
                pool: list::list {
                    length: 0,
                    list: 0 as *mut list::node,
                },
            };
            shuffle::shuffle_init(&mut songs, 7i32 as libc::c_uint);
            /* build the list of songs to shuffle through */
            if !options.file_in.is_null() {
                build_songs_file(
                    mpd,
                    &mut options.ruleset,
                    options.file_in,
                    &mut songs,
                    options.check_uris,
                );
            } else {
                build_songs_mpd(mpd, &mut options.ruleset, &mut songs);
            }
            if shuffle::shuffle_length(&mut songs) == 0 {
                println!("Song pool is empty.");
                return -1;
            } else {
                libc::printf(
                    b"Picking random songs out of a pool of %u.\n\x00" as *const u8
                        as *const libc::c_char,
                    shuffle::shuffle_length(&mut songs),
                );
                /* Seed the random number generator */
                libc::srand(libc::time(0 as *mut libc::time_t) as libc::c_uint);
                /* do the main action */
                if 0 != options.queue_only {
                    let mut i: libc::c_uint = 0i32 as libc::c_uint;
                    while i < options.queue_only {
                        queue_random_song(mpd, &mut songs);
                        i = i.wrapping_add(1)
                    }
                    libc::printf(
                        b"Added %u songs.\n\x00" as *const u8 as *const libc::c_char,
                        options.queue_only,
                    );
                } else {
                    shuffle_idle(mpd, &mut songs, &mut options);
                }
                /* dispose of the rules used to build the song-list */
                let mut i_0: libc::c_uint = 0i32 as libc::c_uint;
                while i_0 < options.ruleset.length {
                    rule::rule_free(list::list_at(&mut options.ruleset, i_0) as *mut rule::song_rule);
                    i_0 = i_0.wrapping_add(1)
                }
                list::list_free(&mut options.ruleset);
                /* free-up our songs */
                shuffle::shuffle_free(&mut songs);
                mpd::mpd_connection_free(mpd);
                return 0;
            }
        }
    };
}
