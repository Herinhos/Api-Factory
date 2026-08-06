from typing import Any

import msgspec
from robyn import Request, Response, SubRouter

from api.models import UserCreate, UserResponse
from api.schemas import User

router = SubRouter("/users")


@router.post("/")
async def create_user(request: Request):
    # step 1 - validate incoming JSON with msgspec
    try:
        user_data: UserCreate = msgspec.json.decode(request.body, type=UserCreate)
    except msgspec.ValidationError as e:
        return Response(
            status_code=400, headers={"Content-Type": "text/plain"}, description=str(e)
        )

    # step 2 - insert using Piccolo ORM
    try:
        # Piccolo insert method is async and returns the inserted rows
        new_user: list[dict[str, Any]] = await User.insert(
            User(username=user_data.username, age=user_data.age)
        ).returning(User.id, User.username, User.age)

        inserted_record: dict[str, Any] = new_user[0]

    except Exception as e:  # noqa: BLE001
        # catching generic DB exceptions
        return Response(
            status_code=409,
            headers={"Content-Type": "text/plain"},
            description=f"User creation failed (likely duplicate). Exception: {e}",
        )

    # 3. Fast Serialization with msgspec
    response_data = UserResponse(
        id=inserted_record["id"],
        username=inserted_record["username"],
        age=inserted_record["age"],
    )

    return Response(
        status_code=201,
        headers={"Content-Type": "application/json"},
        description=msgspec.json.encode(response_data).decode("utf-8"),
    )


@router.get("/:id")
async def get_user(request: Request):
    user_id: str | None = request.path_params.get("id")
    if user_id is not None:
        user_id: int = int(user_id)

    # Piccolo query: SELECT id, username, email FROM users WHERE id = X
    user: dict[str, Any] | None = (
        await User.select(User.id, User.username, User.age)
        .where(User.id == user_id)
        .first()
    )

    if not user:
        return Response(
            status_code=404,
            headers={"Content-Type": "text/plain"},
            description="User not found",
        )

    response_data = UserResponse(
        id=user["id"], username=user["username"], age=user["age"]
    )

    return Response(
        status_code=200,
        headers={"Content-Type": "application/json"},
        description=msgspec.json.encode(response_data).decode("utf-8"),
    )
