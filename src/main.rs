use logics::*;

fn main() {
    loop {
        let base_temp_unit = match get_temp_unit() {
            Ok(u) => u,
            Err(_) => continue,
        };

        let base_temp_val = match get_temp_val() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let temps = match calc_temp(base_temp_unit, base_temp_val) {
            Ok(tmps) => tmps,
            Err(_) => continue,
        };

        temps.show_result();

        if ask_wanna_continue() {
            continue;
        } else {
            break;
        }
    }
}
