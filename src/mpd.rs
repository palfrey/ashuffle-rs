use libc;
use std::ffi::CStr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpd_host {
    pub host: *mut libc::c_char,
    pub password: *mut libc::c_char,
}

impl mpd_host {
    pub fn hostname(self) -> Result<&'static str, ::std::str::Utf8Error> {
        let c_str = unsafe { CStr::from_ptr(self.host) };
        c_str.to_str()
    }
}

extern "C" {
    pub type mpd_connection;
    pub type mpd_song;
    pub type mpd_stats;
    pub type mpd_status;
    pub type node;
    #[no_mangle]
    pub fn mpd_connection_new(
        host: *const libc::c_char,
        port: libc::c_uint,
        timeout_ms: libc::c_uint,
    ) -> *mut mpd_connection;
    #[no_mangle]
    pub fn mpd_connection_free(connection: *mut mpd_connection);
    #[no_mangle]
    pub fn mpd_connection_get_error(connection: *const mpd_connection) -> mpd_error;
    #[no_mangle]
    pub fn mpd_connection_get_error_message(connection: *const mpd_connection) -> *const libc::c_char;
    #[no_mangle]
    pub fn mpd_connection_get_server_error(connection: *const mpd_connection) -> mpd_server_error;
    #[no_mangle]
    pub fn mpd_connection_clear_error(connection: *mut mpd_connection) -> bool;
    #[no_mangle]
    pub fn mpd_send_list_all_meta(connection: *mut mpd_connection, path: *const libc::c_char) -> bool;
    #[no_mangle]
    pub fn mpd_song_free(song: *mut mpd_song);
    #[no_mangle]
    pub fn mpd_song_get_uri(song: *const mpd_song) -> *const libc::c_char;
    #[no_mangle]
    pub fn mpd_recv_song(connection: *mut mpd_connection) -> *mut mpd_song;
    #[no_mangle]
    pub fn mpd_run_idle_mask(connection: *mut mpd_connection, mask: mpd_idle) -> mpd_idle;
    #[no_mangle]
    pub fn mpd_run_password(connection: *mut mpd_connection, password: *const libc::c_char) -> bool;
    #[no_mangle]
    pub fn mpd_run_play_pos(connection: *mut mpd_connection, song_pos: libc::c_uint) -> bool;
    #[no_mangle]
    pub fn mpd_run_pause(connection: *mut mpd_connection, mode: bool) -> bool;
    #[no_mangle]
    pub fn mpd_run_add(connection: *mut mpd_connection, uri: *const libc::c_char) -> bool;
    #[no_mangle]
    pub fn mpd_search_db_songs(connection: *mut mpd_connection, exact: bool) -> bool;
    #[no_mangle]
    pub fn mpd_search_add_uri_constraint(
        connection: *mut mpd_connection,
        oper: mpd_operator,
        value: *const libc::c_char,
    ) -> bool;
    #[no_mangle]
    pub fn mpd_search_commit(connection: *mut mpd_connection) -> bool;
    #[no_mangle]
    pub fn mpd_run_stats(connection: *mut mpd_connection) -> *mut mpd_stats;
    #[no_mangle]
    pub fn mpd_stats_free(stats: *mut mpd_stats);
    #[no_mangle]
    pub fn mpd_run_status(connection: *mut mpd_connection) -> *mut mpd_status;
    #[no_mangle]
    pub fn mpd_status_free(status: *mut mpd_status);
    #[no_mangle]
    pub fn mpd_status_get_single(status: *const mpd_status) -> bool;
    #[no_mangle]
    pub fn mpd_status_get_queue_length(status: *const mpd_status) -> libc::c_uint;
    #[no_mangle]
    pub fn mpd_status_get_state(status: *const mpd_status) -> mpd_state;
    #[no_mangle]
    pub fn mpd_status_get_song_pos(status: *const mpd_status) -> libc::c_int;
    #[no_mangle]
    pub fn mpd_tag_name_iparse(name: *const libc::c_char) -> mpd_tag_type;
    #[no_mangle]
    pub fn mpd_song_get_tag(
        song: *const mpd_song,
        type_0: mpd_tag_type,
        idx: libc::c_uint,
    ) -> *const libc::c_char;
}

pub type mpd_tag_type = libc::c_int;
pub const MPD_TAG_COUNT: mpd_tag_type = 19;
pub const MPD_TAG_ALBUM_ARTIST_SORT: mpd_tag_type = 18;
pub const MPD_TAG_ARTIST_SORT: mpd_tag_type = 17;
pub const MPD_TAG_MUSICBRAINZ_RELEASETRACKID: mpd_tag_type = 16;
pub const MPD_TAG_MUSICBRAINZ_TRACKID: mpd_tag_type = 15;
pub const MPD_TAG_MUSICBRAINZ_ALBUMARTISTID: mpd_tag_type = 14;
pub const MPD_TAG_MUSICBRAINZ_ALBUMID: mpd_tag_type = 13;
pub const MPD_TAG_MUSICBRAINZ_ARTISTID: mpd_tag_type = 12;
pub const MPD_TAG_DISC: mpd_tag_type = 11;
pub const MPD_TAG_COMMENT: mpd_tag_type = 10;
pub const MPD_TAG_PERFORMER: mpd_tag_type = 9;
pub const MPD_TAG_COMPOSER: mpd_tag_type = 8;
pub const MPD_TAG_DATE: mpd_tag_type = 7;
pub const MPD_TAG_GENRE: mpd_tag_type = 6;
pub const MPD_TAG_NAME: mpd_tag_type = 5;
pub const MPD_TAG_TRACK: mpd_tag_type = 4;
pub const MPD_TAG_TITLE: mpd_tag_type = 3;
pub const MPD_TAG_ALBUM_ARTIST: mpd_tag_type = 2;
pub const MPD_TAG_ALBUM: mpd_tag_type = 1;
pub const MPD_TAG_ARTIST: mpd_tag_type = 0;
pub const MPD_TAG_UNKNOWN: mpd_tag_type = -1;
pub type mpd_idle = libc::c_uint;
pub const MPD_IDLE_MESSAGE: mpd_idle = 1024;
pub const MPD_IDLE_SUBSCRIPTION: mpd_idle = 512;
pub const MPD_IDLE_STICKER: mpd_idle = 256;
pub const MPD_IDLE_UPDATE: mpd_idle = 128;
pub const MPD_IDLE_OPTIONS: mpd_idle = 64;
pub const MPD_IDLE_OUTPUT: mpd_idle = 32;
pub const MPD_IDLE_MIXER: mpd_idle = 16;
pub const MPD_IDLE_PLAYER: mpd_idle = 8;
pub const MPD_IDLE_PLAYLIST: mpd_idle = 4;
pub const MPD_IDLE_QUEUE: mpd_idle = 4;
pub const MPD_IDLE_STORED_PLAYLIST: mpd_idle = 2;
pub const MPD_IDLE_DATABASE: mpd_idle = 1;
pub type mpd_operator = libc::c_uint;
pub const MPD_OPERATOR_DEFAULT: mpd_operator = 0;
pub type mpd_state = libc::c_uint;
pub const MPD_STATE_PAUSE: mpd_state = 3;
pub const MPD_STATE_PLAY: mpd_state = 2;
pub const MPD_STATE_STOP: mpd_state = 1;
pub const MPD_STATE_UNKNOWN: mpd_state = 0;
pub type mpd_server_error = libc::c_int;
pub const MPD_SERVER_ERROR_EXIST: mpd_server_error = 56;
pub const MPD_SERVER_ERROR_PLAYER_SYNC: mpd_server_error = 55;
pub const MPD_SERVER_ERROR_UPDATE_ALREADY: mpd_server_error = 54;
pub const MPD_SERVER_ERROR_PLAYLIST_LOAD: mpd_server_error = 53;
pub const MPD_SERVER_ERROR_SYSTEM: mpd_server_error = 52;
pub const MPD_SERVER_ERROR_PLAYLIST_MAX: mpd_server_error = 51;
pub const MPD_SERVER_ERROR_NO_EXIST: mpd_server_error = 50;
pub const MPD_SERVER_ERROR_UNKNOWN_CMD: mpd_server_error = 5;
pub const MPD_SERVER_ERROR_PERMISSION: mpd_server_error = 4;
pub const MPD_SERVER_ERROR_PASSWORD: mpd_server_error = 3;
pub const MPD_SERVER_ERROR_ARG: mpd_server_error = 2;
pub const MPD_SERVER_ERROR_NOT_LIST: mpd_server_error = 1;
pub const MPD_SERVER_ERROR_UNK: mpd_server_error = -1;
pub type mpd_error = libc::c_uint;
pub const MPD_ERROR_SERVER: mpd_error = 9;
pub const MPD_ERROR_CLOSED: mpd_error = 8;
pub const MPD_ERROR_MALFORMED: mpd_error = 7;
pub const MPD_ERROR_RESOLVER: mpd_error = 6;
pub const MPD_ERROR_SYSTEM: mpd_error = 5;
pub const MPD_ERROR_TIMEOUT: mpd_error = 4;
pub const MPD_ERROR_STATE: mpd_error = 3;
pub const MPD_ERROR_ARGUMENT: mpd_error = 2;
pub const MPD_ERROR_OOM: mpd_error = 1;
pub const MPD_ERROR_SUCCESS: mpd_error = 0;
