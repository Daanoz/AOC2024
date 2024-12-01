use std::sync::mpsc;

pub trait ShapeRenderer {
    fn clear(&mut self);
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
    pub color: (f32, f32, f32),
    pub border_size: Option<f32>,
    pub border_color: Option<(f32, f32, f32)>,
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
            color: (1., 1., 1.),
            border_size: None,
            border_color: None,
            borders: (true, true, true, true),
        }
    }

    pub fn with_color(&mut self, color: (f32, f32, f32)) {
        self.color = color;
    }

    pub fn with_border_color(&mut self, color: (f32, f32, f32)) {
        if self.border_size.is_none() {
            self.with_border_size(0.1);
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
    pub color: (f32, f32, f32),
    pub border_color: Option<(f32, f32, f32)>,
    pub content: String,
}
impl Text {
    #[allow(unused)]
    pub(crate) fn new(x: f32, y: f32, content: impl Into<String>) -> Self {
        Self {
            x,
            y,
            color: (1., 1., 1.),
            border_color: None,
            content: content.into(),
        }
    }

    pub fn with_color(&mut self, color: (f32, f32, f32)) {
        self.color = color
    }
    pub fn with_border_color(&mut self, color: (f32, f32, f32)) {
        self.border_color = Some(color);
    }
}
pub struct Line {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub color: (f32, f32, f32),
}
impl Line {
    #[allow(unused)]
    pub(crate) fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self {
            x0,
            y0,
            x1,
            y1,
            color: (1., 1., 1.),
        }
    }

    pub fn with_color(&mut self, color: (f32, f32, f32)) {
        self.color = color;
    }
}
