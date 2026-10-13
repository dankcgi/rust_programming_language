mod creating;
mod updating;
mod iterating;

fn main() {
    creating::initialise();
    creating::hello();

    updating::appending();
    updating::concatenating();

    /*
     Because a string is a collection of bytes, they cannot be indexed
     like in other languages. You would be indexing into bytes, not the
     grapheme cluster (i.e., letter). The below would not compile:

     let hello = "Здравствуйте";
     let answer = &hello[0]; // this is 208, they byte, not 'З'

     For similar reasons, slicing a string with a range is not advisable.

     let slice = &hello[0..4] 
     each char is 2 bytes, so this would be "Зд", but:

     let slice = &hello[0..1]

     would cause a panic because this is only 1 byte.
    */

    iterating::by_characters();
    iterating::by_bytes();
}
