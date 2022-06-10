pub mod test_vector {
    pub fn test() {
        crate::dev_log("vector start");
        // 创建
        let mut vector: Vec<i32> = Vec::new();
        // 默认值创建
        let mut v = vec![1, 2, 3];

        // 更新 vector
        vector.push(12);
        vector.push(13);
        v.push(14);

        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(6) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

        // 遍历
        let mut v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }

        // 创建可变引用， 解引用修改值
        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }

        println!("vector");

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
    
        // 创建枚举
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        for i in &row {
            match i {
                SpreadsheetCell::Int(state) => {println!("vector, {}", state);},
                SpreadsheetCell::Text(string) => {println!("vector, {}", string);},
                SpreadsheetCell::Float(num) => {println!("vector, {}", num);}
            }
        }

        crate::dev_log("vector end");
    }
}
