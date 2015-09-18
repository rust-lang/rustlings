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

### Functions

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/functions.html)

- ["functions1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++call_me%28%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+This+main+function+is+calling+a+function+that+it+expects+to+exist%2C+but+the%0A%2F%2F+function+doesn%27t+exist.+It+expects+this+function+to+have+the+name+%60call_me%60.%0A%2F%2F+It+expects+this+function+to+not+take+any+arguments+and+not+return+a+value.%0A%2F%2F+Sounds+a+lot+like+%60main%60%2C+doesn%27t+it%3F%0A)
- ["functions2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++call_me%283%29%3B%0A%7D%0A%0Afn+call_me%28num%29+%7B%0A++++for+i+in+0..num+%7B%0A++++++++println%21%28%22Ring%21+Call+number+%7B%7D%22%2C+i+%2B+1%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Rust+requires+that+all+parts+of+a+function%27s+signature+have+type+annotations%2C%0A%2F%2F+but+%60call_me%60+is+missing+the+type+annotation+of+%60num%60.%0A)
- ["functions3.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++call_me%28%29%3B%0A%7D%0A%0Afn+call_me%28num%3A+i32%29+%7B%0A++++for+i+in+0..num+%7B%0A++++++++println%21%28%22Ring%21+Call+number+%7B%7D%22%2C+i+%2B+1%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+This+time%2C+the+function+*declaration*+is+okay%2C+but+there%27s+something+wrong%0A%2F%2F+with+the+place+where+we%27re+calling+the+function.%0A)
- ["functions4.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0A%2F%2F+This+store+is+having+a+sale+where+if+the+price+is+an+even+number%2C+you+get%0A%2F%2F+10+%28money+unit%29+off%2C+but+if+it%27s+an+odd+number%2C+it%27s+3+%28money+unit%29+less.%0A%0Afn+main%28%29+%7B%0A++++let+original_price+%3D+51%3B%0A++++println%21%28%22Your+sale+price+is+%7B%7D%22%2C+sale_price%28original_price%29%29%3B%0A%7D%0A%0Afn+sale_price%28price%3A+i32%29+-%3E+%7B%0A++++if+is_even%28price%29+%7B%0A++++++++price+-+10%0A++++%7D+else+%7B%0A++++++++price+-+3%0A++++%7D%0A%7D%0A%0Afn+is_even%28num%3A+i32%29+-%3E+bool+%7B%0A++++num+%25+2+%3D%3D+0%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+error+message+points+to+line+10+and+says+it+expects+a+type+after+the%0A%2F%2F+%60-%3E%60.+This+is+where+the+function%27s+return+type+should+be--+take+a+look+at%0A%2F%2F+the+%60is_even%60+function+for+an+example%21%0A)

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
