use bevy::{prelude::*, render::render_resource::Extent3d};

const TILE_SIZE: usize = 16 * 16;
const TERRAIN_COUNT: u32 = 600;

pub fn image_to_chipset(image: &mut Image, alpha_key: Option<u32>) {
    let mut pixels = rearrange(&image.data.take().unwrap());
    chromakey(&mut pixels, alpha_key);
    add_terrain(&mut pixels);

    image.data = Some(pixels);
    image.texture_descriptor.size = Extent3d {
        width: 16,
        height: 16,
        depth_or_array_layers: 480 + TERRAIN_COUNT,
    };
}

fn rearrange(pixels: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(pixels.len() + TERRAIN_COUNT as usize * TILE_SIZE);

    // rearrange the pixels from squares to contiguous blocks
    // wgpu reads the elements as lines on the image instead of as squares, so they need to be repacked
    // there is definitely a better way to do this but this worked first try so i will not be changing it
    for tile_index in 0..480 {
        let start_x = tile_index % 30;
        let start_y = tile_index / 30;
        for square_index in 0..256 {
            let add_x = square_index % 16;
            let add_y = square_index / 16;
            for byte in 0..4 {
                let pixel =
                    pixels[(start_x * 16 + add_x + (start_y * 16 + add_y) * 480) * 4 + byte];
                buf.push(pixel);
            }
        }
    }

    buf
}

pub fn chromakey(pixels: &mut [u8], key: Option<u32>) {
    if let Some(key) = key {
        for pixel in pixels.chunks_exact_mut(4) {
            let color = u32::from_be_bytes(pixel.try_into().unwrap()) >> 8;
            if color == key {
                pixel[3] = 0x00;
            }
        }
    }
}

fn add_terrain(pixels: &mut Vec<u8>) {
    let offset = |x: usize, y: usize| (y * 30 + x) * TILE_SIZE;

    for i in 0..12 {
        let x = if i < 4 { 0 } else { 6 } + i % 2 * 3;
        let y = (i / 2 * 4 + 8) % 16;
        let region = offset(x, y);

        for i in 0..50 {
            let (top_left, top_right, bottom_right, bottom_left) = match i {
                0..=15 => (
                    // "You should be able to solve this."
                    7 - i % 2 * 5,
                    7 - i / 2 % 2 * 5,
                    7 - i / 4 % 2 * 5,
                    7 - i / 8 * 5,
                ),
                // i am unable to solve this one. it looks completely random
                16 => (6, 6, 6, 6),
                17 => (6, 2, 6, 6),
                18 => (6, 6, 2, 6),
                19 => (6, 2, 2, 6),
                20 => (4, 4, 4, 4),
                21 => (4, 4, 2, 4),
                22 => (4, 4, 4, 2),
                23 => (4, 4, 2, 2),
                24 => (8, 8, 8, 8),
                25 => (8, 8, 8, 2),
                26 => (2, 8, 8, 8),
                27 => (2, 8, 8, 2),
                28 => (10, 10, 10, 10),
                29 => (2, 10, 10, 10),
                30 => (10, 2, 10, 10),
                31 => (2, 2, 10, 10),
                32 => (6, 8, 8, 6),
                33 => (4, 4, 10, 10),
                34 => (3, 3, 3, 3),
                35 => (3, 3, 2, 3),
                36 => (5, 5, 5, 5),
                37 => (5, 5, 5, 2),
                38 => (11, 11, 11, 11),
                39 => (2, 11, 11, 11),
                40 => (9, 9, 9, 9),
                41 => (9, 2, 9, 9),
                42 => (3, 5, 5, 3),
                43 => (3, 3, 9, 9),
                44 => (9, 11, 11, 9),
                45 => (5, 5, 11, 11),
                46 => (3, 5, 11, 9),
                47 => (7, 7, 7, 7),
                48 => (7, 7, 7, 7),
                49 => (0, 0, 0, 0),
                _ => unreachable!(),
            };

            let offset = |x: usize| x / 3 * 7680 + x % 3 * 256;
            let mut copy_pixel = |x: usize| {
                for c in 0..4 {
                    pixels.push(pixels[x * 4 + c]);
                }
            };

            // this is just as ugly as the block from above but it worked first try
            for line in 0..8 {
                for i in 0..8 {
                    // top left
                    copy_pixel(region + offset(top_left) + line * 16 + i);
                }

                for i in 0..8 {
                    // top right
                    copy_pixel(region + offset(top_right) + 8 + line * 16 + i);
                }
            }

            for line in 0..8 {
                for i in 0..8 {
                    // bottom left
                    copy_pixel(region + offset(bottom_left) + (line + 8) * 16 + i);
                }

                for i in 0..8 {
                    // bottom right
                    copy_pixel(region + offset(bottom_right) + 8 + (line + 8) * 16 + i);
                }
            }
        }
    }
}
