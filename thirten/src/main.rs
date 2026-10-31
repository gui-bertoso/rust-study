//memory notes
//
// STACK and HEAP
//
// STACK -> store values in the order it gets then and removes the values in the opossite order -> more plates  = more time to remove a especific plate
// HEAP -> less organized, you request a certain amount of memory -> memory find a empty spot in the heap that is big enough, mark it as being in use and return a pointer
//         access heap data is generally slower than acessing data on the stack because you have to follow a pointer to get there
//         contemporary processors are faster if they jump around les in memory
//
// pointer -> address to memory
// the pointer to the heap is known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer
//
//
// pushing to the stack is fater than allocating on the heap beacause the allocator never has to search for a place to store new data -> that location is always at the top of the stack

fn main() { // is not valid here
    let s = String::from("hello"); // is valid from this point forward

    let s2 = &s; //no problem // add s2 to point for same memory of s, costs less memory ?

    let s3 = &s.clone(); // no problem // clone the variable, create a pointer to other memory with the same variable, costs more
    // do stuff with s

    //let s4 = &mut s; // BIG PROBLEM
    // &mut muttable reference can be updated with a reference (the variable work type a Class Object in python lmao)

    println!("{s}, {s2}, {s3}");

}// this scope is now over, and s in no longer valid
