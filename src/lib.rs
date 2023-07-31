#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod sys;
use sys::*;

use fltk::prelude::*;
use fltk::utils::FlString;
use std::ffi::{CStr, CString};

/// Creates a flow widget
#[derive(Debug)]
pub struct Flow {
    inner: *mut Fl_Flow,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

fltk::macros::widget::impl_widget_ext!(Flow, Fl_Flow);
fltk::macros::widget::impl_widget_base!(Flow, Fl_Flow);
fltk::macros::widget::impl_widget_default!(Flow);
fltk::macros::group::impl_group_ext!(Flow, Fl_Flow);

impl Flow {
    /// Set the flow's rule
    pub fn rule<W: WidgetExt>(&mut self, w: &W, inst: &str) {
        unsafe {
            let inst = CString::safe_new(inst);
            Fl_Flow_rule(self.inner, w.as_widget_ptr() as _, inst.as_ptr());
        }
    }
}
