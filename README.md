## Dependencies

#### Install Diesel
```
cargo install diesel_cli --no-default-features --features postgres
```

#### Install Cargo watch 

```
cargo install cargo-watch
```

## Running the api

#### Run docker compose

```
docker compose up
```

#### Diesel Migration

```
diesel migration run
```

#### Run Rocket

```
cargo watch -x run
```

## Requesting API

#### localhost:8000/users

```js
//GET

//Returns list of all users
[
    {
        "id": 1,
        "name": "Lucas",
        "location": "Brazil",
        "title": "Dev"
    },{
        "id": 2,
        "name": "John",
        "location": "USA",
        "title": "Fullstack"
    },
]
```

#### localhost:8000/users

```js
//POST
{
    "name": "Marvin",
    "location": "Mars",
    "title": "Alien"
}

//Returns created user

{
    "id": 3
    "name": "Marvin",
    "location": "Mars",
    "title": "Alien"
}
```

#### localhost:8000/users/1

```js
//GET

//Returns user by id

{
    "id": 1,
    "name": "Lucas",
    "location": "Brazil",
    "title": "Dev"
}
```

#### localhost:8000/users/1

```js
//POST

{
    "name": "New Name"
}

//Returns updated user

{
    "id": 1,
    "name": "New Name",
    "location": "Brazil",
    "title": "Dev"
}
```

#### localhost:8000/users/1

```js
//DELETE

//Returns no content
```
