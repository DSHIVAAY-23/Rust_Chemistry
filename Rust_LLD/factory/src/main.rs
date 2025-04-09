trait Button {
    fn render(&self);
}
struct WindowsButton;
struct LinuxButton;

impl Button for WindowsButton {
    fn render(&self) {
        println!("Rendering Windows Button.");
    }
}

impl Button for LinuxButton {
    fn render(&self) {
        println!("Rendering Linux Button.");
    }
}
trait Dialog {
    fn create_button(&self) -> Box<dyn Button>; // factory method

    fn render_window(&self) {
        let button = self.create_button();
        button.render(); // client code
    }
}
struct WindowsDialog;
struct LinuxDialog;

impl Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}

impl Dialog for LinuxDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(LinuxButton)
    }
}
fn main() {
    let os = "windows"; // ये dynamic हो सकता है config/env से

    let dialog: Box<dyn Dialog> = match os {
        "windows" => Box::new(WindowsDialog),
        "linux" => Box::new(LinuxDialog),
        _ => panic!("Unknown OS"),
    };

    dialog.render_window();
}
