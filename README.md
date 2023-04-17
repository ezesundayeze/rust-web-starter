# Rust Web Starter Application with Rocket and MongoDB

## Features

- [x] CRUD 
- [x] Authentication
    - [x] login
    - [x] Sign up
    - [x] JWT token encode/decode


## Setup

This app requires the user to create a `.env` file in the root directory of the project and add the following environment variables:

* `MONGODB_URI` - The URI for the MongoDB database, e.g. `mongodb://localhost:27017`
* `DB_NAME` - The name of the database
* `JWT_SECRET` - A secret key for JWT token generation
* `JWT_ALGORITHM` - The algorithm used for JWT token generation

To run the app, follow these steps:

1. Clone the repository or download the source code
2. Navigate to the root directory of the project in the terminal
3. Create the `.env` file and add the required environment variables as described above
4. Run `cargo run` in the terminal
5. The app should now be running on `http://localhost:8000`

If there are any errors, make sure that the environment variables are correctly set up and that the MongoDB database is running on the specified URI.
