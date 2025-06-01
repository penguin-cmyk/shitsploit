use std::sync::Mutex;
use once_cell::sync::Lazy;
use memory_utils::process::Process;

#[allow(non_upper_case_globals)]
pub static process: Lazy<Process> = Lazy::new(|| Process::new(Process::pid("RobloxPlayerBeta.exe").unwrap()));
#[allow(non_upper_case_globals)]
pub static base_address: Lazy<usize> = Lazy::new(|| process.get_base_address().unwrap() as usize);
#[allow(non_upper_case_globals)]
pub static dm: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

#[allow(non_upper_case_globals)]
pub static loaded: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));