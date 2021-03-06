/* automatically generated by rust-bindgen */

import libc::*;

type cairo_bool_t = c_int;

type struct__cairo = c_void;

type cairo_t = struct__cairo;

type struct__cairo_surface = c_void;

type cairo_surface_t = struct__cairo_surface;

type struct__cairo_device = c_void;

type cairo_device_t = struct__cairo_device;

type struct__cairo_matrix = {
    xx: c_double,
    yx: c_double,
    xy: c_double,
    yy: c_double,
    x0: c_double,
    y0: c_double,
};

type cairo_matrix_t = struct__cairo_matrix;

type struct__cairo_pattern = c_void;

type cairo_pattern_t = struct__cairo_pattern;

type cairo_destroy_func_t = *u8;

type struct__cairo_user_data_key = {
    unused: c_int,
};

type cairo_user_data_key_t = struct__cairo_user_data_key;

type enum__cairo_status = c_uint;
const CAIRO_STATUS_SUCCESS: u32 = 0_u32;
const CAIRO_STATUS_NO_MEMORY: u32 = 1_u32;
const CAIRO_STATUS_INVALID_RESTORE: u32 = 2_u32;
const CAIRO_STATUS_INVALID_POP_GROUP: u32 = 3_u32;
const CAIRO_STATUS_NO_CURRENT_POINT: u32 = 4_u32;
const CAIRO_STATUS_INVALID_MATRIX: u32 = 5_u32;
const CAIRO_STATUS_INVALID_STATUS: u32 = 6_u32;
const CAIRO_STATUS_NULL_POINTER: u32 = 7_u32;
const CAIRO_STATUS_INVALID_STRING: u32 = 8_u32;
const CAIRO_STATUS_INVALID_PATH_DATA: u32 = 9_u32;
const CAIRO_STATUS_READ_ERROR: u32 = 10_u32;
const CAIRO_STATUS_WRITE_ERROR: u32 = 11_u32;
const CAIRO_STATUS_SURFACE_FINISHED: u32 = 12_u32;
const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: u32 = 13_u32;
const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: u32 = 14_u32;
const CAIRO_STATUS_INVALID_CONTENT: u32 = 15_u32;
const CAIRO_STATUS_INVALID_FORMAT: u32 = 16_u32;
const CAIRO_STATUS_INVALID_VISUAL: u32 = 17_u32;
const CAIRO_STATUS_FILE_NOT_FOUND: u32 = 18_u32;
const CAIRO_STATUS_INVALID_DASH: u32 = 19_u32;
const CAIRO_STATUS_INVALID_DSC_COMMENT: u32 = 20_u32;
const CAIRO_STATUS_INVALID_INDEX: u32 = 21_u32;
const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: u32 = 22_u32;
const CAIRO_STATUS_TEMP_FILE_ERROR: u32 = 23_u32;
const CAIRO_STATUS_INVALID_STRIDE: u32 = 24_u32;
const CAIRO_STATUS_FONT_TYPE_MISMATCH: u32 = 25_u32;
const CAIRO_STATUS_USER_FONT_IMMUTABLE: u32 = 26_u32;
const CAIRO_STATUS_USER_FONT_ERROR: u32 = 27_u32;
const CAIRO_STATUS_NEGATIVE_COUNT: u32 = 28_u32;
const CAIRO_STATUS_INVALID_CLUSTERS: u32 = 29_u32;
const CAIRO_STATUS_INVALID_SLANT: u32 = 30_u32;
const CAIRO_STATUS_INVALID_WEIGHT: u32 = 31_u32;
const CAIRO_STATUS_INVALID_SIZE: u32 = 32_u32;
const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: u32 = 33_u32;
const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: u32 = 34_u32;
const CAIRO_STATUS_DEVICE_ERROR: u32 = 35_u32;
const CAIRO_STATUS_LAST_STATUS: u32 = 36_u32;

type cairo_status_t = enum__cairo_status;

type enum__cairo_content = c_uint;
const CAIRO_CONTENT_COLOR: u32 = 4096_u32;
const CAIRO_CONTENT_ALPHA: u32 = 8192_u32;
const CAIRO_CONTENT_COLOR_ALPHA: u32 = 12288_u32;

type cairo_content_t = enum__cairo_content;

type cairo_write_func_t = *u8;

type cairo_read_func_t = *u8;

type enum__cairo_operator = c_uint;
const CAIRO_OPERATOR_CLEAR: u32 = 0_u32;
const CAIRO_OPERATOR_SOURCE: u32 = 1_u32;
const CAIRO_OPERATOR_OVER: u32 = 2_u32;
const CAIRO_OPERATOR_IN: u32 = 3_u32;
const CAIRO_OPERATOR_OUT: u32 = 4_u32;
const CAIRO_OPERATOR_ATOP: u32 = 5_u32;
const CAIRO_OPERATOR_DEST: u32 = 6_u32;
const CAIRO_OPERATOR_DEST_OVER: u32 = 7_u32;
const CAIRO_OPERATOR_DEST_IN: u32 = 8_u32;
const CAIRO_OPERATOR_DEST_OUT: u32 = 9_u32;
const CAIRO_OPERATOR_DEST_ATOP: u32 = 10_u32;
const CAIRO_OPERATOR_XOR: u32 = 11_u32;
const CAIRO_OPERATOR_ADD: u32 = 12_u32;
const CAIRO_OPERATOR_SATURATE: u32 = 13_u32;
const CAIRO_OPERATOR_MULTIPLY: u32 = 14_u32;
const CAIRO_OPERATOR_SCREEN: u32 = 15_u32;
const CAIRO_OPERATOR_OVERLAY: u32 = 16_u32;
const CAIRO_OPERATOR_DARKEN: u32 = 17_u32;
const CAIRO_OPERATOR_LIGHTEN: u32 = 18_u32;
const CAIRO_OPERATOR_COLOR_DODGE: u32 = 19_u32;
const CAIRO_OPERATOR_COLOR_BURN: u32 = 20_u32;
const CAIRO_OPERATOR_HARD_LIGHT: u32 = 21_u32;
const CAIRO_OPERATOR_SOFT_LIGHT: u32 = 22_u32;
const CAIRO_OPERATOR_DIFFERENCE: u32 = 23_u32;
const CAIRO_OPERATOR_EXCLUSION: u32 = 24_u32;
const CAIRO_OPERATOR_HSL_HUE: u32 = 25_u32;
const CAIRO_OPERATOR_HSL_SATURATION: u32 = 26_u32;
const CAIRO_OPERATOR_HSL_COLOR: u32 = 27_u32;
const CAIRO_OPERATOR_HSL_LUMINOSITY: u32 = 28_u32;

type cairo_operator_t = enum__cairo_operator;

type enum__cairo_antialias = c_uint;
const CAIRO_ANTIALIAS_DEFAULT: u32 = 0_u32;
const CAIRO_ANTIALIAS_NONE: u32 = 1_u32;
const CAIRO_ANTIALIAS_GRAY: u32 = 2_u32;
const CAIRO_ANTIALIAS_SUBPIXEL: u32 = 3_u32;

type cairo_antialias_t = enum__cairo_antialias;

type enum__cairo_fill_rule = c_uint;
const CAIRO_FILL_RULE_WINDING: u32 = 0_u32;
const CAIRO_FILL_RULE_EVEN_ODD: u32 = 1_u32;

type cairo_fill_rule_t = enum__cairo_fill_rule;

type enum__cairo_line_cap = c_uint;
const CAIRO_LINE_CAP_BUTT: u32 = 0_u32;
const CAIRO_LINE_CAP_ROUND: u32 = 1_u32;
const CAIRO_LINE_CAP_SQUARE: u32 = 2_u32;

type cairo_line_cap_t = enum__cairo_line_cap;

type enum__cairo_line_join = c_uint;
const CAIRO_LINE_JOIN_MITER: u32 = 0_u32;
const CAIRO_LINE_JOIN_ROUND: u32 = 1_u32;
const CAIRO_LINE_JOIN_BEVEL: u32 = 2_u32;

type cairo_line_join_t = enum__cairo_line_join;

type struct__cairo_rectangle = {
    x: c_double,
    y: c_double,
    width: c_double,
    height: c_double,
};

type cairo_rectangle_t = struct__cairo_rectangle;

type struct__cairo_rectangle_list = {
    status: cairo_status_t,
    rectangles: *cairo_rectangle_t,
    num_rectangles: c_int,
};

type cairo_rectangle_list_t = struct__cairo_rectangle_list;

type struct__cairo_scaled_font = c_void;

type cairo_scaled_font_t = struct__cairo_scaled_font;

type struct__cairo_font_face = c_void;

type cairo_font_face_t = struct__cairo_font_face;

type cairo_glyph_t = {
    index: c_ulong,
    x: c_double,
    y: c_double,
};

type cairo_text_cluster_t = {
    num_bytes: c_int,
    num_glyphs: c_int,
};

type enum__cairo_text_cluster_flags = c_uint;
const CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: u32 = 1_u32;

type cairo_text_cluster_flags_t = enum__cairo_text_cluster_flags;

type cairo_text_extents_t = {
    x_bearing: c_double,
    y_bearing: c_double,
    width: c_double,
    height: c_double,
    x_advance: c_double,
    y_advance: c_double,
};

type cairo_font_extents_t = {
    ascent: c_double,
    descent: c_double,
    height: c_double,
    max_x_advance: c_double,
    max_y_advance: c_double,
};

type enum__cairo_font_slant = c_uint;
const CAIRO_FONT_SLANT_NORMAL: u32 = 0_u32;
const CAIRO_FONT_SLANT_ITALIC: u32 = 1_u32;
const CAIRO_FONT_SLANT_OBLIQUE: u32 = 2_u32;

type cairo_font_slant_t = enum__cairo_font_slant;

type enum__cairo_font_weight = c_uint;
const CAIRO_FONT_WEIGHT_NORMAL: u32 = 0_u32;
const CAIRO_FONT_WEIGHT_BOLD: u32 = 1_u32;

type cairo_font_weight_t = enum__cairo_font_weight;

type enum__cairo_subpixel_order = c_uint;
const CAIRO_SUBPIXEL_ORDER_DEFAULT: u32 = 0_u32;
const CAIRO_SUBPIXEL_ORDER_RGB: u32 = 1_u32;
const CAIRO_SUBPIXEL_ORDER_BGR: u32 = 2_u32;
const CAIRO_SUBPIXEL_ORDER_VRGB: u32 = 3_u32;
const CAIRO_SUBPIXEL_ORDER_VBGR: u32 = 4_u32;

type cairo_subpixel_order_t = enum__cairo_subpixel_order;

type enum__cairo_hint_style = c_uint;
const CAIRO_HINT_STYLE_DEFAULT: u32 = 0_u32;
const CAIRO_HINT_STYLE_NONE: u32 = 1_u32;
const CAIRO_HINT_STYLE_SLIGHT: u32 = 2_u32;
const CAIRO_HINT_STYLE_MEDIUM: u32 = 3_u32;
const CAIRO_HINT_STYLE_FULL: u32 = 4_u32;

type cairo_hint_style_t = enum__cairo_hint_style;

type enum__cairo_hint_metrics = c_uint;
const CAIRO_HINT_METRICS_DEFAULT: u32 = 0_u32;
const CAIRO_HINT_METRICS_OFF: u32 = 1_u32;
const CAIRO_HINT_METRICS_ON: u32 = 2_u32;

type cairo_hint_metrics_t = enum__cairo_hint_metrics;

type struct__cairo_font_options = c_void;

type cairo_font_options_t = struct__cairo_font_options;

type enum__cairo_font_type = c_uint;
const CAIRO_FONT_TYPE_TOY: u32 = 0_u32;
const CAIRO_FONT_TYPE_FT: u32 = 1_u32;
const CAIRO_FONT_TYPE_WIN32: u32 = 2_u32;
const CAIRO_FONT_TYPE_QUARTZ: u32 = 3_u32;
const CAIRO_FONT_TYPE_USER: u32 = 4_u32;

type cairo_font_type_t = enum__cairo_font_type;

type cairo_user_scaled_font_init_func_t = *u8;

type cairo_user_scaled_font_render_glyph_func_t = *u8;

type cairo_user_scaled_font_text_to_glyphs_func_t = *u8;

type cairo_user_scaled_font_unicode_to_glyph_func_t = *u8;

type enum__cairo_path_data_type = c_uint;
const CAIRO_PATH_MOVE_TO: u32 = 0_u32;
const CAIRO_PATH_LINE_TO: u32 = 1_u32;
const CAIRO_PATH_CURVE_TO: u32 = 2_u32;
const CAIRO_PATH_CLOSE_PATH: u32 = 3_u32;

type cairo_path_data_type_t = enum__cairo_path_data_type;

type cairo_path_data_t = union__cairo_path_data_t;

type union__cairo_path_data_t = c_void /* FIXME: union type */;

type struct_cairo_path = {
    status: cairo_status_t,
    data: *cairo_path_data_t,
    num_data: c_int,
};

type cairo_path_t = struct_cairo_path;

type enum__cairo_device_type = c_uint;
const CAIRO_DEVICE_TYPE_DRM: u32 = 0_u32;
const CAIRO_DEVICE_TYPE_GL: u32 = 1_u32;
const CAIRO_DEVICE_TYPE_SCRIPT: u32 = 2_u32;
const CAIRO_DEVICE_TYPE_XCB: u32 = 3_u32;
const CAIRO_DEVICE_TYPE_XLIB: u32 = 4_u32;
const CAIRO_DEVICE_TYPE_XML: u32 = 5_u32;

type cairo_device_type_t = enum__cairo_device_type;

type enum__cairo_surface_type = c_uint;
const CAIRO_SURFACE_TYPE_IMAGE: u32 = 0_u32;
const CAIRO_SURFACE_TYPE_PDF: u32 = 1_u32;
const CAIRO_SURFACE_TYPE_PS: u32 = 2_u32;
const CAIRO_SURFACE_TYPE_XLIB: u32 = 3_u32;
const CAIRO_SURFACE_TYPE_XCB: u32 = 4_u32;
const CAIRO_SURFACE_TYPE_GLITZ: u32 = 5_u32;
const CAIRO_SURFACE_TYPE_QUARTZ: u32 = 6_u32;
const CAIRO_SURFACE_TYPE_WIN32: u32 = 7_u32;
const CAIRO_SURFACE_TYPE_BEOS: u32 = 8_u32;
const CAIRO_SURFACE_TYPE_DIRECTFB: u32 = 9_u32;
const CAIRO_SURFACE_TYPE_SVG: u32 = 10_u32;
const CAIRO_SURFACE_TYPE_OS2: u32 = 11_u32;
const CAIRO_SURFACE_TYPE_WIN32_PRINTING: u32 = 12_u32;
const CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: u32 = 13_u32;
const CAIRO_SURFACE_TYPE_SCRIPT: u32 = 14_u32;
const CAIRO_SURFACE_TYPE_QT: u32 = 15_u32;
const CAIRO_SURFACE_TYPE_RECORDING: u32 = 16_u32;
const CAIRO_SURFACE_TYPE_VG: u32 = 17_u32;
const CAIRO_SURFACE_TYPE_GL: u32 = 18_u32;
const CAIRO_SURFACE_TYPE_DRM: u32 = 19_u32;
const CAIRO_SURFACE_TYPE_TEE: u32 = 20_u32;
const CAIRO_SURFACE_TYPE_XML: u32 = 21_u32;
const CAIRO_SURFACE_TYPE_SKIA: u32 = 22_u32;
const CAIRO_SURFACE_TYPE_SUBSURFACE: u32 = 23_u32;

type cairo_surface_type_t = enum__cairo_surface_type;

type enum__cairo_format = c_int;
const CAIRO_FORMAT_INVALID: i32 = -1_i32;
const CAIRO_FORMAT_ARGB32: i32 = 0_i32;
const CAIRO_FORMAT_RGB24: i32 = 1_i32;
const CAIRO_FORMAT_A8: i32 = 2_i32;
const CAIRO_FORMAT_A1: i32 = 3_i32;
const CAIRO_FORMAT_RGB16_565: i32 = 4_i32;

type cairo_format_t = enum__cairo_format;

type enum__cairo_pattern_type = c_uint;
const CAIRO_PATTERN_TYPE_SOLID: u32 = 0_u32;
const CAIRO_PATTERN_TYPE_SURFACE: u32 = 1_u32;
const CAIRO_PATTERN_TYPE_LINEAR: u32 = 2_u32;
const CAIRO_PATTERN_TYPE_RADIAL: u32 = 3_u32;

type cairo_pattern_type_t = enum__cairo_pattern_type;

type enum__cairo_extend = c_uint;
const CAIRO_EXTEND_NONE: u32 = 0_u32;
const CAIRO_EXTEND_REPEAT: u32 = 1_u32;
const CAIRO_EXTEND_REFLECT: u32 = 2_u32;
const CAIRO_EXTEND_PAD: u32 = 3_u32;

type cairo_extend_t = enum__cairo_extend;

type enum__cairo_filter = c_uint;
const CAIRO_FILTER_FAST: u32 = 0_u32;
const CAIRO_FILTER_GOOD: u32 = 1_u32;
const CAIRO_FILTER_BEST: u32 = 2_u32;
const CAIRO_FILTER_NEAREST: u32 = 3_u32;
const CAIRO_FILTER_BILINEAR: u32 = 4_u32;
const CAIRO_FILTER_GAUSSIAN: u32 = 5_u32;

type cairo_filter_t = enum__cairo_filter;

type struct__cairo_region = c_void;

type cairo_region_t = struct__cairo_region;

type struct__cairo_rectangle_int = {
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
};

type cairo_rectangle_int_t = struct__cairo_rectangle_int;

type enum__cairo_region_overlap = c_uint;
const CAIRO_REGION_OVERLAP_IN: u32 = 0_u32;
const CAIRO_REGION_OVERLAP_OUT: u32 = 1_u32;
const CAIRO_REGION_OVERLAP_PART: u32 = 2_u32;

type cairo_region_overlap_t = enum__cairo_region_overlap;

type struct_unnamed1 = {
    _type: cairo_path_data_type_t,
    length: c_int,
};

type struct_unnamed2 = {
    x: c_double,
    y: c_double,
};

#[link_name="cairo"]
extern mod bindgen {

fn cairo_version() -> c_int;

fn cairo_version_string() -> *c_char;

fn cairo_create(++arg0: *cairo_surface_t) -> *cairo_t;

fn cairo_reference(++arg0: *cairo_t) -> *cairo_t;

fn cairo_destroy(++arg0: *cairo_t);

fn cairo_get_reference_count(++arg0: *cairo_t) -> c_uint;

fn cairo_get_user_data(++arg0: *cairo_t, ++arg1: *cairo_user_data_key_t) -> *c_void;

fn cairo_set_user_data(++arg0: *cairo_t, ++arg1: *cairo_user_data_key_t, ++arg2: *c_void, ++arg3: cairo_destroy_func_t) -> cairo_status_t;

fn cairo_save(++arg0: *cairo_t);

fn cairo_restore(++arg0: *cairo_t);

fn cairo_push_group(++arg0: *cairo_t);

fn cairo_push_group_with_content(++arg0: *cairo_t, ++arg1: cairo_content_t);

fn cairo_pop_group(++arg0: *cairo_t) -> *cairo_pattern_t;

fn cairo_pop_group_to_source(++arg0: *cairo_t);

fn cairo_set_operator(++arg0: *cairo_t, ++arg1: cairo_operator_t);

fn cairo_set_source(++arg0: *cairo_t, ++arg1: *cairo_pattern_t);

fn cairo_set_source_rgb(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double);

fn cairo_set_source_rgba(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double);

fn cairo_set_source_surface(++arg0: *cairo_t, ++arg1: *cairo_surface_t, ++arg2: c_double, ++arg3: c_double);

fn cairo_set_tolerance(++arg0: *cairo_t, ++arg1: c_double);

fn cairo_set_antialias(++arg0: *cairo_t, ++arg1: cairo_antialias_t);

fn cairo_set_fill_rule(++arg0: *cairo_t, ++arg1: cairo_fill_rule_t);

fn cairo_set_line_width(++arg0: *cairo_t, ++arg1: c_double);

fn cairo_set_line_cap(++arg0: *cairo_t, ++arg1: cairo_line_cap_t);

fn cairo_set_line_join(++arg0: *cairo_t, ++arg1: cairo_line_join_t);

fn cairo_set_dash(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: c_int, ++arg3: c_double);

fn cairo_set_miter_limit(++arg0: *cairo_t, ++arg1: c_double);

fn cairo_translate(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_scale(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_rotate(++arg0: *cairo_t, ++arg1: c_double);

fn cairo_transform(++arg0: *cairo_t, ++arg1: *cairo_matrix_t);

fn cairo_set_matrix(++arg0: *cairo_t, ++arg1: *cairo_matrix_t);

fn cairo_identity_matrix(++arg0: *cairo_t);

fn cairo_user_to_device(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_user_to_device_distance(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_device_to_user(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_device_to_user_distance(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_new_path(++arg0: *cairo_t);

fn cairo_move_to(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_new_sub_path(++arg0: *cairo_t);

fn cairo_line_to(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_curve_to(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double, ++arg5: c_double, ++arg6: c_double);

fn cairo_arc(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double, ++arg5: c_double);

fn cairo_arc_negative(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double, ++arg5: c_double);

fn cairo_rel_move_to(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_rel_line_to(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_rel_curve_to(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double, ++arg5: c_double, ++arg6: c_double);

fn cairo_rectangle(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double);

fn cairo_close_path(++arg0: *cairo_t);

fn cairo_path_extents(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double);

fn cairo_paint(++arg0: *cairo_t);

fn cairo_paint_with_alpha(++arg0: *cairo_t, ++arg1: c_double);

fn cairo_mask(++arg0: *cairo_t, ++arg1: *cairo_pattern_t);

fn cairo_mask_surface(++arg0: *cairo_t, ++arg1: *cairo_surface_t, ++arg2: c_double, ++arg3: c_double);

fn cairo_stroke(++arg0: *cairo_t);

fn cairo_stroke_preserve(++arg0: *cairo_t);

fn cairo_fill(++arg0: *cairo_t);

fn cairo_fill_preserve(++arg0: *cairo_t);

fn cairo_copy_page(++arg0: *cairo_t);

fn cairo_show_page(++arg0: *cairo_t);

fn cairo_in_stroke(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double) -> cairo_bool_t;

fn cairo_in_fill(++arg0: *cairo_t, ++arg1: c_double, ++arg2: c_double) -> cairo_bool_t;

fn cairo_stroke_extents(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double);

fn cairo_fill_extents(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double);

fn cairo_reset_clip(++arg0: *cairo_t);

fn cairo_clip(++arg0: *cairo_t);

fn cairo_clip_preserve(++arg0: *cairo_t);

fn cairo_clip_extents(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double);

fn cairo_copy_clip_rectangle_list(++arg0: *cairo_t) -> *cairo_rectangle_list_t;

fn cairo_rectangle_list_destroy(++arg0: *cairo_rectangle_list_t);

fn cairo_glyph_allocate(++arg0: c_int) -> *cairo_glyph_t;

fn cairo_glyph_free(++arg0: *cairo_glyph_t);

fn cairo_text_cluster_allocate(++arg0: c_int) -> *cairo_text_cluster_t;

fn cairo_text_cluster_free(++arg0: *cairo_text_cluster_t);

fn cairo_font_options_create() -> *cairo_font_options_t;

fn cairo_font_options_copy(++arg0: *cairo_font_options_t) -> *cairo_font_options_t;

fn cairo_font_options_destroy(++arg0: *cairo_font_options_t);

fn cairo_font_options_status(++arg0: *cairo_font_options_t) -> cairo_status_t;

fn cairo_font_options_merge(++arg0: *cairo_font_options_t, ++arg1: *cairo_font_options_t);

fn cairo_font_options_equal(++arg0: *cairo_font_options_t, ++arg1: *cairo_font_options_t) -> cairo_bool_t;

fn cairo_font_options_hash(++arg0: *cairo_font_options_t) -> c_ulong;

fn cairo_font_options_set_antialias(++arg0: *cairo_font_options_t, ++arg1: cairo_antialias_t);

fn cairo_font_options_get_antialias(++arg0: *cairo_font_options_t) -> cairo_antialias_t;

fn cairo_font_options_set_subpixel_order(++arg0: *cairo_font_options_t, ++arg1: cairo_subpixel_order_t);

fn cairo_font_options_get_subpixel_order(++arg0: *cairo_font_options_t) -> cairo_subpixel_order_t;

fn cairo_font_options_set_hint_style(++arg0: *cairo_font_options_t, ++arg1: cairo_hint_style_t);

fn cairo_font_options_get_hint_style(++arg0: *cairo_font_options_t) -> cairo_hint_style_t;

fn cairo_font_options_set_hint_metrics(++arg0: *cairo_font_options_t, ++arg1: cairo_hint_metrics_t);

fn cairo_font_options_get_hint_metrics(++arg0: *cairo_font_options_t) -> cairo_hint_metrics_t;

fn cairo_select_font_face(++arg0: *cairo_t, ++arg1: *c_char, ++arg2: cairo_font_slant_t, ++arg3: cairo_font_weight_t);

fn cairo_set_font_size(++arg0: *cairo_t, ++arg1: c_double);

fn cairo_set_font_matrix(++arg0: *cairo_t, ++arg1: *cairo_matrix_t);

fn cairo_get_font_matrix(++arg0: *cairo_t, ++arg1: *cairo_matrix_t);

fn cairo_set_font_options(++arg0: *cairo_t, ++arg1: *cairo_font_options_t);

fn cairo_get_font_options(++arg0: *cairo_t, ++arg1: *cairo_font_options_t);

fn cairo_set_font_face(++arg0: *cairo_t, ++arg1: *cairo_font_face_t);

fn cairo_get_font_face(++arg0: *cairo_t) -> *cairo_font_face_t;

fn cairo_set_scaled_font(++arg0: *cairo_t, ++arg1: *cairo_scaled_font_t);

fn cairo_get_scaled_font(++arg0: *cairo_t) -> *cairo_scaled_font_t;

fn cairo_show_text(++arg0: *cairo_t, ++arg1: *c_char);

fn cairo_show_glyphs(++arg0: *cairo_t, ++arg1: *cairo_glyph_t, ++arg2: c_int);

fn cairo_show_text_glyphs(++arg0: *cairo_t, ++arg1: *c_char, ++arg2: c_int, ++arg3: *cairo_glyph_t, ++arg4: c_int, ++arg5: *cairo_text_cluster_t, ++arg6: c_int, ++arg7: cairo_text_cluster_flags_t);

fn cairo_text_path(++arg0: *cairo_t, ++arg1: *c_char);

fn cairo_glyph_path(++arg0: *cairo_t, ++arg1: *cairo_glyph_t, ++arg2: c_int);

fn cairo_text_extents(++arg0: *cairo_t, ++arg1: *c_char, ++arg2: *cairo_text_extents_t);

fn cairo_glyph_extents(++arg0: *cairo_t, ++arg1: *cairo_glyph_t, ++arg2: c_int, ++arg3: *cairo_text_extents_t);

fn cairo_font_extents(++arg0: *cairo_t, ++arg1: *cairo_font_extents_t);

fn cairo_font_face_reference(++arg0: *cairo_font_face_t) -> *cairo_font_face_t;

fn cairo_font_face_destroy(++arg0: *cairo_font_face_t);

fn cairo_font_face_get_reference_count(++arg0: *cairo_font_face_t) -> c_uint;

fn cairo_font_face_status(++arg0: *cairo_font_face_t) -> cairo_status_t;

fn cairo_font_face_get_type(++arg0: *cairo_font_face_t) -> cairo_font_type_t;

fn cairo_font_face_get_user_data(++arg0: *cairo_font_face_t, ++arg1: *cairo_user_data_key_t) -> *c_void;

fn cairo_font_face_set_user_data(++arg0: *cairo_font_face_t, ++arg1: *cairo_user_data_key_t, ++arg2: *c_void, ++arg3: cairo_destroy_func_t) -> cairo_status_t;

fn cairo_scaled_font_create(++arg0: *cairo_font_face_t, ++arg1: *cairo_matrix_t, ++arg2: *cairo_matrix_t, ++arg3: *cairo_font_options_t) -> *cairo_scaled_font_t;

fn cairo_scaled_font_reference(++arg0: *cairo_scaled_font_t) -> *cairo_scaled_font_t;

fn cairo_scaled_font_destroy(++arg0: *cairo_scaled_font_t);

fn cairo_scaled_font_get_reference_count(++arg0: *cairo_scaled_font_t) -> c_uint;

fn cairo_scaled_font_status(++arg0: *cairo_scaled_font_t) -> cairo_status_t;

fn cairo_scaled_font_get_type(++arg0: *cairo_scaled_font_t) -> cairo_font_type_t;

fn cairo_scaled_font_get_user_data(++arg0: *cairo_scaled_font_t, ++arg1: *cairo_user_data_key_t) -> *c_void;

fn cairo_scaled_font_set_user_data(++arg0: *cairo_scaled_font_t, ++arg1: *cairo_user_data_key_t, ++arg2: *c_void, ++arg3: cairo_destroy_func_t) -> cairo_status_t;

fn cairo_scaled_font_extents(++arg0: *cairo_scaled_font_t, ++arg1: *cairo_font_extents_t);

fn cairo_scaled_font_text_extents(++arg0: *cairo_scaled_font_t, ++arg1: *c_char, ++arg2: *cairo_text_extents_t);

fn cairo_scaled_font_glyph_extents(++arg0: *cairo_scaled_font_t, ++arg1: *cairo_glyph_t, ++arg2: c_int, ++arg3: *cairo_text_extents_t);

fn cairo_scaled_font_text_to_glyphs(++arg0: *cairo_scaled_font_t, ++arg1: c_double, ++arg2: c_double, ++arg3: *c_char, ++arg4: c_int, ++arg5: **cairo_glyph_t, ++arg6: *c_int, ++arg7: **cairo_text_cluster_t, ++arg8: *c_int, ++arg9: *cairo_text_cluster_flags_t) -> cairo_status_t;

fn cairo_scaled_font_get_font_face(++arg0: *cairo_scaled_font_t) -> *cairo_font_face_t;

fn cairo_scaled_font_get_font_matrix(++arg0: *cairo_scaled_font_t, ++arg1: *cairo_matrix_t);

fn cairo_scaled_font_get_ctm(++arg0: *cairo_scaled_font_t, ++arg1: *cairo_matrix_t);

fn cairo_scaled_font_get_scale_matrix(++arg0: *cairo_scaled_font_t, ++arg1: *cairo_matrix_t);

fn cairo_scaled_font_get_font_options(++arg0: *cairo_scaled_font_t, ++arg1: *cairo_font_options_t);

fn cairo_toy_font_face_create(++arg0: *c_char, ++arg1: cairo_font_slant_t, ++arg2: cairo_font_weight_t) -> *cairo_font_face_t;

fn cairo_toy_font_face_get_family(++arg0: *cairo_font_face_t) -> *c_char;

fn cairo_toy_font_face_get_slant(++arg0: *cairo_font_face_t) -> cairo_font_slant_t;

fn cairo_toy_font_face_get_weight(++arg0: *cairo_font_face_t) -> cairo_font_weight_t;

fn cairo_user_font_face_create() -> *cairo_font_face_t;

fn cairo_user_font_face_set_init_func(++arg0: *cairo_font_face_t, ++arg1: cairo_user_scaled_font_init_func_t);

fn cairo_user_font_face_set_render_glyph_func(++arg0: *cairo_font_face_t, ++arg1: cairo_user_scaled_font_render_glyph_func_t);

fn cairo_user_font_face_set_text_to_glyphs_func(++arg0: *cairo_font_face_t, ++arg1: cairo_user_scaled_font_text_to_glyphs_func_t);

fn cairo_user_font_face_set_unicode_to_glyph_func(++arg0: *cairo_font_face_t, ++arg1: cairo_user_scaled_font_unicode_to_glyph_func_t);

fn cairo_user_font_face_get_init_func(++arg0: *cairo_font_face_t) -> cairo_user_scaled_font_init_func_t;

fn cairo_user_font_face_get_render_glyph_func(++arg0: *cairo_font_face_t) -> cairo_user_scaled_font_render_glyph_func_t;

fn cairo_user_font_face_get_text_to_glyphs_func(++arg0: *cairo_font_face_t) -> cairo_user_scaled_font_text_to_glyphs_func_t;

fn cairo_user_font_face_get_unicode_to_glyph_func(++arg0: *cairo_font_face_t) -> cairo_user_scaled_font_unicode_to_glyph_func_t;

fn cairo_get_operator(++arg0: *cairo_t) -> cairo_operator_t;

fn cairo_get_source(++arg0: *cairo_t) -> *cairo_pattern_t;

fn cairo_get_tolerance(++arg0: *cairo_t) -> c_double;

fn cairo_get_antialias(++arg0: *cairo_t) -> cairo_antialias_t;

fn cairo_has_current_point(++arg0: *cairo_t) -> cairo_bool_t;

fn cairo_get_current_point(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_get_fill_rule(++arg0: *cairo_t) -> cairo_fill_rule_t;

fn cairo_get_line_width(++arg0: *cairo_t) -> c_double;

fn cairo_get_line_cap(++arg0: *cairo_t) -> cairo_line_cap_t;

fn cairo_get_line_join(++arg0: *cairo_t) -> cairo_line_join_t;

fn cairo_get_miter_limit(++arg0: *cairo_t) -> c_double;

fn cairo_get_dash_count(++arg0: *cairo_t) -> c_int;

fn cairo_get_dash(++arg0: *cairo_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_get_matrix(++arg0: *cairo_t, ++arg1: *cairo_matrix_t);

fn cairo_get_target(++arg0: *cairo_t) -> *cairo_surface_t;

fn cairo_get_group_target(++arg0: *cairo_t) -> *cairo_surface_t;

fn cairo_copy_path(++arg0: *cairo_t) -> *cairo_path_t;

fn cairo_copy_path_flat(++arg0: *cairo_t) -> *cairo_path_t;

fn cairo_append_path(++arg0: *cairo_t, ++arg1: *cairo_path_t);

fn cairo_path_destroy(++arg0: *cairo_path_t);

fn cairo_status(++arg0: *cairo_t) -> cairo_status_t;

fn cairo_status_to_string(++arg0: cairo_status_t) -> *c_char;

fn cairo_surface_create_similar(++arg0: *cairo_surface_t, ++arg1: cairo_content_t, ++arg2: c_int, ++arg3: c_int) -> *cairo_surface_t;

fn cairo_surface_create_for_rectangle(++arg0: *cairo_surface_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double) -> *cairo_surface_t;

fn cairo_surface_reference(++arg0: *cairo_surface_t) -> *cairo_surface_t;

fn cairo_surface_finish(++arg0: *cairo_surface_t);

fn cairo_surface_destroy(++arg0: *cairo_surface_t);

fn cairo_surface_get_reference_count(++arg0: *cairo_surface_t) -> c_uint;

fn cairo_surface_status(++arg0: *cairo_surface_t) -> cairo_status_t;

fn cairo_surface_get_type(++arg0: *cairo_surface_t) -> cairo_surface_type_t;

fn cairo_surface_get_content(++arg0: *cairo_surface_t) -> cairo_content_t;

fn cairo_surface_write_to_png(++arg0: *cairo_surface_t, ++arg1: *c_char) -> cairo_status_t;

fn cairo_surface_write_to_png_stream(++arg0: *cairo_surface_t, ++arg1: cairo_write_func_t, ++arg2: *c_void) -> cairo_status_t;

fn cairo_surface_get_user_data(++arg0: *cairo_surface_t, ++arg1: *cairo_user_data_key_t) -> *c_void;

fn cairo_surface_set_user_data(++arg0: *cairo_surface_t, ++arg1: *cairo_user_data_key_t, ++arg2: *c_void, ++arg3: cairo_destroy_func_t) -> cairo_status_t;

fn cairo_surface_get_font_options(++arg0: *cairo_surface_t, ++arg1: *cairo_font_options_t);

fn cairo_surface_flush(++arg0: *cairo_surface_t);

fn cairo_surface_mark_dirty(++arg0: *cairo_surface_t);

fn cairo_surface_mark_dirty_rectangle(++arg0: *cairo_surface_t, ++arg1: c_int, ++arg2: c_int, ++arg3: c_int, ++arg4: c_int);

fn cairo_surface_set_device_offset(++arg0: *cairo_surface_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_surface_get_device_offset(++arg0: *cairo_surface_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_surface_set_fallback_resolution(++arg0: *cairo_surface_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_surface_get_fallback_resolution(++arg0: *cairo_surface_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_surface_copy_page(++arg0: *cairo_surface_t);

fn cairo_surface_show_page(++arg0: *cairo_surface_t);

fn cairo_surface_has_show_text_glyphs(++arg0: *cairo_surface_t) -> cairo_bool_t;

fn cairo_image_surface_create(++arg0: cairo_format_t, ++arg1: c_int, ++arg2: c_int) -> *cairo_surface_t;

fn cairo_format_stride_for_width(++arg0: cairo_format_t, ++arg1: c_int) -> c_int;

fn cairo_image_surface_create_for_data(++arg0: *c_uchar, ++arg1: cairo_format_t, ++arg2: c_int, ++arg3: c_int, ++arg4: c_int) -> *cairo_surface_t;

fn cairo_image_surface_get_data(++arg0: *cairo_surface_t) -> *c_uchar;

fn cairo_image_surface_get_format(++arg0: *cairo_surface_t) -> cairo_format_t;

fn cairo_image_surface_get_width(++arg0: *cairo_surface_t) -> c_int;

fn cairo_image_surface_get_height(++arg0: *cairo_surface_t) -> c_int;

fn cairo_image_surface_get_stride(++arg0: *cairo_surface_t) -> c_int;

fn cairo_image_surface_create_from_png(++arg0: *c_char) -> *cairo_surface_t;

fn cairo_image_surface_create_from_png_stream(++arg0: cairo_read_func_t, ++arg1: *c_void) -> *cairo_surface_t;

fn cairo_recording_surface_create(++arg0: cairo_content_t, ++arg1: *cairo_rectangle_t) -> *cairo_surface_t;

fn cairo_recording_surface_ink_extents(++arg0: *cairo_surface_t, ++arg1: *c_double, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double);

fn cairo_pattern_create_rgb(++arg0: c_double, ++arg1: c_double, ++arg2: c_double) -> *cairo_pattern_t;

fn cairo_pattern_create_rgba(++arg0: c_double, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double) -> *cairo_pattern_t;

fn cairo_pattern_create_for_surface(++arg0: *cairo_surface_t) -> *cairo_pattern_t;

fn cairo_pattern_create_linear(++arg0: c_double, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double) -> *cairo_pattern_t;

fn cairo_pattern_create_radial(++arg0: c_double, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double, ++arg5: c_double) -> *cairo_pattern_t;

fn cairo_pattern_reference(++arg0: *cairo_pattern_t) -> *cairo_pattern_t;

fn cairo_pattern_destroy(++arg0: *cairo_pattern_t);

fn cairo_pattern_get_reference_count(++arg0: *cairo_pattern_t) -> c_uint;

fn cairo_pattern_status(++arg0: *cairo_pattern_t) -> cairo_status_t;

fn cairo_pattern_get_user_data(++arg0: *cairo_pattern_t, ++arg1: *cairo_user_data_key_t) -> *c_void;

fn cairo_pattern_set_user_data(++arg0: *cairo_pattern_t, ++arg1: *cairo_user_data_key_t, ++arg2: *c_void, ++arg3: cairo_destroy_func_t) -> cairo_status_t;

fn cairo_pattern_get_type(++arg0: *cairo_pattern_t) -> cairo_pattern_type_t;

fn cairo_pattern_add_color_stop_rgb(++arg0: *cairo_pattern_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double);

fn cairo_pattern_add_color_stop_rgba(++arg0: *cairo_pattern_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double, ++arg5: c_double);

fn cairo_pattern_set_matrix(++arg0: *cairo_pattern_t, ++arg1: *cairo_matrix_t);

fn cairo_pattern_get_matrix(++arg0: *cairo_pattern_t, ++arg1: *cairo_matrix_t);

fn cairo_pattern_set_extend(++arg0: *cairo_pattern_t, ++arg1: cairo_extend_t);

fn cairo_pattern_get_extend(++arg0: *cairo_pattern_t) -> cairo_extend_t;

fn cairo_pattern_set_filter(++arg0: *cairo_pattern_t, ++arg1: cairo_filter_t);

fn cairo_pattern_get_filter(++arg0: *cairo_pattern_t) -> cairo_filter_t;

fn cairo_pattern_get_rgba(++arg0: *cairo_pattern_t, ++arg1: *c_double, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double) -> cairo_status_t;

fn cairo_pattern_get_surface(++arg0: *cairo_pattern_t, ++arg1: **cairo_surface_t) -> cairo_status_t;

fn cairo_pattern_get_color_stop_rgba(++arg0: *cairo_pattern_t, ++arg1: c_int, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double, ++arg5: *c_double, ++arg6: *c_double) -> cairo_status_t;

fn cairo_pattern_get_color_stop_count(++arg0: *cairo_pattern_t, ++arg1: *c_int) -> cairo_status_t;

fn cairo_pattern_get_linear_points(++arg0: *cairo_pattern_t, ++arg1: *c_double, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double) -> cairo_status_t;

fn cairo_pattern_get_radial_circles(++arg0: *cairo_pattern_t, ++arg1: *c_double, ++arg2: *c_double, ++arg3: *c_double, ++arg4: *c_double, ++arg5: *c_double, ++arg6: *c_double) -> cairo_status_t;

fn cairo_matrix_init(++arg0: *cairo_matrix_t, ++arg1: c_double, ++arg2: c_double, ++arg3: c_double, ++arg4: c_double, ++arg5: c_double, ++arg6: c_double);

fn cairo_matrix_init_identity(++arg0: *cairo_matrix_t);

fn cairo_matrix_init_translate(++arg0: *cairo_matrix_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_matrix_init_scale(++arg0: *cairo_matrix_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_matrix_init_rotate(++arg0: *cairo_matrix_t, ++arg1: c_double);

fn cairo_matrix_translate(++arg0: *cairo_matrix_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_matrix_scale(++arg0: *cairo_matrix_t, ++arg1: c_double, ++arg2: c_double);

fn cairo_matrix_rotate(++arg0: *cairo_matrix_t, ++arg1: c_double);

fn cairo_matrix_invert(++arg0: *cairo_matrix_t) -> cairo_status_t;

fn cairo_matrix_multiply(++arg0: *cairo_matrix_t, ++arg1: *cairo_matrix_t, ++arg2: *cairo_matrix_t);

fn cairo_matrix_transform_distance(++arg0: *cairo_matrix_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_matrix_transform_point(++arg0: *cairo_matrix_t, ++arg1: *c_double, ++arg2: *c_double);

fn cairo_debug_reset_static_data();

}
