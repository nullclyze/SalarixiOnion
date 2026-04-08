# Application

This is a section that contains the main application logic - APIs, services, computational functions, configs.

This section also contains two **subsections**:

- `./src/api`: API logic, transferring data from the frontend to the service and vice versa
- `./src/service`: Service logic, bot creation, data storage, processing, and so on

## Map

|       Relative Path         |                   Description                 |
|-----------------------------|-----------------------------------------------|
| `./icons`                   | App icons                                     |
| `./src/api`                 | API logic                                     |
| `./src/service/core`        | Core of the service, the main logic of bots   |
| `./src/service/generators`  | Helper functions for generation               |
| `./src/service/quick`       | Quick task logic (hotkey handling)            |
| `./src/service/rpc`         | Discord RPC logic                             |
| `./src/service/script`      | Shell for a custom script                     |
| `./src/service/tools`       | Various tools                                 |
| `./src/service/webhook`     | Webhook sending logic                         |