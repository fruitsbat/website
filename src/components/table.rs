use maud::{html, Markup, Render};



pub struct Table {
    pub rows: Vec<Row>,
}

impl Render for Table {
    fn render(&self) -> Markup {
        html! {
            table {
                @for row in self.rows.iter() {
                    (row)
                }
            }
        }
    }
}

pub struct Row {
    pub key: &'static str,
    pub value: Markup,
}

impl Render for Row {
    fn render(&self) -> Markup {
        html! {
            tr {
                td class="mauve" {(self.key)}
                td {(self.value)}
            }
        }
    }
}
