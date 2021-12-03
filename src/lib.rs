pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

pub trait Draw {
    fn draw(&self);
}

impl Draw for Button {
    fn draw(&self) {
        todo!()
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}