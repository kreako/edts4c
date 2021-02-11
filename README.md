# edts4c

edts4c is a web application allowing to evaluate students according to french law.

Can be used for school "hors contrat" or individuals/family not practicing school.


* backend is written in [rust](https://www.rust-lang.org/) with [Rocket](https://rocket.rs)
* frontend is written with [Vue.js](https://vuejs.org/) with [Quasar Framework](https://quasar.dev/)
* some backend tests in python with the help of [Requests](https://requests.readthedocs.io/en/master/)
* some frontend tests in javascript with [Playwright](https://playwright.dev/)

## Screenshots

![Screenshot of students list](https://raw.githubusercontent.com/kreako/edts4c/main/screenshots/eleves.png)

![Screenshot of socle edition](https://raw.githubusercontent.com/kreako/edts4c/main/screenshots/socle-move.png)

More in [screenshots](https://github.com/kreako/edts4c/tree/main/screenshots) directory.


## Requirements

### For production

* a postgresql database
* a server able to run a rust binary


### For development

* a postgresql database
* a rust/cargo installation (See [rustup](https://rustup.rs/))
* a nodejs installation
* a [quasar-cli](https://quasar.dev/quasar-cli/installation) installation


## Setup for development

### Database

Setup a postgresql database (run once) :

```
$ podman run -d --restart=always --publish 5432:5432 -e POSTGRES_PASSWORD="password" -v ./data/:/var/lib/postgresql/data:z --name edts4c-db postgres
```

Every times after :

```
$ podman start
```

### Backend

Run :

```
$ cd backend
$ cargo run
```

### Frontend

Once :

```
$ cd frontend
$ yarn install
```


Then Run :

```
$ cd frontend
$ quasar dev
```

### Frontend tests

With backend and frontend running.

```
$ yarn test
```

### Backend tests

To run tests for the backend :

Setup (once) :

```
$ cd backend_tests
$ python3 -m venv env
$ . env/bin/activate
$ pip3 install -r requirements.txt
```

Run the tests :

```
$ py.test
```

## Release

Build in release mode:

```
$ ./release.sh
```

* Resulting frontend build is in : `frontend/dist/spa` and should be moved in a `frontend` directory at same level as `backend` binary.
* Resulting binary is in : `backend/target/release`
