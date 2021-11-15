#[allow(unused_imports)]
use chrono::{offset::Utc, DateTime, Timelike};

use rand::Rng;
use std::process;

use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, Path, Stroke},
    executor, time,
    window::Settings as WindowSettings,
    Application, Color, Column, Command, Container, Element, Length, Point, Rectangle, Row,
    Settings, Size, Subscription,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const METABALL_COUNT: usize = 2;
const METABALL_RADIUS: f32 = 10.0;

pub fn main() -> iced::Result {
    Visualizer::run(Settings {
        window: WindowSettings {
            size: (WIDTH, HEIGHT),
            ..WindowSettings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

struct Visualizer {
    clock: Cache,
    metaballs: Vec<MetaBall>,
}

struct MetaBall {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
    EventOccured(iced_native::Event),
}

impl Application for Visualizer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut rng = rand::thread_rng();
        let half_radius: f32 = METABALL_RADIUS / 2.0;
        let mut metaballs: Vec<MetaBall> = Vec::with_capacity(METABALL_COUNT);

        for _ in 0..METABALL_COUNT {
            metaballs.push(MetaBall {
                x: rng.gen_range(half_radius..((WIDTH as f32) - half_radius)),
                y: rng.gen_range(half_radius..((HEIGHT as f32) - half_radius)),
                vx: rng.gen_range(-1..1) as f32,
                vy: rng.gen_range(-1..1) as f32,
                r: METABALL_RADIUS,
            });
        }
        (
            Visualizer {
                clock: Default::default(),
                metaballs,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("RUSTY-MetaBalls")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                // let now = Utc::now().timestamp();
                let _now = local_time;
                self.tick();
                self.clock.clear();
            }
            Message::EventOccured(event) => {
                if let Event::Keyboard(keyboard_event) = event {
                    if let KeyboardEvent::CharacterReceived(ch) = keyboard_event {
                        match ch {
                            'q' => {
                                process::exit(0);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced_native::subscription::events().map(Message::EventOccured),
            time::every(std::time::Duration::from_millis(30))
                .map(|_| Message::Tick(chrono::Local::now())),
        ])
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Container::new(
            Canvas::new(self)
                .width(Length::Units(WIDTH as u16))
                .height(Length::Units(HEIGHT as u16)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .center_y();

        let row = Row::new().push(canvas).width(Length::Fill);
        Column::new().push(row).into()
    }
}

impl Visualizer {
    fn tick(&mut self) {
        for metaball in self.metaballs.iter_mut() {
            let vx: f32;
            if metaball.x < 0.0 || metaball.x > WIDTH as f32 {
                vx = -metaball.vx;
            } else {
                vx = metaball.vx;
            }
            let vy: f32;
            if metaball.y < 0.0 || metaball.y > WIDTH as f32 {
                vy = -metaball.vy;
            } else {
                vy = metaball.vy;
            }

            metaball.x += vx;
            metaball.y += vy;

            metaball.vx = vx;
            metaball.vy = vy;
        }
    }
}
impl canvas::Program<Message> for Visualizer {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        println!("draw");
        let program = self.clock.draw(bounds.size(), |frame| {
            // We create a `Path` representing a simple circle
            let mut bxby: Vec<(Vec<f32>, Vec<f32>)> = Vec::with_capacity(self.metaballs.len());
            for metaball in &self.metaballs {
                let mut bx: Vec<f32> = Vec::with_capacity(WIDTH as usize);
                for i in 0..WIDTH {
                    bx.insert(
                        i as usize,
                        ((metaball.x - i as f32) * (metaball.x - i as f32)).sqrt(),
                    );
                }
                let mut by: Vec<f32> = Vec::with_capacity(HEIGHT as usize);
                for i in 0..HEIGHT {
                    by.insert(
                        i as usize,
                        ((metaball.y - i as f32) * (metaball.y - i as f32)).sqrt(),
                    );
                }

                bxby.push((bx, by));

                let center = Point::new(metaball.x, metaball.y);
                let circle = Path::new(|p| p.circle(center, metaball.r));
                frame.stroke(&circle, Stroke::default());
            }

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let mut m = 1.0;
                    for i in 0..self.metaballs.len() {
                        let bx = &bxby[i].0;
                        let by = &bxby[i].1;
                        m += 20000.0 / (bx[x as usize] + by[y as usize] + 1.0);
                    }
                    frame.fill_rectangle(
                        Point::new(0f32, 0f32),
                        Size::new(WIDTH as f32, HEIGHT as f32),
                        Color::from_rgba(0.0, 0.0, 0.0, m % 255.0),
                    );
                }
            }

            // And fill it with some color

            // frame.fill_rectangle(
            //     Point::new(0f32, 0f32),
            //     Size::new(WIDTH as f32, HEIGHT as f32),
            //     Color::WHITE,
            // );
            // let shift: f32 = (WIDTH as f32 - BAR_WIDTH / 2f32) / self.columns as f32;
            // let mut position = 0f32;
            // frame.fill_text(format!("Frame {}/{}", self.index + 1, self.slides.len()));
            // for data_point in self.slides[self.index].iter() {
            //     let height = HEIGHT as f32 * (*data_point as f32 / self.max as f32) as f32;
            //     frame.fill_rectangle(
            //         Point::new(position, HEIGHT as f32),
            //         Size::new(WIDTH as f32 / self.columns as f32, -height),
            //         Color::BLACK,
            //     );
            //     position += shift;
            // }
        });

        vec![program]
    }
}
