mod from_isr
{
	macro_rules! info {
        ($($arg:tt)+) => ($crate::utils::log_in_isr::add_log(log::Level::Info, format!($($arg)+)))
    }

	pub(crate) use info;
}

pub(crate) use from_isr::info;
use log::Level;
use spin::Mutex;

static LOGS: Mutex<Vec<(Level, String)>> = Mutex::new(Vec::new());

pub(crate) fn add_log(level: Level, message: String)
{
	let mut logs = LOGS.lock();
	logs.push((level, message))
}

pub fn print_logs_from_isr()
{
	let mut logs = LOGS.lock();
	for (log_level, log_message) in logs.iter()
	{
		log::log!(log_level.clone(), "{}", log_message);
	}
	logs.clear();
}
