extern crate foundry;
use foundry::{
    ecs::{
        world::World,
        system::System, entity::Entity,
    },  
};
use crate::gear_core::{
    gear_window::GlGameWindow,
    events::event_handling::{
        EventHandling,
        DefaultEventHandler,
    },
    rendering::renderer::{
        DefaultOpenGlRenderer,
        Renderer
    },
};
use std::time::{Instant, Duration};


pub struct Engine {
    world: World,
    main_timer: Duration,
    engine_state: EngineState,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            world: World::new(),
            main_timer: Duration::ZERO,
            engine_state: EngineState::Stopped,
        }
    }

    pub fn with_window(mut self, event_handler: Option<Box<dyn EventHandling>>, renderer: Option<Box<dyn Renderer>>) -> Engine {
        // create the window system and add it
        let game_window = GlGameWindow::new(event_handler, renderer);
        let window_system = System::new(Box::new(game_window), foundry::ecs::system::UpdateFrequency::PerFrame);
        self.world.register_system(window_system, 0);
        
        self
    }

    pub fn main_loop(mut self) -> Engine {
        // set initial values
        self.main_timer = Duration::ZERO;
        self.engine_state = EngineState::Running;

        let mut last_instant = Instant::now();
        while self.engine_state == EngineState::Running {
            // record last instant, keep track of time
            let delta = last_instant.elapsed();
            self.main_timer += delta;
            last_instant = Instant::now();

            // update the engine
            let mut callback = EngineMessage::None;
            self.world.update(delta.as_secs_f32(), &mut callback);

            match callback {
                EngineMessage::None => {},
                _ => self.handle_message(callback),
            }
        }

        // end of main loop, state back to stopped
        self.engine_state = EngineState::Stopped;

        self
    }

    pub fn handle_message(&mut self, message: EngineMessage) {
        match message {
            EngineMessage::StopEngine => self.engine_state = EngineState::RequestingStop,

            _ => {}
        }
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }


}


#[derive(PartialEq)]
pub enum EngineState {
    Stopped,
    Running,
    RequestingStop,
}


pub enum EngineMessage {
    None,
    StopEngine,
}