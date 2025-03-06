/*
By: <Your Name Here>
Date: 2025-03-05
Program Details: <Program Description Here>
*/
use macroquad::prelude::*;
mod objects {
    pub mod txt_buttons;
}
//use objects::grid::draw_grid;
use objects::txt_buttons::TextButton;
#[macroquad::main("tiktac")]
async fn main() {
    let mut buttons = 0;
    let mut playing: bool = true;
    let mut xwin: i32 = 0;
    let mut owin: i32 = 0;
    let mut ties: i32 = 0;
    let mut turn: &str = "X";
    let mut winner = "".to_string();
    let text1 = TextButton::new(200.0, 100.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let text2 = TextButton::new(300.0, 100.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let text3 = TextButton::new(400.0, 100.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let text4 = TextButton::new(200.0, 200.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let text5 = TextButton::new(300.0, 200.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let text6 = TextButton::new(400.0, 200.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let text7 = TextButton::new(200.0, 300.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let text8 = TextButton::new(300.0, 300.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let text9 = TextButton::new(400.0, 300.0, 100.0, 100.0, "".to_string(), BLUE, GREEN);
    let reset = TextButton::new(50.0, 200.0, 100.0, 100.0, "".to_string(), RED, GRAY);
    let mut btn_list = vec![text1, text2, text3, text4, text5, text6, text7, text8, text9];
    loop {
        clear_background(BLACK);
        draw_text(&format!("turn is {}", turn), 20.0, 20.0, 30.0, WHITE);
        draw_text(&format!("xwins {}", xwin), 20.0, 40.0, 30.0, WHITE);
        draw_text(&format!("owins {}", owin), 20.0, 60.0, 30.0, WHITE);
        draw_text(&format!("ties {}", ties), 20.0, 80.0, 30.0, WHITE);
        if reset.click() {
            for btn in &mut btn_list {
                btn.text = "".to_string();
                btn.enabled = true;
            }
            buttons = 0;
            playing = true;
            winner = "".to_string();
        }
        for btn in &mut btn_list {
            if btn.click() {
                buttons += 1;
                if turn == "X" {
                    btn.text = "X".to_string();
                    btn.enabled = false;
                    turn = "O"
                } else if turn == "O" {
                    btn.text = "O".to_string();
                    btn.enabled = false;
                    turn = "X"
                }
            }
        }
        draw_text("RESET", 65.0, 255.0, 30.0, WHITE);
        for i in (0..9).step_by(3) {
            if btn_list[i].text != "" && btn_list[i].text == btn_list[i + 1].text && btn_list[i + 2].text == btn_list[i].text && playing {
                btn_list = off(btn_list);
                if turn == "O" {
                    winner = "X WINS".to_string();
                    xwin = xwin + 1;
                } else if turn == "X" {
                    winner = "O WINS".to_string();
                    owin = owin + 1;
                }
                playing = false;
            }
        }
        for i in (0..3).step_by(1) {
            if btn_list[i].text != "" && btn_list[i].text == btn_list[i + 3].text && btn_list[i + 6].text == btn_list[i].text && playing {
                btn_list = off(btn_list);
                if turn == "O" {
                    winner = "X WINS".to_string();
                    xwin = xwin + 1;
                } else if turn == "X" {
                    winner = "O WINS".to_string();
                    owin = owin + 1;
                }
                playing = false
            }
        }
        if btn_list[0].text != "" && btn_list[4].text == btn_list[0].text && btn_list[8].text == btn_list[0].text && playing {
            btn_list = off(btn_list);
            if turn == "X" {
                winner = "O WINS".to_string();
                owin = owin + 1;
            } else if turn == "O" {
                winner = "X WINS".to_string();
                xwin = xwin + 1;
            }
            playing = false
        } else if btn_list[2].text != "" && btn_list[4].text == btn_list[2].text && btn_list[6].text == btn_list[2].text && playing {
            btn_list = off(btn_list);
            if turn == "X" {
                winner = "O WINS".to_string();
                owin = owin + 1;
            } else if turn == "O" {
                winner = "X WINS".to_string();
                xwin = xwin + 1;
            }
            playing = false
        }
        if turn == "X" {
            if btn_list[0].text == "O" && btn_list[4].text == btn_list[0].text && btn_list[8].text == btn_list[0].text && playing {
                btn_list = off(btn_list);
                winner = "O WINS".to_string();
                owin = owin + 1;
                playing = false;
            } else if btn_list[2].text == "O" && btn_list[4].text == btn_list[2].text && btn_list[6].text == btn_list[2].text && playing {
                btn_list = off(btn_list);
                winner = "O WINS".to_string();
                owin = owin + 1;
                playing = false;
            }
        }
        if playing && buttons == 9 {
            winner = "Tied".to_string();
            ties += 1;
            playing = false;
        }
        draw_text(&format!("{}", winner), 20.0, 100.0, 30.0, WHITE);

        next_frame().await;
    }
}
// Function that turns off 9 buttons
fn off(mut btn_list: Vec<TextButton>) -> Vec<TextButton> {
    // Set all buttons' enabled flag to false
    for btn in &mut btn_list {
        btn.enabled = false;
    }

    // Return the modified buttons
    btn_list
}
