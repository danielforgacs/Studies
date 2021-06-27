import time
import asyncio

async def get_4():
    print('.get_4')
    await asyncio.sleep(1)
    return 4


async def main():
    print('.main')
    task_a = asyncio.create_task(get_4())
    task_b = asyncio.create_task(get_4())
    task_c = asyncio.create_task(get_4())

    a = await task_a
    b = await task_b
    c = await task_c

    print(a+b+c)


# DIRECT AWAIT DOES NOT WORK:
async def main2():
    print('.main')

    a = await asyncio.create_task(get_4())
    b = await asyncio.create_task(get_4())
    c = await asyncio.create_task(get_4())

    print(a+b+c)


async def main3():
    tasks = []
    for _ in range(10):
        tasks += [asyncio.create_task(get_4())]
    results = 0
    for t in tasks:
        results += await t
    print(results)



# asyncio.run(main())
# asyncio.run(main2())
asyncio.run(main3())
