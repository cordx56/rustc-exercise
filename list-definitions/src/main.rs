#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;

struct MyRustc;
impl rustc_driver::Callbacks for MyRustc {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        for def_id in tcx.hir_crate_items(()).definitions() {
            let node = tcx.hir_node_by_def_id(def_id);
            if let Some(ident) = node.ident() {
                println!("{}", ident);
            }
        }
        rustc_driver::Compilation::Continue
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut MyRustc);
}
