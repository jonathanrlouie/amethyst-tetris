use amethyst::ecs::{Component, DenseVecStorage};

pub struct MessageHandler {

}

impl Component for MessageHandler {
    type Storage = DenseVecStorage<Self>;
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {}
    }

    pub fn recv<T>(&self) -> T {


    }
}