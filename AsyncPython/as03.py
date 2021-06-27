import asyncio
import time

async def say_after(delay, what):
    print('.say_after')
    await asyncio.sleep(delay)
    print(what)


async def main():
    task1 = asyncio.create_task(
        say_after(5, 'hello'))

    task2 = asyncio.create_task(
        say_after(5, 'world'))

    task3 = asyncio.create_task(
        say_after(5, 'world'))

    task4 = asyncio.create_task(
        say_after(5, 'world'))

    print(f"started at {time.strftime('%X')}")

    # Wait until both tasks are completed (should take
    # around 2 seconds.)
    await task1
    await task2
    await task3
    await task4

    print(f"finished at {time.strftime('%X')}")


asyncio.run(main())
