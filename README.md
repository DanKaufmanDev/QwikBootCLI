# QwikBoot CLI

A lightning fast command line tool for bootstrapping modern web, mobile, and desktop applications with zero configuration. Create production ready project templates in seconds!

## Features
- **15+ Project Templates** - From simple HTML to Desktop applications
- **Zero Configuration** - Get started immediately with pre-configured setups
- **Auto Updater** - built in updater to always use the latest most up to date templates
- **Safe & Secure** - Downloads from official sources 
- **Cross Platform** - Works on Windows, macOS, and Linux

## Quick Start

### Installation

MacOS or Linux 
```bash 
curl -fsSl https://raw.githubusercontent.com/DanKaufmanDev/QwikBootCLI/main/install.sh | sh
```
Windows
```powershell
irm https://raw.githubusercontent.com/DanKaufmanDev/QwikBootCLI/main/install.ps1 | iex
```

## Usage
```bash
# Simply type qwikboot from your terminal
qwikboot

# Direct setup
--template: <template_name> <project_name>

# Example
qwikboot --template react my_react_project
```
## Additional Flags
```bash
# Show help information  
--help | -h

# Display the current version installed 
--version | -v

# Update to the latest version 
--update | -u

# Direct setup 
--template <template_name> <project_name>

# Lists all templates
--list | --templates

# Fetch community made templates
--plugins | -p <https://github.com/USERNAME/REPO>
```


## Project Creation Process 
### 1. Interactive Setup
when you run `qwikboot` you'll be guided through:

1. Project name input - Enter the name of the directory 
2. Template Selection - Select a template using your arrow keys to navigate and enter to confirm 
3. Configuration Options - Some Languages may have additional configuration options
4. Auto Installation - Dependencies are automatically installed 

### 2. Template Features 

- **Pre-Configured build tools**
- **Development server setup**
- **Git repository initialization**
- **Essential dependencies pre-installed**
- **README and documentation**
- **Proper project structure**

### 3. Post Creation Instructions
after creation you'll see:

- Project location
- Development commands to start coding
- Next steps for your specific framework

## Auto Update System 
QwikBoot includes a built in update mechanism that:
 
 - Detects your OS
 - Downloads the latest installer from GitHub
 - Executes the update safely 
 - clean up any orphaned files

 ```bash
 # Update to the latest version
 qwikboot --update
 ```

## Supported Project Types
- Static HTML CSS JavaScript
- React 
- Vue
- Svelte 
- SvelteKit
- Angular 
- Next.js
- Nuxt.js
- Vite Vanilla

- Express.js 
- Nest.js
- Remix
- Fastify

- React Native

- Electron
- Tauri

- Go
- Python
- Rust

 ## Support 
 
 - **Issues:** [Known issues](https://github.com/DanKaufmanDev/QwikBootCLI/issues)
 - **Bug Report:** [Found a bug?](https://github.com/DanKaufmanDev/QwikBootCLI/issues/new?labels=bug)

 #
QwikBoot - Bootstrap your next project in seconds!
 #

