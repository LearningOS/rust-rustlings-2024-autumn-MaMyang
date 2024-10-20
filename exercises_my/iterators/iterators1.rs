fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // Step 1: 创建一个迭代器

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));      // Step 2: 验证第一个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));// Step 3: 验证第二个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));     // Step 4: 验证第三个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));       // Step 5: 验证第四个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));   // Step 6: 验证第五个元素
    assert_eq!(my_iterable_fav_fruits.next(), None);                 // Step 7: 验证已经没有更多元素
}
