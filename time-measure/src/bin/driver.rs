#![feature(rustc_private)]

extern crate rustc_borrowck;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_hir_typeck;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;

use rustc_hir::def_id::{LOCAL_CRATE, LocalDefId};
use rustc_middle::{
    query::queries,
    ty::{TyCtxt, TypeckResults},
    util::Providers,
};
use std::time::SystemTime;
use time_measure::*;

fn override_queries(_session: &rustc_session::Session, local: &mut Providers) {
    local.typeck = typeck;
    local.mir_borrowck = mir_borrowck;
}
fn typeck(tcx: TyCtxt<'_>, def_id: LocalDefId) -> &TypeckResults {
    let mut providers = Providers::default();
    rustc_hir_typeck::provide(&mut providers);
    let start = SystemTime::now();
    let result = (providers.typeck)(tcx, def_id);
    let end = SystemTime::now();

    let crate_sym = tcx.crate_name(LOCAL_CRATE);
    let time = end.duration_since(start).unwrap().as_nanos().to_string();
    let output = TimeMeasure::TypeCheck {
        krate: format!("{crate_sym}"),
        time,
    };
    println!("{}", serde_json::to_string(&output).unwrap());

    result
}
fn mir_borrowck(tcx: TyCtxt<'_>, def_id: LocalDefId) -> queries::mir_borrowck::ProvidedValue<'_> {
    let mut providers = Providers::default();
    rustc_borrowck::provide(&mut providers);
    let start = SystemTime::now();
    let result = (providers.mir_borrowck)(tcx, def_id);
    let end = SystemTime::now();

    let crate_sym = tcx.crate_name(LOCAL_CRATE);
    let time = end.duration_since(start).unwrap().as_nanos().to_string();
    let output = TimeMeasure::BorrowCheck {
        krate: format!("{crate_sym}"),
        time,
    };
    println!("{}", serde_json::to_string(&output).unwrap());

    result
}

struct DefaultRustc;
impl rustc_driver::Callbacks for DefaultRustc {}

struct MyRustc {
    krate: Option<String>,
}
impl rustc_driver::Callbacks for MyRustc {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.override_queries = Some(override_queries);
    }
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        let crate_sym = tcx.crate_name(LOCAL_CRATE);
        self.krate = Some(format!("{crate_sym}"));
        rustc_driver::Compilation::Continue
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    for arg in &args {
        if arg == "-vV" || arg == "--version" || arg.starts_with("--print=") {
            rustc_driver::run_compiler(&args, &mut DefaultRustc);
            return;
        }
    }

    let mut my_rustc = MyRustc { krate: None };
    let start = SystemTime::now();
    rustc_driver::run_compiler(&args, &mut my_rustc);
    let end = SystemTime::now();

    let time = end.duration_since(start).unwrap().as_nanos().to_string();
    let output = TimeMeasure::Whole {
        krate: format!("{}", my_rustc.krate.unwrap()),
        time,
    };
    println!("{}", serde_json::to_string(&output).unwrap());
}
