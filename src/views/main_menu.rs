use ::phi::{Phi, View, ViewAction};
use ::phi::gfx::{Sprite, CopySprite};
use ::phi::data::Rectangle;
use ::sdl2::pixels::Color;

const FONTNAME: &'static str = "slkscr";

pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,
    elapsed: f64,
}

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> Self {
        MainMenuView {
            actions: vec![Action::new(phi,
                                      "New Game",
                                      Box::new(|phi| {
                                          ViewAction::ChangeView(Box::new(::views::game::GameView::new(phi)))
                                      })),
                          Action::new(phi, "Options", Box::new(|_| ViewAction::None)),
                          Action::new(phi, "Quit", Box::new(|_| ViewAction::Quit))],
            selected: 0,
            elapsed: 0.0,
        }
    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape {
            return ViewAction::Quit;
        }

        if phi.events.now.key_space {
            return (self.actions[self.selected as usize].func)(phi);
        }

        if phi.events.now.key_up {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        }

        if phi.events.now.key_down {
            self.selected += 1;
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        }

        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        let (win_w, win_h) = phi.output_size();
        let label_h = 50.0;
        let border_width = 3.0;
        self.elapsed += elapsed * 4.0;
        let box_w = 360.0 + 5.0 * self.elapsed.sin();
        let box_h = self.actions.len() as f64 * label_h;
        let margin_h = 10.0 + 5.0 * (self.elapsed + 1.0).sin();

        phi.renderer.set_draw_color(Color::RGB(70, 15, 70));
        phi.renderer
            .fill_rect(Rectangle {
                    w: box_w + border_width * 2.0,
                    h: box_h + border_width * 2.0 + margin_h * 2.0,
                    x: (win_w - box_w) / 2.0 - border_width,
                    y: (win_h - box_h) / 2.0 - margin_h - border_width,
                }
                .to_sdl()
                .unwrap())
            .unwrap();

        phi.renderer.set_draw_color(Color::RGB(140, 30, 140));
        phi.renderer
            .fill_rect(Rectangle {
                    w: box_w,
                    h: box_h + margin_h * 2.0,
                    x: (win_w - box_w) / 2.0,
                    y: (win_h - box_h) / 2.0 - margin_h,
                }
                .to_sdl()
                .unwrap())
            .unwrap();

        for (i, action) in self.actions.iter().enumerate() {
            if self.selected as usize == i {
                let (w, h) = action.hover_sprite.size();
                phi.renderer.copy_sprite(&action.hover_sprite,
                                         Rectangle {
                                             w: w,
                                             h: h,
                                             x: (win_w - w) / 2.0,
                                             y: (win_h - box_h + label_h - h) / 2.0 +
                                                label_h * i as f64,
                                         });
            } else {
                let (w, h) = action.idle_sprite.size();
                phi.renderer.copy_sprite(&action.idle_sprite,
                                         Rectangle {
                                             w: w,
                                             h: h,
                                             x: (win_w - w) / 2.0,
                                             y: (win_h - box_h + label_h - h) / 2.0 +
                                                label_h * i as f64,
                                         });
            }
        }

        ViewAction::None
    }
}

struct Action {
    func: Box<Fn(&mut Phi) -> ViewAction>,
    idle_sprite: Sprite,
    hover_sprite: Sprite,
}

impl Action {
    fn new(phi: &mut Phi, label: &'static str, func: Box<Fn(&mut Phi) -> ViewAction>) -> Self {
        Action {
            func: func,
            idle_sprite: phi.ttf_str_sprite(label, FONTNAME, 32, Color::RGB(220, 220, 220))
                .unwrap(),
            hover_sprite: phi.ttf_str_sprite(label, FONTNAME, 38, Color::RGB(255, 255, 255))
                .unwrap(),
        }
    }
}