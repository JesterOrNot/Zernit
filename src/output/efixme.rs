// Created by Jacob Hrbek <kreyren@rixotstudio.cz> under GPL-3 license (https://www.gnu.org/licenses/gpl-3.0.en.html) in 2020

/*
	Output FIXME message for features that needs implementation

	SYNOPSIS: efixme [message]
*/

// FIXME: Export this in a library

pub fn efixme<S: AsRef<str>>(stringlike: S) {
	let str_ref = stringlike.as_ref();
	println!("FIXME: {:?}", str_ref)
}