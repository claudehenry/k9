# ```k9```

A performance and correctness testing framework, greatly inspired by the wonderful [`k6`](https://github.com/grafana/k6).
If you haven't heard of it, it's _amazing_, go check it out at https://k6.io.

## Project Goals

I would like `k9` to...

- [x] Be async.
  - `k6` has interesting limitations to its threading model.
  - One could say you are currently _forced_ down that path...
- [ ] Be less opinionated about what a 'test' looks like.
  - Currently, this is achieved by a total lack of 'orchestration' tools... whilst not ideal, it's in the spirit of what I want to achieve. Fewer things done for you, less magic, more control.
- [ ] Support a significant subset of `k6`'s features.
  - [ ] Data sinks other than `Trend`.
  - [ ] Out of the box beautiful reports.
- [ ] Provide WebSocket constructs.
- [ ] Exotic reports.
  - [ ] Python notebook integration for custom data analysis pipelienes.
  - now we're really in the weeds :sweat_smile:.

And more generally, I would like `k9` to be a tool for situations where `k6` is not sufficient / ideal.
Let's pause on that thought for a second. I _am_ saying k6 is more than enough for most situations.
If you've landed here, looking for a tool or framework to write performance and or integration tests for your web backend, have a look at k6 first.
By virtue of using an embeded javascript engine, k6 is more analogous to your current or eventual front end.
But this engine, though awesomely performant, has limitations. `k9` is for such situations where you are blocked by one of those limitations.

## What's with the name?

Ha. `k6`, `k9`... nothing much beyond that.
The cocky developer in me wanted to call it `k60` when I did—and I do have to stress this—*the very bare minimum* investigation into `k9`'s performance.
Now that I've writen this, I'm sure to summon out of my code all maner of performance nightmares.

As a secondary concideration for `9` over anything else, is the eventual desire for al logo (depicting a dog, OF COURSE).
Which brings us nicely to:

## Contributing

This software is provided under a permissive license, and so will any contributions made to it.
If you would like to submit feedback, ideas/suggestions, or even code changes, please submit an issue.
I will do my best to review them, in a discretionay fasion... then integrate / adapter as necessary!
