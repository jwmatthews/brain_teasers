// Will run, until memory has been exhausted
fn main() {
    loop {
        let buffer = (0..100000).collect::<Vec<u32>>();
        // 'forget' is 'safe', but will cause memory to be leaked
        // 'forget' skips the destructor, so memory is still allocated and not freed
        std::mem::forget(buffer);

        // std::mem::drop is the typical call we would expect when a variable goes out of scope
        // 'drop' will deallocate the memory

        // 'forgot' could be useful in some advanced cases where memory is shared and you want one
        // portion of the program to literally forget about it, but not deallocate

        print!(".");
    }
}
