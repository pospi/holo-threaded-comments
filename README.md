# Holochain threaded comments

A Holochain zome intended to run as a standalone DHT for managing comments.

Comments can be made against any entry from any other DHT, or indeed for any other universally unique identifier (for example, a URL). The module makes the assumption that anything being commented on must already exist.

Comments can also be made in reference to other comments, simply by providing the originating comment ID as the base. This results in a threaded comments system.


## Commands

We use a node `package.json` to wrap up the commands needed to run this module. You don't need to use it, you can run the commands defined therein yourself if you do not wish to install nodejs.

- `npm install` to ensure all dependencies are configured. Refer to the Holochain docs for prerequisite steps.
- `npm run build` compiles the zome DNA for execution
- `npm start` to execute a local development server for running the zome
- `npm test` to run all tests


## Deploying

Generally, you will want to configure this hApp using a conductor configuration which runs multiple zomes. Instructions for doing so will evolve as the Holochain platform does.


## License

MIT


## TODO

- Package final module for consumption based on Holochain/Rust guidelines
- Find an appropriate legal entity to hold the license
