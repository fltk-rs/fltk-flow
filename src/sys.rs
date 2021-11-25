/* automatically generated by rust-bindgen 0.56.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
pub type custom_handler_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut Fl_Widget,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type custom_draw_callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Flow {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Flow_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Flow;
}
extern "C" {
    pub fn Fl_Flow_x(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_y(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_width(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_height(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_label(arg1: *mut Fl_Flow) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Flow_set_label(arg1: *mut Fl_Flow, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Flow_redraw(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_show(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_hide(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_activate(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_deactivate(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_redraw_label(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_resize(
        arg1: *mut Fl_Flow,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Flow_widget_resize(
        arg1: *mut Fl_Flow,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Flow_tooltip(arg1: *mut Fl_Flow) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Flow_set_tooltip(arg1: *mut Fl_Flow, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Flow_get_type(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_type(arg1: *mut Fl_Flow, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_color(arg1: *mut Fl_Flow) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Flow_set_color(arg1: *mut Fl_Flow, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Flow_measure_label(
        arg1: *const Fl_Flow,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Flow_label_color(arg1: *mut Fl_Flow) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Flow_set_label_color(arg1: *mut Fl_Flow, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Flow_label_font(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_label_font(arg1: *mut Fl_Flow, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_label_size(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_label_size(arg1: *mut Fl_Flow, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_label_type(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_label_type(arg1: *mut Fl_Flow, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_box(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_box(arg1: *mut Fl_Flow, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_changed(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_changed(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_clear_changed(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_align(arg1: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_align(arg1: *mut Fl_Flow, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_delete(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_set_image(arg1: *mut Fl_Flow, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Flow_handle(
        self_: *mut Fl_Flow,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_handle_event(self_: *mut Fl_Flow, event: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_draw(
        self_: *mut Fl_Flow,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_resize_callback(
        self_: *mut Fl_Flow,
        cb: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut Fl_Widget,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
                w: ::std::os::raw::c_int,
                h: ::std::os::raw::c_int,
                arg2: *mut ::std::os::raw::c_void,
            ),
        >,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_set_when(arg1: *mut Fl_Flow, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_when(arg1: *const Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_image(arg1: *const Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_parent(self_: *const Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_selection_color(arg1: *mut Fl_Flow) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Flow_set_selection_color(arg1: *mut Fl_Flow, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Flow_do_callback(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_inside(
        self_: *const Fl_Flow,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_window(arg1: *const Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_top_window(arg1: *const Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_takes_events(arg1: *const Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_user_data(arg1: *const Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_take_focus(self_: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_visible_focus(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_clear_visible_focus(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_visible_focus(self_: *mut Fl_Flow, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_has_visible_focus(self_: *mut Fl_Flow) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Flow_set_user_data(arg1: *mut Fl_Flow, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Flow_draw_data(self_: *const Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_handle_data(self_: *const Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_set_draw_data(self_: *mut Fl_Flow, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Flow_set_handle_data(self_: *mut Fl_Flow, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Flow_damage(self_: *const Fl_Flow) -> ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn Fl_Flow_set_damage(self_: *mut Fl_Flow, flag: ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn Fl_Flow_clear_damage(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_as_window(self_: *mut Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_as_group(self_: *mut Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_set_deimage(arg1: *mut Fl_Flow, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Flow_deimage(arg1: *const Fl_Flow) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Flow_set_callback(
        arg1: *mut Fl_Flow,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_set_deleter(
        arg1: *mut Fl_Flow,
        arg2: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn Fl_Flow_visible(self_: *const Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_visible_r(self_: *const Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_active(self_: *const Fl_Flow) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Flow_active_r(self_: *const Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_callback(self_: *const Fl_Flow) -> Fl_Callback;
}
extern "C" {
    pub fn Fl_Flow_rule(
        self_: *mut Fl_Flow,
        wid: *mut Fl_Widget,
        inst: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Flow_begin(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_end(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_find(
        self_: *mut Fl_Flow,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_add(self_: *mut Fl_Flow, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Flow_insert(
        self_: *mut Fl_Flow,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Flow_remove(self_: *mut Fl_Flow, wid: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Flow_remove_by_index(self_: *mut Fl_Flow, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_clear(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_children(self_: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_child(arg1: *mut Fl_Flow, index: ::std::os::raw::c_int) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Flow_resizable(self_: *mut Fl_Flow, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Flow_set_clip_children(self_: *mut Fl_Flow, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Flow_clip_children(self_: *mut Fl_Flow) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Flow_init_sizes(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_draw_child(self_: *const Fl_Flow, w: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Flow_update_child(self_: *const Fl_Flow, w: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Flow_draw_outside_label(self_: *const Fl_Flow, w: *const Fl_Widget);
}
extern "C" {
    pub fn Fl_Flow_draw_children(self_: *mut Fl_Flow);
}