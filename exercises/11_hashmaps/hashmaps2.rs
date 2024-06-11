// hashmaps2.rs
//
// 我們正在收集不同的水果來烤一個美味的水果蛋糕。為此，我們有一個籃子，我們將用雜湊表來表示。鍵代表我們收集的每種水果的名稱，值代表我們收集了多少這種特定的水果。三種類型的水果 - 蘋果（4）、芒果（2）和荔枝（5）已經在籃子雜湊表中。您必須向籃子中添加水果，以便每種水果至少有一個，總數超過 11 - 我們有很多人要餵養。您不允許再插入這些水果中的任何一種！
//
// 讓我通過測試！
//
// 執行 `rustlings hint hashmaps2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: 如果籃子中還不存在這種水果，則插入新的水果。請注意，您不允許放入已經存在的任何類型的水果！
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 不要修改此函數！
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }
    
    #[test]
    fn all_fruit_types_in_basket() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        for amount in basket.values() {
            assert_ne!(amount, &0);
        }
    }
}
