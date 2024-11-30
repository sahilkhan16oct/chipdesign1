use quick_js::{Context, JsValue};

pub(super) struct RectBool {
    pub(super) left: bool,
    pub(super) right: bool,
    pub(super) top: bool,
    pub(super) bottom: bool,
}

impl RectBool {
    fn index(&self) -> usize {
        let bit_left = self.left as usize;
        let bit_right = (self.right as usize) << 1;
        let bit_top = (self.top as usize) << 2;
        let bit_bottom = (self.bottom as usize) << 3;

        bit_left | bit_right | bit_top | bit_bottom
    }
}

pub(super) struct BoolFunc {
    table: Vec<bool>,
}

impl BoolFunc {
    pub(super) fn value(&self, bool: RectBool) -> bool {
        let index = bool.index();
        self.table[index]
    }

    pub(super) fn value_2x(&self, bool0: RectBool, bool1: RectBool) -> bool {
        let index = bool0.index() | (bool1.index() << 4);
        self.table[index]
    }

    pub(super) fn new(expression: &str, is_two_values: bool) -> Result<Self, String> {
        let context = Context::new().unwrap();

        let js_code = if is_two_values {
            format!(r#"
            (function() {{
                var results = [];
                for (let i = 0; i < 256; i++) {{
                    let left_0 = (i & 1) != 0;
                    let right_0 = (i & 2) != 0;
                    let top_0 = (i & 4) != 0;
                    let bottom_0 = (i & 8) != 0;
                    let left_1 = (i & 16) != 0;
                    let right_1 = (i & 32) != 0;
                    let top_1 = (i & 64) != 0;
                    let bottom_1 = (i & 128) != 0;
                    results.push(eval('{expression}'));
                }}
                return results;
            }}())
            "#)
        } else {
            format!(r#"
            (function() {{
                var results = [];
                for (let i = 0; i < 16; i++) {{
                    let left_0 = (i & 1) != 0;
                    let right_0 = (i & 2) != 0;
                    let top_0 = (i & 4) != 0;
                    let bottom_0 = (i & 8) != 0;
                    results.push(eval('{expression}'));
                }}
                return results;
            }}())
        "#)
        };

        let result = context.eval(&js_code).map_err(|e| e.to_string())?;

        if let JsValue::Array(values) = result {
            let table = values.into_iter().map(|v| {
                match v {
                    JsValue::Bool(b) => b,
                    _ => false,
                }
            }).collect();
            Ok(Self { table })
        } else {
            Err("Expected an array of boolean results".to_string())
        }
    }
}