use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static TILE_SIZE: usize = 16;
static PIXELS: usize = 256;
static MASK_SIZE: usize = 32;

#[wasm_bindgen]
pub fn tile_size() -> usize {
    return TILE_SIZE;
}

#[wasm_bindgen]
pub fn create_diff(source: Vec<u32>, target: Vec<u32>) -> Vec<u8> {
    if source.len() != PIXELS || target.len() != PIXELS {
        panic!("Source and target must be 1024 bytes long");
    }

    let mut bitmap = vec![0u8; MASK_SIZE];
    let mut values : Vec<u32> = Vec::new();

    for i in 0..PIXELS {
        if source[i] != target[i] {
            values.push(target[i]);
            bitmap[i / 8] = bitmap[i / 8] + (1 << (i % 8));
        }
    }

    let mut output = Vec::from(bitmap);
    for value in values {
        output.push(((value >> 24) & 0xFF) as u8);
        output.push(((value >> 16) & 0xFF) as u8);
        output.push(((value >> 8) & 0xFF) as u8);
        output.push(((value) & 0xFF) as u8);
    }

    output
}

#[wasm_bindgen]
pub fn apply_diff(source: Vec<u32>, diff_stream: Vec<u8>) -> Vec<u32> {
    /* Separate the bitmap */
    if diff_stream.len() < MASK_SIZE { panic!("Diff stream is below 32 bytes which is impossible"); }
    if source.len() != PIXELS { panic!("Source must be 1024 bytes long!"); }
    let mut bitmap = vec![0u8; MASK_SIZE];
    for i in 0..MASK_SIZE {
        bitmap[i] = diff_stream[i];
    }

    let mut result = source.clone();
    let mut diff_stream_ptr = MASK_SIZE;

    for i in 0..PIXELS {
        if (bitmap[i / 8] >> (i % 8)) & 0x1 == 1 {
            if diff_stream.len() < (diff_stream_ptr + 4) { panic!("Diff stream truncated"); }
            result[i] = {
                let mut result = (diff_stream[diff_stream_ptr] as u32) << 24;
                result = result + ((diff_stream[diff_stream_ptr + 1] as u32) << 16);
                result = result + ((diff_stream[diff_stream_ptr + 2] as u32) << 8);
                result = result + (diff_stream[diff_stream_ptr + 3] as u32);

                result
            };
            diff_stream_ptr += 4;
        }
    }

    result
}
