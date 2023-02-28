extern crate log;
extern crate wikipedia;
use rhai::{Dynamic, Array, NativeCallContext, EvalAltResult};

pub fn wiki_search(_context: NativeCallContext, d: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let t = howlong::HighResolutionTimer::new();
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    match wiki.search(&d) {
        Ok(result) => {
            let mut res = Array::new();
            for r in result {
                res.push(Dynamic::from(r.clone()));
            }
            log::debug!("{:?} takes to get data from Wikipedia", t.elapsed());
            Result::Ok(Dynamic::from(res))
        }
        _ => Err("Wikipedia search return no result".into()),
    }
}

pub fn wiki_page(_context: NativeCallContext, d: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let t = howlong::HighResolutionTimer::new();
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title(d);
    log::debug!("{:?} takes to get data from Wikipedia", t.elapsed());
    match page.get_content() {
        Ok(result) => Result::Ok(Dynamic::from(result)),
        _ => Err("Wikipedia page return no result".into()),
    }
}
