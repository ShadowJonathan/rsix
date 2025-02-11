//! Process-associated operations.

use crate::imp;

#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
mod sched;

#[cfg(not(target_os = "wasi"))]
pub use id::{getegid, geteuid, getgid, getpid, getppid, getuid};
pub use sched::sched_yield;

/// `EXIT_SUCCESS` for use with [`exit`].
///
/// [`exit`]: std::process::exit
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/stdlib.h.html
/// [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html
pub const EXIT_SUCCESS: i32 = imp::process::EXIT_SUCCESS;

/// `EXIT_FAILURE` for use with [`exit`].
///
/// [`exit`]: std::process::exit
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/stdlib.h.html
/// [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html
pub const EXIT_FAILURE: i32 = imp::process::EXIT_FAILURE;
