from robyn import Request, Robyn

from api.schemas import User

app = Robyn(__file__)

# Application Startup
@app.startup_handler
async def startup_handler():
    await User.create_table(if_not_exists=True)

# Health Check and Greetings
@app.get("/")
async def greetings(request: Request):
    return "Hello World from Robyn!"


if __name__ == "__main__":
    app.start(port=8000)
