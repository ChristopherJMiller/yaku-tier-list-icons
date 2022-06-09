use andrew::{Canvas, Endian, text::{Text, load_font_file}};
use image::ColorType;
use serde::{Serialize, Deserialize};
use std::fs::read_to_string;
use indicatif::ProgressBar;

#[derive(Serialize, Deserialize, Debug)]
pub struct Yaku {
  pub name: String,
  pub han: u32,
  pub closed_only: bool,
  pub additional_on_closed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YakuFile {
  pub yaku: Vec<Yaku>
}

fn draw_icon(font: &Vec<u8>, yaku: Yaku) {
  let (x, y) = (256, 256);
  let mut buf: Vec<u8> = vec![255; 4 * x * y];

  let mut canvas = Canvas::new(&mut buf, x, y, 4 * x, Endian::Little);
  let num_of_lines = yaku.name.clone().split(" ").collect::<Vec<&str>>().len();
  const WORD_HEIGHT: usize = 48;
  for (i, word) in yaku.name.clone().split(" ").enumerate() {
    let name_text = Text::new((5, y - (WORD_HEIGHT * (num_of_lines - i)) - 5), [255, 0, 0, 0], font, WORD_HEIGHT as f32, 1.0, word);
    canvas.draw(&name_text); 
  }

  let han = if yaku.han > 6 {
    "Y".to_string()
  } else {
    if yaku.additional_on_closed.is_some() {
      format!("{}/{}", yaku.han, yaku.han + 1)
    } else {
      format!("{}", yaku.han)
    }
  };

  let han_text = if yaku.additional_on_closed.is_some() {
    Text::new((240 - (42 * 3), 2), [255, 0, 0, 0], font, WORD_HEIGHT as f32 * 2.0, 1.0, han)
  } else {
    Text::new((240 - (36 * 2), 2), [255, 0, 0, 0], font, WORD_HEIGHT as f32 * 2.0, 1.0, han)
  };
  canvas.draw(&han_text); 

  image::save_buffer(format!("output/{}.png", yaku.name), &buf, x as u32, y as u32, ColorType::Rgba8).unwrap();
}

fn main() {
  let yaku_file = read_to_string("yaku.toml").unwrap();
  let yakus: YakuFile = toml::from_str::<YakuFile>(&yaku_file).unwrap();

  let font = load_font_file("font/MochiyPopPOne-Regular.ttf");

  let bar = ProgressBar::new(yakus.yaku.len() as u64);
  for yaku in yakus.yaku {
    bar.inc(1);
    draw_icon(&font, yaku);
  }
}
