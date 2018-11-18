use libc;
use list;
use mpd;

pub type rule_type = libc::c_uint;
pub const RULE_EXCLUDE: rule_type = 0;

#[derive(Clone)]
pub struct song_rule {
    pub type_0: rule_type,
    pub matchers: list::list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule_field {
    pub tag: mpd::mpd_tag_type,
    pub value: *mut libc::c_char,
}
/* Initialize a rule */

pub unsafe fn rule_init(rule: *mut song_rule, type_0: rule_type) -> libc::c_int {
    /* set the type */
    (*rule).type_0 = type_0;
    /* allocate the field list */
    list::list_init(&mut (*rule).matchers);
    return 0i32;
}
/* Add some criteria for this rule to match on */

pub unsafe fn rule_add_criteria(
    rule: *mut song_rule,
    field: *const libc::c_char,
    expected_value: *const libc::c_char,
) -> libc::c_int {
    let mut matcher: rule_field = rule_field {
        tag: mpd::MPD_TAG_ARTIST,
        value: 0 as *mut libc::c_char,
    };
    /* try and parse out the tag to match on */
    matcher.tag = mpd::mpd_tag_name_iparse(field);
    if matcher.tag as libc::c_int == mpd::MPD_TAG_UNKNOWN as libc::c_int {
        return -1i32;
    } else {
        /* copy the string to match on */
        matcher.value = libc::strdup(expected_value);
        /* add our matcher to the array */
        list::list_push(
            &mut (*rule).matchers,
            list::node_from(
                &mut matcher as *mut rule_field as *const libc::c_void,
                ::std::mem::size_of::<rule_field>() as usize,
            ),
        );
        return 0i32;
    };
}
/* Returns true if this song is a positive match
 * in the case of 'include' rules and true for
 * negative matches in the case of 'exclude' rules.
 * False otherwise. */

pub unsafe fn rule_match(rule: *mut song_rule, song: *const mpd::mpd_song) -> bool {
    let mut current_matcher;
    let mut tag_value;
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    while i < (*rule).matchers.length {
        current_matcher = list::list_at(&mut (*rule).matchers, i) as *mut rule_field;
        /* get the first result for this tag */
        tag_value = mpd::mpd_song_get_tag(song, (*current_matcher).tag, 0i32 as libc::c_uint);
        /* if the tag doesn't exist, we can't match on it. */
        if !tag_value.is_null() {
            /* if our match value is at least a substring of the tag's
             * value, we have a match. e.g. de matches 'De La Soul'.
             * If the output of strstr is NULL we don't have a substring
             * match. */
            if !libc::strcasestr(tag_value, (*current_matcher).value).is_null() {
                /* On exclusion matches, if any tag check succeeds, we have
                 * a failed match. */
                if (*rule).type_0 as libc::c_uint == RULE_EXCLUDE as libc::c_int as libc::c_uint {
                    return 0 != 0i32;
                }
            }
        }
        i +=1;
    }
    /* If we've passed all the tests, we have a match */
    return 0 != 1i32;
}
/* Free the memory used to store this rule */

pub unsafe fn rule_free(rule: *mut song_rule) -> libc::c_int {
    let mut field;
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    while i < (*rule).matchers.length {
        field = list::list_at(&mut (*rule).matchers, i) as *mut rule_field;
        libc::free((*field).value as *mut libc::c_void);
        i +=1;
    }
    list::list_free(&mut (*rule).matchers);
    return 0i32;
}
