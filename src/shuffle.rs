use libc;
use list;
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
pub struct shuffle_chain {
    pub max_window: libc::c_uint,
    pub window: list::list,
    pub pool: list::list,
}

/* initialize this shuffle chain */
pub unsafe fn shuffle_init(
    mut s: *mut shuffle_chain,
    mut window_size: libc::c_uint,
) -> libc::c_int {
    list::list_init(&mut (*s).pool);
    list::list_init(&mut (*s).window);
    (*s).max_window = window_size;
    return 0i32;
}
/* Add an the item pointed to by 'data' of size 'size' to
 * the given chain */

pub unsafe fn shuffle_add(
    mut s: *mut shuffle_chain,
    mut data: *const libc::c_void,
    mut size: usize,
) -> libc::c_int {
    list::list_push(&mut (*s).pool, list::node_from(data, size));
    return 0i32;
}
/* return the number of songs in the shuffle chain */
pub unsafe fn shuffle_length(mut s: *mut shuffle_chain) -> libc::c_int {
    return (*s).pool.length.wrapping_add((*s).window.length) as libc::c_int;
}
/* Randomly pick an element added via 'shuffle_add' and return
 * a pointer to it. */
pub unsafe fn shuffle_pick(mut s: *mut shuffle_chain) -> *const libc::c_void {
    let mut data: *const libc::c_void = 0 as *const libc::c_void;
    fill_window(s);
    /* get the first element off the window */
    data = list::list_at(&mut (*s).window, 0i32 as libc::c_uint);
    /* push the retrived element back into the pool */
    list::list_pop_push(&mut (*s).window, &mut (*s).pool, 0i32 as libc::c_uint);
    return data;
}
/* ensure that our window is as full as it can possibly be. */
unsafe fn fill_window(mut s: *mut shuffle_chain) -> libc::c_int {
    /* while our window isn't full and there's songs in the pool */
    while (*s).window.length <= (*s).max_window && (*s).pool.length > 0i32 as libc::c_uint {
        /* push a random song from the pool onto the end of the window */
        list::list_pop_push(
            &mut (*s).pool,
            &mut (*s).window,
            (libc::rand() as libc::c_uint).wrapping_rem((*s).pool.length),
        );
    }
    return 0i32;
}
/* Free memory associated with the shuffle chain. */

pub unsafe fn shuffle_free(mut s: *mut shuffle_chain) -> libc::c_int {
    list::list_free(&mut (*s).pool);
    list::list_free(&mut (*s).window);
    return 0i32;
}
