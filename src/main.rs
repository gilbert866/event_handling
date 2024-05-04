//Define an enum to represent different types of events
enum Event {
    MouseClick { x: i32, y: i32 },
    KeyPress(char),
}

//Define a trait for event handlers
trait EventHandler {
    fn handle_event(&mut self, event: &Event);
}

//A simple event handler that prints events to the console
struct ConsoleEventHandler;

impl EventHandler for ConsoleEventHandler {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::MouseClick { x, y } => {
                println!("Mouse click at ({}, {})", x, y);
            }
            Event::KeyPress(key) => {
                println!("Key press: {}", key);
            }
        }
    }
}
//Function to dispatch events to handlers
fn dispatch_event(event: &Event, handler: &mut dyn EventHandler) {
    handler.handle_event(event);
}

fn main() {
    let mut handler = ConsoleEventHandler;
    let mouse_event = Event::MouseClick { x: 100, y: 200 };
    let key_event = Event::KeyPress('a');
    dispatch_event(&mouse_event, &mut handler);
    dispatch_event(&key_event, &mut handler);
}
