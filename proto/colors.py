from enum import Enum

class Color(Enum):
    BLACK = (33, 33, 33)
    GRAY = (128, 128, 128) 
    WHITE = (222, 222, 222)
    BLUE = (33, 128, 222)
    
    O1 = (102, 6, 120)
    O2 = (61, 20, 99)
    O3 = (38, 20, 99)
    O4 = (12, 33, 105)
    O5 = (49, 110, 133)
    O6 = (31, 105, 90)
    O7 = (36, 120, 65)
    O8 = (99, 156, 59)
    O9 = (110, 138, 10)
    O10 = (163, 154, 47)
    O11 = (150, 71, 2)

    def val(self):
        return self.value if isinstance(self.value, tuple) else self

    def lerp(self, other, t):
        value = other.val() if isinstance(other, Color) else other
        r = self.value[0] + (value[0] - self.value[0]) * t
        g = self.value[1] + (value[1] - self.value[1]) * t
        b = self.value[2] + (value[2] - self.value[2]) * t
        return (r, g, b)
    
    @staticmethod
    def get(idx):
        match idx:
            case 0:
                return Color.O1
            case 1:
                return Color.O2
            case 2:
                return Color.O3
            case 3:
                return Color.O4
            case 4:
                return Color.O5
            case 5:
                return Color.O6
            case 6:
                return Color.O7
            case 7:
                return Color.O8
            case 8:
                return Color.O9
            case 9:
                return Color.O10
            case 10:
                return Color.O11
            case _:
                return Color.BLACK