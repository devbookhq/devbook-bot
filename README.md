# devbook-bot

[Community](https://en.wikipedia.org/wiki/Community) [discord](https://discord.com) [bot](https://en.wikipedia.org/wiki/Bot) for [Devbook](https://usedevbook.com/) [discord server](https://discord.gg/ypuZfadw8H)



### No build time optimisation
**Space:** 8.7M	target/release/devbook-bot \
**CPU:** 0.2% (of 8192mb)\
**Memory on RAM:** 2.3 MiB 


### Build time optimisation
**Space:** 6.0M	target/release/devbook-bot \
**CPU:** 0.1% *or* 0.0% **:O** (of 8192mb) \
**Memory on RAM:** 2.3 MiB \
**Extra's in Cargo.toml:**

```toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

