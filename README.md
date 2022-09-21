# Introdution 
A telegram bot that can search near your position the best fuel station in italy (order by price) and show the price of the fuel in the station.

# Installation and Execution
## Requirements
- Rust or Docker

### Rust
- clone the repository
- set the environment variable `TELOXIDE_FUEL_TOKEN` with your telegram bot token
- run `cargo run`

### Docker
- clone the repository
- run the commend `docker run -e TELOXIDE_FUEL_TOKEN=XXXX -v absolute_local_path:absolute_docker_path tag

# Usage
- send a random message to the bot to start the conversation
- the bot will ask you to send your position
- send your position
- at this the bot ask you to send the radius of the search
- send the radius of the search
- the bot will send you the list of the fuel id near your position
- send the id of the fuel or 0 to exit
- now the bot will send you the list of the fuel station near your position ordered by price

# Where the data come from
The data come from the [MISE(Ministero dello Sviluppo Economico) plattaform for the fuel price](https://carburanti.mise.gov.it/)

# Try me 
You can try the bot by searching @langePriceFuelBot or at this [link](https://t.me/langePriceFuelBot)

## Contributing
If you want to contribute to the project you can open a pull request or open an issue.

## Contact
If you want to contact me you can open an issue on the repository.

## Thanks
Thanks to [@tambup](https://github.com/Tambup) for the help with the dockerfile

## Licence

#### DON'T BE A DICK PUBLIC LICENSE

> Version 1.1, December 2016

> Copyright (C) [2022] Langellotti Christian

Everyone is permitted to copy and distribute verbatim or modified
copies of this license document.

> DON'T BE A DICK PUBLIC LICENSE
> TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

1. Do whatever you like with the original work, just don't be a dick.

   Being a dick includes - but is not limited to - the following instances:

 1a. Outright copyright infringement - Don't just copy this and change the name.
 1b. Selling the unmodified original with no work done what-so-ever, that's REALLY being a dick.
 1c. Modifying the original work to contain hidden harmful content. That would make you a PROPER dick.

2. If you become rich through modifications, related works/services, or supporting the original work,
share the love. Only a dick would make loads off this work and not buy the original work's
creator(s) a pint.

3. Code is provided with no warranty. Using somebody else's code and bitching when it goes wrong makes
you a DONKEY dick. Fix the problem yourself. A non-dick would submit the fix back.


