#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : BOOL) -> BOOL);
pub type BOOL = i32;
