/*
 * TODO:
 * Implement Colour changing capabilities
 * Implement Scroll interaction (Scrolling changes colour)
 */ 


extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


fn main() {
  
 let ctx = sdl2::init().unwrap();
 let ctx_v = ctx.video().unwrap();

 let win = ctx_v.window("Colour Scroll",800,600).position_centered().build().unwrap();

 let mut canvas = win.into_canvas().build().unwrap();

 canvas.set_draw_color(Color::RGB(0,0,255));
 canvas.clear();
 canvas.present();

 let mut event_pump = ctx.event_pump().unwrap();
    
 let mut i:i32 = 0;
 let mut col_iter:u8 = 0;

'run : loop {

        for event in event_pump.poll_iter() {
       
            match event{
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    break 'run 
                },|
            
                Event::MouseWheel {y, ..} => {
                    
                    i += y*2;

                    if y > 0{

                        if i > 255 {
                            
                            i = 0;
                            col_iter = 0;

                        }else {


                            col_iter += 2;
                        }


                    }else if y < 0 {

                        if i < 0 {

                            i = 255;
                            col_iter = 255;
                            
                        }else{

                            col_iter -= 2;
                    }
                    
                }else {
                    
                    panic!("Scrollwheel Evaluation Invalid!");

                }

                // println!("{} | {}",i,col_iter); *debug
                chg_colour( &mut canvas,col_iter);
        
            },

            _ => {}
            

        }


    
    }
        
    canvas.present()

    }
}

fn chg_colour(gx : &mut sdl2::render::WindowCanvas, mut i : u8){
   
    gx.set_draw_color(Color::RGB(i, 64, 255 - i));
    gx.clear();

}
