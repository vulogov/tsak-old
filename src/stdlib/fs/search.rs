extern crate log;
use rhai::{Array, Dynamic, NativeCallContext, EvalAltResult, FnPtr};
use rust_search::{SearchBuilder, FileSize, FilterExt};

pub fn search_find(_context: NativeCallContext, path: String, s: String) -> Result<Vec<rhai::Dynamic>, Box<EvalAltResult>> {
    let mut res = Array::new();
    let r: Vec<String> = SearchBuilder::default()
    .location(path)
    .search_input(s)
    .build()
    .collect();
    for v in r {
        res.push(Dynamic::from(v));
    }
    Result::Ok(res)
}

pub fn search_find_ext(_context: NativeCallContext, path: String, e: String) -> Result<Vec<rhai::Dynamic>, Box<EvalAltResult>> {
    let mut res = Array::new();
    let r: Vec<String> = SearchBuilder::default()
    .location(path)
    .ext(e)
    .build()
    .collect();
    for v in r {
        res.push(Dynamic::from(v));
    }
    Result::Ok(res)
}

pub fn search_find_zero(_context: NativeCallContext, path: String) -> Result<Vec<rhai::Dynamic>, Box<EvalAltResult>> {
    let mut res = Array::new();
    let r: Vec<String> = SearchBuilder::default()
    .location(path)
    .file_size_equal(FileSize::Megabyte(0.0))
    .build()
    .collect();
    for v in r {
        res.push(Dynamic::from(v));
    }
    Result::Ok(res)
}

pub fn search_find_json(_context: NativeCallContext, path: String) -> Result<Vec<rhai::Dynamic>, Box<EvalAltResult>> {
    let mut res = Array::new();
    let r: Vec<String> = SearchBuilder::default()
    .location(path)
    .ext("json")
    .file_size_greater(FileSize::Megabyte(0.0))
    .build()
    .collect();
    for v in r {
        res.push(Dynamic::from(v));
    }
    Result::Ok(res)
}

pub fn search_find_zip(context: NativeCallContext, path: String, f: FnPtr) -> Result<Vec<rhai::Dynamic>, Box<EvalAltResult>> {
    let mut res = Array::new();
    let r: Vec<String> = SearchBuilder::default()
    .location(path)
    .build()
    .collect();
    for v in r {
        let r: Result<Dynamic, Box<EvalAltResult>> = f.call_within_context(&context, (v,));
        match r {
            Ok(val) => res.push(val),
            Err(_) => continue,
        }
    }
    Result::Ok(res)
}
