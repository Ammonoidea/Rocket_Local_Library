# What Is This

This repository is an attempt to use the [Rocket Web Framework](https://rocket.rs/) for Rust to create the example *LocalLibrary* server from  [Mozilla Developer Network](https://developer.mozilla.org/en-US/docs/Learn/Server-side).

# Why

I want to learn more about Rust, and I have followed the above MDN tutorial several times in different programming languages. It seemed like a fun exercise, particularly when I discovered the Rocket framework, which seemed fairly easy to use. 

# Running

Running `cargo run` should be enough on the Rust nightly build. You can populate some test data from a script from [MDN](https://developer.mozilla.org/en-US/docs/Learn/Server-side/Express_Nodejs/mongoose#testing_%E2%80%94_create_some_items).

# Observations

Running a basic CRUD webserver on Rocket is fairly straightforward and performant. Like all small frameworks there is the normal issue that it's hard to find examples, but the examples section in Rocket is very useful as long as you make sure you're looking at the same version/tag (this confused me for some time). There remain a few issues with MongoDb (see below), but overall there's nothing that seems much harder than the Django or Express frameworks MDN offers. 

I admit this is my first large Rust project, I have no idea if I am doing this in the correct manner. This is a learning exercise for me.

# Issues Remaining 

This repo is incomplete, it is not currently the full example LocalLibrary app from MDN. Current progress is equivalent to Chapter 5 of the Express version in [this section](https://developer.mozilla.org/en-US/docs/Learn/Server-side/Express_Nodejs/Displaying_data/Author_list_page).

* Using Mongo in Rocket: There remain several issues here. In particular I think there's a subtle interaction between the async functions in Rocket and the async MongoDb connections, relating to [this issue](https://docs.rs/mongodb/latest/mongodb/#warning-about-timeouts--cancellation) in the Rust MongoDb docs.
	Rocket doesn't come in with built in connections to MongoDb since it doesn't use the Rust r2d2 database thread pool mechanism. So I am connecting directly which seems incorrect. I am not sure how to resolve this issue, there must be an easier way. 
* State management in Rocket: Right now I load up the MongoDb collections as state in Rocket. I am not sure if this is the correct way to do it. Should they be Rocket Fairings?