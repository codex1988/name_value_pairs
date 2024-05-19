> This is an exercise from PPP by Stroustrup (Part 1, Chapter 4, Ex. 19, 20, 21) re-written in Rust;
> The goal here is to be pretty close to production development despite relatively simple problem to solve;
---
19. Write a program where you first enter a set of name-and-value pairs, such
as Joe 17 and Barbara 22. For each pair, add the name to a vector called
names and the number to a vector called scores (in corresponding positions, so that if names[7]=="Joe" then scores[7]==17). Terminate input
with NoName 0. Check that each name is unique and terminate with an
error message if a name is entered twice. Write out all the (name,score)
pairs, one per line.
20. Modify the program from exercise 19 so that when you enter a name, the
program will output the corresponding score or name not found.
21. Modify the program from exercise 19 so that when you enter an integer,
the program will output all the names with that score or score not found.
---
### Problems and expetations
- Repetitive chunks of code in main.rs
- Error-prone input and lack of handling it properly (a user must behave consciously and be awared of the program's expectation; it's in general unacceptalbe)
- Current implementation can't provide a decent level of testing (e.g. build_pairs() function expect user's input from keyboard)
