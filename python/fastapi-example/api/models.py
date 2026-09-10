from sqlalchemy import BigInteger, Boolean, Integer, String
from sqlalchemy.orm import Mapped, mapped_column

from api.database import Base


class Users(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(
        BigInteger, primary_key=True, index=True, nullable=False, autoincrement=True
    )

    name: Mapped[str] = mapped_column(String(50), nullable=False, index=True)

    email: Mapped[str] = mapped_column(String(30), nullable=False)

    ranking: Mapped[int] = mapped_column(Integer, nullable=True)

    is_admin: Mapped[bool] = mapped_column(Boolean, nullable=False)
