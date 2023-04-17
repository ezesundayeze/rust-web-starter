# Rust Web Starter Application with Rocket and MongoDB

This project provides a starter application for building web projects with Rust. It includes the Rocket web framework and MongoDB database integration, as well as several commonly-used libraries and configurations to get you up and running quickly.

With this starter application, you can focus on building the core features of your project, without worrying about setting up the basic infrastructure from scratch.

## Features

### CRUD 

It provides basic CRUD (Create, Read, Update, and Delete) operations for managing user data. Specifically, it includes the following functionality:

- [x] Update user
- [x] Create user
- [x] Delete user
- [x] List users

### Authentication

It includes a user authentication system that allows users to create an account, log in, and log out. Specifically, it includes the following functionality:

- [x] login
- [x] Sign up
- [x] JWT token encode/decode
- [x] Middleware

### Email (Coming soon)

The application includes functionality for sending emails with templates, although this feature is currently under development.

### Robust Error handling (Coming soon)

The application includes robust error handling to ensure that any errors that occur are handled in a graceful manner. This helps to prevent unexpected errors from causing the application to crash or malfunction.




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
