use abstract_factory::gui::Form as _;
use abstract_factory::linux::LinuxForm;
use abstract_factory::macos::MacOSForm;
use abstract_factory::os::{OS, FormsFactory};

const SEP: &str = "=====";

pub fn main() {
    // Approach 1: using concrete factories: LinuxForm and MacOSForm
    let forms = LinuxForm {};
    let button = forms.create_button();
    let checkbox = forms.create_checkbox();
    button.click();
    checkbox.select();

    println!("{}", SEP);

    let forms = MacOSForm {};
    let button = forms.create_button();
    let checkbox = forms.create_checkbox();
    button.click();
    checkbox.select();

    println!("{}", SEP);

    // Approach 2: using FormsFactory
    let linux = FormsFactory::new(OS::Linux);
    let button = linux.create_button();
    let checkbox = linux.create_checkbox();
    button.click();
    checkbox.select();

    println!("{}", SEP);

    let macos = FormsFactory::new(OS::MacOS);
    let button = macos.create_button();
    let checkbox = macos.create_checkbox();
    button.click();
    checkbox.select();
}
