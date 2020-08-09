import unittest
import unittest.mock
import youtube_dl


youtube_dl = unittest.mock.Mock()

ytdl = youtube_dl.YoutubeDL()
result = ytdl.extract_info(
    url='FIQ2F3T1ydM',
    download=False,
    process=False,
)


youtube_dl.extract_info.return_value = 123
print(result)
print(dir(result))
print(result.return_value)
print(result.return_value())


# assert 123 == result.return_value
