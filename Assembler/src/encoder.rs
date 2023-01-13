/*
use std::collections::HashMap;
use once_cell::sync::Lazy;


type EncodeCallback = fn(Vec<&str>) -> [u8; 2];

fn mov_im(args: Vec<&str>) -> [u8; 2] {
    [1u8, 2u8]
}

static MNEMONICS: Lazy<HashMap<&str, EncodeCallback>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("mov", mov_im as EncodeCallback);
    m.insert("load", mov_im as EncodeCallback);
    m
});
*/


