#[cfg(test)]
mod tests {

    #[test]
    fn test_infer() {
        let csv = r#"number,enum,text
1,test,hello
2,test,world
"#;
        let table = Table::from_csv(
            &mut csv::Reader::from_reader(std::io::Cursor::new(csv)),
            csv.len().to_u64().unwrap(),
            FromCsvOptions {
                column_types: None,
                infer_options: InferOptions {
                    enum_max_unique_values: 1,
                },
                ..Default::default()
            },
            &mut |_| {},
        )
        .unwrap();
        insta::assert_debug_snapshot!(table, @r###"
 Table {
     columns: [
         Number(
             NumberTableColumn {
                 name: Some(
                     "number",
                 ),
                 data: [
                     1.0,
                     2.0,
                 ],
             },
         ),
         Enum(
             EnumTableColumn {
                 name: Some(
                     "enum",
                 ),
                 variants: [
                     "test",
                 ],
                 data: [
                     Some(
                         1,
                     ),
                     Some(
                         1,
                     ),
                 ],
                 variants_map: {
                     "test": 1,
                 },
             },
         ),
         Text(
             TextTableColumn {
                 name: Some(
                     "text",
                 ),
                 data: [
                     "hello",
                     "world",
                 ],
             },
         ),
     ],
 }
 "###);
    }

    #[test]
    fn test_column_types() {
        let csv = r#"number,text,enum
1,test,hello
2,test,world
"#;
        let mut column_types = BTreeMap::new();
        column_types.insert("text".to_owned(), TableColumnType::Text);
        column_types.insert(
            "enum".to_owned(),
            TableColumnType::Enum {
                variants: vec!["hello".to_owned(), "world".to_owned()],
            },
        );
        let table = Table::from_csv(
            &mut csv::Reader::from_reader(std::io::Cursor::new(csv)),
            csv.len().to_u64().unwrap(),
            FromCsvOptions {
                column_types: Some(column_types),
                infer_options: InferOptions {
                    enum_max_unique_values: 2,
                },
                ..Default::default()
            },
            &mut |_| {},
        )
        .unwrap();
        insta::assert_debug_snapshot!(table, @r###"
 Table {
     columns: [
         Number(
             NumberTableColumn {
                 name: Some(
                     "number",
                 ),
                 data: [
                     1.0,
                     2.0,
                 ],
             },
         ),
         Text(
             TextTableColumn {
                 name: Some(
                     "text",
                 ),
                 data: [
                     "test",
                     "test",
                 ],
             },
         ),
         Enum(
             EnumTableColumn {
                 name: Some(
                     "enum",
                 ),
                 variants: [
                     "hello",
                     "world",
                 ],
                 data: [
                     Some(
                         1,
                     ),
                     Some(
                         2,
                     ),
                 ],
                 variants_map: {
                     "hello": 1,
                     "world": 2,
                 },
             },
         ),
     ],
 }
 "###);
    }
}
