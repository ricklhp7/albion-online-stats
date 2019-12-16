use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize};

#[derive(Deserialize)]
enum ParamType
{
    Number
}

#[derive(Deserialize)]
struct Param
{
    name: String,
    id: u32,
    param_type: ParamType
}

#[derive(Deserialize)]
struct Message
{
    name: String,
    code: u32,
    params: Vec<Param>
}

fn main() {
    save_file("itemdb.rs", &generate_itemdb());
    save_file("messages.rs", &generate_messages());
}

fn generate_messages() -> String {
    let mut out = String::new();

    out.push_str(r"
        use log::*;

        use photon_decode::Parameters;
        use photon_decode::Value;
    ");

    out.push_str(include_str!("assets/decode_macros.rs"));

    out
}

fn generate_itemdb() -> String {
    let mut out = String::new();

    out.push_str("use std::collections::HashMap;\n");
    out.push_str("lazy_static! {\n");
    out.push_str("pub static ref ITEMDB: HashMap<u32, &'static str> = {[\n");

    include_str!("assets/item_ids.txt").split('\n').filter_map(|line| {
        let v: Vec<&str> = line.split(',').collect();
        let id : u32 = v.get(0)?.parse().ok()?;
        let item  = v.get(1)?.to_owned();
        Some((id, item))
    }).for_each(|(id, item)| {
        out.push_str(&format!("({}, \"{}\"),\n", id, item))
    });
    out.push_str("].iter().cloned().collect()\n");
    out.push_str("};\n");
    out.push_str("}");

    out
}

fn save_file(file_name: &str, content: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(file_name);
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}