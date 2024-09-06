# <img height="50" alt="limestone" src="https://github.com/user-attachments/assets/53b09445-dba6-43c4-9bdf-f4df9ab677a3"/>

Program ID: `LMSToZQenurAeAutm239hcJBCgsaPNaJhNC7nJhrtdB`

> [!NOTE]
> The program will be made immutable once it is deployed to mainnet.

## Building

To build the program, run the following command from the root of the project:

```bash
pnpm programs:build
```

This will create a `.so` in the `target/deploy` directory. You will need to compile the program and restart the test validator every time a new change is introduced.

## Testing

You may run the following command to build the program and run its Rust tests:

```bash
pnpm clients:rust:test
```

For the JavaScript tests, run the following command:

```bash
pnpm clients:js:test
```