from enum import Enum

class Color(Enum):
    BLACK = (33, 33, 33)
    GRAY = (128, 128, 128) 
    WHITE = (222, 222, 222)
    GREEN = (34, 185, 67)
    RED = (185, 34, 67)
    BLUE = (34, 67, 185)
    ORANGE = (181, 100, 0)
    BROWN = (48, 34, 26)

    def lerp(self, other, t):
        r = self.value[0] + (other.value[0] - self.value[0]) * t
        g = self.value[1] + (other.value[1] - self.value[1]) * t
        b = self.value[2] + (other.value[2] - self.value[2]) * t
        return (r, g, b)
    