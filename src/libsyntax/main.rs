use rustc_data_structures::sync::Lrc;

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_data_structures::sync::Lock;
use std::io;
use std::path::PathBuf;
use syntax::ast::CrateConfig;
use syntax::diagnostics::plugin::ErrorMap;
use syntax::feature_gate::UnstableFeatures;
use syntax::parse::lexer::StringReader;
use syntax::parse::{token, ParseSess};
use syntax::source_map::{FilePathMapping, SourceMap};
use syntax_pos::edition::Edition;

fn mk_sess(sm: Lrc<SourceMap>) -> ParseSess {
    let emitter = errors::emitter::EmitterWriter::new(
        Box::new(io::sink()),
        Some(sm.clone()),
        false,
        false,
        false,
    );
    ParseSess {
        span_diagnostic: errors::Handler::with_emitter(true, None, Box::new(emitter)),
        unstable_features: UnstableFeatures::from_environment(),
        config: CrateConfig::default(),
        included_mod_stack: Lock::new(Vec::new()),
        source_map: sm,
        missing_fragment_specifiers: Lock::new(FxHashSet::default()),
        raw_identifier_spans: Lock::new(Vec::new()),
        registered_diagnostics: Lock::new(ErrorMap::new()),
        buffered_lints: Lock::new(vec![]),
        edition: Edition::from_session(),
        ambiguous_block_expr_parse: Lock::new(FxHashMap::default()),
        param_attr_spans: Lock::new(Vec::new()),
        let_chains_spans: Lock::new(Vec::new()),
        async_closure_spans: Lock::new(Vec::new()),
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("expected <PATH>");
    let text = std::fs::read_to_string(&path).unwrap();
    let mut tokens = Vec::with_capacity(10_000_000);
    syntax::with_default_globals(|| {
        let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
        let sf = sm.new_source_file(PathBuf::from(path).into(), text);
        let sess = mk_sess(sm.clone());


        let start = std::time::Instant::now();
        for _ in 0..50 {
            let mut string_reader = StringReader::new(&sess, sf.clone(), None);
            loop {
                let t = string_reader.next_token();
                if t == token::Eof {
                    break;
                }
                tokens.push(t);
            }
        }

        eprintln!("{:?}", start.elapsed());
        if std::env::args().nth(2).is_some() {
            println!("{:?}", tokens)
        }
    });
}
