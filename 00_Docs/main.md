
| Folder     | Purpose                         | What Goes Inside                                                         | Responsibility *    | 
| ---------- | ------------------------------- | ------------------------------------------------------------------------ |---------------------| 
| `config`   | Application configuration       | Config structs, config loaders, environment variables, YAML/TOML parsing |                       |
| `models`   | Data structures (pure data)     | Structs representing system stats, processes, configs                    | Data representation |
| `services` | Business logic                  | Code that gathers system stats, reads `/proc`, interacts with OS         | system logic        |
| `handlers` | Entry points / command handlers | CLI command logic (`syswatch`, `watch`, `top`)                           | user input          |
| `utils`    | Reusable helper functions       | formatting, time conversion, logging helpers                             | helper              |
| `errors`   | Error types                     | Custom error enums and error conversions                                 | Error Handglin      |

 # WORKFLOW: 
> Process

```bash
CLI command
     ↓
handler
     ↓
service
     ↓
model
     ↓
formatter
     ↓
output

```

> Files

```bash
main.rs
   ↓
cli_handler.rs
   ↓
system_collector.rs
   ↓
SystemStats struct
   ↓
formatter.rs
   ↓
terminal output
```

```rs
fn useme(){

}
```
