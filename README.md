# moving_stats
Fixed size queue, which calculate **moving median** when you add new value.

sampling size must be odd number because this can be used for all Ord(and Clone) types.

computational complexity: O(n) for calculating median, O(nlog(n)) only for first calculation because it simply sorts and finds median.

# Example(part of test Code)
```rust
#[test]
fn it_works() {
    let mut queue:MovingMedian<f64> = MovingMedian::new(3).unwrap();
    queue.add(3.0);
    queue.add(7.0);
    assert_eq!(queue.is_full(), false);
    assert_eq!(queue.median(), None);
    queue.add(6.0);
    assert_eq!(queue.is_full(), true);
    assert_eq!(queue.median(), Some(6.0));
    queue.add(9.0);
    assert_eq!(queue.median(), Some(7.0));
    queue.add(1.0);
    assert_eq!(queue.median(), Some(6.0));
    assert_eq!(queue.get(0), Ok(6.0));
    assert_eq!(queue.get(2), Ok(1.0));
    assert_eq!(queue.get(3).is_err(), true);
    assert_eq!(queue.get(-3).is_err(), true);
    assert_eq!(queue.get(-1), Ok(1.0));
    assert_eq!(queue.get(-2), Ok(9.0));
    assert_eq!(queue.add(f64::NAN).is_err(), true);
    assert_eq!(queue.add(5.0).is_err(), false);

    let mut str_queue:MovingMedian<String> = MovingMedian::new(3).unwrap();
    str_queue.add(String::from("abc"));
    str_queue.add(String::from("def"));
    str_queue.add(String::from("adc"));
    assert_eq!(str_queue.is_full(), true);
    assert_eq!(str_queue.median(), Some(String::from("adc")));
    str_queue.add(String::from("bbe"));
    assert_eq!(str_queue.median(), Some(String::from("bbe")));
}
```
