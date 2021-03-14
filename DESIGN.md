The code for this bot is structured a bit differently than I normally do it. The bot is intended to be big and have a large feature set so I figured this would ultimately be a more organized way of structuring the code.

## Plugins
<details open>
  <summary><code>src/plugins</code></summary>
  Pathogen "plugins" are the main features of the bot. They range from just commands to an entire chat XP leveling system.

  A typical plugin will be structured like this:

  ```
    plugins/
    └── plugin_name/
        ├── cmds/
        │   ├── mod.rs
        │   └── your_command.rs
        ├── handlers/
        │   ├── mod.rs
        │   └── your_handler.rs
        ├── mod.rs
        └── README.md
  ```

  A breakdown of the files and directories is as follows:
  <details>
    <summary><code>mod.rs</code></summary>
    The <code>mod.rs</code> file in a plugin's directory is where plugin-specific structures are defined. This is where database models, guild config options and the like are defined. It also acts as an entry point to the <code>cmds</code> modules.
  </details>

  <details>
    <summary><code>README.md</code></summary>
    Each plugin should have a README with an explanation of what the plugin does, what commands theres are, what handlers there are and what is on the roadmap for the plugin.
  </details>

  <details>
    <summary><code>cmds/</code></summary>
    The <code>cmds</code> directory holds all of the commands for a plugin. Refer to the <a href="https://github.com/Benricheson101/pathogen/blob/dev/src/plugins/meta/cmds/ping.rs"><code>ping</code></a> command for an example. All commands should have their own file, unless they are <i>very</i> similar or sub commands (think <code>strike search</code> and <code>strike delete</code>).
    <br>
    <br>
    <code>cmds/mod.rs</code> is where the framework group struct is defined and configured. Refer to the <a href="https://github.com/Benricheson101/pathogen/blob/dev/src/plugins/meta/cmds/mod.rs"><code>meta</code></a> plugin as an example. It should also export all of the command <i>functions</i>, not command <i>modules</i>.
  </details>

  <details>
    <summary><code>handlers/</code></summary>
    Handlers are used if the plugin needs to have code run when the bot receives a gateway event.
    <br>
    <br>
    The files should be named by what data they process (if a handler is used in <code>reaction_add</code>, the file should be called <code>reaction.rs</code>).
    <br>
    <br>
    Exported functions should be named to match <a href="https://docs.rs/serenity/0.10.4/serenity/client/trait.EventHandler.html">Serenity's EventHandler event names</a>, prefixed with <code>on_</code> (so a <code>reaction_add</code> handler function would be named <code>on_reaction_add</code> and placed in <code>reaction.rs</code>). Related handlers should be placed in the same file.
  </details>
</details>

## Database
<details open>
  <summary><code>src/db</code></summary>
  The bot uses <a href="https://www.postgresql.org/">PostgreSQL</a> and <a href="https://redis.io/">Redis</a> as its primary means of data storage. The <code>PathogenDb</code> trait has all of the database interaction methods. Data should be inserted into Redis and queried first before querying the main database to maintain performance.
  <br>
  <br>
  Note to anyone self-hosting: If you would like to use a different kind of database, implementing the <code>PathogenDb</code> trait and error types will allow drop-in replacement of the database.
</details>
