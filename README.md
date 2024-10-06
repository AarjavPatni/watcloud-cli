# WATcloud CLI

WATcloud CLI is a command-line interface tool written in Rust, designed to simplify the management and monitoring of your resources on the WATcloud compute cluster. This tool aggregates various commands into a single interface, allowing users to effortlessly check their quota usage, monitor the status of user daemons, and get updates on the cluster's status.

**⚠️ Note: WATcloud CLI is currently a work in progress.**

<!-- ## Prospective Features

-  **Cluster Status**: Quickly check the status of the WATcloud cluster, including which nodes are up or down and whether any nodes are under maintenance.
-  **Quota Usage**: View your disk, CPU, and memory usage quotas.
-  **Daemon Status**: Monitor the status of user daemons such as Docker rootless. -->

## Milestones
-  [x] `status` – Quickly check the status of the WATcloud cluster, including which nodes are up or down and whether any nodes are under maintenance.
-  [ ] `quota list` – View your disk, CPU, and memory usage quotas.
-  [ ] `daemon status` – Monitor the status of user daemons such as Docker rootless.

## Contributing

We welcome contributions to the WATcloud CLI project! Here’s how you can get started:

### Prerequisites

Ensure you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).

### Fork and Clone the Repository

First, fork the repository on GitHub, then clone your forked repository:

```sh
git clone https://github.com/YOUR_GITHUB_USERNAME/watcloud-cli.git
cd watcloud-cli
```

### Build the CLI

To build the CLI from the source, run the following command:

```sh
cargo build --release
```

After building, you can find the executable in the `target/release` directory. Move it to a directory in your `PATH` for easy access:

```sh
mv target/release/watcloud /usr/local/bin/
```

### Making Changes

1. **Create a new branch**: 
   ```sh
   git checkout -b feature-branch-name
   ```

2. **Make your changes**: Implement your feature or fix a bug.

3. **Commit your changes**: 
   ```sh
   git add .
   git commit -m "Description of your changes"
   ```

4. **Push to your fork**: 
   ```sh
   git push origin feature-branch-name
   ```

5. **Create a Pull Request**: Go to the original repository on GitHub and create a pull request from your fork.


<!-- ## Installation

To install the WATcloud CLI, ensure you have Rust installed on your system. Then, you can build the CLI from the source:

```sh
git clone https://github.com/AarjavPatni/watcloud-cli.git
cd watcloud-cli
cargo build --release
```

After building, you can find the executable in the `target/release` directory. Move it to a directory in your `PATH` for easy access:

```sh
mv target/release/watcloud /usr/local/bin/
``` -->

<!-- ## Usage

The WATcloud CLI provides several commands to interact with the WATcloud compute cluster. Below are some example commands and their usage:

### Check Cluster Status

To get the current status of the WATcloud cluster, use the following command:

```sh
watcloud status
```

This command will display information about which nodes are up or down and whether any nodes are under maintenance.

### Check Quota Usage

To list your current quota usage for disk, CPU, and memory, use:

```sh
watcloud quota list
```

This command will show your current usage and remaining quota. In the future, this command may also support submitting quota edit requests.

### Check Daemon Status

To check the status of user daemons such as Docker rootless, use:

```sh
watcloud daemon status
```

This command will display the status of various user daemons, helping you ensure that your environment is running smoothly. -->


## License

WATcloud CLI is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

<!-- ## Contact

For more information, visit our [website](https://cloud.watonomous.ca/get-involved/join#watcloud-cli) or contact us at support@watonomous.ca. -->

---

By simplifying the process of managing and monitoring your WATcloud resources, the WATcloud CLI aims to enhance your productivity and streamline your workflow. Happy computing!