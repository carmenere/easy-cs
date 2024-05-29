//! `Button` and `Checkbox` are **interfaces** for **products**. <br>
//! `LinuxButton` and `LinuxCheckBox` is a first **family** of **products**. <br>
//! `MacOSButton` and `MacOSCheckBox` is a second **family** of **products**. <br>
//! `Form` is an **abstract factory**. <br>
//! `LinuxForm` is an implementation of **abstract factory** that knows how to create first family of products: `LinuxButton` and `LinuxCheckBox`. <br>
//! `MacOSForm` is an implementation of **abstract factory** that knows how to create second family of products: `MacOSButton` and `MacOSCheckBox`. <br>
//! `FormsFactory` is **factory** that can create **concrete factory**: `LinuxForm` or `MacOSForm`! <br>

pub mod gui;
pub mod linux;
pub mod macos;
pub mod os;