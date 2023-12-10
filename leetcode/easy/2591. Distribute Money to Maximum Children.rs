impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < 8 {
            if money / children <= 0 {
                return -1;
            }
            if money / children != 4 {
                return 0;
            }
        }
        let mut count: i32 = -1;
        let mut curr_money: i32 = 0;
        let mut temp_child: i32 = 1;
        while temp_child <= children && Self::is_can(children, temp_child, money) {
            count = temp_child;
            temp_child += 1;
        }

        if count == -1 {
            if money / children < 8 && money / children > 0 {
                count = 0
            }
        }

        // TEST
        // println!("{}", Self::is_can(children, 1, money));
        // println!("{}", Self::is_can(children, 2, money));
        // println!("{}", Self::is_can(children, 3, money));
        // TEST
        count
    }
    pub fn is_can(children: i32, child_got_8: i32, money: i32) -> bool {
        let mut curr_money: i32 = child_got_8 * 8;
        if curr_money > money {
            return false;
        }
        let money_left: i32 = money - curr_money;
        let child_left: i32 = children - child_got_8;
        // println!("money_left:{}-children_left:{}", money_left, child_left);
        if money_left > 0 && child_left <= 0 {
            return false;
        }
        if money_left > 0 && child_left > 0 {
            if money_left / child_left <= 0 || (child_left == 1 && money_left / child_left == 4) {
                return false;
            }
        }
        if money_left <= 0 && child_left > 0 {
            return false;
        }

        true
    }
}
