use eframe::egui;

fn main() {
    println!("Hello, world!");
    let options = eframe::NativeOptions::default();
    eframe::run_native("My Egui Test App", options, Box::new(|_cc| Box::new(MyApp::default())));
}

struct MyApp {
    name: String,
    age: u32,
    old_name: String,
    old_age: u32,
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            old_name: "Arthur".to_owned(),
            old_age: 42,
            allowed_to_close: bool::default(),
            show_confirmation_dialog: bool::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Test egui App");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
                println!("Hello '{}', age '{}'", self.name, self.age);
            }
            ui.label(format!("Hello '{}', age '{}'", self.name, self.age));
        });
        self.print_if_changed();

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

impl MyApp {
    fn print_if_changed(&mut self) {
        if self.name == self.old_name && self.age == self.old_age {
            return;
        }

        println!("Hello '{}', age '{}'", self.name, self.age);
        self.old_name = self.name.clone();
        self.old_age = self.age;
    }
}