use std::libc;

use common::dynamics;


pub struct ClientMap {
	clients: IdMap,
	idPool : Stack
}

pub struct IdMap {
	cap  : libc::size_t,
	elems: *mut IdMapEntry
}

pub struct IdMapEntry {
	isOccupied: int,
	value     : Client
}

pub struct Stack {
	cap  : libc::size_t,
	size : libc::size_t,
	elems: *mut libc::size_t
}

struct Client {
	socketFD: libc::c_int,
	id      : libc::size_t,
	ship    : dynamics::Body
}


pub fn init_client_map(c: &mut ClientMap, cap: libc::size_t) {
	// Init IdMap
	c.clients.cap = cap;
	let memSize = cap * ::std::mem::size_of::<IdMapEntry>() as libc::size_t;
	c.clients.elems = unsafe { libc::malloc(memSize) as *mut IdMapEntry };
	unsafe { ::std::ptr::set_memory(c.clients.elems, 0, cap as uint) };

	// Init Stack
	c.idPool.cap = cap;
	c.idPool.size = cap;
	let idPoolSize =
		cap * ::std::mem::size_of::<libc::size_t>() as libc::size_t;
	c.idPool.elems = unsafe {
		libc::malloc(idPoolSize) as *mut libc::size_t };

	// Init ids
	let mut i: int = 0;
	while i < cap as int {
		unsafe {
			let ptr = ::std::ptr::mut_offset(c.idPool.elems, i);
			*ptr = (cap as int - i - 1) as libc::size_t; };
		i += 1;
	}
}

pub fn can_add(c: &ClientMap) -> bool {
	c.idPool.size > 0
}

pub fn add(c: &mut ClientMap, socketFD: libc::c_int, pos: ::common::vec::Vec2, vel: ::common::vec::Vec2) {
	// Get id from pool.
	let clientId = unsafe {
		let ptr = ::std::ptr::mut_offset(c.idPool.elems, (c.idPool.size - 1) as int);
		*ptr };
	c.idPool.size -= 1;

	// Construct client
	let client = Client {
		socketFD: socketFD,
		id      : clientId,
		ship    : dynamics::Body { pos: pos, vel: vel } };

	// Add client to map
	unsafe {
		let ptr = ::std::ptr::mut_offset(c.clients.elems, clientId as int);
		(*ptr).isOccupied = 1;
		(*ptr).value = client;
	};
}

pub fn remove(c: &mut ClientMap, id: libc::size_t) {
	unsafe {
		let clientPtr = ::std::ptr::mut_offset(c.clients.elems, id as int);
		let containsClient = (*clientPtr).isOccupied == 1;

		if containsClient {
			// Remove client
			(*clientPtr).isOccupied = 0;

			// Add id back to pool
			let idPtr =
				::std::ptr::mut_offset(c.idPool.elems, c.idPool.size as int);
			(*idPtr) = id;
			c.idPool.size += 1;
		}
	}
}
