# Substrate ERC20

A Parity Substrate SRML module for _single_ ERC20 token. Only one token is implemented as a runtime and is initialized using a custom genesis config.

For multi token ERC20 runtime, check out [substrate-erc20-multi](https://github.com/gautamdhameja/substrate-erc20-multi)

**Important Note:** The Substrate framework and related libraries and APIs are rapidly under development. In case this module does not work with the latest Substrate, apart from submiting an issue in this repo, please try to port the runtime module into a freshly cloned substrate-node-template code. The overall concepts should still be the same but the syntax and setup might change.