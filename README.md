# pathogen
just another general purpose discord bot

---
### running pathogen
**what you need**
- postgres
- redis
- rust + cargo
- discord bot token
- [just](https://github.com/casey/just)

**running the bot**
1. setup `.env` (refer to example file)
2. build with `just run release`

---
### TODO:
[ ] Use redis for caching users
[ ] Starboard plugin
[ ] Moderation plugin (ban, kick, warn, strikes)
[ ] Log plugin
