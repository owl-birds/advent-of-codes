struct Person {
    name: String,
    height: i32,
}
impl Person {
    pub fn new(name: String, height: i32) -> Self {
        Self { name, height }
    }
}
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut temp: Vec<Person> = vec![];
        for i in 0..names.len() {
            temp.push(Person::new(names[i].to_string(), heights[i]));
        }
        temp.sort_by(|a, b| b.height.cmp(&a.height));
        for i in 0..temp.len() {
            result.push(temp[i].name.to_string());
        }
        result
    }
}
