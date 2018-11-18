use libc;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub data: *mut libc::c_void,
    pub next: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub length: libc::c_uint,
    pub list: *mut node,
}
/* create a new node from the given data (can be used
 * in conjunction with list_push to add an element to
 * the list) */

pub unsafe fn node_from(mut data: *const libc::c_void, mut size: usize) -> *mut node {
    let mut node: *mut node = libc::malloc(::std::mem::size_of::<node>() as usize) as *mut node;
    (*node).data = libc::malloc(size);
    libc::memcpy((*node).data, data, size);
    (*node).next = 0 as *mut node;
    return node;
}
/* initialize the received list structure */

pub unsafe fn list_init(mut list: *mut list) -> libc::c_int {
    (*list).length = 0i32 as libc::c_uint;
    (*list).list = 0 as *mut node;
    return 0i32;
}
/* Return a pointer to the data at 'index'. Returns NULL
 * if there's not data at that index */

pub unsafe fn list_at(mut l: *const list, mut index: libc::c_uint) -> *mut libc::c_void {
    let mut found: *mut node = list_node_at(l, index);
    if found.is_null() {
        return 0 as *mut libc::c_void;
    } else {
        return (*found).data;
    };
}
/* get the low-level node at a given index */

pub unsafe fn list_node_at(mut l: *const list, mut index: libc::c_uint) -> *mut node {
    /* if there's no data in the list, fail */
    if (*l).list.is_null() {
        return 0 as *mut node;
    } else {
        let mut current: *mut node = (*l).list;
        while index > 0i32 as libc::c_uint {
            if (*current).next.is_null() {
                return 0 as *mut node;
            } else {
                current = (*current).next;
                index = index.wrapping_sub(1)
            }
        }
        return current;
    };
}
/* Pop item at index 'index' in list 'from' and push
 * it onto the end of list 'to' */

pub unsafe fn list_pop_push(
    mut from: *mut list,
    mut to: *mut list,
    mut index: libc::c_uint,
) -> libc::c_int {
    let mut extracted: *mut node = list_node_extract(from, index);
    if extracted.is_null() {
        return -1i32;
    } else {
        return list_push(to, extracted);
    };
}
/* remove the current node from the list, but don't free its
 * contents. */

pub unsafe fn list_node_extract(mut l: *mut list, mut index: libc::c_uint) -> *mut node {
    if (*l).list.is_null() {
        return 0 as *mut node;
    } else {
        let mut current: *mut node = (*l).list;
        let mut previous: *mut *mut node = &mut (*l).list;
        while index > 0i32 as libc::c_uint {
            if (*current).next.is_null() {
                return 0 as *mut node;
            } else {
                previous = &mut (*current).next;
                current = (*current).next;
                index = index.wrapping_sub(1)
            }
        }
        /* set the previous node's 'next' value to the current
         * nodes next value */
        *previous = (*current).next;
        /* null out this node's next value since it's not part of
         * a list anymore */
        (*current).next = 0 as *mut node;
        (*l).length = (*l).length.wrapping_sub(1);
        return current;
    };
}
/* add an item to the end of the list */

pub unsafe fn list_push(mut l: *mut list, mut n: *mut node) -> libc::c_int {
    /* allocate a pointer that points to the location we'll
     * eventually store our node into */
    let mut next: *mut *mut node = &mut (*l).list;
    while !(*next).is_null() {
        next = &mut (**next).next
    }
    *next = n;
    (*l).length = (*l).length.wrapping_add(1);
    return 0i32;
}
/* Remove the item at 'index' from the list */

pub unsafe fn list_pop(mut l: *mut list, mut index: libc::c_uint) -> libc::c_int {
    let mut extracted: *mut node = list_node_extract(l, index);
    if extracted.is_null() {
        return -1i32;
    } else {
        libc::free((*extracted).data);
        libc::free(extracted as *mut libc::c_void);
        return 0i32;
    };
}
/* free all elements of the list */
pub unsafe fn list_free(mut l: *mut list) -> libc::c_int {
    let mut current: *mut node = (*l).list;
    let mut tmp: *mut node = 0 as *mut node;
    while !current.is_null() {
        libc::free((*current).data);
        tmp = current;
        current = (*current).next;
        libc::free(tmp as *mut libc::c_void);
    }
    list_init(l);
    return 0i32;
}
