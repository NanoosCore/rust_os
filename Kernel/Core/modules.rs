// "Tifflin" Kernel
// - By John Hodge (thePowersGang)
//
// Core/modules.rs
// - Runtime-initialised modules (handling load order deps)
use _common::*;

#[repr(packed)]
#[allow(missing_copy_implementations)]
pub struct ModuleInfo
{
	pub name: &'static str,
	pub init: fn(),
	pub deps: &'static [&'static str],
	pub _rsvd: usize,
}

#[derive(Clone,PartialEq)]
enum ModuleState
{
	Uninitialised,
	Resolving,
	Initialised,
}

extern "C" {
	static modules_base: ();
	static modules_end: ();
}

/// Initialise statically linked modules
///
/// This is the core initialisation method for the kernel, called to initialise
/// the rest of the kernel.
///
/// `requests` is a list of modules that should be loaded as soon as possible (e.g. the GUI)
pub fn init(requests: &[&str])
{
	let baseptr = &modules_base as *const _ as *const ModuleInfo;
	let size = &modules_end as *const _ as usize - baseptr as usize;
	let count = size / ::core::mem::size_of::<ModuleInfo>();
	
	unsafe {
		let mods = ::core::slice::from_raw_parts(baseptr, count);
		init_modules(mods, requests);
	}
}

/// Initialise modules from a slice
fn init_modules(mods: &[ModuleInfo], requests: &[&str])
{
	log_debug!("s_modules={:?},{:#x}", mods.as_ptr(), mods.len());
	let mut modstates = Vec::from_elem(mods.len(), ModuleState::Uninitialised);
	for m in mods.iter() {
		log_debug!("mod = {:p} {:?} '{}'", &m.name, m.name.as_ptr(), m.name);
	}
	
	for req in requests
	{
		init_module_by_name(modstates.slice_mut(), mods, "", req);
	}
	
	for i in (0 .. mods.len())
	{
		init_module(modstates.slice_mut(), mods, i);
	}
}

/// Initialise a module by name, as required by another module
///
/// `req` = requesting module, `name` = required module
fn init_module_by_name(modstates: &mut [ModuleState], mods: &[ModuleInfo], req: &str, name: &str)
{
	// Locate module
	let depid = match mods.iter().enumerate().find( |&(_,v)| v.name == name ) {
		Some( (depid,_) ) => depid,
		None => panic!("Dependency '{}' for module '{}' missing", name, req),
		};
	// Check if not being initialised
	if modstates[depid] == ModuleState::Resolving {
		panic!("Circular dependency '{}' requires '{}' which is already being resolved", req, name);
	}
	
	// Initialise
	init_module(modstates, mods, depid);
}

/// Initialise a module (does nothing if the module is already initialised)
fn init_module(modstates: &mut [ModuleState], mods: &[ModuleInfo], i: usize)
{
	let module = &mods[i];
	if modstates[i] == ModuleState::Uninitialised
	{
		modstates[i] = ModuleState::Resolving;
		log_debug!("#{}: {} Deps", i, module.name);
		for name in module.deps.iter() {
			init_module_by_name(modstates, mods, module.name, *name);
		}
		// TODO: Do module initialisation in worker threads, and handle waiting for deps before calling init
		log_debug!("#{}: {} Init", i, module.name);
		(module.init)();
		modstates[i] = ModuleState::Initialised;
	}
}

// vim: ft=rust

