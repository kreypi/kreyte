// Created by Jacob Hrbek <kreyren@rixotstudio.cz> under GPL-3 license (https://www.gnu.org/licenses/gpl-3.0.en.html) in 2020

/*
	Output FIXME message for features that needs implementation

	SYNOPSIS: efixme [message]
*/

// FIXME: Allow 'FIXME:' prefix to be configurable from 'EINFO_PREFIX' env variable

pub fn einfo<S: AsRef<str>>(stringlike: S) {
	let str_ref = stringlike.as_ref();
	println!("INFO: {:?}", str_ref)
}