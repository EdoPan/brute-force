# Brute Force Rust
A simple Rust brute force script.
## How to run the script
In order to run the script you have to:
* Compile the project: ```cargo build```
* Run the script: ```cargo run```
* Follow the instructions

## Modes
Once the URL and the endpoint have been set is possible to choose between three modes:
* Single Mode: Insert manually username and password.
* Fixed Mode: Insert manually username and a file.txt for the passwords. All the passwords will be iterated.
* Advanced Mode: Insert a file.txt for the usernames and a file.txt for the passwords. Every username will be iterated with all the available passwords.

The script ends once the credentials are found or the files have been completely parsed.
> **Example of URL and endpoint:** <br>
> "Insert URL: http://localhost:3000" <br>
> "Insert endpoint: /login"

## Requests
The POST operation has been implemented with the [reqwest](https://docs.rs/reqwest/latest/reqwest/) crate. The payload for the request is in JSON format. The request is performed inside the `make_request` method. All the requests have been tested with [Mockoon](https://mockoon.com/).
> **Example of the JSON payload:** <br>
> {"username":"some_username", "password":"some_password"}

A GET request was performed inside the `get_client` method in order to retrieve and set the session cookie for the client instance.
## Performance
The Single Mode obviously is not efficient due to the fact that the user needs to insert the credentials manually. The Fixed Mode should not have performance issues. About the Advanced Mode, here the performance degrades due to the fact that the password's file.txt needs to be parsed and subsequently a vector with all the passwords is created (this is not a problem for small files).