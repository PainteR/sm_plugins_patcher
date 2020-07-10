use std::env;
use std::fs;
use std::io;

fn main() {
    let signatures: Vec<Vec<i16>> = vec![
        vec![85, 137, 229, 83, 87, 86, 129, 236, 44, 1, 0, 0, 139, 69, 8], // sm 1.10.0.6478
        vec![
            85, 83, 87, 86, 129, 236, 44, 1, 0, 0, 139, 132, 36, 64, 1, 0, 0, // sm 1.10.0.6455
        ],
        vec![
            85, 139, 236, 106, 255, 104, -1, -1, -1, -1, 100, 161, 0, 0, 0, 0, 80, 129, 236, 52, 1,
            0, 0, // windows sm 1.10
        ],
    ];

    let offsets: Vec<Vec<u8>> = vec![
        vec![57, 15, 58, 132, 133],   // sm 1.10.0.6478
        vec![51, 15, 52, 132, 133],   // sm 1.10.0.6455
        vec![128, 117, 129, 19, 116], // windows sm 1.10
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_start_message();
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    let file_name = &args[1];
    let mut content = fs::read(file_name).expect("Файл не найден");

    let mut find = false;

    for (index, _) in signatures.iter().enumerate() {
        let idx = find_start_index(&content, &signatures[index]);

        if idx == 0
            || content[idx + offsets[index][0] as usize] != offsets[index][1]
            || content[idx + offsets[index][2] as usize] != offsets[index][3]
        {
            if content[idx + offsets[index][0] as usize] == offsets[index][4] {
                print_error_message("файл уже пропатчен.");
                io::stdin().read_line(&mut String::new()).unwrap();
                return;
            }
        } else {
            println!(
                "index: {}, value: {}, current: {}",
                index,
                offsets[index][4],
                content[idx + offsets[index][2] as usize]
            );
            content[idx + offsets[index][2] as usize] = offsets[index][4];
            find = true;
            break;
        }
    }

    if !find {
        print_error_message(
            "не найдена сигнатура, версия не поддерживается, напишите в дискорд: PainteR#0327",
        );
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    print_success_message();
    let res = format!("{}{}", file_name, "_patched.so");
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

fn find_start_index(arr: &Vec<u8>, search: &Vec<i16>) -> usize {
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
