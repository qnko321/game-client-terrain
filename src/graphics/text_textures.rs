use anyhow::Result;
use nalgebra_glm as glm;
use std::alloc::{alloc, Layout};
use std::collections::{BTreeMap, HashMap};
use vulkanalia::prelude::v1_0::*;

extern crate rusttype;
extern crate unicode_normalization;

use crate::graphics::font_data::{FontData, Rect};
use crate::graphics::shared_textures::create_texture_image_from_byte_buffer;
use fontdue::{Font, FontSettings};
use image::{EncodableLayout, ImageBuffer, Luma, Rgb, Rgba};
use lazy_static::lazy_static;
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    TargetBin,
};
use std::fmt::Write;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{slice, thread};
use crate::core::app_data::AppData;

#[derive(Clone, Debug)]
pub(crate) struct Character {
    pub(crate) character: u32,
    pub(crate) texture_coordinates: Option<(f32, f32, f32, f32)>,
    pub(crate) size: glm::Vec2,
    pub(crate) bearing: glm::Vec2,
    pub(crate) bounding_box: Rect,
    pub(crate) advance: u16,
}

#[derive(Debug)]
struct Bitmap {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

lazy_static! {
    static ref font_data: Vec<u8> = std::fs::read("D:\\Downloads\\ArialTh.ttf").unwrap();
}

pub(crate) fn create_bitmaps(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<(HashMap<u32, Character>, vk::Image, vk::DeviceMemory)> {
    let face = match ttf_parser::Face::parse(&font_data, 0) {
        Ok(f) => f,
        Err(e) => {
            eprint!("Error: {}.", e);
            std::process::exit(1);
        }
    };
    let face = Arc::new(face);
    data.font_data = FontData {
        line_gap: face.line_gap(),
        global_bounding_box: Rect::from_ttf_parser_rect(face.global_bounding_box()),
    };
    let fontdue_font = Font::from_bytes(
        font_data.as_slice(),
        FontSettings {
            collection_index: 0,
            scale: 40.0,
        },
    )
    .unwrap();
    let fontdue_font = Arc::new(fontdue_font);
    let mut characters = HashMap::new();
    characters.insert(
        32,
        Character {
            character: 32,
            texture_coordinates: Some((0.0, 0.0, 0.0, 0.0)),
            size: glm::vec2(0.0, 0.0),
            bearing: Default::default(),
            bounding_box: Rect {
                x_min: 0,
                y_min: 0,
                x_max: 0,
                y_max: 0,
            },
            advance: face
                .glyph_hor_advance(face.glyph_index(' ').unwrap())
                .unwrap(),
        },
    );
    let characters: Arc<Mutex<HashMap<u32, Character>>> = Arc::new(Mutex::new(characters));
    let character_ids = Arc::new(Mutex::new(vec![]));
    let rects_to_place = Arc::new(Mutex::new(GroupedRectsToPlace::new()));
    let bitmaps = Arc::new(Mutex::new(HashMap::new()));
    let mut texture_image: Option<vk::Image> = None;
    let mut texture_image_memory: Option<vk::DeviceMemory> = None;

    let start: u32 = 33;
    let end: u32 = 126;

    let mut handles = vec![];

    unsafe {
        for character_digit in start..=end {
            let characters_clone = Arc::clone(&characters);
            let character_ids_clone = Arc::clone(&character_ids);
            let rects_to_lace_clone = Arc::clone(&rects_to_place);
            let bitmaps_clone = Arc::clone(&bitmaps);
            let fontdue_font_clone = Arc::clone(&fontdue_font);
            let face_clone = Arc::clone(&face);

            let handle = thread::spawn(move || {
                let character = match char::from_u32(character_digit) {
                    None => {
                        println!("couldn't convert digit \"{}\" to char", character_digit);
                        return;
                    }
                    Some(value) => value,
                };

                let glyph_id = match face_clone.glyph_index(character) {
                    None => {
                        println!("Failed getting glyph index at character \"{}\", with character digit \"{}\"", character, character_digit);
                        return;
                    }
                    Some(value) => value,
                };

                let bounding_box = match face_clone.glyph_bounding_box(glyph_id) {
                    None => {
                        return;
                    }
                    Some(value) => value,
                };

                let (metrics, bmp) =
                    fontdue_font_clone.rasterize(char::from_u32(character_digit).unwrap(), 720.0);

                let bitmap_width = metrics.width;
                let bitmap_height = metrics.height;

                let img = ImageBuffer::<Luma<u8>, _>::from_raw(
                    metrics.width as u32,
                    metrics.height as u32,
                    bmp,
                )
                .unwrap();

                let layout =
                    Layout::from_size_align((bitmap_width * bitmap_height * 4) as usize, 4)
                        .unwrap();

                let mut offset: isize = 0;
                let bitmap_ptr = alloc(layout);

                let no_pixel: [u8; 4] = [0, 0, 0, 0];

                for (x, y, pixel) in img.enumerate_pixels() {
                    if pixel[0] == 0 {
                        std::ptr::copy(no_pixel.as_ptr(), bitmap_ptr.offset(offset), 4);
                    } else {
                        let pixel = [pixel[0], pixel[0], pixel[0], 255];
                        std::ptr::copy(pixel.as_ptr(), bitmap_ptr.offset(offset), 4);
                    }

                    offset += 4;
                }

                let char_width = bounding_box.x_max - bounding_box.x_min;
                let char_height = bounding_box.y_max - bounding_box.y_min;

                let mut bitmaps = bitmaps_clone.lock().unwrap();

                let bitmap_slice =
                    slice::from_raw_parts(bitmap_ptr, (bitmap_width * bitmap_height * 4) as usize);
                let bitmap_vec = bitmap_slice.to_vec();

                bitmaps.insert(
                    character_digit,
                    Bitmap {
                        width: bitmap_width as u32,
                        height: bitmap_height as u32,
                        data: bitmap_vec,
                    },
                );

                let bearing_x = match face_clone.glyph_hor_side_bearing(glyph_id) {
                    None => 0,
                    Some(value) => value,
                };

                let bearing_y = match face_clone.glyph_ver_side_bearing(glyph_id) {
                    None => 0,
                    Some(value) => value,
                };

                let advance = match face_clone.glyph_hor_advance(glyph_id) {
                    None => 0,
                    Some(value) => value,
                };

                let bounding_box = match face_clone.glyph_bounding_box(glyph_id) {
                    None => Rect {
                        x_min: 0,
                        y_min: 0,
                        x_max: 0,
                        y_max: 0,
                    },
                    Some(value) => Rect::from_ttf_parser_rect(value),
                };

                let character_struct = Character {
                    character: character_digit,
                    texture_coordinates: None,
                    size: glm::vec2(char_width as f32, char_height as f32),
                    bearing: glm::vec2(bearing_x as f32, bearing_y as f32),
                    bounding_box,
                    advance,
                };

                let mut characters = characters_clone.lock().unwrap();
                characters.insert(character_digit, character_struct);
                let mut character_ids = character_ids_clone.lock().unwrap();
                character_ids.push(character_digit);

                let mut rects_to_place = rects_to_lace_clone.lock().unwrap();
                rects_to_place.push_rect(
                    character_digit,
                    Some(vec![0]),
                    RectToInsert::new(bitmap_width as u32, bitmap_height as u32, 1),
                );
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let rects_to_place = Arc::try_unwrap(rects_to_place)
        .unwrap()
        .into_inner()
        .unwrap();

    let mut target_bins = BTreeMap::new();
    target_bins.insert(0, TargetBin::new(10000000, 10000000, 1));

    let rectangle_placements = pack_rects(
        &rects_to_place,
        &mut target_bins,
        &volume_heuristic,
        &contains_smallest_box,
    )
    .unwrap();

    let mut raw_texture_width: u32 = 0;
    let mut raw_texture_height: u32 = 0;

    let mut character_ids = Arc::try_unwrap(character_ids)
        .unwrap()
        .into_inner()
        .unwrap();

    for i in &character_ids {
        let packed_location = rectangle_placements.packed_locations().get(&i).unwrap().1;

        let reach_right = packed_location.x() + packed_location.width();
        if reach_right > raw_texture_width {
            raw_texture_width = reach_right;
        }

        let reach_bot = packed_location.y() + packed_location.height();
        if reach_bot > raw_texture_height {
            raw_texture_height = reach_bot;
        }
    }

    let mut target_bins = BTreeMap::new();
    target_bins.insert(0, TargetBin::new(raw_texture_width, raw_texture_height, 1));

    let rectangle_placements = pack_rects(
        &rects_to_place,
        &mut target_bins,
        &volume_heuristic,
        &contains_smallest_box,
    )
    .unwrap();

    let layout =
        Layout::from_size_align((raw_texture_width * raw_texture_height * 4) as usize, 4).unwrap();

    let bitmaps = Arc::try_unwrap(bitmaps).unwrap().into_inner().unwrap();
    let mut characters = Arc::try_unwrap(characters).unwrap().into_inner().unwrap();

    unsafe {
        let raw_texture_ptr = alloc(layout);

        for i in character_ids {
            let bitmap = bitmaps.get(&i).unwrap();
            let bitmap_ptr = bitmap.data.as_ptr();

            let packed_location = rectangle_placements.packed_locations().get(&i).unwrap().1;
            let (width, height) = (bitmap.width, bitmap.height);

            for image_y in 0..height {
                let start_offset = (image_y * width * 4) as isize;
                let texture_start_offset = ((packed_location.y() + image_y) * raw_texture_width * 4
                    + packed_location.x() * 4) as isize;

                std::ptr::copy(
                    bitmap_ptr.offset(start_offset),
                    raw_texture_ptr.offset(texture_start_offset),
                    (width * 4) as usize,
                );
            }

            let x_start = (packed_location.x()) as f32 / raw_texture_width as f32;
            let x_end =
                (packed_location.x() + packed_location.width()) as f32 / raw_texture_width as f32;

            let y_start = (packed_location.y()) as f32 / raw_texture_height as f32;
            let y_end =
                (packed_location.y() + packed_location.height()) as f32 / raw_texture_height as f32;

            characters.get_mut(&i).unwrap().texture_coordinates =
                Some((x_start, x_end, y_start, y_end));
        }

        let texture_data = slice::from_raw_parts(
            raw_texture_ptr,
            (raw_texture_width * raw_texture_height * 4) as usize,
        );

        let (image, memory) = create_texture_image_from_byte_buffer(
            instance,
            device,
            data,
            raw_texture_width,
            raw_texture_height,
            texture_data,
        )
        .unwrap();

        texture_image = Some(image);
        texture_image_memory = Some(memory);
    }

    Ok((
        characters,
        texture_image.unwrap(),
        texture_image_memory.unwrap(),
    ))
}
