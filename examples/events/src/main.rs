// #![feature(static_nobundle)]

use events::event;
use iced::{
    executor, Align, Application, Checkbox, Column, Command, Container,
    Element, Length, Settings, Subscription, Text,
};
use multiinput::{DeviceType, RawInputManager, XInputInclude};

fn main() -> iced::Result {
    GUI::run(Settings::default())
}

#[derive(Debug, Default)]
struct GUI {
    last: Vec<event::Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccured(event::Event),
    Toggled(bool),
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (GUI::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Events")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EventOccured(event) => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }
            }
            Message::Toggled(enabled) => self.enabled = enabled,
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.enabled {
            let mut manager = RawInputManager::new().unwrap();
            manager
                .register_devices(DeviceType::Joysticks(XInputInclude::False));
            let devices = manager.get_device_list();

            let joystick = devices.joysticks.first().unwrap();
            manager.filter_devices(vec![joystick.name.clone()]);

            let events = event::Events::new(manager);
            iced::Subscription::from_recipe(events).map(Message::EventOccured)
        } else {
            Subscription::none()
        }
    }

    fn view(&mut self) -> Element<Message> {
        let events = self.last.iter().fold(
            Column::new().spacing(10),
            |colmun, event| {
                colmun.push(Text::new(format!("{:?}", event)).size(40))
            },
        );

        let toggle = Checkbox::new(
            self.enabled,
            "Listen to gamepad events",
            Message::Toggled,
        );

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(events)
            .push(toggle);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
