use std::{fmt::Display, hash::Hash, sync::mpsc};

use crate::collections::HashGrid;

pub mod prelude {
    pub use super::{Color, GridShapeType, Renderer, ShapeRenderer};
}

pub trait ShapeRenderer {
    fn clear(&mut self);
    fn grid<K, D>(&mut self, grid: &HashGrid<K, D>)
    where
        K: Eq + Hash + Ord + Copy + TryInto<i16>,
        D: Display;
    fn grid_with<K, D, F>(&mut self, grid: &HashGrid<K, D>, cell_builder: F)
    where
        K: Eq + Hash + Ord + Copy + TryInto<i16>,
        F: FnMut(&D, &mut Text, &mut GridShape);
    fn grid_shape(&mut self, x: impl Into<f32>, y: impl Into<f32>);
    fn grid_shape_with<F>(&mut self, x: impl Into<f32>, y: impl Into<f32>, builder: F)
    where
        F: FnOnce(&mut GridShape);
    fn line(
        &mut self,
        x0: impl Into<f32>,
        y0: impl Into<f32>,
        x1: impl Into<f32>,
        y1: impl Into<f32>,
    );
    fn line_with<F>(
        &mut self,
        x0: impl Into<f32>,
        y0: impl Into<f32>,
        x1: impl Into<f32>,
        y1: impl Into<f32>,
        builder: F,
    ) where
        F: FnOnce(&mut Line);
    fn print(&mut self, content: impl Into<String>);
    fn print_with<F>(&mut self, content: impl Into<String>, builder: F)
    where
        F: FnOnce(&mut Text);
    fn println(&mut self, content: impl Into<String>);
    fn println_with<F>(&mut self, content: impl Into<String>, builder: F)
    where
        F: FnOnce(&mut Text);
    fn print_at<F>(
        &mut self,
        x: impl Into<f32>,
        y: impl Into<f32>,
        content: impl Into<String>,
        builder: F,
    ) where
        F: FnOnce(&mut Text);
}

pub enum RenderCommand {
    Clear,
    Text(Text),
    Shape(GridShape),
    Line(Line),
}

pub type RenderPipeline = mpsc::Sender<RenderCommand>;

pub struct Renderer {
    #[cfg(feature = "render")]
    cursor: (f32, f32),
    #[cfg(feature = "render")]
    render_pipeline: Option<RenderPipeline>,
}

impl Renderer {
    #[cfg(feature = "render")]
    pub(crate) fn new(render_pipeline: Option<RenderPipeline>) -> Self {
        Self {
            cursor: (0.0, 0.0),
            render_pipeline,
        }
    }

    #[cfg(feature = "render")]
    fn send_command(&self, command: RenderCommand) {
        if let Some(pipeline) = &self.render_pipeline {
            pipeline.send(command).expect("Valid pipeline");
        }
    }

    #[cfg(not(feature = "render"))]
    pub(crate) fn new(_render_pipeline: Option<RenderPipeline>) -> Self {
        Self {}
    }
}

#[cfg(feature = "render")]
impl ShapeRenderer for Renderer {
    fn clear(&mut self) {
        self.cursor = (0.0, 0.0);
    }
    fn grid<K, D>(&mut self, grid: &HashGrid<K, D>)
    where
        K: Eq + Hash + Ord + Copy + TryInto<i16>,
        D: Display,
    {
        self.grid_with(grid, |d, text, _| {
            text.with_content(d.to_string());
        });
    }
    fn grid_with<K, D, F>(&mut self, grid: &HashGrid<K, D>, mut cell_builder: F)
    where
        K: Eq + Hash + Ord + Copy + TryInto<i16>,
        F: FnMut(&D, &mut Text, &mut GridShape),
    {
        grid.iter().for_each(|((x, y), content)| {
            // Cast down to i16 so we can convert to f32
            let x: Option<f32> = (*x).try_into().map(|x: i16| x.into()).ok();
            let y: Option<f32> = (*y).try_into().map(|y: i16| y.into()).ok();
            match (x, y) {
                (Some(x_pos), Some(y_pos)) => {
                    let mut text = Text::new(x_pos, y_pos, "");
                    let mut cell = GridShape::new(x_pos, y_pos);
                    cell.without_color();
                    cell.with_border_color(Color::gray());
                    cell_builder(content, &mut text, &mut cell);
                    if !text.content.is_empty() {
                        self.send_command(RenderCommand::Text(text));
                    }
                    if cell.color.is_some() || cell.border_color.is_some() {
                        self.send_command(RenderCommand::Shape(cell));
                    }
                }
                _ => {}
            }
        });
    }
    fn grid_shape(&mut self, x: impl Into<f32>, y: impl Into<f32>) {
        self.grid_shape_with(x, y, |_| {});
    }
    fn grid_shape_with<F>(&mut self, x: impl Into<f32>, y: impl Into<f32>, builder: F)
    where
        F: FnOnce(&mut GridShape),
    {
        let mut cell = GridShape::new(x.into(), y.into());
        builder(&mut cell);
        self.send_command(RenderCommand::Shape(cell));
    }
    fn line(
        &mut self,
        x0: impl Into<f32>,
        y0: impl Into<f32>,
        x1: impl Into<f32>,
        y1: impl Into<f32>,
    ) {
        self.line_with(x0, y0, x1, y1, |_| {});
    }
    fn line_with<F>(
        &mut self,
        x0: impl Into<f32>,
        y0: impl Into<f32>,
        x1: impl Into<f32>,
        y1: impl Into<f32>,
        builder: F,
    ) where
        F: FnOnce(&mut Line),
    {
        let mut line = Line::new(x0.into(), y0.into(), x1.into(), y1.into());
        builder(&mut line);
        self.send_command(RenderCommand::Line(line));
    }
    fn print(&mut self, content: impl Into<String>) {
        self.print_with(content, |_| {});
    }
    fn print_with<F>(&mut self, content: impl Into<String>, builder: F)
    where
        F: FnOnce(&mut Text),
    {
        let content: String = content.into();
        let chars = content.len() as f32;
        self.print_at(self.cursor.0, self.cursor.1, content, builder);
        self.cursor.0 += chars;
    }
    fn println(&mut self, content: impl Into<String>) {
        self.println_with(content, |_| {});
    }
    fn println_with<F>(&mut self, content: impl Into<String>, builder: F)
    where
        F: FnOnce(&mut Text),
    {
        self.print_at(self.cursor.0, self.cursor.1, content, builder);
        self.cursor.0 = 0.;
        self.cursor.1 += 1.;
    }
    fn print_at<F>(
        &mut self,
        x: impl Into<f32>,
        y: impl Into<f32>,
        content: impl Into<String>,
        builder: F,
    ) where
        F: FnOnce(&mut Text),
    {
        let mut text = Text::new(x.into(), y.into(), content);
        builder(&mut text);
        self.send_command(RenderCommand::Text(text));
    }
}

/// All these methods will be inlined into nothing
#[cfg(not(feature = "render"))]
impl ShapeRenderer for Renderer {
    #[inline]
    fn clear(&mut self) {}
    #[inline]
    fn grid<K, D>(&mut self, _grid: &HashGrid<K, D>)
    where
        K: Eq + Hash + Ord + Copy + TryInto<i16>,
        D: Display,
    {
    }
    #[inline]
    fn grid_with<K, D, F>(&mut self, _grid: &HashGrid<K, D>, _cell_builder: F)
    where
        K: Eq + Hash + Ord + Copy + TryInto<i16>,
        F: FnMut(&D, &mut Text, &mut GridShape),
    {
    }
    #[inline]
    fn grid_shape(&mut self, _x: impl Into<f32>, _y: impl Into<f32>) {}
    #[inline]
    fn grid_shape_with<F>(&mut self, _x: impl Into<f32>, _y: impl Into<f32>, _builder: F)
    where
        F: FnOnce(&mut GridShape),
    {
    }
    #[inline]
    fn line(
        &mut self,
        _x0: impl Into<f32>,
        _y0: impl Into<f32>,
        _x1: impl Into<f32>,
        _y1: impl Into<f32>,
    ) {
    }
    #[inline]
    fn line_with<F>(
        &mut self,
        _x0: impl Into<f32>,
        _y0: impl Into<f32>,
        _x1: impl Into<f32>,
        _y1: impl Into<f32>,
        _builder: F,
    ) where
        F: FnOnce(&mut Line),
    {
    }
    #[inline]
    fn print(&mut self, _content: impl Into<String>) {}
    #[inline]
    fn print_with<F>(&mut self, _content: impl Into<String>, _builder: F)
    where
        F: FnOnce(&mut Text),
    {
    }
    #[inline]
    fn println(&mut self, _content: impl Into<String>) {}
    #[inline]
    fn println_with<F>(&mut self, _content: impl Into<String>, _builder: F)
    where
        F: FnOnce(&mut Text),
    {
    }
    #[inline]
    fn print_at<F>(
        &mut self,
        _x: impl Into<f32>,
        _y: impl Into<f32>,
        _content: impl Into<String>,
        _builder: F,
    ) where
        F: FnOnce(&mut Text),
    {
    }
}

pub enum GridShapeType {
    Rectangle(f32, f32),
    Circle(f32),
}

pub struct GridShape {
    pub x: f32,
    pub y: f32,
    pub shape: GridShapeType,
    pub color: Option<Color>,
    pub border_size: Option<f32>,
    pub border_color: Option<Color>,
    /// top, right, bottom, left
    pub borders: (bool, bool, bool, bool),
}
impl GridShape {
    #[allow(unused)]
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            shape: GridShapeType::Rectangle(1., 1.),
            color: Some(Color::white()),
            border_size: None,
            border_color: None,
            borders: (true, true, true, true),
        }
    }

    pub fn with_color(&mut self, color: Color) {
        self.color = Some(color);
    }
    pub fn without_color(&mut self) {
        self.color = None;
    }

    pub fn with_border_color(&mut self, color: Color) {
        if self.border_size.is_none() {
            self.with_border_size(0.02);
        }
        self.border_color = Some(color);
    }

    pub fn with_border_size(&mut self, size: f32) {
        self.border_size = Some(size);
    }

    pub fn with_borders(&mut self, top: bool, right: bool, bottom: bool, left: bool) {
        if self.border_size.is_none() {
            self.with_border_size(0.1);
        }
        self.borders = (top, left, bottom, right);
    }

    pub fn with_shape(&mut self, shape: GridShapeType) {
        self.shape = shape;
    }
}

pub struct Text {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub border_color: Option<Color>,
    pub content: String,
}
impl Text {
    #[allow(unused)]
    pub(crate) fn new(x: f32, y: f32, content: impl Into<String>) -> Self {
        Self {
            x,
            y,
            color: Color::white(),
            border_color: None,
            content: content.into(),
        }
    }

    pub fn with_content(&mut self, content: impl Into<String>) {
        self.content = content.into()
    }
    pub fn with_color(&mut self, color: Color) {
        self.color = color
    }
    pub fn with_border_color(&mut self, color: Color) {
        self.border_color = Some(color);
    }
}
pub struct Line {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub color: Color,
}
impl Line {
    #[allow(unused)]
    pub(crate) fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self {
            x0,
            y0,
            x1,
            y1,
            color: Color::white(),
        }
    }

    pub fn with_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color(f32, f32, f32);
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b)
    }
    pub fn white() -> Self {
        Self(1., 1., 1.)
    }
    pub fn black() -> Self {
        Self(0., 0., 0.)
    }
    pub fn red() -> Self {
        Self(1., 0., 0.)
    }
    pub fn green() -> Self {
        Self(0., 1., 0.)
    }
    pub fn blue() -> Self {
        Self(0., 0., 1.)
    }
    pub fn yellow() -> Self {
        Self(1., 1., 0.)
    }
    pub fn pink() -> Self {
        Self(1., 0., 1.)
    }
    pub fn cyan() -> Self {
        Self(0., 1., 1.)
    }
    pub fn orange() -> Self {
        Self(1., 0.5, 0.)
    }
    pub fn gray() -> Self {
        Self(0.5, 0.5, 0.5)
    }
    pub fn light_gray() -> Self {
        Self(0.827, 0.827, 0.827)
    }
    pub fn from_palette(index: impl Into<usize>) -> Self {
        COLOR_PALETTE[index.into() % COLOR_PALETTE.len()]
    }
    pub fn to_array(&self) -> [f32; 3] {
        [self.0, self.1, self.2]
    }
}
impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self(r, g, b)
    }
}

lazy_static::lazy_static!(
    static ref COLOR_PALETTE: Vec<Color> = vec![
        (0.4117647, 0.4117647, 0.4117647).into(),
        (0.6627451, 0.6627451, 0.6627451).into(),
        (0.8627451, 0.8627451, 0.8627451).into(),
        (0.18431373, 0.30980393, 0.30980393).into(),
        (0.33333334, 0.41960785, 0.18431373).into(),
        (0.54509807, 0.27058825, 0.07450981).into(),
        (0.41960785, 0.5568628, 0.13725491).into(),
        (0.09803922, 0.09803922, 0.4392157).into(),
        (0.54509807, 0.0, 0.0).into(),
        (0.28235295, 0.23921569, 0.54509807).into(),
        (0.37254903, 0.61960787, 0.627451).into(),
        (0.0, 0.5019608, 0.0).into(),
        (0.23529412, 0.7019608, 0.44313726).into(),
        (0.7372549, 0.56078434, 0.56078434).into(),
        (0.4, 0.2, 0.6).into(),
        (0.7411765, 0.7176471, 0.41960785).into(),
        (0.27450982, 0.50980395, 0.7058824).into(),
        (0.8235294, 0.4117647, 0.11764706).into(),
        (0.6039216, 0.8039216, 0.19607843).into(),
        (0.8039216, 0.36078432, 0.36078432).into(),
        (0.0, 0.0, 0.54509807).into(),
        (0.19607843, 0.8039216, 0.19607843).into(),
        (0.85490197, 0.64705884, 0.1254902).into(),
        (0.56078434, 0.7372549, 0.56078434).into(),
        (0.5019608, 0.0, 0.5019608).into(),
        (0.6901961, 0.1882353, 0.3764706).into(),
        (0.8235294, 0.7058824, 0.54901963).into(),
        (0.4, 0.8039216, 0.6666667).into(),
        (1.0, 0.27058825, 0.0).into(),
        (0.0, 0.80784315, 0.81960785).into(),
        (1.0, 0.54901963, 0.0).into(),
        (1.0, 0.84313726, 0.0).into(),
        (0.78039217, 0.08235294, 0.52156866).into(),
        (0.0, 0.0, 0.8039216).into(),
        (0.49803922, 1.0, 0.0).into(),
        (0.0, 1.0, 0.0).into(),
        (0.7294118, 0.33333334, 0.827451).into(),
        (0.5411765, 0.16862746, 0.8862745).into(),
        (0.0, 1.0, 0.49803922).into(),
        (0.25490198, 0.4117647, 0.88235295).into(),
        (0.9137255, 0.5882353, 0.47843137).into(),
        (0.8627451, 0.078431375, 0.23529412).into(),
        (0.0, 1.0, 1.0).into(),
        (0.0, 0.7490196, 1.0).into(),
        (0.95686275, 0.6431373, 0.3764706).into(),
        (0.5764706, 0.4392157, 0.85882354).into(),
        (0.0, 0.0, 1.0).into(),
        (1.0, 0.3882353, 0.2784314).into(),
        (0.84705883, 0.7490196, 0.84705883).into(),
        (1.0, 0.0, 1.0).into(),
        (0.85882354, 0.4392157, 0.5764706).into(),
        (0.9411765, 0.9019608, 0.54901963).into(),
        (1.0, 1.0, 0.32941177).into(),
        (0.39215687, 0.58431375, 0.92941177).into(),
        (0.8666667, 0.627451, 0.8666667).into(),
        (0.5647059, 0.93333334, 0.5647059).into(),
        (0.5294118, 0.80784315, 0.92156863).into(),
        (1.0, 0.078431375, 0.5764706).into(),
        (0.6862745, 0.93333334, 0.93333334).into(),
        (0.93333334, 0.50980395, 0.93333334).into(),
        (0.49803922, 1.0, 0.83137256).into(),
        (1.0, 0.4117647, 0.7058824).into(),
        (1.0, 0.89411765, 0.76862746).into(),
        (1.0, 0.7137255, 0.75686276).into(),
    ];
);
