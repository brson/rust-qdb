# Rust Quotes from Mozilla QDB

Total quotes: 57

---

## Quote #3109

```
* timeless cries
<timeless> my minefield blew up
<shaver> does what it says on the tin
<Neil> timeless: don't click on any of the squares next to an '8' next time ;-)
```

- **Rating:** 24 (24 votes)
- **Score:** 25.00
- **Submitted:** 2007-11-19 11:34:29
- **Approved:** true
- **Tags:** rust

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

**Notes:** http://en.wikipedia.org/wiki/Fanny#In_slang

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

---

## Quote #6813

```
<benh> rust is the best language, my code is broken and someone else on the other side of the world is already working on making it work before I even know it
```

- **Rating:** 3 (3 votes)
- **Score:** 4.00
- **Submitted:** 2013-02-26 18:45:56
- **Approved:** true

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

---

## Quote #6830

```
<bstrie> in rust it's turtles all the way down, until you get to a @. at that point the turtles all turn into pelicans and have a techno dance party in a warehouse.
```

- **Rating:** 11 (11 votes)
- **Score:** 12.00
- **Submitted:** 2013-03-07 21:34:50
- **Approved:** true

**Notes:** bstrie explaining ownership semantics in rust

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

---

## Quote #6933

```
<gareth0> rust follows the yoda school of error handling: do, or do not, there is no try
```

- **Rating:** 8 (8 votes)
- **Score:** 9.00
- **Submitted:** 2013-05-21 20:21:13
- **Approved:** true

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

**Notes:** Overloaded index operator changed in Rust.

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

---

## Quote #7378

```
<SiegeLord> All I want to do is return some closures, not prove NP=P
```

- **Rating:** -1 (9 votes)
- **Score:** 0.83
- **Submitted:** 2014-12-27 04:30:16
- **Approved:** true

**Notes:** #rust

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

## Quote #7541

```
<KiChjang> my favourite episode of Rust Wars is Episode IV: A New Homu
```

- **Rating:** -6 (10 votes)
- **Score:** 0.33
- **Submitted:** 2016-05-19 16:54:29
- **Approved:** true

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

**Notes:** #rust-offtopic

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

