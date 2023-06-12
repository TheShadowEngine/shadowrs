use ndarray::prelude::*;
use num::ToPrimitive;
use shadow_table::{
    EnumTableColumn, EnumTableColumnView, NumberTableColumn, NumberTableColumnView, TableColumn,
    TableColumnView, TableValue,
};
use shadow_zip::zip;

#[derive(Clone, Debug)]
pub struct IdentityFeatureGroup {
    pub source_column_name: String,
}

impl IdentityFeatureGroup {
    pub fn compute_table(&self, column: TableColumnView, progress: &impl Fn(u64)) -> TableColumn {
        let column = match column {
            TableColumnView::Unknown(_) => unimplemented!(),
            TableColumnView::Number(column) => {
                TableColumn::Number(self.compute_table_for_number_column(column))
            }
            TableColumnView::Enum(column) => {
                TableColumn::Enum(self.compute_table_for_enum_column(column))
            }
            TableColumnView::Text(_) => unimplemented!(),
        };
        progress(column.len().to_u64().unwrap());
        column
    }

    pub fn compute_array_f32(
        &self,
        features: ArrayViewMut2<f32>,
        column: TableColumnView,
        progress: &impl Fn(),
    ) {
        match column {
            TableColumnView::Unknown(_) => unimplemented!(),
            TableColumnView::Number(column) => {
                self.compute_array_f32_for_number_column(features, column, progress)
            }
            TableColumnView::Enum(column) => {
                self.compute_array_f32_for_enum_column(features, column, progress)
            }
            TableColumnView::Text(_) => unimplemented!(),
        }
    }

    pub fn compute_array_value(
        &self,
        features: ArrayViewMut2<TableValue>,
        column: TableColumnView,
        progress: &impl Fn(),
    ) {
        match column {
            TableColumnView::Unknown(_) => unimplemented!(),
            TableColumnView::Number(column) => {
                self.compute_array_value_for_number_column(features, column, progress)
            }
            TableColumnView::Enum(column) => {
                self.compute_array_value_for_enum_column(features, column, progress)
            }
            TableColumnView::Text(_) => unimplemented!(),
        }
    }

    fn compute_table_for_number_column(&self, column: NumberTableColumnView) -> NumberTableColumn {
        NumberTableColumn::new(
            column.name().map(|name| name.to_owned()),
            column.as_slice().to_owned(),
        )
    }

    fn compute_table_for_enum_column(&self, column: EnumTableColumnView) -> EnumTableColumn {
        EnumTableColumn::new(
            column.name().map(|name| name.to_owned()),
            column.variants().to_owned(),
            column.as_slice().to_owned(),
        )
    }

    fn compute_array_f32_for_number_column(
        &self,
        mut features: ArrayViewMut2<f32>,
        column: NumberTableColumnView,
        progress: &impl Fn(),
    ) {
        for (feature, value) in zip!(features.iter_mut(), column.view().iter()) {
            *feature = *value;
            progress()
        }
    }

    fn compute_array_f32_for_enum_column(
        &self,
        mut features: ArrayViewMut2<f32>,
        column: EnumTableColumnView,
        progress: &impl Fn(),
    ) {
        for (feature, value) in zip!(features.iter_mut(), column.view().iter()) {
            *feature = value.map(|v| v.get().to_f32().unwrap()).unwrap_or(0.0);
            progress()
        }
    }

    fn compute_array_value_for_number_column(
        &self,
        mut features: ArrayViewMut2<TableValue>,
        column: NumberTableColumnView,
        progress: &impl Fn(),
    ) {
        for (feature_column, column_value) in zip!(features.column_mut(0), column.iter()) {
            *feature_column = TableValue::Number(*column_value);
            progress()
        }
    }

    fn compute_array_value_for_enum_column(
        &self,
        mut features: ArrayViewMut2<TableValue>,
        column: EnumTableColumnView,
        progress: &impl Fn(),
    ) {
        for (feature_column, column_value) in zip!(features.column_mut(0), column.iter()) {
            *feature_column = TableValue::Enum(*column_value);
            progress()
        }
    }
}
