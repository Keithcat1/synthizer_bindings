extern crate walkdir;
extern crate cc;
use walkdir::WalkDir;
use std::ffi::OsStr;
fn main() {
	let mut compiler=cc::Build::new();
	let extensions=[OsStr::new("c"), OsStr::new("cpp"), OsStr::new("cc"), OsStr::new("cxx")];
	for i in WalkDir::new("vendor/synthizer/src").into_iter().filter_map(|v| v.ok()) {
		let i=i.path();
		let ext=i.extension();
		if ext==None {
			continue;
		}
		let ext=ext.unwrap();
		if !extensions.contains(&ext) {
			continue;
		}
		compiler.file(i);
	}
	if compiler.get_compiler().is_like_msvc() {
		if std::env::var("OPT_LEVEL").unwrap().parse::<u32>().unwrap()>0 {
			compiler
			.define("NDEBUG", None)
			;}
		compiler
		.flag("/wd4068")
		.flag("/wd4244")
		.flag("/wd4267")
		.define("_ENABLE_EXTENDED_ALIGNED_STORAGE", None)
		.define("WIN32", None)
		.define("_WINDOWS", None)
		.flag("/Zc:preprocessor")
		.flag("/EHsc")
		.flag("-std:c++17")
		.flag("/GR")
		.flag("/wd5105");
	}
	if compiler.get_compiler().is_like_gnu() {
		compiler
		.flag("-std=c++17");
	}
	compiler
	.cpp(true)
	.warnings(false)
	.extra_warnings(false)
	.define("BUILDING_SYNTHIZER", None)
	.define("WDL_RESAMPLE_TYPE", Some("float"))
	.include("vendor/synthizer/include")
	.include("vendor/synthizer/third_party/miniaudio")
	.include("vendor/synthizer/third_party/wdl")
	.include("vendor/synthizer/third_party/cpp11-on-multicore/common")
	.include("vendor/synthizer/third_party/plf_colony")
	.include("vendor/synthizer/third_party/concurrentqueue")
	.include("vendor/synthizer/third_party/dr_libs")
	.include("vendor/synthizer/third_party/stb")
	.file("vendor/synthizer/third_party/wdl/WDL/resample.cpp")
	.compile("synthizer");
}