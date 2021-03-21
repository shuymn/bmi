use crate::joystick;
use iced_futures::futures;
use multiinput::{RawEvent, RawInputManager};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Joystick(joystick::Event),
    None,
}

pub struct Events {
    manager: RawInputManager,
}

impl Events {
    pub fn new(manager: RawInputManager) -> Events {
        Events { manager }
    }
}

impl<H, E> iced_native::subscription::Recipe<H, E> for Events
where
    H: std::hash::Hasher,
{
    type Output = Event;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, E>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            self.manager,
            |mut manager| async move {
                if let Some(event) = manager.get_event() {
                    match event {
                        RawEvent::JoystickButtonEvent(
                            _device_id,
                            button_code,
                            state,
                        ) => match button_code {
                            1 => match state {
                                multiinput::State::Pressed => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyPressed(
                                            joystick::ButtonCode::Button1,
                                        ),
                                    ),
                                    manager,
                                )),
                                multiinput::State::Released => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyReleased(
                                            joystick::ButtonCode::Button1,
                                        ),
                                    ),
                                    manager,
                                )),
                            },
                            2 => match state {
                                multiinput::State::Pressed => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyPressed(
                                            joystick::ButtonCode::Button2,
                                        ),
                                    ),
                                    manager,
                                )),
                                multiinput::State::Released => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyReleased(
                                            joystick::ButtonCode::Button2,
                                        ),
                                    ),
                                    manager,
                                )),
                            },
                            3 => match state {
                                multiinput::State::Pressed => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyPressed(
                                            joystick::ButtonCode::Button3,
                                        ),
                                    ),
                                    manager,
                                )),
                                multiinput::State::Released => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyReleased(
                                            joystick::ButtonCode::Button3,
                                        ),
                                    ),
                                    manager,
                                )),
                            },
                            4 => match state {
                                multiinput::State::Pressed => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyPressed(
                                            joystick::ButtonCode::Button4,
                                        ),
                                    ),
                                    manager,
                                )),
                                multiinput::State::Released => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyReleased(
                                            joystick::ButtonCode::Button4,
                                        ),
                                    ),
                                    manager,
                                )),
                            },
                            5 => match state {
                                multiinput::State::Pressed => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyPressed(
                                            joystick::ButtonCode::Button5,
                                        ),
                                    ),
                                    manager,
                                )),
                                multiinput::State::Released => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyReleased(
                                            joystick::ButtonCode::Button5,
                                        ),
                                    ),
                                    manager,
                                )),
                            },
                            6 => match state {
                                multiinput::State::Pressed => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyPressed(
                                            joystick::ButtonCode::Button6,
                                        ),
                                    ),
                                    manager,
                                )),
                                multiinput::State::Released => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyReleased(
                                            joystick::ButtonCode::Button6,
                                        ),
                                    ),
                                    manager,
                                )),
                            },
                            7 => match state {
                                multiinput::State::Pressed => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyPressed(
                                            joystick::ButtonCode::Button6,
                                        ),
                                    ),
                                    manager,
                                )),
                                multiinput::State::Released => Some((
                                    Event::Joystick(
                                        joystick::Event::KeyReleased(
                                            joystick::ButtonCode::Button4,
                                        ),
                                    ),
                                    manager,
                                )),
                            },
                            _ => Some((Event::None, manager)),
                        },
                        RawEvent::JoystickAxisEvent(
                            _device_id,
                            _axis,
                            rotation,
                        ) => Some((
                            Event::Joystick(joystick::Event::Scratched(
                                rotation,
                            )),
                            manager,
                        )),
                        _ => Some((Event::None, manager)),
                    }
                } else {
                    Some((Event::None, manager))
                }
            },
        ))
    }
}
