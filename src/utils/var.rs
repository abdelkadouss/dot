use std::sync::MutexGuard;

use crate::var::Var;

pub fn format_string_using_vars(str: &mut String, vars: MutexGuard<Vec<Var>>) {
    for var in vars.iter() {
        *str = str.replace(&format!("?{}", var.name), &var.value.to_string());
    }
}
