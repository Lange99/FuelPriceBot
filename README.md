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
- run the commend `docker run -e TELOXIDE_FUEL_TOKEN=<your token> -v absolute_local_path:absolute_docker_path tag

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

## Contributing
If you want to contribute to the project you can open a pull request or open an issue.

## Contact
If you want to contact me you can send me an email to [christianlangellotti@gmail.com] or open an issue on the repository.

## Thanks
Thanks to @tambup for the help with the dockerfile


