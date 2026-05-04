pub fn egg_count(mut display_value: u32) -> usize {
    let mut count = 0;

    // Loop continues until all set bits have been cleared
    while display_value > 0 {
        // Increment the count for the bit we are about to clear
        count += 1;

        // This is Brian Kernighan's trick:
        // display_value & (display_value - 1) clears the 
        // rightmost (least significant) '1' bit in display_value.
        display_value &= (display_value - 1);
    }

    count
}
