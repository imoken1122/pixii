use std::path::Path;
use std::{fs};
use image::{self,GenericImageView, DynamicImage};
use image::imageops::FilterType;
use clap::{Parser};
use ansi_term::Color;


const ASCII_CHARS: &[char] = &['.', ',', ':','"', '`', '^',
 '~', '=', '*', 'i', 'l', 'i', 'r', 't', 'f', '?', 
 'v', 'c', 'o', 'n', 'z', 'x', 'e', 's', 'a', 'p', 'g', '7',
  'L', 'J', '0', '4', 'T', 'Y', 'b', 'k', 'R', 'H', '5', '6', 
  'A', 'E', '9', 'B', '&', '#', '%', '@', '$', 'N', 'W', 'M'];



#[derive(Parser)]
struct Args{
    //converting to ascii art is image file 
    #[arg(short='i',long="image",value_name = "PATH")]
    image_file : String,

    //resize width
    #[arg(short='x',long="width",value_name="RESIZE_WIDTH")]
    resize_width: Option<u32>,
    //resize height
    #[arg(short='y',long="height",value_name="RESIZE_HEIGHT")]
    resize_height: Option<u32>,

    //When image resize, specify the resize ratio(0<ratio<1)
    #[arg(short='r',long="ratio",value_name="RESIZE_RATIO")]
    resize_ratio : Option<f64>,

    //image pixels convert to char in text file
    #[arg(short='t',long="text",value_name="PATH")]
    text_path: Option<String>
}


struct AsciiArtGenerator {
    img: DynamicImage,
    width:u32,
    height:u32,
}

impl AsciiArtGenerator {
    fn new(img: DynamicImage) -> Self{
        let (width,height) = img.clone().dimensions();
        Self{ img,width,height }

    }
    fn generate_ascii_art(&self) {
        
        for j in 0..self.height{
            for i in 0..self.width{
                let (r,g,b,a) = self.get_rgb(&self.img, i, j);
                let brightness= self.rgb_to_brightness(r, g, b).unwrap() as u8;

                let mut ascii_char = ' '; 
                if a >= 100 { 
                    ascii_char = self.convert_brightness_to_ascii(brightness);
                }
                let colored_char = Color::RGB(r, g, b).paint(ascii_char.to_string());
                print!("{}{}", colored_char,colored_char);
            }
            println!();
        }
    }

    fn generate_ascii_art_by_text(&self,text:String) {
        
        let text_len = text.len();
        let mut index = 0;
        for j in 0..self.height{
            for i in 0..self.width{
                let (r,g,b,a) = self.get_rgb(&self.img, i, j);

                let mut ascii_char = ' '; 
                if a >= 100 { 
                    for _ in 0..2{
                        ascii_char = text.chars().nth(index).unwrap();
                        index = (index + 1) % text_len;
                        let colored_char = Color::RGB(r, g, b).paint(ascii_char.to_string());
                        print!("{}", colored_char);
                    }
                } else{

                    let colored_char = Color::RGB(r, g, b).paint(ascii_char.to_string());
                    print!("{}{}",colored_char, colored_char);
                }
            }
            println!();
        }
    }

    fn convert_brightness_to_ascii(&self, brightness: u8) -> char {
        let ascii_char_len = ASCII_CHARS.len();
        let char_index = (ascii_char_len as f32 * (brightness as f32 / 255.0)) as usize;
        if char_index == ascii_char_len {
            ASCII_CHARS[ascii_char_len - 1]
        } else {
            ASCII_CHARS[char_index]
        }
    }
    

    fn get_rgb(&self,img: &DynamicImage, i: u32, j: u32) -> (u8, u8, u8, u8) {
        let pixel = img.get_pixel(i, j) ;
        (pixel[0], pixel[1], pixel[2], pixel[3])
    }

    fn rgb_to_brightness(&self,r:u8,g:u8,b:u8) -> Result<f64,String>{

        let gray= 0.299 * r as f64  + 0.587 *g as f64 + 0.114 *b as f64;
        Ok(gray)

}

}



fn resize_with_dimentions(img:DynamicImage, resize_width:Option<u32>,resize_height:Option<u32>) -> Result<DynamicImage,String>{
    let (org_w,org_h) = img.dimensions();
    let (new_w,new_h) = match (resize_width,resize_height){
        (Some(w), None) => {
            let h = ((org_h as f32/org_w as f32) * w as f32) as u32;
            (w,h)
        }
        (None, Some(h)) => {
            let w = ((org_w as f32/org_h as f32) * h as f32) as u32;
            (w,h)
        }
        (Some(w), Some(h)) => {
            (w,h)
        }
        (None, None) => return Err("Either width or height must be specified for resizing.".to_string()),

    };
    if new_w ==0 || new_h == 0{
        return Err("resize width or height is 0".to_string())
    }
    let resize_img = img.resize_exact(new_w, new_h, FilterType::Lanczos3);
    Ok(resize_img)

   
}

fn resize_with_ratio(img:DynamicImage, resize_ratio:f64) -> Result<DynamicImage,String>{
    if 0.>=resize_ratio || resize_ratio >1.{
        panic!("resize_ratio must spcify ratio that range 0.0 < ratio < 1.0");
    }
    let (width,height) = img.dimensions();
    let new_width = (width as f64 * resize_ratio) as u32;
    let new_height = (height as f64 * resize_ratio) as u32;
    if  new_width  > 1 ||  new_height > 1{
        let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);
        Ok(resized_img)
    }
    else{
      Err("The width or height after resizing is less than 1 for the given resize_ratio".to_string()) 
    }
}


fn load_img(path:&String) -> Result<DynamicImage, String> {
    let image_path = Path::new(path);
    let img = image::open(image_path).unwrap_or_else(|err|{
        panic!("Failed to load image: {}",err);
    });
    Ok(img)
}

fn load_text(path:String)->Result<String,String> {
    let text = fs::read_to_string(path).unwrap();
    let filtered_text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    if filtered_text.len() > 0{
        Ok(filtered_text)
    }
    else{
        Err("Failed load text file".to_string())
    }

}

fn main() {
    let args = Args::parse();
    let img =match load_img(&args.image_file){
                            Ok(img)=>img,
                            Err(err) => panic!("Failed to load img: {} ",err)
                            };

    
    let resized_img = if let Some(ratio) = args.resize_ratio {
        match resize_with_ratio(img, ratio) {
            Ok(img) => img,
            Err(err) => panic!("Failed to resize image: {}", err),
        }
    } else {
        match (args.resize_width, args.resize_height) {
            (width, height) => match resize_with_dimentions(img, width, height) {
                Ok(img) => img,
                Err(err) => panic!("Failed to resize image: {}", err),
            },
            _ => img,
        }
    };
    
    let (w,h) = &resized_img.dimensions();

    let text = match args.text_path{
        Some(p) => match load_text(p){
            Ok(text) => text,
            Err(err) => panic!("Failed to load text: {} ",err)
        },
        None => "".to_string()
    };

    let converter = AsciiArtGenerator::new(resized_img);
    if text.len() == 0 {
        converter.generate_ascii_art();
    }
    else{
        converter.generate_ascii_art_by_text( text);

    }

    println!("w:{} h:{}",w,h);


}
