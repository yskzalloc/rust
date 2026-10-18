// Verifies that SanitizerCoverage requested with `-Cpasses=sancov-module`, as
// the Linux kernel builds Rust code, instruments the program before the
// sanitizer passes do, as clang orders them: the shadow-memory checks of
// `-Zsanitizer=kernel-address` are not traced as comparisons of the program,
// and the program's own comparison still is. Without a sanitizer, the extra
// pass keeps its place after the pipeline.

//@ add-minicore
//@ compile-flags: -Copt-level=1 -Cpasses=sancov-module
//@ compile-flags: -Cllvm-args=-sanitizer-coverage-level=1
//@ compile-flags: -Cllvm-args=-sanitizer-coverage-trace-pc
//@ compile-flags: -Cllvm-args=-sanitizer-coverage-trace-compares
//@ revisions: kasan_x86_64 kasan_aarch64 none
//@[kasan_x86_64] compile-flags: -Zsanitizer=kernel-address --target x86_64-unknown-none
//@[kasan_x86_64] needs-llvm-components: x86
//@[kasan_aarch64] compile-flags: -Zsanitizer=kernel-address --target aarch64-unknown-none
//@[kasan_aarch64] needs-llvm-components: aarch64
//@[none] compile-flags: --target x86_64-unknown-none
//@[none] needs-llvm-components: x86

#![crate_type = "rlib"]
#![feature(no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

// One load, checked by KASAN, and one comparison of the program's, against
// a constant. Traced once: the shadow byte comparison KASAN adds is not.
// CHECK-LABEL: define {{.*}} @load_and_compare(
// none-NOT:      __asan
// CHECK:         call void @__sanitizer_cov_trace_pc()
// CHECK-NOT:     call void @__sanitizer_cov_trace_{{(const_)?}}cmp1(
// kasan_x86_64:  call void @__asan_report_load4
// kasan_aarch64: call void @__asan_report_load4
// CHECK-NOT:     call void @__sanitizer_cov_trace_{{(const_)?}}cmp1(
// CHECK:         call void @__sanitizer_cov_trace_{{const_cmp4|switch}}(
// CHECK-NOT:     call void @__sanitizer_cov_trace_{{(const_)?}}cmp
// none-NOT:      __asan
// CHECK:         }
#[no_mangle]
pub fn load_and_compare(b: &u32) -> bool {
    match *b {
        7 => true,
        _ => false,
    }
}
