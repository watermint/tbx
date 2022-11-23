# tbx

This project aims to aggregate valuable tools and provide a single,
executable binary like [watermint toolbox](https://toolbox.watermint.org).
The author is currently in the process of learning Rust, and for this reason,
no practical tools will be implemented in the next dozen months.

## Non-goals

* Performance (this is often trade-off to ease-of-development)
* Minimal footprints
* Compatibility (because this project is experimental)

## Target platform

This project will be developed and tested on the following platforms.
It may work on other platforms but is not supported.
For example, in the case of Linux, it will most likely work on non-Ubuntu or earlier versions of Ubuntu,
but we will not be able to provide support for any problems that may arise.

* Windows 10 or above
* macOS latest
* Ubuntu 22.04 or above

# Package structure

* *tbx_essentials*: The essential libraries. 
* *tbx_foundation*: The framework of the application.
* *tbx_model*: Domain model.
* *tbx_operation*: Business logic.
* *tbx*: The main application launcher.

