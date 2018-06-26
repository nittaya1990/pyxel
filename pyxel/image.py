from .glwrapper import GLTexture


class Image:
    def __init__(self, width, height):
        self._width = width
        self._height = height
        self._tex = GLTexture(width, height, 1, nearest=True)
        self._data = self._tex.data

    @property
    def width(self):
        return self._width

    @property
    def height(self):
        return self._height

    @property
    def data(self):
        self._tex.update()
        return self._data

    def set(self, x, y, width, height, data):
        self._data[y:y + height, x:x + width] = [
            list(map(lambda x: int(x, 16), line)) for line in data
        ]
        self._tex.update()

    def save(self):
        # todo
        pass

    def load(self):
        self._tex.update()
        # todo
