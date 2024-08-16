use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use nalgebra::Vector4;

use crate::rend::types::{MeshV4Plus, TriangleV4Plus, UVVec, UV};

// gavno pizdec no uje luchshe
fn get_digits_f32_in_slice(chars_slice: &[char]) -> Vec<f32> {
    let mut digits: Vec<f32> = Vec::new();
    let mut digit: Vec<char> = Vec::new();
    let chars_slice_end = chars_slice.len()-1;

    for (i,ch) in chars_slice.iter().enumerate() {
        if i == chars_slice_end {
            digit.push(*ch);
            digits.push(digit.iter().collect::<String>().parse::<f32>().unwrap_or(0.0));
            digit.clear()
        } else {
            if ch.is_numeric() || ch == &'.' || ch == &'-' {
                digit.push(*ch)
            } else {
                digits.push(digit.iter().collect::<String>().parse::<f32>().unwrap_or(0.0));
                digit.clear()
            }
        }
    }
    return digits
}

pub fn load_from_obj(filename: &Path) -> Result<MeshV4Plus, bool> {
    let file = File::open(filename);
    if file.is_err() {
        return Err(false);
    }
    let mut tris: MeshV4Plus = MeshV4Plus::new();
    let mut verts: Vec<Vector4<f32>> = Vec::new();
    let mut texs: Vec<UVVec> = Vec::new();

    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let mut chars:Vec<char> = Vec::new();
        for ch in line.chars() {
            chars.push(ch);
        }
        if chars[0] == 'v' {
            if chars[1] == 't' {
                let digits = get_digits_f32_in_slice(&chars[3..]);

                let u = digits[0];
                let v = digits[1];

                texs.push(UVVec::new(u,v,1.0));
            }
            else {
                let digits = get_digits_f32_in_slice(&chars[2..]);

                let x = digits[0];
                let y = digits[1];
                let z = digits[2];

                verts.push(Vector4::new(x,y,z,1.0))
            }
        }
        if chars[0] == 'f' {
            let digits = get_digits_f32_in_slice(&chars[2..]);
            tris.push(TriangleV4Plus::new(
                verts[digits[0] as usize - 1],
                verts[digits[2] as usize - 1],
                verts[digits[4] as usize - 1],
                1.0,
                UV::new(
                    texs[digits[1] as usize - 1],
                    texs[digits[3] as usize - 1],
                    texs[digits[5] as usize - 1]
                )
            ))
        }
    }
    return Ok(tris)
}