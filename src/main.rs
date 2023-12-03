#![windows_subsystem = "windows"]
use fastrand;
use goldberg::{goldberg_stmts, goldberg_int};
use mki::Keyboard;
use rsautogui;
use rusty_tesseract::*;
use screenshots::Screen;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

lazy_static::lazy_static! {
    static ref NUM_OF_SCROLLS: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}
fn main() {
    loop {
        start_listen();
    }
}
fn start_listen() {
    Keyboard::X.bind(|_| {
        let num = fastrand::u64(0..250);
        thread::sleep(Duration::from_millis(num));
        let dis = set_dis();
        let scrolls = adjust_game_to_new_dis(&dis) as usize;
         *goldberg_stmts!(NUM_OF_SCROLLS.lock().unwrap()) = scrolls;
        thread::sleep(Duration::from_millis(goldberg_int!(250)));
    });

    Keyboard::BackwardSlash.bind(|_| {
        for _ in goldberg_stmts!(goldberg_int!(0)..*goldberg_stmts!(NUM_OF_SCROLLS.lock().unwrap())) {
             let delay = fastrand::u64(10..150);
            thread::sleep(Duration::from_millis(delay));
            rsautogui::mouse::scroll(rsautogui::mouse::ScrollAxis::Y, goldberg_int!(-1));
        }
    });
}

fn generate_random_string(length: usize) -> String {
    let random_string: String = (0..length).map(|_| fastrand::alphanumeric()).collect();

    random_string
}

fn adjust_game_to_new_dis(dis: &str) -> i32 {
    let dis_to_scroll: HashMap<i32, i32> = HashMap::from_iter(vec![
        (0, 0),
        (50, 1),
        (100, 2),
        (200, 3),
        (300, 4),
        (400, 5),
        (500, 6),
        (600, 7),
        (700, 8),
        (800, 9),
        (900, 10),
        (1000, 11),
    ]);
    let mut scrolls = goldberg_int!(0);
    let delay = fastrand::u64(0..250);
    let dis_num = dis.parse::<i32>().unwrap_or(0);
    let nearest_scroll_num = dis_to_scroll.keys().min_by_key(|&x| (x - dis_num).abs());
    if dis_num < 200 {
        return 0;
    }
    if let Some(&nearest) = nearest_scroll_num {
        scrolls = dis_to_scroll[&nearest];
        for _ in 0..scrolls {
            let delay = fastrand::u64(10..50);
            thread::sleep(Duration::from_millis(delay));
            rsautogui::mouse::scroll(rsautogui::mouse::ScrollAxis::Y, 1);
        }
        goldberg_stmts!{Keyboard::LeftAlt.press()};
    }
    thread::sleep(Duration::from_millis(delay));
    Keyboard::LeftAlt.release();
    return scrolls;
}

fn set_dis() -> String {
    let area = generate_random_string(10) + ".png";
    let white_mask = generate_random_string(10) + ".png";
    screenshot(&area);
    show_image(&area, &white_mask);
    let dis = get_dis(&white_mask);
    clean_up(&area, &white_mask);
    return dis;
}

fn screenshot(filename: &str) {
    let screen = Screen::from_point(100, 100).unwrap();
    let area = screen.capture_area(1010, 410, 100, 100).unwrap();
    area.save(&filename).unwrap();
}

fn show_image(area_filename: &str, white_mask_filename: &str) {
    let img = opencv::imgcodecs::imread(area_filename, opencv::imgcodecs::IMREAD_COLOR).unwrap();
    let mut gray_img = opencv::core::Mat::default();
    opencv::imgproc::cvt_color(&img, &mut gray_img, opencv::imgproc::COLOR_BGR2GRAY, 0).unwrap();
    let mut white_mask = opencv::core::Mat::default();
    opencv::core::in_range(
        &gray_img,
        &opencv::core::Scalar::new(200.0, 200.0, 200.0, 0.0),
        &opencv::core::Scalar::new(255.0, 255.0, 255.0, 0.0),
        &mut white_mask,
    )
    .unwrap();
    opencv::imgcodecs::imwrite(
        &white_mask_filename,
        &white_mask,
        &opencv::core::Vector::new(),
    )
    .unwrap();
    // let window = "image";
    // opencv::highgui::named_window(window, 0).unwrap();
    // opencv::highgui::imshow(window, &white_mask).unwrap();
    // opencv::highgui::wait_key(0).unwrap();
    // opencv::highgui::destroy_window(window).unwrap();
}
fn clean_up(areafilename: &str, white_mask_filename: &str) {
    std::fs::remove_file(&areafilename).unwrap();
    std::fs::remove_file(&white_mask_filename).unwrap();
}
fn get_dis(white_mask_filename: &str) -> String {
    let img = Image::from_path(&white_mask_filename).unwrap();
    let my_args = Args {
        lang: "eng".to_string(),
        config_variables: HashMap::from_iter(vec![(
            "tessedit_char_whitelist".to_string(),
            "0123456789".to_string(),
        )]),
        dpi: Some(150),
        psm: Some(6),
        oem: Some(3),
    };
    let output = image_to_string(&img, &my_args).unwrap();
    return output.trim().to_string();
}
