use std::env;
use std::fs;
use std::io;

fn main() {
    let sm_1_10 = vec![85, 137, 229, 83, 87, 86, 129, 236, 44, 1, 0, 0, 139, 69, 8];
    let sm_1_10_windows = vec![
        85, 139, 236, 106, 255, 104, -1, -1, -1, -1, 100, 161, 0, 0, 0, 0, 80, 129, 236, 52, 1, 0,
        0,
    ];
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_start_message();
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    let file_name = &args[1];
    let mut content = fs::read(file_name).expect("Файл не найден");

    if file_name.ends_with(".dll") {
        let idx = find_start_index(&content, sm_1_10_windows);

        if idx == 0 || content[idx + 128] != 117 || content[idx + 129] != 19 {
            if content[idx + 128] == 116 {
                print_error_message("файл уже пропатчен.");
            } else {
                print_error_message("не найдена сигнатура, версия не поддерживается.");
            }
            io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
        content[idx + 128] = 116;
    } else {
        let idx = find_start_index(&content, sm_1_10);

        if idx == 0 || content[idx + 57] != 15 || content[idx + 58] != 132 {
            if content[idx + 58] == 133 {
                print_error_message("файл уже пропатчен.");
            } else {
                print_error_message("не найдена сигнатура, версия не поддерживается.");
            }
            io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
        content[idx + 58] = 133;
    }

    print_success_message();
    let res = format!("{}{}", file_name, "_patched");
    fs::write(res, content).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn print_start_message() {
    println!("------------------[SM PLUGINS LIST PATCHER]-------------------");
    println!("# Автор: PainteR");
    println!("# Специально для HLMOD.RU");
    println!("# Описание: Утилита патчит бинарник sourcemod");
    println!("# Поддержка SM: 1.10, 1.11, Windows + Linux");
    println!("---------------------------------------------------------------");
    println!("");
    println!("Вам нужно перенести на файл (.exe) свой (sourcemod.2.[GAME].[so|dll]) для патча.");
    println!("Отображение списка плагинов блокируется ТОЛЬКО НА КЛИЕНТЕ.");
    println!("");
    println!("Для продолжения нажмите любую клавишу...");
}

fn print_success_message() {
    println!("---------------------------[SUCCESS]-----------------------------");
    println!("# Успех: Файл успешно пропатчен.");
    println!("---------------------------------------------------------------");
    println!("");
    println!("Для продолжения нажмите любую клавишу...");
}

fn print_error_message(msg: &str) {
    println!("---------------------------[ERROR]-----------------------------");
    println!("# Ошибка: {}", msg);
    println!("---------------------------------------------------------------");
    println!("");
    println!("Для продолжения нажмите любую клавишу...");
}

fn find_start_index(arr: &Vec<u8>, search: Vec<i16>) -> usize {
    let mut ti = 0;
    for i in 0..arr.len() {
        if search[ti] == -1 {
            ti = ti + 1;
        } else {
            if search[ti] == arr[i] as i16 {
                ti = ti + 1;
            } else {
                ti = 0;
            }
        }
        if ti == search.len() {
            return (i - search.len() + 1) as usize;
        }
    }
    return 0;
}
