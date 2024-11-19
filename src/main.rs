use screenshots::Screen;
use tesseract::Tesseract;
use rdev::{simulate, EventType, Key, SimulateError};
use std::thread;
use std::time::Duration;

fn preprocess_image(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?.to_luma8(); // グレースケール化
    let mut processed = img.clone();

    for pixel in processed.pixels_mut() {
        pixel[0] = if pixel[0] > 128 { 255 } else { 0 };
    }

    processed.save(output_path)?;
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 画面キャプチャ
    let screen = Screen::all().unwrap();
    let screenshot = screen[0].capture_area(772, 425, 350, 30)?;
    screenshot.save("screenshot.png")?;
    println!("Screenshot saved!");

    // 2. 画像の前処理
    preprocess_image("screenshot.png", "processed_screenshot.png")?;
    println!("Image preprocessed!");

    // 3. OCRで文字認識
    let mut tess = Tesseract::new(Some("./"), Some("eng"))?;
    tess = tess.set_image("processed_screenshot.png")?;
    let mut extracted_text = tess.get_text()?;
    println!("Extracted text before filter: {}", extracted_text);

    // 4. 最初の文字が`!`なら削除（ただし、変数の中身が`!`だけの場合はそのまま）
    if extracted_text.starts_with('!') && extracted_text.len() > 1 {
        extracted_text = extracted_text[1..].to_string();
    }
    println!("Extracted text after filter: {}", extracted_text);

    // 5. 自動タイピング
    let filtered_text: String = extracted_text.chars().filter(|c| c.is_ascii()).collect();
    for c in filtered_text.chars() {
        if let Some((key, shift)) = char_to_key(c) {
            type_key(key, shift)?; // 修飾キーを考慮してキー入力
            thread::sleep(Duration::from_millis(60)); // 遅延
        }
    }
    println!("Typing completed!");

    Ok(())
}


fn char_to_key(c: char) -> Option<(Key, bool)> {
    match c {
        'a' => Some((Key::KeyA, false)),
        'b' => Some((Key::KeyB, false)),
        'c' => Some((Key::KeyC, false)),
        'd' => Some((Key::KeyD, false)),
        'e' => Some((Key::KeyE, false)),
        'f' => Some((Key::KeyF, false)),
        'g' => Some((Key::KeyG, false)),
        'h' => Some((Key::KeyH, false)),
        'i' => Some((Key::KeyI, false)),
        'j' => Some((Key::KeyJ, false)),
        'k' => Some((Key::KeyK, false)),
        'l' => Some((Key::KeyL, false)),
        'm' => Some((Key::KeyM, false)),
        'n' => Some((Key::KeyN, false)),
        'o' => Some((Key::KeyO, false)),
        'p' => Some((Key::KeyP, false)),
        'q' => Some((Key::KeyQ, false)),
        'r' => Some((Key::KeyR, false)),
        's' => Some((Key::KeyS, false)),
        't' => Some((Key::KeyT, false)),
        'u' => Some((Key::KeyU, false)),
        'v' => Some((Key::KeyV, false)),
        'w' => Some((Key::KeyW, false)),
        'x' => Some((Key::KeyX, false)),
        'y' => Some((Key::KeyY, false)),
        'z' => Some((Key::KeyZ, false)),
        'A' => Some((Key::KeyA, true)),
        'B' => Some((Key::KeyB, true)),
        'C' => Some((Key::KeyC, true)),
        'D' => Some((Key::KeyD, true)),
        'E' => Some((Key::KeyE, true)),
        'F' => Some((Key::KeyF, true)),
        'G' => Some((Key::KeyG, true)),
        'H' => Some((Key::KeyH, true)),
        'I' => Some((Key::KeyI, true)),
        'J' => Some((Key::KeyJ, true)),
        'K' => Some((Key::KeyK, true)),
        'L' => Some((Key::KeyL, true)),
        'M' => Some((Key::KeyM, true)),
        'N' => Some((Key::KeyN, true)),
        'O' => Some((Key::KeyO, true)),
        'P' => Some((Key::KeyP, true)),
        'Q' => Some((Key::KeyQ, true)),
        'R' => Some((Key::KeyR, true)),
        'S' => Some((Key::KeyS, true)),
        'T' => Some((Key::KeyT, true)),
        'U' => Some((Key::KeyU, true)),
        'V' => Some((Key::KeyV, true)),
        'W' => Some((Key::KeyW, true)),
        'X' => Some((Key::KeyX, true)),
        'Y' => Some((Key::KeyY, true)),
        'Z' => Some((Key::KeyZ, true)),
        '0' => Some((Key::Num0, false)),
        '1' => Some((Key::Num1, false)),
        '2' => Some((Key::Num2, false)),
        '3' => Some((Key::Num3, false)),
        '4' => Some((Key::Num4, false)),
        '5' => Some((Key::Num5, false)),
        '6' => Some((Key::Num6, false)),
        '7' => Some((Key::Num7, false)),
        '8' => Some((Key::Num8, false)),
        '9' => Some((Key::Num9, false)),
        // ' ' => Some((Key::Space, false)),
        '-' => Some((Key::Minus, false)),  // ハイフン
        '?' => Some((Key::Slash, true)),  // クエスチョンマーク (Shift + /)
        '!' => Some((Key::Num1, true)),   // エクスクラメーションマーク (Shift + 1)
        ',' => Some((Key::Comma, false)),
        // '.' => Some((Key::Dot, false)),
        _ => None,
    }
}

fn type_key(key: Key, shift: bool) -> Result<(), SimulateError> {
    if shift {
        simulate(&EventType::KeyPress(Key::ShiftLeft))?; // Shiftキーを押す
    }

    simulate(&EventType::KeyPress(key))?; // 実際のキーを押す
    simulate(&EventType::KeyRelease(key))?; // 実際のキーを離す

    if shift {
        simulate(&EventType::KeyRelease(Key::ShiftLeft))?; // Shiftキーを離す
    }

    Ok(())
}
