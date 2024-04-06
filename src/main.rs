static mut MEMORY: [u8; 65536] = [0; 65536];
static mut POINTERS: [u16; 512] = [0; 512];
static mut SIZES: [u16; 512] = [0; 512];
static mut LAST_POINTER: u16 = 0;

// writes data to a location in memory
unsafe fn write(mut loc: u16, data: &[u8]) {
    let _size = MEMORY[loc as usize] as u16 * 256 + MEMORY[(loc + 1) as usize] as u16;

    if data.len() > _size as usize {
        panic!("Data too large for location");
    }

    loc += 2;
    for i in 0..data.len() {
        MEMORY[(loc + i as u16) as usize] = data[i];
    }
    for i in data.len().._size as usize {
        MEMORY[(loc as usize + i) as usize] = 0;
    }
}

// Pointer[0] = tail
// alloc => allocates 2 bytes for size, then the rest for the data
// scans through the POINTERS array to find the first available space
unsafe fn alloc(_size: u16) -> u16 {
    for i in 1..POINTERS.len() { // start at 1 because 0 is the tail

        // if the size is 0, then we have reached the end of the important pointers
        if SIZES[i as usize] == 0 {
            break;
        }

        // if the size is greater than or equal to the requested size, then we can use this location
        else if SIZES[i as usize] >= _size + 2 {
            let loc = POINTERS[i as usize];

            MEMORY[loc as usize] = (_size / 256) as u8;
            MEMORY[(loc + 1) as usize] = (_size % 256) as u8;

            // if the size is equal to the requested size, then we can remove this pointer
            if SIZES[i as usize] == _size + 2 {

                if LAST_POINTER as usize == i {
                    LAST_POINTER -= 1;
                }
                else {
                    // move the last pointer to the current pointers location
                    POINTERS[i as usize] = POINTERS[LAST_POINTER as usize];
                    SIZES[i as usize] = SIZES[LAST_POINTER as usize];

                    // clear the last pointer
                    POINTERS[LAST_POINTER as usize] = 0; // Breaks if it is the last pointer : FIX THIS
                    SIZES[LAST_POINTER as usize] = 0;

                    // decrement the last pointer
                    LAST_POINTER -= 1;
                }
            }
            // if the size is greater than the requested size, then we can move the pointer and reduce the size
            else {
                POINTERS[i as usize] += _size + 2;
                SIZES[i as usize] -= _size + 2;
            }

            return loc;
        }
    }

    let loc = POINTERS[0];

    MEMORY[loc as usize] = (_size / 256) as u8;
    MEMORY[(loc + 1) as usize] = (_size % 256) as u8;

    POINTERS[0] += _size + 2;
    return loc;
}

unsafe fn dealloc(loc: u16) { // if pointer[0] is < a pointer
    let _size = MEMORY[loc as usize] as u16 * 256 + MEMORY[(loc + 1) as usize] as u16;
    let new_loc = loc + _size + 2;

    // if the new location is the tail
    if new_loc == POINTERS[0] {
        // set the tail to the current location
        POINTERS[0] = loc;

// check if the last chunk of space is free as well
        for i in 1..POINTERS.len() {
            if SIZES[i as usize] == 0 {
                break;
            }
            else if POINTERS[i as usize] + SIZES[i as usize] == loc {
                POINTERS[0] = POINTERS[i as usize];

                if LAST_POINTER as usize == i {
                    POINTERS[i as usize] = 0;
                    SIZES[i as usize] = 0;
                }
                else {
// move the last pointer to the current pointers location
                    POINTERS[i as usize] = POINTERS[LAST_POINTER as usize];
                    SIZES[i as usize] = SIZES[LAST_POINTER as usize];

                    // clear the last pointer
                    POINTERS[LAST_POINTER as usize] = 0; // Breaks if it is the last pointer : FIX THIS
                    SIZES[LAST_POINTER as usize] = 0;

                }
                LAST_POINTER -= 1;

                break;
            }
        }

        return;
    }
    
    for i in 1..POINTERS.len() {
        if POINTERS[i as usize] == 0 {
            break;
        }

        if POINTERS[i as usize] + SIZES[i as usize] == loc {
            SIZES[i as usize] += _size + 2;

            for j in 1..POINTERS.len() {
                if SIZES[j as usize] == 0 {
                    break;
                }

                else if POINTERS[j as usize] == new_loc {
                    SIZES[i as usize] += SIZES[j as usize];

                    if LAST_POINTER as usize == j {
                        POINTERS[j as usize] = 0;
                        SIZES[j as usize] = 0;
                    }
                    else {
                        POINTERS[j as usize] = POINTERS[LAST_POINTER as usize];
                        SIZES[j as usize] = SIZES[LAST_POINTER as usize];

                        POINTERS[LAST_POINTER as usize] = 0;
                        SIZES[LAST_POINTER as usize] = 0;
                    }
                    LAST_POINTER -= 1;

                    break;
                }
            }
        
            return;
        }
    
        if POINTERS[i as usize] == new_loc {
            POINTERS[i as usize] = loc;
            SIZES[i as usize] += _size + 2;

            for j in 1..POINTERS.len() {
                if SIZES[j as usize] == 0 {
                    break;
                }

                else if POINTERS[j as usize] + SIZES[j as usize] == loc {
                    SIZES[j as usize] += SIZES[i as usize];

                    if LAST_POINTER as usize == i {
                        POINTERS[i as usize] = 0;
                        SIZES[i as usize] = 0;
                    }
                    else {
                        POINTERS[i as usize] = POINTERS[LAST_POINTER as usize];
                        SIZES[i as usize] = SIZES[LAST_POINTER as usize];

                        POINTERS[LAST_POINTER as usize] = 0;
                        SIZES[LAST_POINTER as usize] = 0;
                    }
                    LAST_POINTER -= 1;

                    break;
                }
            }
            return;
        }
    }

    LAST_POINTER += 1;
    POINTERS[LAST_POINTER as usize] = loc;
    SIZES[LAST_POINTER as usize] = _size + 2;

    return;
}

fn main() {unsafe{
    let test = alloc(8);
    let test1 = alloc(8);
    let test2 = alloc(8);
    let test3 = alloc(8);
    let test4 = alloc(8);
    let test5 = alloc(8);
    let test6 = alloc(8);
    let test7 = alloc(8);
    let test8 = alloc(8);

    dealloc(test5);
    dealloc(test1);
    dealloc(test4);
    dealloc(test3);
    dealloc(test6);
    dealloc(test7);
    dealloc(test8);
    dealloc(test);
    dealloc(test2);
    dbg!(POINTERS[0]);
    dbg!(LAST_POINTER);

    
    // dbg!(POINTERS[0]);
    // dbg!(LAST_POINTER);

    // for i in 1..5 {
    //     dbg!(POINTERS[i]);
    //     dbg!(SIZES[i]);
    // }

    for i in 1..5 {
        dbg!(POINTERS[i]);
        dbg!(SIZES[i]);
    }
}}

// Still breaks if an inner chunk is deallocated, then the outer chunk is deallocated (pointer[0] doesent go down)
// test overwriting last part thing (i think its fine due to 0 indexing)