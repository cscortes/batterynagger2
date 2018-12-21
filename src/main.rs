extern crate piston_window;
extern crate sdl2_window;

use piston_window::*;
use sdl2_window::Sdl2Window;

// Window Properties
// ========================================================================
static WIN_WIDTH : u32 = 600;
static WIN_HEIGHT: u32 = 200;
static WIN_POS: [i32; 2]  = [10,10];
static BANNER_WIDTH: u32 = 392;

fn main() {
    // Build my window with SDL2 backend
    //
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("Battery Nagger 2", [WIN_WIDTH, WIN_HEIGHT])
        .build()
        .unwrap();

    window.set_position(WIN_POS);
    window.set_size([WIN_WIDTH,WIN_HEIGHT]);
    window.set_max_fps(4);
    window.set_ups(2);

    // sdl context for audio
    // 
    let sdl = window.window.sdl_context.to_owned();
        
    // Banner PNG for our app
    // 
    let banner_logo: G2dTexture = Texture::from_path(
            &mut window.factory,
            "assets/battery_nagger.png",
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    
    // Pull in FONTS
    // 
    let mut title_font = Glyphs::new("assets/monofonto.ttf", window.factory.clone(), TextureSettings::new()).unwrap();
    let mut stats_font = Glyphs::new("assets/5ceta_mono.ttf", window.factory.clone(), TextureSettings::new()).unwrap();

    let mut blinker = batteryinfo2::blinker::Blinker::new(1);
    let mut blogic  = batteryinfo2::batterylogic::BatteryLogic::new(2);
    let hide_timeout = batteryinfo2::timer::TimerObject::new(5,Some(1));
    let show_timeout = batteryinfo2::timer::TimerObject::new(20,None);

    // Setup Drawing Objects
    //
    let b = 4.0;
    let headerbk = rectangle::rectangle_by_corners(0.0 + b, 76.0, WIN_WIDTH as f64 -b , 136.0);
    let rowbk = rectangle::rectangle_by_corners(0.0 + b, 136.0 + b, WIN_WIDTH as f64 - b, 136.0 + 60.0 );

    let banner_pos = [((WIN_WIDTH - BANNER_WIDTH) as f64 / 2.0), 0.0];

    // Color for Stats 
    let scolor = batteryinfo2::colordefs::COLOR_BLACK;

    let light_border = rectangle::rectangle_by_corners(12.0-2.0, 149.0-2.0, 52.0+2.0, 189.0+2.0 );
    let light_normal = rectangle::rectangle_by_corners(12.0, 149.0, 52.0, 189.0 );


    music::start_context::<batteryinfo2::soundutil::Music, batteryinfo2::soundutil::Sound, _>(&sdl, 16, || {
        while let Some(e) = window.next() {

            // Check all my timer events 
            //
            blogic.check_alarm_interval();

            // this fires a few seconds after the window has come up
            // just to get it out of the way.  It is basically minimized.
            //
            if hide_timeout.triggered() {
                window.window.window.minimize();
            }

            // This is meant to be annoying, it is to bring up the 
            // app to restore it every 30 seconds IF it is in the 
            // fatal state. 
            //
            if (blogic.get_status() == batteryinfo2::batterylogic::BatteryStatus::Fatal) && show_timeout.triggered() {
                window.window.window.restore();
                window.set_position(WIN_POS);
            }

            window.draw_2d(&e, |context, graphics| { 
                clear(batteryinfo2::colordefs::BKCOLOR, graphics);

                rectangle(batteryinfo2::colordefs::COLOR_BLACK, headerbk, context.transform, graphics);
                rectangle(batteryinfo2::colordefs::COLOR_GRAY, rowbk, context.transform, graphics);

                let banner_positioning = context.transform.trans(banner_pos[0], banner_pos[1]);
                image(&banner_logo, banner_positioning, graphics);

                // Title
                let transform = context.transform.trans(10.0, 114.0);
                text::Text::new_color(batteryinfo2::colordefs::COLOR_WHITE, 25).draw(
                    "Alarm   Type   Count Down   Status/Percent",
                    &mut title_font,
                    &context.draw_state,
                    transform, graphics
                ).unwrap();

                // Stats -- type
                let mytext = format!("Laptop");
                let transform = context.transform.trans(114.0, 174.0);
                text::Text::new_color(scolor, 20).draw(
                    mytext.as_str(),
                    &mut stats_font,
                    &context.draw_state,
                    transform, graphics
                ).unwrap();

                // Stats -- count down
                let mytext = format!("{:?}", blogic.get_countdown() );
                let transform = context.transform.trans(270.0, 174.0);
                text::Text::new_color(scolor, 20).draw(
                    mytext.as_str(),
                    &mut stats_font,
                    &context.draw_state,
                    transform, graphics
                ).unwrap();

                // Stats -- Percent/Status
                let mytext = format!("{:?} {:?}", blogic.get_battery_cap(), blogic.get_status() );

                let transform = context.transform.trans(424.0, 174.0);
                text::Text::new_color(scolor, 20).draw(
                    mytext.as_str(),
                    &mut stats_font,
                    &context.draw_state,
                    transform, graphics
                ).unwrap();

                // Light
                rectangle(batteryinfo2::colordefs::COLOR_WHITE, light_border, context.transform, graphics);
                let blinkcolor = blinker.get_blink_color(blogic.get_status());
                rectangle(blinkcolor, light_normal, context.transform, graphics);
            });
        }
    });
}
