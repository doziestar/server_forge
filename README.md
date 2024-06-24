# ServerForge

ServerForge is a robust, customizable server setup and maintenance tool written in Rust. It's designed to automate the process of configuring and deploying servers across multiple Linux distributions, with support for containerization and advanced security features.

## Features

- Multi-distribution support (Ubuntu, CentOS, Fedora)
- Modular architecture for easy customization and extension
- Containerization support with Docker and Kubernetes options
- Advanced security measures implementation
- Automatic system updates configuration
- Monitoring setup with Prometheus and Grafana
- Backup system configuration
- Application deployment (traditional and containerized)
- Rollback capability for all major operations

## Prerequisites

- Rust 1.54 or higher
- Root access on the target Linux system

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/doziestar/server_forge.git
   ```

2. Navigate to the project directory:
   ```
   cd server_forge
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run ServerForge with root privileges:

```
sudo ./target/release/server_forge
```

Follow the interactive prompts to configure your server. ServerForge will ask for information such as:

- Linux distribution
- Server role
- Security level
- Monitoring preferences
- Backup frequency
- Update schedule
- Containerization preferences
- Applications to deploy

## Modules

ServerForge is composed of the following modules:

- `main.rs`: The entry point of the application, orchestrating the setup process.
- `config.rs`: Defines the configuration structure for the server setup.
- `utils.rs`: Contains utility functions used throughout the application.
- `setup.rs`: Handles initial system setup and essential package installation.
- `security.rs`: Implements security measures and configures security tools.
- `updates.rs`: Sets up automatic system updates.
- `monitoring.rs`: Configures monitoring tools like Prometheus and Grafana.
- `backup.rs`: Sets up the backup system.
- `deployment.rs`: Handles traditional application deployment.
- `containerization.rs`: Manages Docker and Kubernetes setup and container deployment.
- `rollback.rs`: Provides rollback functionality for all major operations.
- `distro.rs`: Handles distribution-specific operations and package management.

## Customization

ServerForge is designed to be easily customizable. To add or modify functionality:

1. Locate the relevant module file (e.g., `security.rs` for security features).
2. Add or modify functions as needed.
3. Update the `main.rs` file if you've added new high-level functionality.

## Contributing

Contributions to ServerForge are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Write your code and tests.
4. Submit a pull request with a clear description of your changes.

## License

ServerForge is released under the MIT License. See the LICENSE file for details.

## Disclaimer

ServerForge is a powerful tool that makes significant changes to your system. Always use it in a testing environment first and ensure you have backups before running it on a production server.

## Support

For bug reports and feature requests, please open an issue on the GitHub repository.

## Acknowledgments

ServerForge was inspired by the need for a flexible, cross-distribution server setup tool in the Rust ecosystem. Special thanks to the Rust community and the developers of the libraries used in this project.