from msgspec import Struct


class UserCreate(Struct):
    username: str
    age: int


class UserResponse(Struct):
    id: int
    username: str
    age: int
