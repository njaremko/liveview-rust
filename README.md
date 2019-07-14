# liveview-rust
PoC of LiveView in rust

This was inspired by the [Phoenix Live View](https://github.com/phoenixframework/phoenix_live_view) project.

We follow a similar model, with the only difference being that we send the full html on each render 
and let the client calculate the diff, instead of sending only diffs to client and letting them apply the change.

What works?
  - We click, text input, and submit events are send to server and template is re-rendered and sent to client, then morphdom
  applies the change to the dom.
  - Build on Actix-Web at the moment, potentially working with Rocket at some point as well.
  
Whats left?
  - Testing framework
  - HTML diffs on server side (not nessesary for PoC)
  - Write some macros to make implementation nicer
