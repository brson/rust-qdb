# Rust Quotes from Mozilla QDB

Total quotes: 143

---

## Quote #5718

```
<graydon> ow. my friend just broke my funny bone.
<graydon> "Have you heard the funny anagram for Banach-Tarski?"
<tjc> nope
<graydon> "Banach-Tarski Banach-Tarski"
<tjc> nyuk nyuk
<graydon> a joke that cannot possibly end well
```

- **Rating:** 12 (12 votes)
- **Score:** 13.00
- **Submitted:** 2011-04-12 17:21:23
- **Approved:** true
- **Tags:** #rust, math

---

## Quote #5813

```
<lkuper> So, which rust-ers are in Toronto this week?
<eholk> hey, with all the normal rusters out of town, it's our chance to replace the syntax with S-Expressions!
```

- **Rating:** 11 (11 votes)
- **Score:** 12.00
- **Submitted:** 2011-06-20 17:38:09
- **Approved:** true

---

## Quote #6040

```
<sully> * The purpose of assembly is to get back into C.
<sully> * The purpose of C is to get back into ML or Haskell. (Or Rust, soon!)
<sully> I mean, the purpose of C is to build your kernel and your runtime and your garbage collector for your safe language
<elly> as a C programmer by trade, I object :)
<elly> the purpose of C is to write large, insecure, buggy applications!
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2011-10-27 02:05:33
- **Approved:** true

---

## Quote #6156

```
<nmatsakis> are rustbots broken in some way?  webpage looks...odd
<marijn> win1 is wedged, mac1 is missing, and i had to restart earlier to get any response at all. in other words, nothing out of the usual
```

- **Rating:** 3 (5 votes)
- **Score:** 2.50
- **Submitted:** 2012-01-03 19:50:48
- **Approved:** true
- **Tags:** #rust, rustbot

---

## Quote #6221

```
<graydon> you know I left the math program in part because I was perpetually annoyed at math's unwillingness to be syntactically unambiguous, right?
<tjc> you know you're a nerd when math isn't precise enough for you
```

- **Rating:** 20 (20 votes)
- **Score:** 21.00
- **Submitted:** 2012-01-26 21:23:00
- **Approved:** true
- **Tags:** #rust

---

## Quote #6293

```
<erickt> is it just me, or did compiling rust just become a lot slower?
<brson> compiling rustc became a lot slower because it grew my almost 9kloc this week
<erickt> Sad. It now seems to take a minute+ to link a stage on my laptop
<pcwalton> hopefully CCI will help
<erickt> what's that?
<pcwalton> cross-crate inlining
<pcwalton> the reason that rust grew 9kloc
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2012-02-29 02:39:47
- **Approved:** true

---

## Quote #6409

```
<graydon> dherman: list::mk, list::hd, list::tl :)
<dherman> sr, mks sns
<graydon> ok: make, head and tail if you like Mr. Vowelpants
<graydon> this is a solved problem for lists
<dherman> I kid!
* lkuper is totally going to start calling dherman "Mr. Vowelpants"
<dherman> it wouldn't be my first -pants title
<lkuper> oh, pardon: "Dr. Vowelpants"
```

- **Rating:** 10 (10 votes)
- **Score:** 11.00
- **Submitted:** 2012-04-28 07:17:07
- **Approved:** true
- **Tags:** #rust, dherman, graydon, lkuper

---

## Quote #6494

```
* lkuper wonders what graydon's law in fact is
<graydon> "nothing every works right"
<graydon> *ever
<Wensley|bstrie> haha
<Wensley|bstrie> a self-affirming law
<graydon> quite
```

- **Rating:** 15 (15 votes)
- **Score:** 16.00
- **Submitted:** 2012-07-10 18:25:34
- **Approved:** true
- **Tags:** #rust, bstrie, graydon, lkuper

---

## Quote #6510

```
<sully> ok, argh
<sully> do we have a standard terminology for the different heaps
<sully> because we refer to both ~ and @ as "shared" in different places
<eholk> i like calling ~ the exchange heap
<bstrie> one's the squiggle heap and one's the curly heap
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2012-07-17 00:38:12
- **Approved:** true

**Notes:** http://dl.rust-lang.org/doc/tutorial.html#boxes-and-pointers

---

## Quote #6513

```
<jld> (Meanwhile, at work, I am dealing with problems which I am pretty sure the hypothetical widespread adoption of Rust would not help.  Sigh.)
<Jesse> there are problems in the world that widespread adoption of Rust would not solve??
<Jesse> i thought it was going to solve everything from dangling pointers to discrimination against gays to that awkward pause when two people try to talk at the same time
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2012-07-19 17:58:13
- **Approved:** true
- **Tags:** #rust

---

## Quote #6523

```
<pfox__> how do you type 'bikeshed' in dvorak? ;)
```

- **Rating:** -1 (3 votes)
- **Score:** 0.67
- **Submitted:** 2012-08-06 23:07:42
- **Approved:** true
- **Tags:** #rust

---

## Quote #6563

```
<brson> oh, if only i had valground 2 hours ago
<tjc> is "valground" the official past tense of "valgrind"?
<brson> yes, officially
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2012-09-14 06:06:35
- **Approved:** true
- **Tags:** #servo

---

## Quote #6573

```
<bstrie> brson: do `do` statements have return values?
<nejucomo> bstrie: do is syntactic sugar for an expression, and every expression evaluates to a value.
<nejucomo> So yes, although calling them "return values" may be misleading, because there's a function called "return".
...
<nejucomo> Doh!  I thought this was the haskell channel!  Please completely disregard what I said.
```

- **Rating:** 12 (12 votes)
- **Score:** 13.00
- **Submitted:** 2012-09-19 18:53:23
- **Approved:** true
- **Tags:** #rust

---

## Quote #6581

```
<bstrie> "We're impressed by your “rust” tag answers on Stack Overflow. We appreciate your contributions, and would like to invite you to create a professional profile on Stack Overflow Careers 2.0."
<bstrie> finally, that rust-related job I've always dreamt of is within my grasp
```

- **Rating:** 6 (12 votes)
- **Score:** 2.50
- **Submitted:** 2012-09-27 20:49:46
- **Approved:** true

---

## Quote #6618

```
<mgoodwin> All languages suck in their own special ways
<mgoodwin> I can write bad code in every language I know
<dveditz> Rust!
<mgoodwin> Rust is on my list. I need to learn to write bad code in Rust
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2012-10-20 01:56:12
- **Approved:** true

---

## Quote #6623

```
<pcwalton> I think probably a constraint solver could solve it
<pcwalton> our name resolution is kind of like sudoku
<nmatsakis-at-FOOL> there is some work here
<nmatsakis-at-FOOL> on reformulating constraint problems
<nmatsakis-at-FOOL> as games
<nmatsakis-at-FOOL> with simple interfaces like angry birds
<nmatsakis-at-FOOL> and then have people on facebook play it
<nmatsakis-at-FOOL> and hence type your programs for free
<nmatsakis-at-FOOL> (not joking)
<nmatsakis-at-FOOL> it's brilliant :)
<pcwalton> i imagine that would make your build times rather slow
<nmatsakis-at-FOOL> have you seen the runtimes of some of these constraint solvers?
```

- **Rating:** 10 (10 votes)
- **Score:** 11.00
- **Submitted:** 2012-10-23 14:05:13
- **Approved:** true
- **Tags:** angry-birds, constraint-solver, rust, sudoku

---

## Quote #6624

```
<bstrie> I find it interesting that the rust repo contains the human genome
<bstrie> should the unthinkable happen to mankind, any future civilization that manages to clone the rust repo will be able to revive our species
<lkuper> gosh, if I'd known that the rust repo might outlive humanity, I might have not left all those FIXMEs lying around
```

- **Rating:** 9 (11 votes)
- **Score:** 5.50
- **Submitted:** 2012-10-23 18:54:12
- **Approved:** true
- **Tags:** bstrie, lkuper, rust

---

## Quote #6625

```
<burg> I love naming upstream as 'ups'
<burg> git pushups!
```

- **Rating:** 1 (3 votes)
- **Score:** 1.50
- **Submitted:** 2012-10-23 21:42:42
- **Approved:** true
- **Tags:** #servo

---

## Quote #6636

```
<kspaans> I'm trying to understand the rust build system
<bstrie> kspaans: you mean the makefile? I wouldn't advise it
<bstrie> kspaans: I suppose you haven't heard the tale of Jacobin, the lost Sixth rust developer who was never heard from again after attempting to understand the makefile
```

- **Rating:** 12 (12 votes)
- **Score:** 13.00
- **Submitted:** 2012-10-31 18:08:22
- **Approved:** true
- **Tags:** #rust, bstrie, kspaans

---

## Quote #6650

```
<pcwalton> basically you have something like ~ClosedFile, and your open() function takes a ~ClosedFile by move and returns an ~OpenFile
<pcwalton> this pattern is called "session types"
<burg> pcwalton: I usually associate the term session types with the pi calculus
<lkuper> I associate "session types" with phil wadler taking off his shirt
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2012-11-07 19:19:37
- **Approved:** true
- **Tags:** #rust

---

## Quote #6652

```
<pcwalton> I copy and pasted the integer literal inference code to make floating point literal code and then did a search and replace int -> float
<pcwalton> and "intersection" changed to "floatersection"
<pcwalton>         // Otherwise, take the floatersection of the two sets of possible types.
<pcwalton>         let floatersection = floatersection(a_pt, b_pt);
<pcwalton>         if *floatersection == INT_TY_SET_EMPTY {
<pcwalton>             return Err(ty::terr_no_floategral_type);
<pcwalton>         }
```

- **Rating:** 19 (19 votes)
- **Score:** 20.00
- **Submitted:** 2012-11-08 19:44:48
- **Approved:** true
- **Tags:** #rust, floatersection, paste

---

## Quote #6679

```
<bstrie> #[kick_ass] macro_rules! component_each(...
<Dzmitry> bstrie: you read my mind
<bjz> bstrie: you need a hyphen in that 'kick_ass' - won't compile
<bjz> #[kick-ass]
<bjz> as per Dzmitry's specifications
<bstrie> aha
<bstrie> think we might need to modify the lexer though
<bstrie> or just really hope that nobody overloads subtraction on verbs and body parts
<bjz> hee
<bjz> what is a kick without an ass?
<bstrie> bjz: well you could conceivably kick many things. for example if you wished to teach rust
         programming to children, you could use #[kick-fanny] instead
<bjz> now, now
<bjz> I'll make you scrub the logs
 bstrie hopes that "fanny" is not secretly a vulgar word in british english
<bjz> HAH
<bjz> poor bstrie
 [momentary pause]
<bstrie> oh god
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2012-11-26 16:30:43
- **Approved:** true
- **Tags:** #rust, english, fanny

**Notes:** http://en.wikipedia.org/wiki/Fanny#In_slang

---

## Quote #6683

```
<graydon> I guess I added that code!
<graydon> yay revision control telling you to blame yourself
<tjc> I blame myself anyway
<tjc> that's one thing I don't need software for ;-)
```

- **Rating:** 7 (7 votes)
- **Score:** 8.00
- **Submitted:** 2012-11-27 22:55:33
- **Approved:** true
- **Tags:** #rust

---

## Quote #6694

```
<rntz> "match (match ...) { ... }" aha, finally my favorite SML idiom comes to rust
<graydon> we'll be linear ML yet if it kills us
<graydon> (with macros. in BCPL clothing.)
<graydon> (how did this happen?)
<rntz> well...
<rntz> it's linear ML because: you hired a bunch of PL geeks to help make a language, what did you expect?
<rntz> it has macros because: you hired a bunch of PL geeks to help make a language, what did you expect?
<rntz> it looks like BCPL because: you need to convert the C++ programmers, apparently
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2012-12-05 01:42:36
- **Approved:** true
- **Tags:** #rust, c++, ml

---

## Quote #6767

```
<bstrie> well, if you write even a basic application in rust you'd almost certainly be able to claim on your resume that you're one of the world's leading rust experts
<bstrie> definitely in the top 100
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2013-01-28 17:09:55
- **Approved:** true
- **Tags:** bstrie, rust

---

## Quote #6813

```
<benh> rust is the best language, my code is broken and someone else on the other side of the world is already working on making it work before I even know it
```

- **Rating:** 3 (3 votes)
- **Score:** 4.00
- **Submitted:** 2013-02-26 18:45:56
- **Approved:** true
- **Tags:** rust

---

## Quote #6815

```
<bstrie> also apparently github is having a meetup tomorrow in pittsburgh
<bstrie> I think this means that RUSTCON FEBRUARY '012 must happen coterminously
<lkuper> bstrie: '012?
<bstrie> lkuper: yes, pronounced "oh-twelve"
<bstrie> we've learned from the mistakes of y2k
<lkuper> bstrie: ...okay, carry on
<benh> I can't wait for rustcon '013 next year
```

- **Rating:** 7 (7 votes)
- **Score:** 8.00
- **Submitted:** 2013-02-27 20:45:37
- **Approved:** true
- **Tags:** #rust, benh, bstrie, lkuper

**Notes:** Conversation took place in February 2013.

---

## Quote #6816

```
<pcwalton> Luqman: I think we should put a limit on the levenshtein distance ;)
<pcwalton> /Users/pwalton/Source/rust/master/src/libcore/task/local_data.rs:85:17: 85:34 error: unresolved name: `rt::rust_get_task`. Did you mean: `modify_fn`?
```

- **Rating:** 13 (13 votes)
- **Score:** 14.00
- **Submitted:** 2013-02-28 13:42:24
- **Approved:** true
- **Tags:** #rust

**Notes:** http://en.wikipedia.org/wiki/Levenshtein_distance

---

## Quote #6818

```
<seth> so what sucks less
<seth> propagating const to a gazillion places (the right solution)
<seth> or using const_cast in one place (the pragmatic solution)
<RyanVM> besides being tedious, what's wrong with the former?
<seth> RyanVM: it creates a huge patch that touches tons of stuff, basically
<Jesse> seth: or switch to Rust, where immutable is the default. (may involve changing slightly more code)
<seth> Jesse: that'd be my preferred solution, trust me
<mjrosenb> seth: do the right thing!
<mjrosenb> seth: unless it changes code that i'll need to rebase on top of :-p
```

- **Rating:** 0 (4 votes)
- **Score:** 1.00
- **Submitted:** 2013-03-01 23:13:19
- **Approved:** true

---

## Quote #6819

```
-!- sanxiyn has joined #rust
 * sanxiyn is at home
<sanxiyn> At work, I couldn't join here because security team started to block chat.mibbit.com because it is "chat site"
<sanxiyn> I filed a complaint
<sanxiyn> Last year, security team blocked GitHub because it is "file sharing site"
```

- **Rating:** 15 (15 votes)
- **Score:** 16.00
- **Submitted:** 2013-03-02 01:59:29
- **Approved:** true

---

## Quote #6826

```
<Luqman> and with that, things like const FOO: uint = 2; let v: [int * FOO*3] = [0,.. FOO+2+2]; will work
<bstrie> Luqman: at least until someone redefines FOO to anything but 2 :P
<bstrie> but hey, it's a great way to ensure that constants remain constant...
* pcwalton waits until someone abuses this to add static_assert to rust
<pcwalton> yeah, you can write static_assert now
<pcwalton> if you want to static_assert c1 == c2
<pcwalton> const A: [int * c1] = [0, ..c2]
<pcwalton> please don't do this
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2013-03-06 02:14:06
- **Approved:** true
- **Tags:** #rust, bstrie, luqman, pcwalton

---

## Quote #6830

```
<bstrie> in rust it's turtles all the way down, until you get to a @. at that point the turtles all turn into pelicans and have a techno dance party in a warehouse.
```

- **Rating:** 11 (11 votes)
- **Score:** 12.00
- **Submitted:** 2013-03-07 21:34:50
- **Approved:** true
- **Tags:** #rust

**Notes:** bstrie explaining ownership semantics in rust

---

## Quote #6840

```
<brson> ugh. I'm not going to try to fix the bsd bots. I don't understand how this works
<pcwalton> :( we need graydon
<strcat> graydon.clone()
<benh> strcat: I tried saying let graydon2 = graydon; let graydon3 = graydon; etc a bunch but it turns out that only moved him. He wasn't actually more productive afterwards, just really mad.
```

- **Rating:** 15 (15 votes)
- **Score:** 16.00
- **Submitted:** 2013-03-14 19:33:43
- **Approved:** true
- **Tags:** #rust, benh, brson, graydon, pcwalton, strcat

**Notes:** who could have expected that graydon would be so unique

---

## Quote #6841

```
<jamil> anyway, what is the meaning of this error: illegal borrow: creating immutable alias to mutable field
<jamil> why is it illegal ?
<bstrie> jamil: are you on 0.5?
<jamil> bstrie: yep
<bstrie> jamil: the meaning of that error is boundless despair. it urges you to upgrade to the latest git master
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2013-03-14 20:53:17
- **Approved:** true
- **Tags:** #rust

**Notes:** The never ending joy of having to many language changes since last release.

---

## Quote #6847

```
<graydon> I think it'll be a little sad if declaring a constant requires you to write a word you are morally repulsed by
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2013-03-18 23:01:30
- **Approved:** true
- **Tags:** #rust

---

## Quote #6848

```
<benh> I used to assume that stack unwinding is magic
<benh> following along the discussion about return-based unwinding, I now know that it's not actually magic, but it's handled by dwarves, which seems close enough.
```

- **Rating:** 19 (19 votes)
- **Score:** 20.00
- **Submitted:** 2013-03-19 21:09:13
- **Approved:** true
- **Tags:** #rust, magic

---

## Quote #6853

```
<bstrie> oh god damn it
<bstrie> I had to examine the responses of the failed http requests to determine that blogger suddenly won't allow me to load any page because I'm socks'ing through my linode right now
<bstrie> it's supposed to show a captcha, but it can't because of the aforementioned stupid goddamn loading screen
<bstrie> my rage is infinite
<bstrie> I get to watch the error count in firebug increment in real-time as this page reloads the useless, undisplayed captcha screen every 0.5 seconds
<kimundi> Use that infinite power for good!
<lkuper> bstrie: have a baby ferret. http://cuteoverload.com/2011/08/23/baby-ferret-dreams-of-changing-the-world/
<bstrie> lkuper: I will raise it to be an attack ferret and unleash it upon the architects of this website
<bstrie> I will instill the essence of vengeance in its heart
<bstrie> kimundi: indeed, if only you could build a dyson sphere around me right now to harness all this fury that I am emanating
```

- **Rating:** 1 (1 votes)
- **Score:** 2.00
- **Submitted:** 2013-03-22 14:43:20
- **Approved:** true
- **Tags:** #rust, bstrie, kimundi, lkuper

---

## Quote #6863

```
<ghrust> rust/auto f43e6af Matthijs Hofstra: Removed libcore/mutable.rs, Mut<T> is now dead.
<bstrie> Mut<T> is dead, long live @mut
<thiez> :D
<thiez> it would have been dead 4 days ago if not for a lan-party...
<kimundi> xD
<kimundi> priorities, I see
<bstrie> rust would have been at 1.0 in 2009 if not for lan parties
<bstrie> what do you think they actually do at mozilla all day?
<tjc> bstrie: this pretty much represents every day at Mozilla: https://twitter.com/eassumption/status/185082256535203841/photo/1
```

- **Rating:** 3 (3 votes)
- **Score:** 4.00
- **Submitted:** 2013-04-02 20:46:24
- **Approved:** true
- **Tags:** #rust, bieber, tjc

---

## Quote #6869

```
<benh> this is incidentally when i curse stackoverflow for ruining google searches for info on stack overflows
```

- **Rating:** 24 (24 votes)
- **Score:** 25.00
- **Submitted:** 2013-04-04 17:33:43
- **Approved:** true
- **Tags:** #rust

---

## Quote #6872

```
<Jesse_> i make sure to use at least one obscure word every time i help someone on IRC, so people don't get in the habit of thinking they can avoid using google by asking me
```

- **Rating:** 30 (40 votes)
- **Score:** 6.00
- **Submitted:** 2013-04-08 22:32:28
- **Approved:** true
- **Tags:** #rust, google, lmgtfy, rust

---

## Quote #6883

```
<_Vi> Are there more Haskellers or Lispers in the Rust dev team?
<jclements> _Vi: I'm not sure you'd find anyone on the Rust dev team that would actually label themselves either a "Haskeller" or a "Lisper", but I can only speak for myself.
<strcat> so what's our word for a rust user?
<steven_is_false> beta-tester
```

- **Rating:** 6 (8 votes)
- **Score:** 4.00
- **Submitted:** 2013-04-17 16:25:58
- **Approved:** true
- **Tags:** rust

---

## Quote #6895

```
* nmatsakis wonders why he is bothering to think about this. It seems like the def'n of "ship has sailed" in some sense.
<jclements> this dead cat is a ship that has sailed. We're sailing in a dead cat.
<graydon> jclements: but crucially: _has it left the barn yet_?
<graydon> if you've sailed the dead cat (or dead horse) out of the barn, I think maybe it's time to put it to .. bed?
<graydon> mixed metaphors ahoy
<jclements> I, for one, have no plans to share *my* bed with a sailing skinned cat/horse. I'm sure my wife would agree.
```

- **Rating:** 3 (7 votes)
- **Score:** 2.00
- **Submitted:** 2013-04-25 20:45:46
- **Approved:** true
- **Tags:** #rust

---

## Quote #6899

```
<dbaupp> 50 comments on 4819. yay, bikeshedding! :P
<just_bstrie> dbaupp: I don't think that any discussion involving the word "quaternion" can be classified  as bikeshedding
<jensnockert> just_bstrie: I think all discussions that involve the word "Quaternion" can be classified as bikeshedding.
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2013-04-29 13:43:39
- **Approved:** true
- **Tags:** #rust, quaternion

---

## Quote #6900

```
<dymk> so I guess that fail!() won't also come with a stack trace any time soon then, eh?
<erickt1> dymk: no, but you can use gdb and break on upcall_fail
<erickt1> then do bt to get a stack trace
<dymk> ah, gdb
<dymk> my liver can take the continued abuse, alright
<dymk> ;)
```

- **Rating:** 3 (3 votes)
- **Score:** 4.00
- **Submitted:** 2013-04-30 05:39:21
- **Approved:** true
- **Tags:** #rust

---

## Quote #6922

```
<doomlord> lifetime refers to borrowed pointers ?
<bstrie> doomlord: a lifetime is essentially the potential scope of any given borrow
<jack> that almost sounds like sage wisdom
```

- **Rating:** 23 (23 votes)
- **Score:** 24.00
- **Submitted:** 2013-05-13 16:33:58
- **Approved:** true
- **Tags:** #rust

---

## Quote #6933

```
<gareth0> rust follows the yoda school of error handling: do, or do not, there is no try
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2013-05-21 20:21:13
- **Approved:** true
- **Tags:** #rust, exceptions

---

## Quote #6937

```
<jdm> hooray, I fixed the leak!
<jdm> no we only leak a rate of 10mb/s
<jdm> *now
<jdm> what would be mortifying in any other project is cause for success
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2013-05-22 16:55:41
- **Approved:** true
- **Tags:** #servo, jdm

---

## Quote #6952

```
<bblum> compiling the compiler takes sooooo long
<Eridius> bblum: that's because you need to compile the compiler that compiles the compiler before you can compile the compiler
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2013-05-29 22:04:50
- **Approved:** true
- **Tags:** rust

**Notes:** The downside of self-hosted compilers.

---

## Quote #6961

```
<brson> my laptop can no longer do 4 rust builds in parallel
<brson> 8GB is not enough ram
<Jesse> why are you trying to do 4 rust builds in parallel?
<brson> i've got nothing better to do while waiting for builds than to launch more builds
```

- **Rating:** 12 (12 votes)
- **Score:** 13.00
- **Submitted:** 2013-06-05 22:09:52
- **Approved:** true
- **Tags:** swordfighting

---

## Quote #6965

```
<moonchrome> I'm using macros to work around missing associated items, copy-pasting to work around macros not expanding multiple times in another macro, implementing ToStr manually because #[deriving(ToStr)] doesn't work in macro, converting values to [] and using .to_str on it because fmt! can't use a macro constructed string
<moonchrome> the workarounds have workarounds xD
<kimundi> In other words, business as usual in rust land. :D
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2013-06-06 19:36:16
- **Approved:** true
- **Tags:** #rust

---

## Quote #6971

```
<bstrie> is "complete" alias analysis undecidable?
<bblum> yep
<bstrie> ok :)
<tjc> Rice's Theorem applies
<tjc> every interesting static analysis is undecidable :-)
<bstrie> haha
<tjc> that's more or less what it really says
<tjc> also known as the full employment theorem for compiler writers
```

- **Rating:** 13 (13 votes)
- **Score:** 14.00
- **Submitted:** 2013-06-11 21:49:52
- **Approved:** true
- **Tags:** #rust

---

## Quote #6993

```
<dbaupp> Does anyone know why vec::push_slow is #[inline(never)]?
<bstrie> dbaupp: because then it wouldn't be slow
```

- **Rating:** 27 (27 votes)
- **Score:** 28.00
- **Submitted:** 2013-06-27 13:04:10
- **Approved:** true
- **Tags:** #rust

---

## Quote #7004

```
<pcwalton> macros are way better at generating code than trans is
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2013-07-02 22:54:07
- **Approved:** true
- **Tags:** #rust

---

## Quote #7016

```
* tjc once blew people's minds by using quicksort in real life when sorting log sheets as part of volunteering at a radio station
<cmr> http://rosettacode.org/wiki/Sorting_algorithms/Sleep_sort#Rust
<cmr> Oi, look at that!
<tjc> they were amazed that I could sort a pile of papers in 15 minutes instead of 2 hours
<ecr> tjc: that's brilliant
<kmc> most real-world sorting you can do radix sort
<kmc> make piles based on first digit/letter etc.
<sigma> libraries use a really efficient sorting system
<tjc> ecr: the guy who was supervising me called someone else over and said… "He said he… used a… sorting algorithm"
<kmc> I'm told this is how elementary school teachers usually sort things
<kmc> so they know more about sorting algorithms than 50% of college CS grads ;)
```

- **Rating:** 18 (18 votes)
- **Score:** 19.00
- **Submitted:** 2013-07-11 21:18:49
- **Approved:** true
- **Tags:** tjc, sorting, #rust

---

## Quote #7020

```
<engla> sully: maybe strcat knows
<cmr> strcat: yeah, please don't get hit by a bus
<MaikKlein1> ??
<cmr> MaikKlein1: http://en.wikipedia.org/wiki/Bus_factor
<engla> I guess you are fine with us others getting hit
<thpickert> engla: Only if you *have to* get hit, in order to save strcat.
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2013-07-17 19:19:17
- **Approved:** true
- **Tags:** bus, factor, rust

---

## Quote #7027

```
<sigma> yeh, they don't mention the word "alpha" anywhere in that article
<cmr> pre-alpha!
<Ms2ger> What's pre-alpha? Omega?
<dbaupp> Ms2ger: ΰ
<dbaupp> "GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS" apparently
```

- **Rating:** 5 (11 votes)
- **Score:** 2.25
- **Submitted:** 2013-07-24 14:16:17
- **Approved:** true
- **Tags:** #rust

---

## Quote #7036

```
<graydon> I just feel sorry for our pointer types
<graydon> someone tries to kill one of them every day
<graydon> :(
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2013-07-31 08:45:07
- **Approved:** true
- **Tags:** graydon, pointer, rust

**Notes:** During a discussion about unsafe pointers.

---

## Quote #7043

```
* cmr bed
* strcat table
<strcat> wait what?
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2013-08-03 09:03:59
- **Approved:** true
- **Tags:** #rust, cmr, strcat

---

## Quote #7044

```
<aatch> Also, this is groundwork for improving the inline asm.
<Luqman> aatch: speaking of which, found a rather amusing use for asm! today
<aatch> Luqman, oh?
<Luqman> aatch: using it to insert nop's so we could hexedit the binary without messing up any offsets :P
<aatch> Luqman, oh wow.
<aatch> That is mad
```

- **Rating:** 13 (13 votes)
- **Score:** 14.00
- **Submitted:** 2013-08-03 12:32:25
- **Approved:** true
- **Tags:** #rust

---

## Quote #7054

```
toddaaro> rusti: 2^8
-rusti:#rust- 10
<toddaaro> what
<toddaaro> oh god
<toddaaro> is that not exponent
<strcat> it's XOR
<toddaaro> fuuuuuuuuuuuuuuuuuuu
```

- **Rating:** 8 (14 votes)
- **Score:** 3.00
- **Submitted:** 2013-08-22 03:54:01
- **Approved:** true
- **Tags:** #rust, rusti, strcat, toddaaro

---

## Quote #7058

```
* bjz loves killing his babies
```

- **Rating:** 4 (14 votes)
- **Score:** 1.67
- **Submitted:** 2013-08-24 08:10:19
- **Approved:** true
- **Tags:** #rust, bjz, nocontext

---

## Quote #7065

```
<jorendorff> This sentence from a comment in the Rust runtime
<jorendorff> "When a task blocks, it turns its ~Task into a BlockedTask by storing a the transmuted ~Task pointer inside the KillHandle's kill flag."
<jorendorff> reminds me of http://www.penny-arcade.com/comic/2010/04/19
<till> jorendorff: that can't make sense to anybody
<jorendorff> till: INSIDE THE KILLHANDLE'S KILL FLAG
* jorendorff grabs till's shoulders and shakes him violently
```

- **Rating:** 1 (5 votes)
- **Score:** 1.33
- **Submitted:** 2013-08-30 22:47:06
- **Approved:** true

**Notes:** unrelated chatter removed

---

## Quote #7105

```
<lkuper_> yeah, I'm a much bigger fan of "we are nice because it's the decent human thing to do".  That was the point of my post -- someone shouldn't have to be a big shot in the community to be able to say "be nice"
<bstrie> lkuper_: btw that will be eight social capital dollars
<lkuper_> cha-ching!
<bstrie> but right now I'm in canada so I'll need that in loonies
```

- **Rating:** 3 (3 votes)
- **Score:** 4.00
- **Submitted:** 2013-10-07 03:17:40
- **Approved:** true
- **Tags:** #rust, bstrie, lkuper

**Notes:** In the context of a discussion about "Matz is nice and so we are nice"

---

## Quote #7102

```
<amatus> >.<
<amatus> i can't open /dev/null
<amatus> BECAUSE IT'S NOT A FILE
<amatus> this is the 2nd time i've coded everything up the "rustic" way and had to back down to raw libc calls!
<mcpherrin> welcome to bugland, home of the bugs.  Would you like a bug with your bug?
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2013-10-01 03:15:12
- **Approved:** true
- **Tags:** #rust

---

## Quote #7066

```
<dpc> How does RustOS sound? :D
<tjc> sounds like a snack food :-)
<bstrie> haha
<bstrie> thus was the downfall of the effort to write an OS in the Dorit programming language
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2013-09-05 17:28:37
- **Approved:** true
- **Tags:** #rust, bstrie, dpc, tjc

---

## Quote #7073

```
<tjc> out of battery, 'night all!
<strcat> irrefutable proof tjc is a cyborg
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2013-09-16 11:37:22
- **Approved:** true
- **Tags:** rust, strcat, tjc

---

## Quote #7077

```
<mark_edward_> ? should be unary, and return the type of a thing
<mark_edward_> let x: int = 5; let y:x? = 1;
<Eridius> ? should be a postfix operator that prints the value's repr to stdout
<tikue_> i think ? should reverse the order of arguments
<tikue_> |x| x + 1 ? spawn
<Eridius> if you type a ?, clippy shows up and asks if you need help writing your program
```

- **Rating:** 7 (7 votes)
- **Score:** 8.00
- **Submitted:** 2013-09-20 08:49:01
- **Approved:** true
- **Tags:** #rust

---

## Quote #7079

```
<jesseray> Sometimes, I wish there was an impure-Haskell. That would be heaven for me =P
<slack1256> jesseray: just use unsafeIO everywhere and you are done
<slack1256> that is what i do
<slack1256> and my computer doesn't cra
* slack1256 (slack@moz-B239C631.baf.movistar.cl) has left #rust (Konversation terminated!)
```

- **Rating:** 3 (3 votes)
- **Score:** 4.00
- **Submitted:** 2013-09-22 01:55:45
- **Approved:** true
- **Tags:** rust

---

## Quote #7087

```
<Jesse> rusti: "botsnack";
* rusti ()
<kimundi> rusti: version.slice_to(5).iter().map(|c| match c { 'r' => 'y', 's' => 'm', 't' | 'c' => '!', c => c}).collect::<~str>()
* rusti ~"yum!!"
```

- **Rating:** 10 (12 votes)
- **Score:** 6.00
- **Submitted:** 2013-09-24 13:58:24
- **Approved:** true
- **Tags:** #rust

---

## Quote #7095

```
<zws3mb> pastebin has syntax support for Whitespace and Brainfuck, but not for Rust!?
```

- **Rating:** 2 (2 votes)
- **Score:** 3.00
- **Submitted:** 2013-09-30 03:13:00
- **Approved:** true

---

## Quote #7111

```
<eholk> this code makes me wonder if the trait system might be turing complete
<tjc> paging Oleg
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2013-10-12 02:06:41
- **Approved:** true
- **Tags:** #rust, eholk, tjc

---

## Quote #7119

```
<eddyb> bstrie: uint isn't unsigned int, it's uintptr_t which should be as big as size_t
<eddyb> bstrie: uint >= uintptr_t = size_t
<eddyb> on a 16bit system, they could decide to make (u)int 32bit, but it could be 16bit as well
<kimundi> eddyb: Our uint and int are defined as pointer sized though
<bstrie> kimundi: I think what eddyb is saying is that size_t isn't defined as pointer-sized
<eddyb> bstrie: nope, it is
* bstrie throws up his hands
* kimundi catches bstries hands and hands them back
* bstrie reluctantly takes back his half-digested hands
```

- **Rating:** 2 (4 votes)
- **Score:** 2.00
- **Submitted:** 2013-10-17 13:49:49
- **Approved:** true
- **Tags:** #rust, bstrie, eddyb, ffi, kimundi

---

## Quote #7137

```
<brson> i need to come up with an interesting coding interview question by tomorrow, preferably something simply and data-structury but with a twist that is relevant to rust
<dbaupp> brson: will the subject(s) be on this channel?
<brson> lol
<brson> good question
<dbaupp> setting their own interview questions... haha
<mletterle> that's real hacking right there.
```

- **Rating:** 12 (12 votes)
- **Score:** 13.00
- **Submitted:** 2013-11-05 08:14:00
- **Approved:** true
- **Tags:** #rust

---

## Quote #7150

```
<jeaye> The rust-dev "daily" digest seems a lot more than daily.
   <cmr> jeaye: it depends on activity iirc.
<bstrie> jeaye: click _here_ to subscribe the #rust minutely digest
   <cmr> jeaye: did you see my ncurses-rs PR?
 <jeaye> cmr: Nope!
<bstrie> jeaye: thank you for subscribing to the #rust minutely digest
 <jeaye> bstrie: Exactly.
<bstrie> jeaye: < cmr> jeaye: did you see my ncurses-rs PR? < jeaye> cmr: Nope!
   <cmr> bstrie: please don't ;_;
<bstrie> jeaye: this has been the rust minutely digest. click _here_ to unsubscribe
   jeaye spams clicks _here_
<bstrie> jeaye: please log in to unsubscribe from the rust minutely digest
 <jeaye> fuck
<bstrie> jeaye: < cmr> bstrie: please don't ;_; < jeaye> fuck
   <cmr> jeaye: I suggest shooting bstrie
```

- **Rating:** 3 (5 votes)
- **Score:** 2.50
- **Submitted:** 2013-11-14 20:17:00
- **Approved:** true

**Notes:** #rust

---

## Quote #7159

```
<pnkfelix> acrichto: which PR was the native mutex PR ?
<cmr> https://github.com/mozilla/rust/pull/10479
<acrichto> pnkfelix: 10479
<pnkfelix> cmr: thx
<cmr> (I have every PR for the last ~5 months in my browser history)
<acrichto> ah beaten
<cmr> I type in any two words and chances are a rust PR comes up :P
<Luqman> cmr: haha, so what you're saying is i should go through you now?
<cmr> Luqman: if you want to annoy me, yes :P
<eddyb> cmr-pr-pedia
```

- **Rating:** 1 (3 votes)
- **Score:** 1.50
- **Submitted:** 2013-11-26 08:51:37
- **Approved:** true
- **Tags:** #rust, cmr, github

---

## Quote #7163

```
<mib_lyavkj> So, who thought it was a good idea to name this language after iron oxide?
<aatch> mib_lyavkj, no idea, who though it was a good idea to name a language after a letter in the alphabet?
<mib_lyavkj> Exactly!
<aatch> Or a family of snakes, or crystalline aluminium oxide
<mib_lyavkj> I'd much rather use rust or Go, but at least python sounds cool! Hardly anyone creates cool names for these things.
<aatch> Well there are three languages named after one guy.
<brendanc> You're talking about Dylan Oberon Pascal?
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2013-11-28 01:32:26
- **Approved:** true
- **Tags:** #rust

**Notes:** http://en.wikipedia.org/wiki/Haskell_Curry

---

## Quote #7174

```
<killerswan> no, really I just wanted to munge strings back and forth without ever caring about which hobbit borrowed what thing from which dragon
```

- **Rating:** 0 (8 votes)
- **Score:** 1.00
- **Submitted:** 2013-12-14 12:47:39
- **Approved:** true

**Notes:** #rust

---

## Quote #7187

```
<bstrie> are there any photoshop masters out there who volunteer to shop pcwalton's head onto the marine in http://www.threeguysonejoystick.com/wp/wp-content/uploads/2011/11/Doom1-Cover.jpg , and turn all the demons into @ symbols
<SiegeLord> ...did John Romero put an (R) symbol on his signature?
<bstrie> SiegeLord: hahahaha
<bstrie> I.... I think he did.
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2014-01-09 20:49:44
- **Approved:** true
- **Tags:** #rust-internals, bstrie, siegelord

---

## Quote #7201

```
<phroa> oh god xales, rust is beautiful
<xales> you don't even know how beautiful yet
<xales> use it
<xales> enjoy the tagged enums
<xales> slather them all over your body
<xales> then let rustc lick them off
```

- **Rating:** 7 (7 votes)
- **Score:** 8.00
- **Submitted:** 2014-01-30 04:34:07
- **Approved:** true
- **Tags:** rust

---

## Quote #7202

```
<dherman> c
<dherman> confirm
<wycats> don't worry I have taught #sweet.js about "c"
<dherman> macro c { rule c => confirm }
<wycats> hm you're missing some curlies
* dherman fail
<wycats> this is not the rust channel
```

- **Rating:** 2 (8 votes)
- **Score:** 1.50
- **Submitted:** 2014-01-30 05:46:35
- **Approved:** true
- **Tags:** dherman, programming, typos

**Notes:** #sweet.js

---

## Quote #7220

```
<Ms2ger> "(expected type error but found u8)"
<kimundi> Ms2ger: Rust is very strict, even the errors get typechecked :|
<Jesse> what input triggers that?
<kimundi> Probably follow error. A prior error was emitted saying that no type could be determined, and the compiler kept going spewing this one out
<eddyb> I did think we should refrain from printing type errors containing ty_err
<sfackler> I don't think there's a context in which it's ever useful
<bstrie> "expected type error but found u8" is basically the compiler saying, "look at you, you can't even screw up properly"
```

- **Rating:** 18 (18 votes)
- **Score:** 19.00
- **Submitted:** 2014-02-17 19:59:55
- **Approved:** true
- **Tags:** #rust

---

## Quote #7252

```
* kmc is having quite a time fighting with rust's inline assembly
<kmc> if any experts are around, assistance would be greatly appreciated
<jdm> kmc: ...are you adding inline assembly to the html parser?
<kmc> i'm not NOT adding inline assembly to the HTML parser
```

- **Rating:** 7 (7 votes)
- **Score:** 8.00
- **Submitted:** 2014-04-05 00:09:58
- **Approved:** true

**Notes:** Servo's future is bright.

---

## Quote #7253

```
<cajbir> hopefully nrc's gecko skills havent...rusted
```

- **Rating:** 12 (12 votes)
- **Score:** 13.00
- **Submitted:** 2014-04-08 05:30:25
- **Approved:** true
- **Tags:** cajbir, gecko, nrc, rust

---

## Quote #7266

```
<mbrubeck> we could implement <iframe browser> so you can build a browser in HTML, Gaia-style
<jdm> oppa gaia style
```

- **Rating:** 3 (5 votes)
- **Score:** 2.50
- **Submitted:** 2014-04-30 22:38:38
- **Approved:** true
- **Tags:** #servo, gaia, gangnamstyle

---

## Quote #7271

```
<Ms2ger> This one always makes me happy, no matter how often I see it
<Ms2ger>     /// This is the main entry point for receiving and dispatching DOM events.
<Ms2ger>     /// TODO: Actually perform DOM event dispatch.
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2014-05-13 14:34:24
- **Approved:** true
- **Tags:** servo

---

## Quote #7272

```
<jdm> woah
<jdm> woah
<jdm> about-mozilla just loaded and didn't crash
<jdm> this could be big
<jdm> crashes on shutdown, but no biggie
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2014-05-14 18:28:03
- **Approved:** true
- **Tags:** servo, jdm

---

## Quote #7278

```
<Ms2ger> "why's it slow? because, single-threaded why's it single-threaded? because, locks, mutable etc why's it mutable? because, performance!"
```

- **Rating:** 30 (30 votes)
- **Score:** 31.00
- **Submitted:** 2014-05-26 07:27:24
- **Approved:** true
- **Tags:** #rust

---

## Quote #7284

```
-!- jdm [jdm@moz-3D201070.cable.virginm.net] has joined #servo
<crowbot> jdm: Ms2ger said r????
<jdm> old news
<jdm> when will someone want me for my mind instead of just my r+ :(
<larsberg> jdm: it's even worse for me! I was just a standing, replacement jdm... not even as good as a real r+ from "the" jdm...
<jdm> aww
<jdm> there there, we can't all be me
```

- **Rating:** 3 (3 votes)
- **Score:** 4.00
- **Submitted:** 2014-06-11 22:28:25
- **Approved:** true
- **Tags:** #servo, jdm, larsberg

---

## Quote #7292

```
<Ms2ger> glob|away, logbot seems to be gone from #servo 
<mihneadb> Ms2ger: it couldn't take it anymore
<Ms2ger> Pah
<mihneadb> burnout
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2014-07-07 10:54:43
- **Approved:** true
- **Tags:** burnout, logbot, mihneadb, ms2ger, servo

---

## Quote #7297

```
<sfackler> acrichto: https://github.com/sfackler/rust-postgres/commit/28744034bbea218d1216808d36c58875906c0f34
<acrichto> sfackler: woo! thanks
<sfackler> RIP Index
<eddyb> "we didn't love you anyway"
<kimundi> haha
<sfackler> I did ;_;
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2014-07-10 19:41:29
- **Approved:** true
- **Tags:** rust

**Notes:** Overloaded index operator changed in Rust.

---

## Quote #7300

```
<kmc> if Google can translate English to Arabic using a huge machine learning corpus, maybe we can translate HTML syntax to DOMs the same way ;)
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2014-07-15 18:31:21
- **Approved:** true
- **Tags:** #servo, html, language, parsing, translation

---

## Quote #7310

```
<arrrrrrrrr> the stdlib contains too many jokes to fit on embedded devices
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2014-08-02 19:39:08
- **Approved:** true
- **Tags:** jokes, rust, stdlib

---

## Quote #7314

```
<jgraham> (there is a reason I propose that; the rust meetup is on and based on last time having a reasonable lunch beforehand can be helpful)
<jgraham> (so it's not just selfishness)
<jonco> what is it about rust that requires a full stomach? :)
```

- **Rating:** -4 (12 votes)
- **Score:** 0.56
- **Submitted:** 2014-08-13 11:17:52
- **Approved:** true
- **Tags:** food, hungry, jgraham, jonco, rust

---

## Quote #7331

```
* jdm is seeing what happens when delaying layout for as long as possible
<jdm> ie. until a layout query is issued or we return to the event loop
<zwarich> wait, what's the current policy?
<jdm> HOLY CRAP THE DOM CHANGED BETTER RELAYOUT THE ENTIRE PAGE JUST IN CASE
```

- **Rating:** 19 (19 votes)
- **Score:** 20.00
- **Submitted:** 2014-09-16 02:34:17
- **Approved:** true
- **Tags:** perf, layout, #servo

---

## Quote #7348

```
<bstrie> eddyb: we'll be stabilizing all comments for 1.0. code can change, but comments become part of the public API
<bstrie> typo fixes will have to be deferred to 2.0
<nmatsakis> bstrie: you laugh but once in an aquisition attempt...
<nmatsakis> I had written some code to do a linaer search, or binary search, or something like that
<nmatsakis> and I wrote "// Super clever search algorithmn. Patent pending."
<nmatsakis> and apparently this cost us many months
<nmatsakis> where the lawyers were like
<nmatsakis> "patent? what patent? we didn't see that in the paperwork!"
<nmatsakis> our CEO told me later...
```

- **Rating:** 19 (19 votes)
- **Score:** 20.00
- **Submitted:** 2014-10-30 17:27:30
- **Approved:** true
- **Tags:** #rust-internals

---

## Quote #7351

```
<bstrie> "If this query were assigned to one of the variables used in $query, the query beast awakened." #officialPHPdocs
<bstrie> Turning and turning in the widening gyre
<bstrie> The program cannot hear the programmer;
<bstrie> Things fall apart; the database cannot hold;
<bstrie> Mere anarchy is loosed upon the system;
<bstrie> The malicious query is injected, and everywhere
<bstrie> The ceremony of security is drowned;
<bstrie> The best lack all conviction, while the worst
<bstrie> Wield bonafide Zend PHP Certification.
<bstrie> The connection drops again; but now I know
<bstrie> That twenty centuries of stony sleep(1)
<bstrie> Were vexed to nightmare by string interpolation,
<bstrie> And what rough beast, its hour come round at last,
<bstrie> Slouches towards MySQL to be run?
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2014-11-04 16:28:08
- **Approved:** true
- **Tags:** #rust-offtopic, php, yeats

**Notes:** http://php.net/manual/en/security.database.sql-injection.php

---

## Quote #7371

```
<Ms2ger> Hi mdas 
<mdas> Ms2ger: hey, how's it going?
<Ms2ger> Cursing rust, all good otherwise :)
<nigelb> Ms2ger: I hope your sheriffing skills aren't... rusting ;)
<Ms2ger> nigelb, you know the most appropriate music for a rust dev to listen to?
<nigelb> Ms2ger: death metal?
<Ms2ger> nigelb, correct
```

- **Rating:** 3 (15 votes)
- **Score:** 1.43
- **Submitted:** 2014-12-16 08:33:55
- **Approved:** true
- **Tags:** mdas, metal, ms2ger, rust

---

## Quote #7378

```
<SiegeLord> All I want to do is return some closures, not prove NP=P
```

- **Rating:** -1 (9 votes)
- **Score:** 0.83
- **Submitted:** 2014-12-27 04:30:16
- **Approved:** true
- **Tags:** closures, p=np, rust

**Notes:** #rust

---

## Quote #7390

```
<Ms2ger> Don't you like my spec?
<Manishearth> why is it purple
```

- **Rating:** -5 (13 votes)
- **Score:** 0.50
- **Submitted:** 2015-02-03 16:43:11
- **Approved:** true
- **Tags:** #servo, dom, purple, standards, style

**Notes:** https://html5.org/specs/dom-range.html

---

## Quote #7393

```
<larsberg> footgun status: fired
<jdm> why do we even have a gun that fires feet
<larsberg> how else would we fill all these clownshoes?
```

- **Rating:** 19 (19 votes)
- **Score:** 20.00
- **Submitted:** 2015-02-10 19:42:59
- **Approved:** true
- **Tags:** #servo, clownshoes

---

## Quote #7394

```
<jdm> it seems like my assumptions are always invalidated by looking at the code
<jdm> I should just not look at the code in the future
```

- **Rating:** 12 (12 votes)
- **Score:** 13.00
- **Submitted:** 2015-02-10 21:48:31
- **Approved:** true
- **Tags:** #servo

---

## Quote #7397

```
<pcwalton> never did I think I would be rick rolling myself over and over as part of this job
<larsberg> plus, if the first version of youtube media support internally redirected to Rick Astley, it would probably take a while before people learned that was all servo supported...
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2015-02-18 01:35:20
- **Approved:** true
- **Tags:** larsberg, pcwalton, rick, rolling, servo

---

## Quote #7405

```
<eddyb> https://github.com/rust-lang/rfcs/pull/907 aka "future-proof language design is hard"
<eddyb> but at a glance, that RFC fixes the hole in the roof by plugging it with kittens
```

- **Rating:** 3 (5 votes)
- **Score:** 2.50
- **Submitted:** 2015-03-03 02:14:57
- **Approved:** true
- **Tags:** #rust, eddyb, kittens

---

## Quote #7408

```
<steveklabnik> ETE?
<wycats> Emerging Technologies for the... Enterprise
<steveklabnik> ahh
<steveklabnik> neat
<wycats> either it's a Java conference or it takes place in the Star Trek universe
<wycats> I have gone a few times and I'm unsure which
```

- **Rating:** 12 (12 votes)
- **Score:** 13.00
- **Submitted:** 2015-03-13 15:29:43
- **Approved:** true
- **Tags:** #rust, enterprise, startrek

---

## Quote #7409

```
<pcwalton> argh
<pcwalton> I hate Rust
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2015-03-14 00:43:03
- **Approved:** true

**Notes:** Everybody says it, nobody means it.

---

## Quote #7416

```
<Manishearth> looks like jgraham from the automation team broke our automation
<Manishearth> sabotage I tell you
...
<jgraham> Manishearth: It's "automationb and tools"
<jgraham> In this case I was playing the role of "tool"
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2015-04-05 05:04:44
- **Approved:** true
- **Tags:** #servo, jgraham, manishearth

**Notes:** jgraham's pull request broke our bots

---

## Quote #7440

```
<Gankro> There's something deeply visceral about seeing *PR posted* *PR closed* *branch deleted* within a few hour span and no comments.
<Gankro> The only way I think it could be better is if it was followed by *account deleted*
<Quxxy> Gankro: Followed a few days later by a notice in the local newspaper of a man found hanging from his ceiling fan and "git reset --hard" scrawled on his chest in blood
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2015-06-30 05:40:57
- **Approved:** true
- **Tags:** #rust-offtopic, gankro, quxxy

---

## Quote #7447

```
<pcwalton> got select() working on ipc-channel on Mac :)
<pcwalton> Mach port sets \o/
pcwalton quit (pcwalton@moz-nh2.o1i.216.216.IP) Client exited
<jdm> every time pcwalton disappears after making some comment about mac IPC code I imagine he's triggered a new kernel oops
```

- **Rating:** 7 (7 votes)
- **Score:** 8.00
- **Submitted:** 2015-07-15 18:39:30
- **Approved:** true
- **Tags:** #servo, e10s, macosx, oops

---

## Quote #7453

```
<proc> and having a vm parsing it
<cmr> and there's a compielr.
<eddyb> what does a compielr compiel?
<WindowsBunny> eddyb: coed
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2015-08-18 21:53:35
- **Approved:** true
- **Tags:** erlang, perl, rust-offtopic

**Notes:** similarities between perl and Erlang

---

## Quote #7483

```
<kidnapped_robot> is there any such thing as a shared memory ipc library for rust?
<tikue> anyone know of a shared memory ipc library?
<tikue> omg
<kidnapped_robot> lol
<tikue> awkward
```

- **Rating:** 1 (1 votes)
- **Score:** 2.00
- **Submitted:** 2015-10-09 00:04:50
- **Approved:** true
- **Tags:** jinx, coincidence, #rust

---

## Quote #7493

```
<proc> when you doubt your code so much that you don't trust the IDE output anymore, delete the whole build and rebuild it manually..
<rphmeier> proc: "cargo clean && cargo build" is just as effective :P
```

- **Rating:** 2 (2 votes)
- **Score:** 3.00
- **Submitted:** 2015-10-29 03:48:23
- **Approved:** true
- **Tags:** #rust

**Notes:** when the build log seems too short and error lacking

---

## Quote #7494

```
<pcwalton> is there a faster way to linearly interpolate varyings on a quad than to triangulate the quad and use barycentric coordinates on the triangle the point is in?
<pcwalton> (apologies for the basic question)
```

- **Rating:** 17 (17 votes)
- **Score:** 18.00
- **Submitted:** 2015-11-04 22:59:58
- **Approved:** true
- **Tags:** #servo, gfx, opengl, technobabble

---

## Quote #7506

```
* WindowsBunny installs bunny toolbar
<WindowsBunny> which actually isn't a toolbar
<WindowsBunny> it just causes bunnies to hop around websites inside your browser
<proc> ff bunny theme, animated
<WindowsBunny> that would be an _awesome_ extension
<WindowsBunny> they'd climb on various page elements
<WindowsBunny> nibbling the corners off of images
<proc> WindowsBunny: as long as they don't click on them
<WindowsBunny> proc: You can also pet the bunnies with the mouse cursor
<proc> WindowsBunny: cookie-clicker is out of the league with that 
<WindowsBunny> I'll probably have to have a button on one of the toolbars though where you can go to get more bunnies or get food to feed them
<proc> WindowsBunny: think of them climbing between the layers of youtube-videos
<proc> WindowsBunny: but only for money :P
<proc> and if you don't feed em every day, you loose some
<WindowsBunny> proc: I'll be rich from microtransactions
<proc> WindowsBunny: implemented as an microservice
<WindowsBunny> damnit, now I really want to make this extension
```

- **Rating:** 2 (2 votes)
- **Score:** 3.00
- **Submitted:** 2015-12-05 02:28:41
- **Approved:** true
- **Tags:** #rust-offtopic, #windowsbunny

---

## Quote #7521

```
<caitp> a support group for victims of C++ software development
<Yoric> caitp: It's called #rust.
```

- **Rating:** 14 (14 votes)
- **Score:** 15.00
- **Submitted:** 2016-02-19 16:46:29
- **Approved:** true
- **Tags:** introduction

---

## Quote #7528

```
* bluss bravely runs away
<Quxxy> Brave Sir Blussie ran away
<Quxxy> Bravely ran away, away
<Quxxy> When dev work reared its ugly head, he bravely logged from chat and fled
<Quxxy> Yes Brace Sir Blussie turned about, we guarantee his chickened out
<Quxxy> *he chickened out, *crap* screwed it up
* Quxxy hurls a towel on the ground for some reason and walks out of the recording booth
<bluss> I knew they would write songs about me
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2016-04-13 17:26:20
- **Approved:** true

**Notes:** #rust-offtopic

---

## Quote #7534

```
<kamalmarhubi> where can I get started with profiling compilation
<Mutabah> kamalmarhubi: -C time-passes iirc
<Kingsqueeee> Mutabah: that's a pretty zen sounding flag
```

- **Rating:** 4 (4 votes)
- **Score:** 5.00
- **Submitted:** 2016-04-30 05:02:59
- **Approved:** true
- **Tags:** #rust

---

## Quote #7538

```
<ajeffrey> I can see your argument about not using ho functions for side-effects,
<ajeffrey> as it makes the control flow, er, difficult to follow.
<Ms2ger> "ho functions", huh
<ajeffrey> 4th order functions are ho ho ho functions.
<nox> Santa functions.
<ajeffrey> the borrow checker knows if you've been naughty or nice.
```

- **Rating:** 3 (13 votes)
- **Score:** 1.50
- **Submitted:** 2016-05-09 15:00:46
- **Approved:** true
- **Tags:** #servo, christmas, fp, rust

---

## Quote #7541

```
<KiChjang> my favourite episode of Rust Wars is Episode IV: A New Homu
```

- **Rating:** -6 (10 votes)
- **Score:** 0.33
- **Submitted:** 2016-05-19 16:54:29
- **Approved:** true
- **Tags:** #servo, bors, homu, movies, rust, starwars

---

## Quote #7543

```
<pcwalton> ignore the fact that the FPS is low in both cases; that’s because my test app draws a ton of doges on the CPU
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2016-05-26 21:16:38
- **Approved:** true
- **Tags:** #servo, doge, performance

---

## Quote #7546

```
<ajeffrey> jgraham: (why are we whispering?)
<jdm> spec editors lurk under every bridge
<jdm> they have very sharp hearing
<ajeffrey> jdm: okay, at least you are validating my confusion :)
<ajeffrey> jdm: do spec editors eat goats?
<jdm> yes, to reduce our supply of sacrificial ones
<jdm> which limits our ability to divine the spec's true intentions
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2016-06-08 15:07:55
- **Approved:** true
- **Tags:** #servo, ajeffrey, jdm, specs

---

## Quote #7548

```
<pcwalton> I don’t want to make a decision like this with measurements.
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2016-06-21 12:45:06
- **Approved:** true
- **Tags:** data, measurements, servo

---

## Quote #7557

```
<breeden_> i feel like we should get a @rust-lang/offtopic team
<scott> haha
<ubsan> breeden_ :+1:
<scott> i demand offtopic meetings
<Havvy> breeden_:  That'd be hilarious.
<scott> any mis-filed issue will be assigned to the offtopic team
<Havvy> scott:  So we'll be the ones that deal with issues about Rust-game?
<scott> anything about the Rust game is the offtopic team's domain
<scott> :thumbs up emoji:
* Havvy claps.
<scott> an RFC about forming the offtopic team would be blocked on account of only the offtopic team having authority to merge it
<scott> since it's utterly frivolous

[later]

<scott> offtopic team members should have to use nixos
<dikaiosune> scott: there's an offtopic team?
<Havvy> scott:  Except WinBunny?
<dikaiosune> if so, i vote temple os
<scott> dikaiosune: it was proposed earlier today
<dikaiosune> i will kill everyone in my way to being on the offtopic team.
<curtism> TempleOS is a fine piece of software
<scott> dikaiosune: good
<curtism> dikaiosune: Not if I kill you first
<Havvy> dikaiosune:  The proposal for the offtopic team would be approved by the offtopic team after the offtopic team was created.
<dikaiosune> Havvy: so it must be approved by fiat
<dikaiosune> I SO APPROVE
<Havvy> dikaiosune:  It's more approved via time loop.
<scott> curtism: i wasn't happy about it either
<dikaiosune> Havvy: that was my future self approving it, they're gone now
<dikaiosune> it was really weird, almost created a paradox
```

- **Rating:** -1 (21 votes)
- **Score:** 0.92
- **Submitted:** 2016-07-21 05:23:50
- **Approved:** true
- **Tags:** offtopic, rust, team

**Notes:** #rust-offtopic

---

## Quote #7571

```
<Ixrec> and from misuses of event systems
<Ixrec> our internal framework had events before it had promises so that happened quite a bit
<Ixrec> so much fun writing tests that call foo() and then wait for an event to get fired
<Ixrec> then I realize I have to start waiting for the event *before* I call foo()
<Ixrec> or else it never fires and the test times out
<fmtq> Ixrec: hahaha you tried to unit test it
<Ixrec> we *do* unit test it!
```

- **Rating:** 1 (1 votes)
- **Score:** 2.00
- **Submitted:** 2016-09-24 12:36:09
- **Approved:** true
- **Tags:** events, js, promises, rust-offtopic

**Notes:** Talking about JS and rewriting event based code to promises

---

## Quote #7572

```
<spudowiar> * the pin drop echoes *
<sp3d> shhh
<jonas> What happened next?
<spudowiar> jonas: I stepped on the pin
<durka42> error: attempted use of pin after drop
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2016-10-06 18:21:44
- **Approved:** true
- **Tags:** #rust, error

---

## Quote #7577

```
< fmtq> sleffy: I'm also a Java dev
< fmtq> and C++ (OLD C++) dev
< fmtq> current project is iOS/Android app and backend >_>
< fmtq> I everything
< sleffy> Wow
< fmtq> I'm ridiculously good at the borrow checker though
< fmtq> in Rust.
< bstrie> fmtq: once you've mastered borrow checkers, you may move on to borrow chess
```

- **Rating:** 10 (16 votes)
- **Score:** 3.50
- **Submitted:** 2016-12-11 06:51:18
- **Approved:** true
- **Tags:** #rust-offtopic, rust

---

## Quote #7581

```
<SimonSapin> I’m tempted to return my new micro-wave oven to the store
<SimonSapin> because of bad kerning
<SimonSapin> it has a four-digit display like xx:yy, fairly standard
<SimonSapin> but there’s too much space on either side of the ":"
<SimonSapin> and every time I see it I hate it
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2017-01-14 10:02:43
- **Approved:** true
- **Tags:** #servo-offtopic

**Notes:** First world problems.

---

## Quote #7587

```
<mqudsi> Is there anything prettier than Result<Option<T>,E>?
<mqudsi> In particular, the syntax for return Ok(Some(value)) makes me want to cry :(
<mbrubeck> I think that I shall never see
<mbrubeck> a poem as pretty as Result<Option<T>,E>
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2017-04-06 23:18:15
- **Approved:** true
- **Tags:** #rust, poetry

---

## Quote #7590

```
<nox> bz: But also because when I started reading the spec,
<nox> I'm pretty sure there was a note about them not being able to be nested,
<nox> and a few days later, the note was gone, of course.
<bz> nox: note:not(:not(:not(:not(:matches(spec)))))
```

- **Rating:** 5 (7 votes)
- **Score:** 3.50
- **Submitted:** 2017-04-25 17:18:14
- **Approved:** true
- **Tags:** #servo, css, not, selectors, specs

---

## Quote #7592

```
<John-Galt> jimb: Why does all of our rust stuff only compile on one core?
<John-Galt> I mean... pretty much the whole point of rust is to make parallelism easy, but I always wind up getting to the end of the compile and then waiting another 8 minutes for the rust stuff to finish while 7 cores sit idle...
<agashlin>  John-Galt: because rust is irony?
```

- **Rating:** 18 (22 votes)
- **Score:** 7.00
- **Submitted:** 2017-08-04 04:22:59
- **Approved:** true
- **Tags:** agashlin, john-galt, rust

---

## Quote #7593

```
<@SimonSapin> bholley: Good news, stylo stylesheet parsing microbench just went from ~39ms to ~29ms with my WIP patch. Bad news, I don’t know why.
```

- **Rating:** 9 (9 votes)
- **Score:** 10.00
- **Submitted:** 2017-08-21 13:23:48
- **Approved:** true
- **Tags:** #quantum, #servo

---

## Quote #7594

```
<@brson> i just rejected a rust-dev post from somebody complaining about the performance of the rust game
<@acrichto> brson: haha
<@acrichto> at some point those posts will start to decline
<@acrichto> hopefully...
<cmr> ... maybe
<cmr> the game does perform pretty poorly
<cmr> it'd clearly be better if it were written in rust
<@brson> what can we do to sabbotage this game?
<@brson> make it go away
 * cmr plays it
<@acrichto> brson: I think a more reasonable course of action would be to rename the languge
<@brson> lol
<jack> brson: it's free to play so i assume it will die eventually.
<cmr> jack: no it's not
<jack> cmr: hmm. is there another game like that that is free to play that i am confusing it with?
<jack> I should go onto the steam forums and start asking borrow checker questions.
<cmr> !!
<cmr> that is the best path forward.
<cmr> jack: dunno
<cmr> I don't know of anything else called rust
<jack> cmr: i think i was thinking of path of exile
<@tjc> pay off the game makers to rename their game to something else :-)
<@tjc> such as "Go"
<kimundi> "Rust and Go"
<kimundi> ... that sounded funner in my head
<cmr> "Go and Rust" is more assertive :p
<jack> maybe we can get these confused people to pay us $20 a pop for our language.
<@tjc> heh
<@tjc> who says you can't make money selling a compiler? You can, as long as you convince people it's a video game.
<cmr> QOTW right there
<@tjc> "It's a text-based adventure where you fight a monster called the Borrow Checker by typing cryptic symbols like ~ and @"
<jack> Isn't it one? It's kind of like a text adventure where you have to discover memory safety in your dumb, broken program.
<@tjc> exactly
<eddyb> cmr: someone nominated something else in #rust, yesterday or so
<cmr> eddyb: I got it
<jack> You have been eaten by a dynamically sized type.
<cmr> "eddyb> who needs pure functional when you have pure cool?"
<@tjc> jack++
<@tjc> also, no refunds if you see the words "Internal compiler error", it just means the monsters won
<jack> PLACATE BORROWCK
<jack> You have found an internal compiler error. YOU WIN!
<@tjc> heh
<@tjc> this could be a good way to outsource fuzz testing without having to rewrite the fuzzer ;-)
<jack> The cave is dark an damp. The cackles of deranged programming language designers echo off every surface. There is a sigil on the floor.
<larsberg> "You are in a nondescript text file of nondescript size. Pointers are named north, south, and west. Oh, and there's a Borrowck over there in the corner."
<jack> Written in blood on the far wall you see 'r-'.
<larsberg> acrichto: moving the toolchain to android-18 sadly did not change anything
<larsberg> also, still getting "warning: RUST_LOG set, but no crate map found." at startup
<larsberg> which leads me to think it's not working
<larsberg> maybe a grue ate it?
<eddyb> cmr: heh. I wasn't sure it was me, I didn't remember who said and what was said :P
<eddyb> larsberg: s/grue/internal symbol stripper
 * jack MOVE NORTH
<jack> error: Use of already moved value, north.
<larsberg> lol
<jack> GO NORTH
<jack> error: Go is not a systems programming language.
<eddyb> haha, I was going to make a war in the north reference, but this is much better
<jack> WALK NORTH
<jack> error: The tree is infinite. You starve. YOU LOSE.
```

- **Rating:** 1 (33 votes)
- **Score:** 1.06
- **Submitted:** 2017-08-26 01:26:01
- **Approved:** true
- **Tags:** #game, #rust, #rust-internals

**Notes:** from around 2014

---

## Quote #7596

```
<durka42> in #rust "fearless concurrency" means getting 5 answers to your question in parallel
```

- **Rating:** 13 (13 votes)
- **Score:** 14.00
- **Submitted:** 2017-09-20 16:39:04
- **Approved:** true
- **Tags:** #rust

---

## Quote #7607

```
<durka42> I really enjoy acrichto's rapidly changing certainty levels in PR descriptions
<durka42> > we for sure probably don't want to upstream this and otherwise it seems not too bad for now at least
```

- **Rating:** 7 (7 votes)
- **Score:** 8.00
- **Submitted:** 2018-02-06 20:05:13
- **Approved:** true
- **Tags:** #rust

---

## Quote #7610

```
<mbrubeck> Pre-RFC: Rename "attributes" to "hashtags"
<durka42> best argument for changing to @-syntax I've heard yet
```

- **Rating:** 1 (1 votes)
- **Score:** 2.00
- **Submitted:** 2018-03-15 17:58:58
- **Approved:** true
- **Tags:** #rust, syntax

---

## Quote #7612

```
<j_ey> eval: println!("{:?}", std::borrow::Cow::from(vec![1, 2, 3].into_boxed_slice()))
-eval- error[E0277]: the trait bound `std::borrow::Cow<'_, _>: std::convert::From<std::boxed::Box<[{integer}]>>` is not satisfied
-eval- ~~~ Full output: https://play.rust-lang.org/?gist=04234cc0057890e32c9d23c790870edc&version=stable&mode=debug
<sebk> j_ey: I guess you have to define your own cow
<sebk> CowSlice ?
<j_ey> Beef<>
```

- **Rating:** 2 (20 votes)
- **Score:** 1.20
- **Submitted:** 2018-05-27 12:00:45
- **Approved:** true
- **Tags:** rust

---

## Quote #7613

```
<bstrie> excel asked me if I wanted to restart in order to apply updates. I said no. in response, excel turned all text in its UI transparent. got me to restart and update. the system works!
```

- **Rating:** 5 (7 votes)
- **Score:** 3.50
- **Submitted:** 2018-07-30 22:58:09
- **Approved:** true
- **Tags:** #bstrie, #rust-offtopic

---

## Quote #7614

```
<panicbit> This is a nice actual tree view https://i.imgur.com/R8iZYDL.jpg
<panicbit> one of the many openly available
<fmtq> you fuck
<fmtq> I clicked this
<panicbit> mission accomplished
```

- **Rating:** 5 (5 votes)
- **Score:** 6.00
- **Submitted:** 2018-08-01 17:29:30
- **Approved:** true
- **Tags:** #rust-offtopic

---

## Quote #7618

```
<eijebong> nox: Bugs in script/ are so rewarding, I feel like every time you change something in there, it fixes 40 bugs but take 40 minutes to compile :p
<lqd> therefore, we should make it compile slower
<eijebong> lqd: no
<lqd> too late im doing it
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2018-09-17 22:51:19
- **Approved:** true
- **Tags:** servo

---

## Quote #7620

```
<Manishearth> aah the bug i am bisecting is not one bug but TWO bugs
<mbrubeck> git trisect
<mbrubeck> Hope that it's not a bug in ANGLE
```

- **Rating:** 6 (6 votes)
- **Score:** 7.00
- **Submitted:** 2018-11-07 22:52:55
- **Approved:** true
- **Tags:** #servo, bisect, geometry, math

---

## Quote #7622

```
<lina> myk: srsly, I'm so excited to start using rkv. and the rust xpcom helpers, too :-) thank you for working on this and getting it over the finish line!
<myk> lina: me too, and you bet!
<chutten> Still the best pun at mozlando
<chutten> I'll remember "key value proposition" 'til my end of days
<lina> chutten: it's great we didn't make a hash of it
<chutten> I dunno. I like breakfast potatoes.
<dylan> lina: as a js developer, I Object to that pun.
<lina> dylan: I guess this is as good of a place as any to arrays your objections
```

- **Rating:** -3 (13 votes)
- **Score:** 0.67
- **Submitted:** 2019-02-07 18:30:50
- **Approved:** true
- **Tags:** #developers

---

## Quote #7630

```
<@chutten> Why didn't the compiler catch it?
<@janerik> well ... if written ... the easy way the compiler _did_ catch it and told me it's never-ending recursive
<@janerik> then I tried to outsmart the compiler
<@janerik> turns out I'm not smarter than the compiler
```

- **Rating:** 6 (32 votes)
- **Score:** 1.43
- **Submitted:** 2019-09-09 13:32:56
- **Approved:** true
- **Tags:** compiler, recursion, rust, smart

---

## Quote #7633

```
<jesopo> ,config c karma-pattern
[Notice] -BitBot to #rust-offtopic- [Config] You do not have permission to do this
<jesopo> oh you little shitbag
```

- **Rating:** -3 (27 votes)
- **Score:** 0.81
- **Submitted:** 2019-09-27 16:30:07
- **Approved:** true
- **Tags:** #rust-offtopic

**Notes:** 3rd command without permissions

---

## Quote #7635

```
<pcwalton> can we just talk about how amazing would it would be if issue 65536 came down to a 16-bit integer overflow
```

- **Rating:** 2 (22 votes)
- **Score:** 1.18
- **Submitted:** 2019-11-07 16:20:51
- **Approved:** true

**Notes:** https://github.com/rust-lang/rust/issues/65536

---

