fn main() {
    // This is an example of a line comment.
    // there are two slashes at the beginning of the line.
    /*
    * This is another type of comment, a block comment . In general,
    *  line comments are the recommended comment style. But block comment
    * are extremely useful for temporarily disabling chunks of code
    */
    // like this part of the code
    // that i'm writing like now
    let x = 5 + /* 90 + */ 5;
    println!("is  `x` 10 or 100? x = {}", x);
}