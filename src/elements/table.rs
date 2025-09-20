use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct Table {
    #[pyo3(get, set)]
    title: String,
    _rows: HashMap<String, String>,
    _max_length_row: usize,
}

#[pymethods]
impl Table {
    #[new]
    fn new(title: String) -> Self {
        Table {
            title,
            _rows: HashMap::new(),
            _max_length_row: 0,
        }
    }

    pub fn add(&mut self, attribute: String, value: String) -> PyResult<()> {
        self._max_length_row = self._max_length_row.max(attribute.len() + value.len());
        self._rows.insert(attribute, value);
        Ok(())
    }

    pub fn build(&self) -> PyResult<String> {
        let length_table = self.title.len()
            + self._max_length_row
            + 20
            + (self.title.len() + self._max_length_row) % 2;

        let mut table = Vec::new();
        table.push(self._get_header(length_table));

        for (key, value) in &self._rows {
            table.push(self._get_row(length_table / 2, key, value));
            table.push(self._get_separation(length_table));
        }

        Ok(table.join("\n"))
    }

    fn _get_header(&self, length_table: usize) -> String {
        let border = "+".to_string() + &"-".repeat(length_table) + "+";
        let title_line = format!("|{:^width$}|", self.title, width = length_table);
        format!("{}\n{}\n{}", border, title_line, border)
    }

    fn _get_row(&self, half_length: usize, a: &str, b: &str) -> String {
        let left = format!("| {:<width$}", a, width = half_length - 1);
        let right = format!("|{:^width$}|", b, width = half_length - 1);
        format!("{}{}", left, right)
    }

    fn _get_separation(&self, length_table: usize) -> String {
        format!(
            "+{}+{}+",
            "-".repeat(length_table / 2),
            "-".repeat(length_table / 2 - 1)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_single_row() {
        let mut table = Table::new("Test Table".to_string());
        table.add("Name".to_string(), "Alice".to_string()).unwrap();

        assert_eq!(table._rows.len(), 1);
        assert!(table._rows.contains_key("Name"));
        assert_eq!(table._rows["Name"], "Alice");
    }

    #[test]
    fn test_add_multiple_rows() {
        let mut table = Table::new("Users".to_string());
        table.add("Name".to_string(), "Bob".to_string()).unwrap();
        table.add("Age".to_string(), "25".to_string()).unwrap();

        assert_eq!(table._rows.len(), 2);
        assert_eq!(table._max_length_row, "Name".len() + "Bob".len());
    }

    #[test]
    fn test_build_table_output() {
        let mut table = Table::new("My Table".to_string());
        table
            .add("Language".to_string(), "Rust".to_string())
            .unwrap();
        table
            .add("Level".to_string(), "Advanced".to_string())
            .unwrap();

        let output = table.build().unwrap();

        // Check the title exists
        assert!(output.contains("My Table"));

        // Check headers and formatting
        assert!(output.contains("+"));
        assert!(output.contains("| Language"));
        assert!(output.contains("Rust"));

        // Optional: print for debug (can be removed)
        println!("{}", output);
    }

    #[test]
    fn test_empty_table() {
        let table = Table::new("Empty".to_string());
        let output = table.build().unwrap();

        // Should still contain header and no panic
        assert!(output.contains("Empty"));
        assert!(output.contains("+"));
    }
}
