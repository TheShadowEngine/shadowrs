use super::charts_config::{ChartColors, ChartConfig};
use shadow_number_formatter::NumberFormatter;
use num::ToPrimitive;
use std::borrow::Cow;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy)]
pub struct ComputeRectsOptions<'a> {
    pub chart_config: &'a ChartConfig,   
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}