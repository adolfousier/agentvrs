use crate::agent::AgentId;

pub struct Camera {
    pub offset_x: f64,
    pub offset_y: f64,
    pub zoom: f64,
    pub rotation: u8,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            zoom: 1.0,
            rotation: 0,
        }
    }

    pub fn zoom_by(&mut self, factor: f64) {
        self.zoom = (self.zoom * factor).clamp(0.3, 4.0);
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 3) % 4;
    }
}

pub struct ViewState {
    pub camera: Camera,
    pub selected_agent: Option<AgentId>,
    pub selected_index: usize,
    pub is_dragging: bool,
    pub drag_start: (f64, f64),
    pub tick_count: u64,
    pub status_message: Option<String>,
    pub sidebar_visible: bool,
}

impl ViewState {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            selected_agent: None,
            selected_index: 0,
            is_dragging: false,
            drag_start: (0.0, 0.0),
            tick_count: 0,
            status_message: None,
            sidebar_visible: true,
        }
    }
}
