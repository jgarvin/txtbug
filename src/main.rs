#![deny(unused_must_use)] // otherwise can ignore errors from void functions
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Color, FontRenderer, Image, PixelFormat, Surface, VectorFont},
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            size: Vector::new(600.0, 480.0),
            title: "AGame",
            ..Settings::default()
        },
        app,
    );
}

struct RenderTools<'a> {
    gfx: &'a mut Graphics,
    font: &'a mut FontRenderer,
    size: Vector,
}

struct TextChunk {
    position: Vector,
    size: Vector,
    text: String,
}

impl TextChunk {
    fn new(render_tools: &mut RenderTools, text: String) -> Result<TextChunk> {
        let mut chunk = TextChunk {
            position: Vector::new(0.0, 0.0),
            size: Vector::new(0.0, 0.0),
            text: text,
        };
        chunk.size = render_tools.font.layout_glyphs(
            render_tools.gfx,
            &chunk.text,
            None,
            //render_tools.size.x.into(),
            |gfx, layout| {},
        )?;
        dbg!(chunk.size, chunk.text.len(), &chunk.text);
        return Ok(chunk);
    }

    fn render(&mut self, render_tools: &mut RenderTools) -> Result<()> {
        self.size = render_tools.font.draw(
            render_tools.gfx,
            &*self.text,
            //render_tools.size.x.into(),
            Color::BLACK,
            self.position,
        )?;
        Ok(())
    }
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
    let ttf = VectorFont::load("retro.ttf").await?;

    let mut font = ttf.to_renderer(&gfx, 32.0)?;
    gfx.fit_to_window(&window);

    let mut render_tools = RenderTools {
        gfx: &mut gfx,
        font: &mut font,
        size: window.size().clone(),
    };

    loop {
        while let Some(_) = input.next_event().await {}
        render_tools.gfx.clear(Color::WHITE);
        let mut chunk = TextChunk::new(&mut render_tools, "lab".into())?;
        if chunk.size.y == 0.0 {
            break;
        }
        chunk.position.y = 200.0;
        chunk.render(&mut render_tools)?;
        render_tools.gfx.present(&window)?;
    }

    Ok(())
}
