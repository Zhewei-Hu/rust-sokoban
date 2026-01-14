use ggez::{
    graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment},
    Context,
};
use glam::Vec2;
use hecs::{Entity, World};
use std::collections::HashMap;

use crate::components::*;
use crate::constants::*;

pub fn run_rendering(world: &World, context: &mut Context) {
    // 清空屏幕并设置背景色
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));

    // 纹理缓存 - 只在第一次使用时加载图像
    let mut texture_cache: HashMap<String, Image> = HashMap::new();

    // 获取所有可渲染的实体及其位置，按 z 轴排序
    // 这样可以实现正确的图层显示
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    // 遍历所有位置和可渲染组件对，加载图像并在指定位置绘制
    for (_, (position, renderable)) in rendering_data.iter() {
        // 从缓存中获取或加载图像
        let image = texture_cache
            .entry(renderable.path.clone())
            .or_insert_with(|| Image::from_path(context, renderable.path.clone()).unwrap());

        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        // 绘制
        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        canvas.draw(image, draw_params);
    }

    // ANCHOR: draw_gameplay_state
    // 渲染游戏状态文本
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;
    draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
    draw_text(&mut canvas, &gameplay.moves_count.to_string(), 525.0, 100.0);
    // ANCHOR_END: draw_gameplay_state

    let fps = format!("FPS: {:.0}", context.time.fps());
    draw_text(&mut canvas, &fps, 525.0, 120.0);

    // 呈现画布，将所有内容显示在屏幕上
    canvas.finish(context).expect("expected to present");
}

// ANCHOR: draw_text
pub fn draw_text(canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
    let text = Text::new(TextFragment {
        text: text_string.to_string(),
        color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
        scale: Some(PxScale::from(20.0)),
        ..Default::default()
    });

    canvas.draw(&text, Vec2::new(x, y));
}
// ANCHOR_END: draw_text
