#[derive(Default, Debug, Copy, Clone)]
pub struct CellEditorAge {
    pub age: f32,
    pub prev_age: Option<f32>,
}

impl CellEditorAge {
    #[must_use]
    pub const fn new(age: f32) -> Self {
        Self { age, prev_age: None }
    }

    #[must_use]
    pub fn delta(&self) -> f32 {
        self.age - self.prev_age.unwrap_or(0.0)
    }

    #[must_use]
    pub fn is_increasing(&self) -> bool {
        self.delta() > 0.
    }

    #[must_use]
    pub fn is_decreasing(&self) -> bool {
        self.delta() < 0.
    }
}
