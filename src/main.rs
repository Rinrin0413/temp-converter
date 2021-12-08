// Modules
use std::io; // 入出力

// DB
const LINE:&str = "========================================";

fn main() {
    loop {
        println!("基準の温度単位を選んで下さい(a, b, c)");
        println!("a.摂氏温度(°C)");
        println!("b.華氏温度(°F)");
        println!("c.絶対温度(K)");
        let mut input_temp_unit = String::new();
        io::stdin().read_line(&mut input_temp_unit).expect("取得に失敗しました。");
        let input_temp_unit:&str = &input_temp_unit.trim(); // String型なので &strに変換
        
        match input_temp_unit { // trimメソッドで入力時の改行入力を除去
            "a" | "b" | "c" => {
                println!("温度を入力して下さい。");
                let mut input_temp_value = String::new();
                io::stdin().read_line(&mut input_temp_value).expect("取得に失敗しました。");
                let input_temp_value:f64 = match input_temp_value.trim().parse() { // 改行入力削除及び変換
                    Ok(num) => num,
                    _ => { println!("数値を入力して下さい"); continue }
                };

                println!("計算中...");
                const AVOA0:f64 = 273.15; // 絶対零度の絶対値
                let c_temp = match input_temp_unit {
                    "a" => { input_temp_value } // input_temp_value が既に摂氏温度
                    "b" => { (input_temp_value - 32.0)/1.8 } // 華氏 -> 摂氏
                    "c" => { input_temp_value - AVOA0 } // ケルビン -> 摂氏
                    _ => { err_msg(); continue }
                }; 
                let f_temp = match input_temp_unit {
                    "a" => { input_temp_value*1.8 + 32.0 } // 摂氏 -> 華氏
                    "b" => { input_temp_value } // 華氏温度そのもの
                    "c" => { (input_temp_value - AVOA0)*1.8 + 32.0 } // ケルビン -> 華氏
                    _ => { err_msg(); continue }
                };
                let k_temp = match input_temp_unit {
                    "a" => { input_temp_value + AVOA0 } // 摂氏 -> ケルビン
                    "b" => { (input_temp_value - 32.0)/1.8 + AVOA0 } // 華氏 -> ケルビン
                    "c" => { input_temp_value } // 絶対温度そのもの
                    _ => { err_msg(); continue }
                };

                let wheth_too_small = if k_temp < 0.0 { 
                    let msg = "※温度が絶対温度を下回っています。"; msg 
                } else {
                    let msg = "[正常に換算されました]"; msg
                };
                println!("{}", wheth_too_small);
                println!("摂氏温度 : {}°C", c_temp);
                println!("華氏温度 : {}°F", f_temp);
                println!("絶対温度 : {}K", k_temp);

                println!("\n換算を続けますか?[y, n]");
                let mut wheth_continue = String::new();
                io::stdin().read_line(&mut wheth_continue).expect("取得に失敗しました。");

                match wheth_continue.trim() {
                    "y" => continue,
                    _ => break
                };
            }
            _ => {
                println!("[無効な値]");
                println!("a または b または c を入力してください。");
                println!("{}", LINE);
                continue
            }
        }
    }
}

fn err_msg() {
    println!("計算に失敗しました\n{}", LINE);
}