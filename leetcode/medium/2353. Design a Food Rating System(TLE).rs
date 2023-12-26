use std::collections::HashMap;
use std::cmp::Ordering;

// pub fn is_lexic_greater(str_1: &String, str_2: &String) -> bool { 
//     let mut temp_1: Vec<char> = str_1.chars().collect();
//     let mut temp_2: Vec<char> = str_2.chars().collect();

//     for i in 0..temp_1.len() {
//         if i >= temp_2.len() {break}
//     }

//     true
// } // https://www.hackertouch.com/examples-of-string-comparison-in-rust.html

#[derive(Debug)]
struct FoodDesc {
    food_name: String,
    // FIRST VERSION
    // food_type: String,
    // FIRST VERSION
    rating: i32
}
impl FoodDesc {
    fn new(food_name: String, 
    // FIRST VERSION
    // food_type: String, 
    // FIRST VERSION
    rating: i32) -> Self {
        Self {
            food_name,
            // food_type,
            rating
        }
    }
    fn change_rating(&mut self, new_rating: i32) {
        self.rating = new_rating;
    }
}

struct FoodRatings {
    // // first version
    // all_foods: Vec<FoodDesc>

    // second version
    type_foods: HashMap<String, Vec<FoodDesc>>,
    food_name_type: HashMap<String, String>,
    // HIGHEST RATED FOOD RATING? CHACE
    // highest rated based on the food type
    type_highest_rated_food: HashMap<String, FoodDesc>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        // // FIRST VERSION
        // let mut temp_all_foods: Vec<FoodDesc> = vec![];
        // for i in 0..foods.len() {
        //     temp_all_foods.push(FoodDesc::new(foods[i].to_string(), cuisines[i].to_string(), ratings[i]));
        // }
        // Self::sort_foods(&mut temp_all_foods);

        // // // TEST
        // // println!("a:{}", 'a' as usize);
        // // println!("b:{}", 'b' as i32);
        // // println!("c:{}", 'c' as u32);
        // // // TEST
        // Self {
        //     all_foods: temp_all_foods
        // }        
        // // FIRST VERSION

        // SECOND VERSION
        let mut temp_type_foods: HashMap<String, Vec<FoodDesc>> = HashMap::new();
        let mut temp_food_name_type: HashMap<String, String> = HashMap::new();
        let mut temp_type_highest_rated_food: HashMap<String, FoodDesc> = HashMap::new();
        
        for i in 0..cuisines.len() {
            temp_food_name_type.insert(foods[i].to_string(), cuisines[i].to_string());
            if let Some(foods_vec) = temp_type_foods.get_mut(&cuisines[i]) {
                foods_vec.push(FoodDesc::new(foods[i].to_string(), ratings[i]));
            
                //
                let mut prev_max_rating: i32 = temp_type_highest_rated_food.get(&cuisines[i]).unwrap().rating;
                let mut prev_food: String = temp_type_highest_rated_food.get(&cuisines[i]).unwrap().food_name.to_string();
                if ratings[i] > prev_max_rating {
                    temp_type_highest_rated_food.insert(cuisines[i].to_string(), FoodDesc::new(foods[i].to_string(), ratings[i]));
                }
                if ratings[i] == prev_max_rating && foods[i] < prev_food {
                    temp_type_highest_rated_food.insert(cuisines[i].to_string(), FoodDesc::new(foods[i].to_string(), ratings[i]));
                }

            } else {
                temp_type_foods.insert(cuisines[i].to_string(), vec![FoodDesc::new(foods[i].to_string(), ratings[i])]);

                //

                temp_type_highest_rated_food.insert(cuisines[i].to_string(), FoodDesc::new(foods[i].to_string(), ratings[i]));
            }
        }
        // for (_, foods_vec) in temp_type_foods.iter_mut() {
        //     Self::sort_foods(foods_vec);
        // }
        Self {
            type_foods: temp_type_foods,
            food_name_type: temp_food_name_type,
            type_highest_rated_food:  temp_type_highest_rated_food
        }
        // SECOND VERSION
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        // // FIRST VERSION
        // let mut i: usize = 0;
        // while i < self.all_foods.len() && self.all_foods[i].food_name != food {
        //     i += 1;
        // }
        // // println!("{:?}", self.all_foods[i]);
        // self.all_foods[i].rating = new_rating;
        // Self::sort_foods(&mut self.all_foods);
        // // FIRST VERSION
        
        // SECOND VERSION
        let what_type: String = self.food_name_type.get(&food).unwrap().to_string();
        if let Some(foods_vec) = self.type_foods.get_mut(&what_type) {
            // let mut idx: i32 = -1; 
            for i in 0..foods_vec.len() {
                if foods_vec[i].food_name == food {
                    foods_vec[i].rating = new_rating;
                    // idx = i as i32;
                    break;
                }
            }
            // Self::sort_foods(foods_vec);
            
            // finding the new
            // let prev_max_rating: i32 = self.type_highest_rated_food.get(&what_type).unwrap().rating;
            let prev_food: String = self.type_highest_rated_food.get(&what_type).unwrap().food_name.to_string();
            if prev_food == food {
                self.type_highest_rated_food.insert(what_type.to_string(), FoodDesc::new(prev_food, new_rating));
            }
            for i in 0..foods_vec.len() {
                let prev_max_rating: i32 = self.type_highest_rated_food.get(&what_type).unwrap().rating;
                let prev_food: String = self.type_highest_rated_food.get(&what_type).unwrap().food_name.to_string();
                if foods_vec[i].rating > prev_max_rating {
                    self.type_highest_rated_food.insert(what_type.to_string(), FoodDesc::new(foods_vec[i].food_name.to_string(), foods_vec[i].rating));
                    continue;
                }
                if foods_vec[i].rating == prev_max_rating && prev_food > foods_vec[i].food_name {
                    self.type_highest_rated_food.insert(what_type.to_string(), FoodDesc::new(foods_vec[i].food_name.to_string(), foods_vec[i].rating));
                    continue;
                }
            }
            // if new_rating > prev_max_rating {
            //     self.type_highest_rated_food.insert(what_type.to_string(), FoodDesc::new(food.to_string(), new_rating));
            // }
            // if idx >= 0 && new_rating == prev_max_rating && prev_food > foods_vec[idx as usize].food_name {
            //     self.type_highest_rated_food.insert(what_type.to_string(), FoodDesc::new(food.to_string(), new_rating));
            // }
        }
        // SECOND VERSION
    }
    
    fn highest_rated(&self, cuisine: String) -> String {
        let mut ans: String = format!("{} food type, not found", cuisine);
        // // FIRST VERSION
        // for i in 0..self.all_foods.len() {
        //     if cuisine == *self.all_foods[i].food_type {
        //         ans = self.all_foods[i].food_name.to_string();
        //         break;
        //     }
        // }
        // // println!("{:?}", self.all_foods);
        // // FIRST VERSION

        // SECOND VERSION
        // println!("type then foods : {:?}\nfood then type : {:?}\nhighest rated based on type : {:?}", self.type_foods, self.food_name_type, self.type_highest_rated_food);
        // println!("-----");
        // if let Some(foods_vec) = self.type_foods.get(&cuisine) {
        if let Some(food) = self.type_highest_rated_food.get(&cuisine) {
            ans = food.food_name.to_string();
        }
        // SECOND VERSION
        ans
    }
    fn sort_foods(foods_vec: &mut Vec<FoodDesc>) {
        foods_vec.sort_by(|a, b| {
            if a.rating != b.rating {return b.rating.cmp(&a.rating);}
            // Ordering::Equal
            if a.food_name <= b.food_name {return Ordering::Less}
            Ordering::Greater
        });
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */