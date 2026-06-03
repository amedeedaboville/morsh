Morsh: a mosh re-implementation from the 2020s
---
This is a project to re-implement the ideas behind mosh "the mobile shell", which is
a smart ssh replacement that works well despite slow and unreliable networks.

The idea is to use modern tech like QUIC, the great libraries in the Rust ecosystem, and Rust
itself to deliver a smaller, and battle tested version of mosh by standing on the shoulders
of giants.

Specifically, I want to use:

* QUIC which has TLS built in instead of mosh implementing its own AES-OCB encryption
* tokio for great async and reliability
* a great Rust term library like `alacritty-terminal` or `wezterm-term`

QUIC should automatically bring 2 out of the 3 mosh headline features for free: roaming and encryption.
It typically provides a reliable (TCP-like) data stream with re-transmission, but the RFC 9221 
Unreliable Datagrams extension also provides a UDP-like interface.

It is a learning project atm but I intend to use it myself. I plan on doing:
Phase 1: Transport
[ ] - Create pluggable Transport interface starting with TCP
[ ] - Get a ssh handshake going and use the tcp transport, essentially acting as a ssh proxy
[ ] - Integrate quinn (probably quinn-udp) to implement a QUIC transport
[ ] - Testing: model the network to test the Transport implementations, making sure they are reliable

Phase 2: Terminal emulation and SSP diffing
[ ] - Make the server a terminal emulator, and have it send bytes over the Transport at first
[ ] - Implement a diff for the terminal state, and send over diffs instead of bytes (over reliable transport)
[ ] - Testing: drive the server with some scripted shell sessions (htop or vim), snapshot the client terminal and compare to server
[ ] - Testing: PBT for `apply(old, diff(old, new)) == new`. Generate random grids and random patches and assert the round-trip.
[ ] - Testing: do fault injection on the Transport, (make it drop packets) and assert the client grid eventually converges to the server grid once datagrams get through. 

Phase 3: Optimistic updates/emulation
[ ] - Do the mosh "Prediction Engine" stuff 

I really want to do "Antithesis/FoundationDB style" testing, probably uing turmoil or madsim.

AI/LLM Policy
----
I would like this project to be useful to others, and for that it has to be trusted. There's serious complexity and risk in this
kind of codebase (encryption, networking, remote connections) and potential for unreliability that what people want out is a 
product they can trust, not just a featureful one. This is especially important given that one of the motivations for this 
project is to give mosh (C++, older crypto) more reliable footing. So:

* Every line of code in the application and documentation has to be typed in by a human 

It doesn't have to be exactly human fingers, you can use voice dictation or whatever input method you like. 

LLMs can be used to analyze the code, consult about issues, diagnose bugs, etc. I may use them to suggest fixes or features,
but will type in every line of code as if I wrote it.

* Supporting code like deployment scripts, test scenarios, CI configs may be LLM-assisted. 

Things that are not "the content" of the app like tests, deployment scripts and other utility scripts might be LLM written.

The test infrastructure itself sort of straddles the two cases: It's technically not bundled in the app that users download, but it needs to be reliable and vetted and able to be trusted by users. I want to learn new deterministic simulation testing techniques so I'll probably implement it myself for now.



