//! This module gives the ability to the printer to pause a print and resume it later.
//!
//! For now this module only stores the pause state in a static [`AtomicBool`], then other parts of this firmware make a decision on what
//! to do based on what [`is_paused`] returns.

use core::sync::atomic::{AtomicBool, Ordering};

static IS_PAUSED: AtomicBool = AtomicBool::new(false);

/// Makes [`is_paused`] return `false`.
///
/// # Examples
/// ```
/// # use firmware_core::printer::components::pauser;
/// #
/// pauser::resume();
/// assert_eq!(pauser::is_paused(), false);
/// ```
pub fn resume()
{
	IS_PAUSED.store(false, Ordering::Relaxed)
}

/// Makes [`is_paused`] return the opposite of what it would return now.
///
/// # Examples
/// ```
/// # use firmware_core::printer::components::pauser;
/// #
/// pauser::resume();
/// assert_eq!(pauser::is_paused(), false);
///
/// pauser::toggle_pause();
/// assert_eq!(pauser::is_paused(), true);
///
/// pauser::toggle_pause();
/// assert_eq!(pauser::is_paused(), false);
///
/// pauser::toggle_pause();
/// assert_eq!(pauser::is_paused(), true);
/// ```
pub fn toggle_pause()
{
	IS_PAUSED.fetch_xor(true, Ordering::Relaxed);
}

/// Returns `false` if you've never called any function in this module (as a default value) or if after calling them ([`resume`] and [`toggle_pause`])
/// you end up with `false` being the current state.
///
/// Check the [`module's`] documentation for more details.
///
/// # Examples
/// ```
/// # use firmware_core::printer::components::pauser;
/// #
/// assert_eq!(pauser::is_paused(), false);
///
/// pauser::resume();
/// assert_eq!(pauser::is_paused(), false);
///
/// pauser::resume();
/// assert_eq!(pauser::is_paused(), false);
///
/// pauser::toggle_pause();
/// assert_eq!(pauser::is_paused(), true);
/// ```
///
/// [`module's`]: self
pub fn is_paused() -> bool
{
	IS_PAUSED.load(Ordering::Relaxed)
}
