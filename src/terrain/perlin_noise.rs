pub(crate) enum NormalizeMode {
    Local = 0,
    Global = 1,
}

pub(crate) fn perlin_noise3d(mut x: f64, mut y: f64, mut z: f64) -> f64 {
    x += 0.5;
    y += 0.5;
    z += 0.5;

    let x0 = x.floor() as usize & 255;
    let y0 = y.floor() as usize & 255;
    let z0 = z.floor() as usize & 255;

    let x = x - x.floor();
    let y = y - y.floor();
    let z = z - z.floor();

    let u = fade(x);
    let v = fade(y);
    let w = fade(z);

    let a = P[x0] + y0;
    let aa = P[a] + z0;
    let ab = P[a + 1] + z0;
    let b = P[x0 + 1] + y0;
    let ba = P[b] + z0;
    let bb = P[b + 1] + z0;

    return lerp(
        w,
        lerp(
            v,
            lerp(u, grad3d(P[aa], x, y, z), grad3d(P[ba], x - 1.0, y, z)),
            lerp(
                u,
                grad3d(P[ab], x, y - 1.0, z),
                grad3d(P[bb], x - 1.0, y - 1.0, z),
            ),
        ),
        lerp(
            v,
            lerp(
                u,
                grad3d(P[aa + 1], x, y, z - 1.0),
                grad3d(P[ba + 1], x - 1.0, y, z - 1.0),
            ),
            lerp(
                u,
                grad3d(P[ab + 1], x, y - 1.0, z - 1.0),
                grad3d(P[bb + 1], x - 1.0, y - 1.0, z - 1.0),
            ),
        ),
    );
}

// Hide the mirroring (maybe with octaves)
pub(crate) fn perlin_noise2d(mut x: f64, mut y: f64) -> f64 {
    x = x.abs();
    y = y.abs();
    x += 0.1;
    y += 0.1;

    let x0 = x.floor() as usize & 255;
    let y0 = y.floor() as usize & 255;

    let x = x - x.floor();
    let y = y - y.floor();

    let u = fade(x);
    let v = fade(y);

    let a = P[x0] + y0 & 0xff;
    let b = P[x0 + 1] + y0 & 0xff;

    return lerp(
        v,
        lerp(u, grad2d(P[a], x, y), grad2d(P[b], x - 1.0, y)),
        lerp(
            u,
            grad2d(P[a + 1], x, y - 1.0),
            grad2d(P[b + 1], x - 1.0, y - 1.0),
        ),
    );
}

/*pub(crate) fn noise_map2d(seed: u64, mut scale: f64, octaves: i32, persistance: f64, lacunarity: f64, x_offset: f64, y_offset: f64, normalize_mode: NormalizeMode) -> &[f64] {
    let mut height_map: [f64; (Chunk::chunk_size() * Chunk::chunk_size()) as usize] = [0.0; Chunk::chunk_size() * Chunk::chunk_size()];

    let mut prng = StdRng::seed_from_u64(seed);
    let octave_offsets = vec![glm::Vec2; octaves as usize];

    let mut max_possible_height = 0.0;
    let mut amplitude = 1;
    let mut frequency = 1;

    for i in 0..octaves {
        let offset_x = prng.gen_range(-100000, 100000) + x_offset;
        let offset_y = prng.gen_range(-100000, 100000) + y_offset;
        octave_offsets[i] = glm::vec2(offset_x, offset_y);

        max_possible_height += amplitude;
        amplitude *= persistance;
    }

    if scale <= 0.0 {
        scale = 0.0001;
    }

    let mut max_local_noise_height = f64::MIN;
    let mut min_local_noise_height = f64::MAX;

    let half_width = width as f64 / 2.0;
    let half_height = height as f64 / 2.0;

    for y in 0..height {
        for x in 0..width {
            amplitude = 1;
            frequency = 1;
            let mut noise_height = 0.0;

            for i in 0..octaves {
                let sample_x = (x - half_width + octave_offsets[i].x) / scale * frequency;
                let sample_y = (y - half_height + octave_offsets[i].y) / scale * frequency;

                let perlin_value = perlin_noise2d(sample_x, sample_y) * 2 - 1;
                noise_height += perlin_value * amplitude;

                amplitude *= persistance;
                frequency *= lacunarity;
            }

            if noise_height > max_local_noise_height {
                max_local_noise_height = noise_height;
            } else if noise_height < min_local_noise_height {
                min_local_noise_height = noise_height
            }
            let _ = std::mem::replace(&mut height_map[x * Chunk::chunk_size() + y], noise_height);
        }
    }

    for y in 0..height {
        for x in 0..width {
            if normalize_mode == NormalizeMode::Local {
                let previous_height: f64 = height_map.get(x * width + y).unwrap() as f64;
                let lerp_height = lerp(previous_height, min_local_noise_height, max_local_noise_height);
                let normalized_height = (lerp_height - min_local_noise_height) * 1 / max_local_noise_height - min_local_noise_height;
                println!("normal: {}", normalized_height);
            }
        }

    }

    height_map.as_slice()
}*/

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

fn grad3d(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else {
        if h == 12 || h == 14 {
            x
        } else {
            z
        }
    };

    return if h & 1 == 0 { u } else { -u } + if h & 2 == 0 { v } else { -v };
}

fn grad2d(hash: usize, x: f64, y: f64) -> f64 {
    let a = if hash & 1 == 0 { x } else { -x };

    let b = if hash & 2 == 0 { y } else { -y };

    a + b
}

static P: [usize; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];
