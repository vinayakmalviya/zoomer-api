# Zoomer API

> A simple API to manage meeting rooms on your own.

[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#) [![Postgres](https://img.shields.io/badge/Postgres-%23316192.svg?logo=postgresql&logoColor=white)](#) [![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff)](#)

The rooms are divided into 2 categories: Available and Active

You can occupy, freeup and join any added rooms.

There is also a UI consuming this API: [zoomer-ui](https://github.com/vinayakmalviya/zoomer-ui)

![Zoomer cover image](https://github.com/vinayakmalviya/zoomer-api/blob/main/images/github-cover.svg?raw=true)

## Models

1. Room

Defines a room in the default state (available)

```rust
struct Room {
    id: Uuid,
    name: String,
    room_id: String,
    capacity: i32,
    time_limit: String,
    link: String,
    comments: String,
}
```

2. Active Room

Defines a room actively being used in a meeting

```rust
struct ActiveRoom {
    id: Uuid,
    name: String,
    room_id: String,
    capacity: i32,
    time_limit: String,
    link: String,
    comments: String,
    is_active: bool,
    occupied_until: DateTime<Utc>,
    meeting_title: String,
    meeting_comments: String,
}
```

3. Occupancy

Defines the details of a meeting

```rust
struct Occupancy {
    id: i32,
    occupied_room_id: Uuid,
    occupied_until: DateTime<Utc>,
    meeting_title: String,
    comments: String,
}
```

## Endpoints

1. `/rooms`

    Sends the current state of rooms as the response. Current state includes a list of active and available rooms

    Method: `GET`

    Response:

    ```
    {
        "active_rooms": Vec<ActiveRoom>,
        "available_rooms": Vec<Room>
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 500 | `InternalServerError` |

2. `/rooms/available`

    Sends a list of available rooms as the response.

    Method: `GET`

    Response:

    ```
    {
        "available_rooms": Vec<Room>
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 500 | `InternalServerError` |

3. `/rooms/active`

    Sends a list of active rooms as the response.

    Method: `GET`

    Response:

    ```
    {
        "active": Vec<ActiveRoom>
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 500 | `InternalServerError` |

4. `/rooms/{id}`

    Sends the details for the room which matches the specified `id`.

    Method: `GET`

    Response:

    ```
    {
        "room_details": Room
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 404 | `RoomNotFoundError` |
    | 500 | `InternalServerError` |

5. `/rooms/occupancies`

    Sends all the occupancies currently active.

    Method: `GET`

    Response:

    ```
    {
        "occupancies": Vec<Occupancy>
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 500 | `InternalServerError` |

6. `/rooms/new`

    Add a new room to the database. This new room will be available by default.

    Method: `POST`

    Payload:

    ```
    {
        "name": String,
        "room_id": String,
        "capacity": i32,
        "time_limit": u64,
        "link": String,
        "comments": String,
    }
    ```
    Response:

    ```
    {
        "room_details": Room
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 403 | `RoomWithNameExistsError` |
    | 403 | `RoomWithIdExistsError` |
    | 500 | `InternalServerError` |

7. `/rooms/edit/{id}`

    Edit an existing room's details.

    Method: `POST`

    Payload:

    ```
    {
        "name": String,
        "room_id": String,
        "capacity": i32,
        "time_limit": u64,
        "link": String,
        "comments": String,
    }
    ```

    Response:

    ```
    {
        "room_details": Room
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 404 | `RoomNotFoundError` |
    | 500 | `InternalServerError` |

8. `/rooms/occupy`

    Occupy an available room using this endpoint.

    Method: `POST`

    Payload:

    ```
    {
        "occupied_room_id": Uuid,
        "occupied_until": DateTime<Utc>,
        "meeting_title": String,
        "comments": String,
    }
    ```

    Response:

    ```
    {
        "room_details": ActiveRoom
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 404 | `RoomNotFoundError` |
    | 403 | `RoomOccupiedError` |
    | 500 | `InternalServerError` |

9. `/rooms/freeup/{id}`

    Frees up an already occupied room.

    Method: `GET`

    Response:

    ```
    {
        "success": true,
        "message": "Room freed up successfully"
    }
    ```

    Possible error codes:

    | http code | error |
    | ---- | ---- |
    | 404 | `RoomNotFoundError` |
    | 403 | `RoomNotOccupiedError` |
    | 500 | `InternalServerError` |

**Note**: Every `POST` request with incomplete/invalid payload will send a 403 Bad request as the response

### Error messages

| error | message |
| ---- | ---- |
| `RoomNotFoundError` | Requested room does not exist |
| `RoomWithNameExistsError` | Room with same name exists |
| `RoomWithIdExistsError` | Room with same room id exists |
| `RoomOccupiedError` | Room is already occupied, check selected room |
| `RoomNotOccupiedError` | Room is not occupied, check selected room |
| `InternalServerError` | Internal server error |

## Getting started

To setup the API on your local/server, use the following steps:

1. Clone the repository
2. Create a `.env` file with the following variables
   ```sh
   DB_STRING=<url for your postgres db>
   RUST_LOG=<debug/info> # for logging level
   PORT=<set a port if you want to use something other than 4000>
   ```

3. Start the server using `cargo run`

To reload the server on every save,

```sh
cargo watch -q -c -w src/ -x run
```

## License

Distributed under the MIT license. See `LICENSE` for more information.
