use aoc_procmacro_internals::{get_aoc_data, AocDataType};

#[derive(Debug, Clone)]
pub struct Puzzle {
    input: String,
    render_pipeline: Option<RenderPipeline>,
}

impl Puzzle {
    pub(crate) fn new(day: u32, year: u32) -> Self {
        let input = get_aoc_data(AocDataType::Input, day, year).expect("Failed to get input");
        Self {
            input,
            render_pipeline: None,
        }
    }
    pub(crate) fn set_render_pipeline(&mut self, render_pipeline: Option<RenderPipeline>) {
        self.render_pipeline = render_pipeline;
    }
    pub fn input_as_str(&self) -> &str {
        &self.input
    }
    pub fn get_input(&self) -> String {
        self.input.clone()
    }
    pub fn get_input_lines(&self) -> Vec<&str> {
        self.input.lines().collect()
    }
}

mod render_components;
pub use render_components::{GridShapeType, RenderCommand, RenderPipeline, ShapeRenderer};
impl Puzzle {
    pub fn renderer(&self) -> impl render_components::ShapeRenderer {
        render_components::Renderer::new(self.render_pipeline.clone())
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.input)
    }
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        Self {
            input: input.into(),
            render_pipeline: None,
        }
    }
}

impl From<String> for Puzzle {
    fn from(input: String) -> Self {
        Self {
            input,
            render_pipeline: None,
        }
    }
}

impl From<Puzzle> for String {
    fn from(val: Puzzle) -> Self {
        val.to_string()
    }
}
