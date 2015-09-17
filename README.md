# rustlings

Small exercises to get you used to reading and writing Rust code. Includes practice reading and responding to
compiler messages!

This repo is very much the smallest thing that could possibly work :)

## To do these exercises

Thanks to [btbytes'](https://twitter.com/btbytes) [prlinks](https://github.com/btbytes/prlink), you can now click on the links below to load the exercises in the rust playpen!

There are infinite correct answers-- the exercises are sometimes left very open-ended. Scroll down in the playpen to find comments that have hints.

If you need more help or would like to compare solutions, you can ask in [#rust on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust), the [user forum](https://users.rust-lang.org/), or [the subreddit](https://reddit.com/r/rust). If an exercise could be improved in any way, please [create an issue](https://github.com/carols10cents/rustlings/issues/new) or submit a pull request!

### Variable bindings

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/variable-bindings.html)

- ["variables1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++x+%3D+5%3B%0A++++println%21%28%22x+has+the+value+%7B%7D%22%2C+x%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Hint%3A+The+declaration+on+line+4+is+missing+a+keyword+that+is+needed+in+Rust%0A%2F%2F+to+create+a+new+variable+binding.%0A)
- ["variables2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+x%3B%0A++++if+x+%3D%3D+10+%7B%0A++++++++println%21%28%22Ten%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Not+ten%21%22%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+compiler+message+is+saying+that+Rust+cannot+infer+the+type+that+the%0A%2F%2F+variable+binding+%60x%60+has+with+what+is+given+here.%0A%2F%2F+What+happens+if+you+annotate+line+4+with+a+type+annotation%3F%0A%2F%2F+What+if+you+give+x+a+value%3F%0A%2F%2F+What+if+you+do+both%3F%0A%2F%2F+What+type+should+x+be%2C+anyway%3F%0A%2F%2F+What+if+x+is+the+same+type+as+10%3F+What+if+it%27s+a+different+type%3F%0A)
- ["variables3.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+x+%3D+3%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A++++x+%3D+5%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+In+Rust%2C+variable+bindings+are+immutable+by+default.+But+here+we%27re+trying%0A%2F%2F+to+reassign+a+different+value+to+x%21+There%27s+a+keyword+we+can+use+to+make%0A%2F%2F+a+variable+binding+mutable+instead.%0A)
- ["variables4.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+x%3A+i32%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Oops%21+In+this+exercise%2C+we+have+a+variable+binding+that+we%27ve+created+on%0A%2F%2F+line+4%2C+and+we%27re+trying+to+use+it+on+line+5%2C+but+we+haven%27t+given+it+a%0A%2F%2F+value.+We+can%27t+print+out+something+that+isn%27t+there%3B+try+giving+x+a+value%21%0A%2F%2F+This+is+an+error+that+can+cause+bugs+that%27s+very+easy+to+make+in+any%0A%2F%2F+programming+language+--+thankfully+the+Rust+compiler+has+caught+this+for+us%21%0A)

### Uncategorized

A few exercises based on things I've encountered or had trouble with getting used to.

- ["ex1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Afn+main%28%29+%7B%0A++++println%21%28%29%3B%0A%7D%0A)
- ["ex2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Afn+something%28%29+-%3E+String+%7B%0A++++%22hi%21%22%0A%7D%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22%7B%7D%22%2C+something%28%29%29%3B%0A%7D%0A)
- ["ex3.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Astruct+Foo+%7B%0A++++capacity%3A+i32%2C%0A%7D%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22%7B%3A%3F%7D%22%2C+Foo+%7B+capacity%3A+3+%7D%29%3B%0A%7D%0A)
- ["ex4.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Afn+something%28%29+-%3E+Result%3Ci32%2C+std%3A%3Anum%3A%3AParseIntError%3E+%7B%0A++++let+x%3Ai32+%3D+%223%22.parse%28%29%3B%0A++++Ok%28x+*+4%29%0A%7D%0A%0Afn+main%28%29+%7B%0A++++match+something%28%29+%7B%0A++++++++Ok%28..%29+%3D%3E+println%21%28%22You+win%21%22%29%2C%0A++++++++Err%28e%29+%3D%3E+println%21%28%22Oh+no+something+went+wrong%3A+%7B%7D%22%2C+e%29%2C%0A++++%7D%0A%7D%0A)
- ["ex5.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Aenum+Reaction%3C%27a%3E+%7B%0A++++Sad%28%26%27a+str%29%2C%0A++++Happy%28%26%27a+str%29%2C%0A%7D%0A%0Afn+express%28sentiment%3A+Reaction%29+%7B%0A++++match+sentiment+%7B%0A++++++++Reaction%3A%3ASad%28s%29+%3D%3E+println%21%28%22%3A%28+%7B%7D%22%2C+s%29%2C%0A++++++++Reaction%3A%3AHappy%28s%29+%3D%3E+println%21%28%22%3A%29+%7B%7D%22%2C+s%29%2C%0A++++%7D%0A%7D%0A%0Afn+main+%28%29+%7B%0A++++let+x+%3D+Reaction%3A%3AHappy%28%22It%27s+a+great+day+for+Rust%21%22%29%3B%0A++++express%28x%29%3B%0A++++express%28x%29%3B%0A++++let+y+%3D+Reaction%3A%3ASad%28%22This+code+doesn%27t+compile+yet.%22%29%3B%0A++++express%28y%29%3B%0A%7D%0A)

## To help with this repo/TODO list

* File issues for problems or suggestions!
* Contribute more exercises! Anything that took you time to get used to, or that you had trouble with, or that deserves practice would be a good exercise!
* Organize the exercises (by topic? progression of difficulty?)
* Add some exercises that are "make the test pass" instead of "make this compile"
* Write some hints or sample answers
* Figure out a nice way to link the code directly into playpen
